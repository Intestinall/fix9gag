use crate::cloudflare_runner::kv::{get_cached_value, write_cached_value};
use crate::cloudflare_runner::ok_or_return_err_as_ok;
use crate::config::Config;
use crate::core::{QueryParams, generate_oembed_json};
use axum::extract::Query;
use axum::http::{StatusCode, header};
use axum::response::IntoResponse;
use worker::KvStore;

pub async fn handler(
    req: worker::HttpRequest,
    config: Config,
    kv_store: KvStore,
    endpoint: String,
) -> worker::Result<axum::http::Response<axum::body::Body>> {
    let json_string = match get_cached_value(&kv_store, &endpoint).await {
        Some(oembed_json_string) => oembed_json_string,
        None => {
            let result = Query::try_from_uri(req.uri())
                .map_err(|_| (StatusCode::BAD_REQUEST, "Invalid query parameters").into_response());
            let query: Query<QueryParams> = ok_or_return_err_as_ok!(result);
            let oembed_json = generate_oembed_json(query.0, config).await;
            let json_string = serde_json::to_string(&oembed_json).unwrap();
            write_cached_value(&kv_store, &endpoint, &json_string).await;
            json_string
        }
    };
    Ok((
        StatusCode::OK,
        [(header::CONTENT_TYPE, "application/json")],
        json_string,
    )
        .into_response())
}
