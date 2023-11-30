use clap::{command, Parser, Subcommand};
use env_logger::{init_from_env, Env};
use log::*;
use resalt_api::config::get_config;
use resalt_config::ResaltConfig;
use resalt_salt::SaltAPI;
use resalt_storage::StorageImpl;
use resalt_storage_mysql::StorageMySQL;
use resalt_storage_redis::StorageRedis;
use serde_json::to_string_pretty;
use std::error::Error;

// pub const RESALT_CLI_SYSTEM_SERVICE_USERNAME: &str = "$superadmin/svc/resalt-cli$";

/// Command-line interface for Resalt
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[clap(subcommand)]
    subcmd: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Config,
    User {
        #[clap(subcommand)]
        subcmd: Option<UserCommands>,
    },
}

#[derive(Subcommand, Debug)]
enum UserCommands {
    // Create {
    //     #[clap(short, long, default_value = "World")]
    //     name: String,
    // },
    List,
}

async fn init_db() -> Box<dyn StorageImpl> {
    let db_type = &ResaltConfig::DATABASE_TYPE.clone();
    let db_type = db_type.as_str();
    debug!("Database type: \"{}\"", db_type);
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

async fn run() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    // Logging
    init_from_env(Env::new().default_filter_or("Debug"));

    // Database
    let _db: Box<dyn StorageImpl> = init_db().await;

    // Salt
    let _salt_api = SaltAPI::new();

    // Cli
    match cli.subcmd {
        Some(Commands::Config) => {
            let config = get_config(false).await;
            match config {
                Ok(config) => info!("Config: {}", to_string_pretty(&config).unwrap()),
                Err(e) => error!("Error: {}", e),
            }
        }
        Some(Commands::User { subcmd }) => match subcmd {
            Some(UserCommands::List) => {
                // let query: Query<PaginateQuery> = Query(Paginate {
                //     limit: Some(i64::MAX),
                //     offset: None,
                // });
                // let _users = route_users_get(query, State(db), Extension(auth)).await;
                // println!("Users: {:?}", users);
            }
            None => {
                println!("No subcommand was used");
            }
        },
        None => {
            println!("No subcommand was used");
        }
    }

    Ok(())
}

#[tokio::main]
pub async fn main() {
    run().await.unwrap();
}
