use crate::cli::*;
use clap::Parser;
use env_logger::{init_from_env, Env};
use resalt_salt::SaltAPI;
use resalt_storage::Storage;

mod cli;

// pub const RESALT_CLI_SYSTEM_SERVICE_USERNAME: &str = "$superadmin/svc/resalt-cli$";

async fn run() -> Result<(), String> {
    let cli = Cli::parse();

    // Logging
    init_from_env(Env::new().default_filter_or("Error"));

    // Database
    let data: Storage = Storage::init_db().await;

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
            panic!("Error: {}", e);
        }
    }
}
