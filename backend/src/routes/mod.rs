// export route_config in config

mod auth_login;
mod auth_token;
mod auth_user;
mod config;
mod events;
mod fallback;
mod frontend;
mod index;
mod jobs;
mod minions;
mod pipeline;

pub use self::config::*;
pub use auth_login::*;
pub use auth_token::*;
pub use auth_user::*;
pub use events::*;
pub use fallback::*;
pub use frontend::*;
pub use index::*;
pub use jobs::*;
pub use minions::*;
pub use pipeline::*;
