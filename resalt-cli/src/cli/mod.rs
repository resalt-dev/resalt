mod permission;
mod permission_group;
mod user;

use self::{
    permission::{run_cli_permission, PermissionCommands},
    user::{cli_user, UserCommands},
};
use clap::{command, Parser, Subcommand};
use resalt_api::config::get_config;
use resalt_models::ApiError;
use resalt_salt::SaltAPI;
use resalt_storage::Storage;
use serde_json::to_string_pretty;

/// Command-line interface for Resalt
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[clap(subcommand)]
    pub subcmd: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    #[clap(about = "Print the current config", aliases = &["c"])]
    Config,
    #[clap(about = "Manage permissions", aliases = &["p", "perms"])]
    Permission {
        #[clap(subcommand)]
        subcmd: PermissionCommands,
    },
    #[clap(about = "Manage users", aliases = &["u"])]
    User {
        #[clap(subcommand)]
        subcmd: UserCommands,
    },
    #[clap(about = "Print the current version", aliases = &["v"])]
    Version,
}

pub async fn run_cli(data: Storage, salt_api: SaltAPI, cmd: Commands) -> Result<(), ApiError> {
    match cmd {
        Commands::Config => {
            let config = get_config(false).await?;
            println!("Config: {}", to_string_pretty(&config).unwrap());
        }
        Commands::Permission { subcmd } => run_cli_permission(data, salt_api, subcmd).await?,
        Commands::User { subcmd } => cli_user(data, salt_api, subcmd).await?,
        Commands::Version => {
            println!("Version: {}", env!("CARGO_PKG_VERSION"));
        }
    }

    Ok(())
}
