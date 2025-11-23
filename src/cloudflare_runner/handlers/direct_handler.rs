use crate::cloudflare_runner::kv::{get_cached_value, write_cached_value};
use crate::cloudflare_runner::ok_or_return_err_as_ok;
use crate::config::Config;
use crate::core::generate_embed_html;
use crate::shared::is_discord_bot;
use axum::response::Html;
use axum::response::IntoResponse;
use axum::response::Redirect;
use worker::KvStore;

pub async fn handler(
    req: worker::HttpRequest,
    config: Config,
    kv_store: KvStore,
    post_id: &str,
) -> worker::Result<axum::http::Response<axum::body::Body>> {
    if !is_discord_bot(&req.headers()).await {
        let url = format!("https://9gag.com/gag/{post_id}");
        return Ok(Redirect::permanent(&url).into_response());
    }

    let html = match get_cached_value(&kv_store, post_id).await {
        Some(html) => html,
        None => {
            let result = generate_embed_html(post_id, config).await;
            let html = ok_or_return_err_as_ok!(result);
            write_cached_value(&kv_store, post_id, &html).await;
            html
        }
    };
    Ok(Html(html).into_response())
}
