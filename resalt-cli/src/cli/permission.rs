use super::permission_group::{cli_permission_group, PermissionGroupCommands};
use clap::Subcommand;
use resalt_salt::SaltAPI;
use resalt_storage::Storage;

#[derive(Subcommand, Debug)]
pub enum PermissionCommands {
    // Groups
    #[clap(about = "Manage permission groups", aliases = &["g"])]
    Group {
        #[clap(subcommand)]
        subcmd: PermissionGroupCommands,
    },
}

pub async fn run_cli_permission(
    data: Storage,
    _salt_api: SaltAPI,
    cmd: PermissionCommands,
) -> Result<(), String> {
    match cmd {
        PermissionCommands::Group { subcmd } => {
            cli_permission_group(data, _salt_api, subcmd).await?
        }
    }
    Ok(())
}
