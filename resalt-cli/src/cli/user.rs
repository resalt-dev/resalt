use clap::Subcommand;
use log::*;
use rand::Rng;
use resalt_api::{
    permission::{add_user_to_group, create_permission_group},
    user::{create_user, delete_user, get_user_by_id, get_user_by_username, get_users},
};
use resalt_models::{ApiError, Paginate};
use resalt_salt::SaltAPI;
use resalt_storage::StorageImpl;
use serde_json::{json, to_string_pretty};

#[derive(Subcommand, Debug)]
pub enum UserCommands {
    // Create {
    //     #[clap(short, long, default_value = "World")]
    //     name: String,
    // },
    #[clap(about = "Delete a user", aliases = &["d"])]
    Delete { id: String },
    #[clap(about = "List users", aliases = &["l", "ls"])]
    List {
        #[clap(short, long)]
        raw: bool,
    },
    #[clap(about = "Initialize the admin user")]
    InitAdmin,
}

pub async fn cli_user(
    data: Box<dyn StorageImpl>,
    _salt_api: SaltAPI,
    cmd: UserCommands,
) -> Result<(), ApiError> {
    match cmd {
        UserCommands::Delete { id } => {
            let user = get_user_by_id(&data, &id)?;
            if let Some(user) = user {
                debug!("Deleting user: {}", user.id);
                delete_user(&data, &user.id)?;
                println!("Deleted user: {}", user.username);
            } else {
                error!("User not found");
                std::process::exit(1);
            }
        }
        UserCommands::List { raw } => {
            let users = get_users(&data, Paginate::None)?;
            if raw {
                println!("{}", to_string_pretty(&users).unwrap());
            } else {
                println!(
                    "{0: <42} {1: <22} {2: <14} {3: <22} {4: <6}",
                    "ID", "Username", "Has Password", "Last Login", "Email"
                );
                for user in users {
                    println!(
                        "{0: <42} {1: <22} {2: <14} {3: <22} {4: <6}",
                        user.id,
                        user.username,
                        user.password.is_some(),
                        user.last_login
                            .map(|d| d.to_string())
                            .unwrap_or("None".to_string()),
                        user.email.unwrap_or("None".to_string())
                    );
                }
            }
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
            println!("Created ADMIN user (!)");
            println!("\tUsername: admin");
            println!("\tPassword: {}", random_password);
        }
    }
    Ok(())
}
