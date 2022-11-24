use actix_web::{http::header, middleware::*, web, App, HttpServer};
use log::error;
use prelude::*;
use tokio::task;

mod auth;
mod components;
mod config;
mod pipeline;
mod prelude;
mod routes;
mod salt;
mod scheduler;
mod storage;
mod update;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("Debug"));

    // SSE
    let pipeline = PipelineServer::new();

    // Database
    let database_url = SConfig::database_url();
    let db = Storage::connect(&database_url)
        .await
        .unwrap_or_else(|_| panic!("Error connecting to {}", &database_url));

    // Salt WebSocket
    let salt_listener_pipeline = pipeline.clone();
    let salt_listener_db = db.clone();
    std::thread::spawn(move || {
        let rt = tokio::runtime::Runtime::new().unwrap();
        let ls = task::LocalSet::new();
        ls.block_on(&rt, async {
            // Wait a few seconds before starting SSE, so web server gets time to start
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
            let salt_ws = SaltEventListener::new(salt_listener_pipeline, salt_listener_db);
            salt_ws.start().await;
        });
    });

    // Scheduler
    let mut scheduler = scheduler::Scheduler::new();
    scheduler.add_system_jobs();
    scheduler.start();

    HttpServer::new(move || {
        // Salt API
        let salt_api = SaltAPI::new();

        App::new()
            .app_data(web::Data::new(pipeline.clone()))
            .app_data(web::Data::new(db.clone()))
            .app_data(web::Data::new(salt_api.clone()))
            .app_data(web::Data::new(scheduler.clone()))
            // Prevent sniffing of content type
            .wrap(DefaultHeaders::new().add((header::X_CONTENT_TYPE_OPTIONS, "nosniff")))
            // Removes trailing slash in the URL to make is sowe don't need as many services
            .wrap(NormalizePath::trim())
            // enable logger - always register Actix Web Logger middleware last
            .wrap(Logger::default())
            .service(
                web::scope("/api/1")
                    .wrap(auth::ValidateAuth::new(db.clone(), salt_api.clone()))
                    .route("/", web::get().to(route_index_get))
                    .route("/config", web::get().to(route_config_get))
                    // auth
                    .service(
                        web::scope("/auth")
                            .route("/login", web::post().to(route_auth_login_post))
                            .route("/token", web::post().to(route_auth_token_post))
                            .service(
                                web::scope("/user")
                                    .wrap(auth::RequireAuth::new())
                                    .route("", web::get().to(route_auth_user_get))
                                    .default_service(route_fallback_404),
                            )
                            .default_service(route_fallback_404),
                    )
                    // metrics
                    .service(
                        web::scope("/metrics")
                            .wrap(auth::RequireAuth::new())
                            .route("", web::get().to(route_metrics_get))
                            .default_service(route_fallback_404),
                    )
                    // minions
                    .service(
                        web::scope("/minions")
                            .wrap(auth::RequireAuth::new())
                            .route("", web::get().to(route_minions_get))
                            .route("/refresh", web::post().to(route_minions_refresh_post))
                            .route("/{id}", web::get().to(route_minion_get))
                            .default_service(route_fallback_404),
                    )
                    // jobs
                    .service(
                        web::scope("/jobs")
                            .wrap(auth::RequireAuth::new())
                            .route("", web::get().to(route_jobs_get))
                            .route("", web::post().to(route_jobs_post))
                            .route("/{jid}", web::get().to(route_job_get))
                            .default_service(route_fallback_404),
                    )
                    // events
                    .service(
                        web::scope("/events")
                            .wrap(auth::RequireAuth::new())
                            .route("", web::get().to(route_events_get))
                            .default_service(route_fallback_404),
                    )
                    // pipeline
                    .service(
                        web::scope("/pipeline")
                            .wrap(auth::RequireAuth::new())
                            .route("", web::get().to(route_pipeline_get))
                            .default_service(route_fallback_404),
                    )
                    // users
                    .service(
                        web::scope("/users")
                            .wrap(auth::RequireAuth::new())
                            .route("", web::get().to(route_users_get))
                            .route("/{user_id}", web::get().to(route_user_get))
                            .route(
                                "/{user_id}/password",
                                web::post().to(route_user_password_post),
                            )
                            .route(
                                "/{user_id}/permissions/{group_id}",
                                web::post().to(route_user_permission_post),
                            )
                            .route(
                                "/{user_id}/permissions/{group_id}",
                                web::delete().to(route_user_permission_delete),
                            )
                            .default_service(route_fallback_404),
                    )
                    // keys
                    .service(
                        web::scope("/keys")
                            .wrap(auth::RequireAuth::new())
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
                            .wrap(auth::RequireAuth::new())
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
            // Proxy web interface
            .service(web::scope("").default_service(route_frontend_get))
    })
    .bind(("0.0.0.0", SConfig::http_port()))?
    .run()
    .await?;

    // run update check
    if let Err(e) = update::get_remote_version().await {
        error!("{}", e);
    }

    Ok(())
}
