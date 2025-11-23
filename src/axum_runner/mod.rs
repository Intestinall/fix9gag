mod direct_router;
mod oembed_router;
mod redirect_middleware;

pub use direct_router::router as direct_route;
pub use oembed_router::router as oembed_route;
use redirect_middleware::non_discord_bot_redirect;
