use crate::cloudflare_runner::config::get_config;
use crate::cloudflare_runner::handlers::{direct_handler, oembed_handler};
use crate::cloudflare_runner::kv::get_kv_store;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use regex::Regex;

pub async fn internal_fetch(
    req: worker::HttpRequest,
    env: worker::Env,
    _ctx: worker::Context,
) -> worker::Result<axum::http::Response<axum::body::Body>> {
    let kv_store = match get_kv_store(&env) {
        Ok(v) => v,
        Err(_) => return Ok(StatusCode::INTERNAL_SERVER_ERROR.into_response()),
    };
    let re = Regex::new(r"^/gag/([A-Za-z0-9]{5,9})$").unwrap();
    let endpoint = req.uri().path_and_query().unwrap().to_string();
    let config = get_config(env);

    let oembed_endpoint = format!(
        "/{}",
        &config
            .oembed_endpoint
            .trim_end_matches("/")
            .trim_start_matches("/")
    );

    if endpoint.starts_with(&oembed_endpoint) {
        Ok(oembed_handler::handler(req, config, kv_store, endpoint).await?)
    } else if let Some(caps) = re.captures(&endpoint) {
        Ok(direct_handler::handler(req, config, kv_store, &caps[1]).await?)
    } else {
        Ok(StatusCode::NOT_FOUND.into_response())
    }
}
