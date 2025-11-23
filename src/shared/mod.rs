mod print;
mod utils;

use axum::http::StatusCode;
pub use utils::is_discord_bot;

pub type ResponseResult<T, E = (StatusCode, String)> = Result<T, E>;
