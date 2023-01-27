use std::sync::{Arc, Mutex};

use actix_web::{http::header, middleware::*, web, App, HttpServer};
use middleware::{RequireAuth, ValidateAuth};
use resalt_config::SConfig;
use resalt_pipeline::PipelineServer;
use resalt_salt::{SaltAPI, SaltEventListener, SaltEventListenerStatus};
use resalt_storage::{StorageCloneWrapper, StorageImpl};
use resalt_storage_mysql::StorageMySQL;
use routes::*;
use tokio::task;

mod auth;
mod middleware;
mod routes;
mod scheduler;
mod update;

async fn init_db() -> Box<dyn StorageImpl> {
    match SConfig::database_type().to_lowercase().as_str() {
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
        _ => todo!(),
    }
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("Debug"));

    // SSE
    let pipeline = PipelineServer::new();

    // Database
    let db: Box<dyn StorageImpl> = init_db().await;
    let db_clone_wrapper = StorageCloneWrapper {
        storage: db.clone(),
    };

    // Salt WebSocket
    let salt_listener_pipeline = pipeline.clone();
    let salt_listener_db = db.clone();
    let listener_status: Arc<Mutex<SaltEventListenerStatus>> =
        Arc::new(Mutex::new(SaltEventListenerStatus { connected: false }));
    let salt_listener_status = listener_status.clone();
    std::thread::spawn(move || {
        let rt = tokio::runtime::Runtime::new().unwrap();
        let ls = task::LocalSet::new();
        ls.block_on(&rt, async {
            // Wait a few seconds before starting SSE, so web server gets time to start
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
            let salt_ws = SaltEventListener::new(
                salt_listener_pipeline,
                salt_listener_db,
                salt_listener_status,
            );
            salt_ws.start().await;
        });
    });

    // Scheduler
    let mut scheduler = scheduler::Scheduler::new(db_clone_wrapper.clone());
    scheduler.register_system_jobs();
    scheduler.start();

    HttpServer::new(move || {
        // Salt API
        let salt_api = SaltAPI::new();

        App::new()
            .app_data(web::Data::new(pipeline.clone()))
            .app_data(web::Data::new(db_clone_wrapper.clone().storage))
            .app_data(web::Data::new(salt_api.clone()))
            .app_data(web::Data::new(listener_status.clone()))
            .app_data(web::Data::new(scheduler.clone()))
            // Prevent sniffing of content type
            .wrap(DefaultHeaders::new().add((header::X_CONTENT_TYPE_OPTIONS, "nosniff")))
            // Removes trailing slash in the URL to make is sowe don't need as many services
            .wrap(NormalizePath::trim())
            // enable logger - always register Actix Web Logger middleware last
            .wrap(Logger::default())
            .service(
                web::scope("/api/1")
                    .wrap(ValidateAuth::new(
                        db_clone_wrapper.clone().storage,
                        salt_api,
                    ))
                    .route("/", web::get().to(route_index_get))
                    .route("/config", web::get().to(route_config_get))
                    .route("/metrics", web::get().to(route_metrics_get))
                    // auth
                    .service(
                        web::scope("/auth")
                            .route("/login", web::post().to(route_auth_login_post))
                            .route("/token", web::post().to(route_auth_token_post))
                            .service(
                                web::scope("/user")
                                    .wrap(RequireAuth::new())
                                    .route("", web::get().to(route_auth_user_get))
                                    .default_service(route_fallback_404),
                            )
                            .default_service(route_fallback_404),
                    )
                    // status
                    .service(
                        web::scope("/status")
                            .wrap(RequireAuth::new())
                            .route("", web::get().to(route_status_get))
                            .default_service(route_fallback_404),
                    )
                    // minions
                    .service(
                        web::scope("/minions")
                            .wrap(RequireAuth::new())
                            .route("", web::get().to(route_minions_get))
                            .route("/{id}", web::get().to(route_minion_get))
                            .route("/{id}/refresh", web::post().to(route_minion_refresh_post))
                            .default_service(route_fallback_404),
                    )
                    // presets
                    .service(
                        web::scope("/presets")
                            .wrap(RequireAuth::new())
                            .route("", web::get().to(route_presets_get))
                            .route("", web::post().to(route_presets_post))
                            .route("/{id}", web::get().to(route_preset_get))
                            .route("/{id}", web::put().to(route_preset_put))
                            .route("/{id}", web::delete().to(route_preset_delete))
                            .default_service(route_fallback_404),
                    )
                    // grains
                    .service(
                        web::scope("/grains")
                            .wrap(RequireAuth::new())
                            .route("", web::get().to(route_grains_get))
                            .default_service(route_fallback_404),
                    )
                    // jobs
                    .service(
                        web::scope("/jobs")
                            .wrap(RequireAuth::new())
                            .route("", web::get().to(route_jobs_get))
                            .route("", web::post().to(route_jobs_post))
                            .route("/{jid}", web::get().to(route_job_get))
                            .default_service(route_fallback_404),
                    )
                    // events
                    .service(
                        web::scope("/events")
                            .wrap(RequireAuth::new())
                            .route("", web::get().to(route_events_get))
                            .default_service(route_fallback_404),
                    )
                    // pipeline
                    .service(
                        web::scope("/pipeline")
                            .wrap(RequireAuth::new())
                            .route("", web::get().to(route_pipeline_get))
                            .default_service(route_fallback_404),
                    )
                    // users
                    .service(
                        web::scope("/users")
                            .wrap(RequireAuth::new())
                            .route("", web::get().to(route_users_get))
                            .route("", web::post().to(route_users_post))
                            .route("/{user_id}", web::get().to(route_user_get))
                            .route("/{user_id}", web::delete().to(route_user_delete))
                            .route(
                                "/{user_id}/password",
                                web::post().to(route_user_password_post),
                            )
                            .route(
                                "/{user_id}/permissions/{group_id}",
                                web::post().to(route_user_permissions_post),
                            )
                            .route(
                                "/{user_id}/permissions/{group_id}",
                                web::delete().to(route_user_permissions_delete),
                            )
                            .default_service(route_fallback_404),
                    )
                    // keys
                    .service(
                        web::scope("/keys")
                            .wrap(RequireAuth::new())
                            .route("", web::get().to(route_keys_get))
                            .route("/{state}/{id}/accept", web::put().to(route_key_accept_put))
                            .route("/{state}/{id}/reject", web::put().to(route_key_reject_put))
                            .route(
                                "/{state}/{id}/delete",
                                web::delete().to(route_key_delete_delete),
                            )
                            .default_service(route_fallback_404),
                    )
                    // permissions
                    .service(
                        web::scope("/permissions")
                            .wrap(RequireAuth::new())
                            .route("", web::get().to(route_permissions_get))
                            .route("", web::post().to(route_permissions_post))
                            .route("/{id}", web::get().to(route_permission_get))
                            .route("/{id}", web::put().to(route_permission_update))
                            .route("/{id}", web::delete().to(route_permission_delete))
                            .default_service(route_fallback_404),
                    )
                    // fallback to 404
                    .default_service(route_fallback_404),
            )
            // Embed web interface
            .service(web::scope("").default_service(route_frontend_get))
    })
    .bind(("0.0.0.0", SConfig::http_port()))?
    .run()
    .await?;

    Ok(())
}
