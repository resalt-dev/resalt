// export route_config in config

mod auth_login;
mod auth_token;
mod auth_user;
mod config;
mod fallback;
mod index;
mod minions;

pub use self::config::*;
pub use auth_login::*;
pub use auth_token::*;
pub use auth_user::*;
pub use fallback::*;
pub use index::*;
pub use minions::*;
