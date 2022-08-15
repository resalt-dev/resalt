#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

use actix_web::{http::header, middleware::*, web, App, HttpServer};
use prelude::*;
use tokio::task;

mod auth;
mod components;
mod models;
mod pipeline;
mod prelude;
mod routes;
mod salt;
mod schema;
mod storage;

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
    db.init().await;

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

    HttpServer::new(move || {
        // Salt API
        let salt_api = SaltAPI::new();

        App::new()
            .app_data(web::Data::new(pipeline.clone()))
            .app_data(web::Data::new(db.clone()))
            .app_data(web::Data::new(salt_api))
            // Prevent sniffing of content type
            .wrap(DefaultHeaders::new().add((header::X_CONTENT_TYPE_OPTIONS, "nosniff")))
            // Removes trailing slash in the URL to make is sowe don't need as many services
            .wrap(NormalizePath::trim())
            // enable logger - always register Actix Web Logger middleware last
            .wrap(Logger::default())
            .service(
                web::scope(&format!("{}/api/1", &SConfig::sub_path()))
                    .wrap(auth::ValidateAuth::new(db.clone()))
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
                            .route("/{id}", web::get().to(route_user_get))
                            .default_service(route_fallback_404),
                    )
                    // fallback to 404
                    .default_service(route_fallback_404),
            )
            // Serve UI
            .service(web::scope(&SConfig::sub_path()).default_service(route_frontend_get))
            // Redirect 302 to UI if outside sub_path using lambda
            .default_service(route_fallback_redirect)
    })
    .bind(("0.0.0.0", SConfig::http_port()))?
    .run()
    .await
}
