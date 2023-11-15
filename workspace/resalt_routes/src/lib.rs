mod config;
mod events;
mod fallback;
mod frontend;
mod grains;
mod index;
mod jobs;
mod keys;
mod login;
mod metrics;
mod minions;
mod myself;
mod permissions;
mod presets;
mod settings;
mod status;
mod token;
mod users;

pub use self::config::*;
pub use events::*;
pub use fallback::*;
pub use frontend::*;
pub use grains::*;
pub use index::*;
pub use jobs::*;
pub use keys::*;
pub use login::*;
pub use metrics::*;
pub use minions::*;
pub use myself::*;
pub use permissions::*;
pub use presets::*;
pub use settings::*;
pub use status::*;
pub use token::*;
pub use token::*;
pub use users::*;

use actix_web::guard;
use resalt_models::AuthStatus;

pub fn guard_require_auth(ctx: &guard::GuardContext<'_>) -> bool {
    // Check if req has extension AuthStatus{}
    match ctx.req_data().get::<AuthStatus>() {
        Some(auth_status) => match auth_status.salt_token {
            Some(_) => true,
            None => false,
        },
        None => false,
    }
}
