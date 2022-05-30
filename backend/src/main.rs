#[macro_use]
extern crate diesel;

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

    let ldap = LdapHandler::new().await;

    // SSE
    let pipeline = PipelineServer::new();

    // Database
    let database_url = SConfig::database_url();
    let db = Storage::connect(&database_url)
        .await
        .expect(&format!("Error connecting to {}", &database_url));
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
            .app_data(web::Data::new(ldap.clone()))
            .app_data(web::Data::new(pipeline.clone()))
            .app_data(web::Data::new(db.clone()))
            .app_data(web::Data::new(salt_api.clone()))
            // Prevent sniffing of content type
            .wrap(DefaultHeaders::new().add((header::X_CONTENT_TYPE_OPTIONS, "nosniff")))
            // Removes trailing slash in the URL to make is sowe don't need as many services
            .wrap(NormalizePath::trim())
            // enable logger - always register Actix Web Logger middleware last
            .wrap(Logger::default())
            .service(
                web::scope(&format!("{}/api/1", &SConfig::sub_path()))
                    .wrap(auth::ValidateAuth::new(db.clone()))
                    .configure(|cfg| {
                        cfg.service(web::resource("/").route(web::get().to(route_index_get)));
                        cfg.service(
                            web::resource("/config").route(web::get().to(route_config_get)),
                        );
                        cfg.service(
                            web::resource("/auth/login")
                                .route(web::post().to(route_auth_login_post)),
                        );
                        cfg.service(
                            web::resource("/auth/token")
                                .route(web::post().to(route_auth_token_post)),
                        );
                        cfg.service(
                            web::resource("/auth/user")
                                .wrap(auth::RequireAuth::new())
                                .route(web::get().to(route_auth_user_get)),
                        );
                        cfg.service(
                            web::resource("/minions")
                                .wrap(auth::RequireAuth::new())
                                .route(web::get().to(route_minions_get)),
                        );
                        cfg.service(
                            web::resource("/events")
                                .wrap(auth::RequireAuth::new())
                                .route(web::get().to(route_events_get)),
                        );
                        cfg.service(
                            web::resource("/pipeline")
                                .wrap(auth::RequireAuth::new())
                                .route(web::get().to(route_pipeline_get)),
                        );
                    }),
            )
            // default (fallback to frontend)
            .default_service(route_fallback_get)
    })
    .bind(("0.0.0.0", 8000))?
    .run()
    .await
}
