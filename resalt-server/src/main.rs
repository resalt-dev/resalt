use axum::{
    middleware::{from_fn, from_fn_with_state},
    routing::{delete, get, post, put},
    Router, Server, ServiceExt,
};
use env_logger::{init_from_env, Env};
use log::info;
use resalt_config::ResaltConfig;
use resalt_routes::*;
use resalt_salt::{SaltAPI, SaltEventListener, SaltEventListenerStatus};
use resalt_storage::StorageImpl;
use resalt_storage_mysql::StorageMySQL;
use resalt_storage_redis::StorageRedis;
use std::{
    error::Error,
    net::SocketAddr,
    sync::{Arc, Mutex},
};
use tokio::task;
use tower::Layer;

async fn init_db() -> Box<dyn StorageImpl> {
    let db_type = ResaltConfig::DATABASE_TYPE;
    let db_type = db_type.to_lowercase();
    let db_type = db_type.as_str();
    info!("Database type: \"{}\"", db_type);
    match db_type {
        "files" => {
            let path: String = ResaltConfig::DATABASE_HOST.clone();
            Box::new(
                resalt_storage_files::StorageFiles::connect(&path)
                    .unwrap_or_else(|_| panic!("Error connecting to {}", &path)),
            )
        }
        "mysql" => {
            let database_url = format!(
                "mysql://{}:{}@{}:{}/{}",
                *ResaltConfig::DATABASE_USERNAME,
                *ResaltConfig::DATABASE_PASSWORD,
                *ResaltConfig::DATABASE_HOST,
                *ResaltConfig::DATABASE_PORT,
                *ResaltConfig::DATABASE_DATABASE
            );
            Box::new(
                StorageMySQL::connect(&database_url)
                    .await
                    .unwrap_or_else(|_| panic!("Error connecting to {}", &database_url)),
            )
        }
        "redis" => {
            let database_url = format!(
                "redis://{}:{}@{}:{}/{}",
                *ResaltConfig::DATABASE_USERNAME,
                *ResaltConfig::DATABASE_PASSWORD,
                *ResaltConfig::DATABASE_HOST,
                *ResaltConfig::DATABASE_PORT,
                *ResaltConfig::DATABASE_DATABASE
            );

            println!("Connecting to {}", &database_url);
            Box::new(
                StorageRedis::connect(&database_url)
                    .await
                    .unwrap_or_else(|_| panic!("Error connecting to {}", &database_url)),
            )
        }
        _ => panic!(),
    }
}

fn start_salt_websocket_thread(db: Box<dyn StorageImpl>) -> SaltEventListenerStatus {
    let listener_status: SaltEventListenerStatus = SaltEventListenerStatus {
        connected: Arc::new(Mutex::new(false)),
    };
    let salt_listener_status = listener_status.clone();
    std::thread::spawn(move || {
        let rt = tokio::runtime::Runtime::new().unwrap();
        let ls = task::LocalSet::new();
        ls.block_on(&rt, async {
            // Wait a few seconds before starting SSE, so web server gets time to start
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
            let salt_ws = SaltEventListener::new(db.clone(), salt_listener_status);
            salt_ws.start().await;
        });
    });
    listener_status
}

async fn start_server(
    db: Box<dyn StorageImpl>,
    listener_status: SaltEventListenerStatus,
) -> Result<(), Box<dyn Error>> {
    let salt_api = SaltAPI::new();
    let shared_state = AppState {
        data: db,
        salt_api: salt_api.clone(),
        listener_status: listener_status.clone(),
    };

    let router_auth = Router::new()
        .route("/myself", get(route_myself_get))
        .route("/status", get(route_status_get))
        .route("/minions", get(route_minions_get))
        .route("/minions/:minion_id", get(route_minion_get))
        .route(
            "/minions/:minion_id/refresh",
            post(route_minion_refresh_post),
        )
        .route("/presets", get(route_presets_get))
        .route("/presets", post(route_presets_post))
        .route("/presets/:preset_id", get(route_preset_get))
        .route("/presets/:preset_id", put(route_preset_put))
        .route("/presets/:preset_id", delete(route_preset_delete))
        .route("/grains", get(route_grains_get))
        .route("/jobs", get(route_jobs_get))
        .route("/jobs", post(route_jobs_post))
        .route("/jobs/:jid", get(route_job_get))
        .route("/events", get(route_events_get))
        .route("/users", get(route_users_get))
        .route("/users", post(route_users_post))
        .route("/users/:user_id", get(route_user_get))
        .route("/users/:user_id", delete(route_user_delete))
        .route("/users/:user_id/password", post(route_user_password_post))
        .route(
            "/users/:user_id/permissions/:group_id",
            post(route_user_permissions_post),
        )
        .route(
            "/users/:user_id/permissions/:group_id",
            delete(route_user_permissions_delete),
        )
        .route("/keys", get(route_keys_get))
        .route("/keys/:state/:id/accept", put(route_key_accept_put))
        .route("/keys/:state/:id/reject", put(route_key_reject_put))
        .route("/keys/:state/:id/delete", delete(route_key_delete_delete))
        .route("/permissions", get(route_permissions_get))
        .route("/permissions", post(route_permissions_post))
        .route("/permissions/:id", get(route_permission_get))
        .route("/permissions/:id", put(route_permission_put))
        .route("/permissions/:id", delete(route_permission_delete))
        .route("/settings/import", post(route_settings_import_post))
        .route("/settings/export", get(route_settings_export_get))
        .route_layer(from_fn_with_state(shared_state.clone(), middleware_auth))
        .fallback(route_fallback_404);

    let router_noauth = Router::new()
        .route("/", get(route_index_get))
        .route("/config", get(route_config_get))
        .route("/metrics", get(route_metrics_get))
        .route("/login", post(route_login_post))
        .route("/token", post(route_token_post))
        .fallback(route_fallback_404);

    let app = Router::new()
        .nest("/api/auth", router_auth)
        .nest("/api", router_noauth)
        // Embed web interface
        .fallback(route_frontend_get)
        .with_state(shared_state.clone());

    // Normalize path
    let normalize_path = from_fn(middleware_normalize_path);
    let app = normalize_path.layer(app);
    // Defalt Headers
    let default_headers = from_fn(middleware_default_headers);
    let app = default_headers.layer(app);
    // Logging
    let logging = from_fn_with_state(shared_state, middleware_logging);
    let app = logging.layer(app);

    let socket = SocketAddr::from(([0, 0, 0, 0], *ResaltConfig::HTTP_PORT));
    Server::bind(&socket)
        .serve(app.into_make_service_with_connect_info::<SocketAddr>())
        .await?;

    Ok(())
}

async fn run() -> Result<(), Box<dyn Error>> {
    // Logging
    init_from_env(Env::new().default_filter_or("Debug"));

    // Database
    let db: Box<dyn StorageImpl> = init_db().await;

    // Salt WebSocket Thread
    let listener_status = start_salt_websocket_thread(db.clone());

    // Web Server
    start_server(db, listener_status).await?;

    Ok(())
}

#[tokio::main]
pub async fn main() {
    run().await.unwrap();
}
