mod config;
mod handlers;
mod kv;
mod macros;
mod pseudo_main;

use crate::cloudflare_runner::macros::ok_or_return_err_as_ok;
use crate::cloudflare_runner::pseudo_main::internal_fetch;

#[worker::event(fetch)] //, respond_with_errors)]
pub async fn fetch(
    req: worker::HttpRequest,
    env: worker::Env,
    ctx: worker::Context,
) -> worker::Result<axum::http::Response<axum::body::Body>> {
    internal_fetch(req, env, ctx).await
}
