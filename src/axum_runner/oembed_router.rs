use crate::config::Config;
use crate::core::{OembedResponse, QueryParams, generate_oembed_json};
use axum::Json;
use axum::Router;
use axum::extract::{Query, State};
use axum::routing::get;

async fn handler(
    Query(query_params): Query<QueryParams>,
    State(config): State<Config>,
) -> Json<OembedResponse> {
    Json(generate_oembed_json(query_params, config).await)
}

pub fn router(config: Config) -> Router {
    let oembed_endpoint = format!(
        "/{}",
        &config
            .oembed_endpoint
            .trim_end_matches("/")
            .trim_start_matches("/")
    );
    Router::new()
        .route(&oembed_endpoint, get(handler))
        .with_state(config)
}
