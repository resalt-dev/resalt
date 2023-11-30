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

#[derive(serde::Deserialize)]
pub struct PaginateQuery {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

impl PaginateQuery {
    fn parse_query(&self) -> resalt_models::Paginate {
        match (self.limit, self.offset) {
            (Some(limit), Some(offset)) => Some((limit, offset)),
            (Some(limit), None) => Some((limit, 0)),
            _ => None,
        }
    }
}
