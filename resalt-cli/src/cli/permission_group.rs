use clap::Subcommand;
use log::*;
use resalt_api::permission::{
    delete_permission_group, get_permission_group_by_id, get_permission_groups,
};
use resalt_models::Paginate;
use resalt_salt::SaltAPI;
use resalt_storage::Storage;
use serde_json::to_string_pretty;

#[derive(Subcommand, Debug)]
pub enum PermissionGroupCommands {
    // #[clap(about = "Create a permission group", aliases = &["c"])]
    // Create {
    //     #[clap(short, long)]
    //     name: String,
    //     #[clap(short, long)]
    //     description: Option<String>,
    //     #[clap(short, long)]
    //     permissions: Option<Vec<String>>,
    // },
    #[clap(about = "Delete a permission group", aliases = &["d"])]
    Delete { id: String },
    #[clap(about = "List permission groups", aliases = &["l", "ls"])]
    List {
        #[clap(short, long)]
        raw: bool,
    },
    // #[clap(about = "Add a permission to a group", aliases = &["a"])]
    // AddPermission {
    //     #[clap(short, long)]
    //     group_id: String,
    //     #[clap(short, long)]
    //     permission: String,
    // },
    // #[clap(about = "Remove a permission from a group", aliases = &["r"])]
    // RemovePermission {
    //     #[clap(short, long)]
    //     group_id: String,
    //     #[clap(short, long)]
    //     permission: String,
    // },
}

pub async fn cli_permission_group(
    data: Storage,
    _salt_api: SaltAPI,
    cmd: PermissionGroupCommands,
) -> Result<(), String> {
    match cmd {
        PermissionGroupCommands::Delete { id } => {
            let group = get_permission_group_by_id(&data, &id)
                .map_err(|e| format!("Failed to get group: {}", e))?;
            if let Some(group) = group {
                debug!("Deleting group: {}", group.id);
                delete_permission_group(&data, &group.id)
                    .map_err(|e| format!("Failed to delete group: {}", e))?;
                println!("Deleted group: {}", group.name);
            } else {
                return Err(format!("Group not found: {}", id));
            }
        }
        PermissionGroupCommands::List { raw } => {
            let groups = get_permission_groups(&data, Paginate::None)
                .map_err(|e| format!("Failed to get groups: {}", e))?;
            if raw {
                println!("{}", to_string_pretty(&groups).unwrap());
            } else {
                println!("{0: <42} {1: <22}", "ID", "Name",);
                for group in groups {
                    println!("{0: <42} {1: <22}", group.id, group.name,);
                }
            }
        }
    }
    Ok(())
}
