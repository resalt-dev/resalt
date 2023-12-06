use clap::{command, Parser, Subcommand};
use env_logger::{init_from_env, Env};
use log::*;
use rand::Rng;
use resalt_api::{
    config::get_config,
    permission::{add_user_to_group, create_permission_group},
    user::{create_user, get_user_by_username, get_users},
};
use resalt_config::ResaltConfig;
use resalt_models::{ApiError, Paginate};
use resalt_salt::SaltAPI;
use resalt_storage::StorageImpl;
use resalt_storage_mysql::StorageMySQL;
use resalt_storage_redis::StorageRedis;
use serde_json::{json, to_string_pretty};

// pub const RESALT_CLI_SYSTEM_SERVICE_USERNAME: &str = "$superadmin/svc/resalt-cli$";

/// Command-line interface for Resalt
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[clap(subcommand)]
    subcmd: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    #[clap(about = "Print the current config")]
    Config,
    #[clap(about = "Manage users")]
    User {
        #[clap(subcommand)]
        subcmd: UserCommands,
    },
    #[clap(about = "Print the current version")]
    Version,
}

#[derive(Subcommand, Debug)]
enum UserCommands {
    // Create {
    //     #[clap(short, long, default_value = "World")]
    //     name: String,
    // },
    List,
    InitAdmin,
}

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
    init_from_env(Env::new().default_filter_or("Debug"));

    // Database
    let data: Box<dyn StorageImpl> = init_db().await;

    // Salt
    let _salt_api = SaltAPI::new();

    // Cli
    match cli.subcmd {
        Commands::Config => {
            let config = get_config(false).await;
            match config {
                Ok(config) => info!("Config: {}", to_string_pretty(&config).unwrap()),
                Err(e) => error!("Error: {}", e),
            }
        }
        Commands::User { subcmd } => match subcmd {
            UserCommands::List => {
                let users = get_users(&data, Paginate::None)?;
                info!("Users: {}", to_string_pretty(&users).unwrap());
            }
            UserCommands::InitAdmin => {
                // Check if "admin" user exists
                if let Some(_) = get_user_by_username(&data, "admin")? {
                    error!("User \"admin\" already exists");
                    std::process::exit(1);
                }

                // Create Admin group
                let perms = json!([
                    ".*".to_string(),
                    "@runner".to_string(),
                    "@wheel".to_string(),
                    {
                        "@resalt": [
                            "admin.superadmin".to_string(),
                        ]
                    }
                ])
                .to_string();
                let group_id = create_permission_group(&data, None, "$superadmins", Some(perms))?;
                // Create Admin user
                let random_password = rand::thread_rng()
                    .sample_iter(&rand::distributions::Alphanumeric)
                    .take(15)
                    .map(|c| c.to_string())
                    .collect::<String>();
                let user = create_user(
                    &data,
                    ("admin").to_string(),
                    Some(random_password.clone()),
                    None,
                )?;
                // Add Admin user to Admin group
                add_user_to_group(&data, &user.id, &group_id)?;

                // Announce randomly generated password
                warn!(
                    "CREATED DEFAULT USER: admin WITH PASSWORD: {}",
                    random_password
                );
            }
        },
        Commands::Version => {
            info!("Version: {}", env!("CARGO_PKG_VERSION"));
        }
    }

    Ok(())
}

#[tokio::main]
pub async fn main() {
    match run().await {
        Ok(_) => {}
        Err(e) => {
            error!("Error: {}", e.message());
            std::process::exit(1);
        }
    }
}
