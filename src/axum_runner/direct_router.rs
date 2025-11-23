use crate::axum_runner::non_discord_bot_redirect;
use crate::config::Config;
use crate::core::generate_embed_html;
use axum::extract::{Path, State};
use axum::response::Response;
use axum::response::{Html, IntoResponse};
use axum::routing::get;
use axum::{Router, middleware};

async fn handler(Path(post_id): Path<String>, State(config): State<Config>) -> Response {
    match generate_embed_html(&post_id, config).await {
        Ok(html) => Html(html).into_response(),
        Err(response) => response,
    }
}

pub fn router(config: Config) -> Router {
    // For an unknown reason, WASM build only allow simple Response for this function but not for
    // the oembed_router function
    Router::new()
        .route("/gag/{post_id}", get(handler))
        .route_layer(middleware::from_fn(non_discord_bot_redirect))
        .with_state(config)
}
