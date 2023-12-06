use crate::cli::*;
use clap::Parser;
use env_logger::{init_from_env, Env};
use log::*;
use resalt_config::ResaltConfig;
use resalt_models::ApiError;
use resalt_salt::SaltAPI;
use resalt_storage::StorageImpl;
use resalt_storage_mysql::StorageMySQL;
use resalt_storage_redis::StorageRedis;

mod cli;

// pub const RESALT_CLI_SYSTEM_SERVICE_USERNAME: &str = "$superadmin/svc/resalt-cli$";

async fn init_db() -> Box<dyn StorageImpl> {
    let db_type = &ResaltConfig::DATABASE_TYPE.clone();
    let db_type = db_type.as_str();
    debug!("Database type: \"{}\"", db_type);
    match db_type {
        "files" => {
            let path: String = ResaltConfig::DATABASE_HOST.clone();
            debug!("Database path: \"{}\"", path);
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
            debug!("Database URL: \"{}\"", database_url);
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
            debug!("Database URL: \"{}\"", database_url);
            Box::new(
                StorageRedis::connect(&database_url)
                    .await
                    .unwrap_or_else(|_| panic!("Error connecting to {}", &database_url)),
            )
        }
        _ => panic!(),
    }
}

async fn run() -> Result<(), ApiError> {
    let cli = Cli::parse();

    // Logging
    init_from_env(Env::new().default_filter_or("Error"));

    // Database
    let data: Box<dyn StorageImpl> = init_db().await;

    // Salt
    let salt_api = SaltAPI::new();

    // Cli
    run_cli(data, salt_api, cli.subcmd).await?;

    Ok(())
}

#[tokio::main]
pub async fn main() {
    match run().await {
        Ok(_) => {}
        Err(e) => {
            panic!("Error: {}", e.message());
        }
    }
}
