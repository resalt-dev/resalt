use std::{
    error::Error,
    sync::{Arc, Mutex},
};

use actix_web::{guard::fn_guard, http::header, middleware::*, web, App, HttpServer};
use env_logger::{init_from_env, Env};
use log::info;
use resalt_config::SConfig;
use resalt_middleware::ValidateAuth;
use resalt_routes::*;
use resalt_salt::{SaltAPI, SaltEventListener, SaltEventListenerStatus};
use resalt_scheduler::Scheduler;
use resalt_storage::{StorageCloneWrapper, StorageImpl};
use resalt_storage_mysql::StorageMySQL;
use resalt_storage_redis::StorageRedis;
use tokio::task;

async fn init_db() -> Box<dyn StorageImpl> {
    let db_type = SConfig::database_type();
    let db_type = db_type.to_lowercase();
    let db_type = db_type.as_str();
    info!("Database type: {}", db_type);
    match db_type {
        "files" => {
            let path = SConfig::database_host();
            Box::new(
                resalt_storage_files::StorageFiles::connect(&path)
                    .unwrap_or_else(|_| panic!("Error connecting to {}", &path)),
            )
        }
        "mysql" => {
            let database_url = format!(
                "mysql://{}:{}@{}:{}/{}",
                SConfig::database_username(),
                SConfig::database_password(),
                SConfig::database_host(),
                SConfig::database_port(),
                SConfig::database_database()
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
                SConfig::database_username(),
                SConfig::database_password(),
                SConfig::database_host(),
                SConfig::database_port(),
                SConfig::database_database()
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

fn start_salt_websocket_thread(db: Box<dyn StorageImpl>) -> Arc<Mutex<SaltEventListenerStatus>> {
    let listener_status: Arc<Mutex<SaltEventListenerStatus>> =
        Arc::new(Mutex::new(SaltEventListenerStatus { connected: false }));
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

fn start_scheduler(db: Box<dyn StorageImpl>) {
    let mut scheduler = Scheduler::new(StorageCloneWrapper { storage: db });
    scheduler.register_system_jobs();
    scheduler.start();
}

async fn run() -> Result<(), Box<dyn Error>> {
    init_from_env(Env::new().default_filter_or("Debug"));

    // Database
    let db: Box<dyn StorageImpl> = init_db().await;
    let db_clone_wrapper = StorageCloneWrapper {
        storage: db.clone(),
    };

    // Salt WebSocket Thread
    let listener_status = start_salt_websocket_thread(db.clone());

    // Scheduler
    start_scheduler(db);

    HttpServer::new(move || {
        // Salt API
        let salt_api = SaltAPI::new();

        let guard_auth = fn_guard(guard_require_auth);

        App::new()
            .wrap(DefaultHeaders::new().add((header::X_CONTENT_TYPE_OPTIONS, "nosniff")))
            .wrap(NormalizePath::trim())
            .wrap(Logger::default())
            .service(
                web::scope("/api")
                    .app_data(web::Data::new(db_clone_wrapper.clone().storage))
                    .app_data(web::Data::new(salt_api.clone()))
                    .app_data(web::Data::new(listener_status.clone()))
                    .wrap(ValidateAuth::new(
                        db_clone_wrapper.clone().storage,
                        salt_api,
                    ))
                    .service(route_index_get)
                    .service(route_config_get)
                    .service(route_metrics_get)
                    .service(route_login_post)
                    .service(route_token_post)
                    .service(
                        web::scope("/auth")
                            .guard(guard_auth)
                            .service(route_myself_get)
                            .service(route_status_get)
                            .service(route_minions_get)
                            .service(route_minion_get)
                            .service(route_minion_refresh_post)
                            .service(route_presets_get)
                            .service(route_presets_post)
                            .service(route_preset_get)
                            .service(route_preset_put)
                            .service(route_preset_delete)
                            .service(route_grains_get)
                            .service(route_jobs_get)
                            .service(route_jobs_post)
                            .service(route_job_get)
                            .service(route_events_get)
                            .service(route_users_get)
                            .service(route_users_post)
                            .service(route_user_get)
                            .service(route_user_delete)
                            .service(route_user_password_post)
                            .service(route_user_permissions_post)
                            .service(route_user_permissions_delete)
                            .service(route_keys_get)
                            .service(route_key_accept_put)
                            .service(route_key_reject_put)
                            .service(route_key_delete_delete)
                            .service(route_permissions_get)
                            .service(route_permissions_post)
                            .service(route_permission_get)
                            .service(route_permission_update)
                            .service(route_permission_delete)
                            .service(route_settings_import_post)
                            .service(route_settings_export_get)
                            .default_service(route_fallback_404),
                    ),
            )
            // Embed web interface
            .service(web::scope("").default_service(route_frontend_get))
    })
    .bind(("0.0.0.0", SConfig::http_port()))?
    .run()
    .await?;

    Ok(())
}

pub fn start() -> Result<(), Box<dyn Error>> {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(run())
}

pub fn main() {
    start().unwrap();
}
