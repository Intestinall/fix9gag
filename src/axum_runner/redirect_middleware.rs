use crate::shared::is_discord_bot;
use axum::extract::Path;
use axum::response::{IntoResponse, Redirect};
use axum::{extract::Request, http::HeaderMap, middleware::Next, response::Response};

pub async fn non_discord_bot_redirect(
    headers: HeaderMap,
    Path(post_id): Path<String>,
    request: Request,
    next: Next,
) -> Result<Response, Response> {
    match is_discord_bot(&headers).await {
        true => Ok(next.run(request).await),
        false => {
            // Early redirect for non discord bot user agents
            println!("[REDIRECTION] User Agent is not discord bot");
            let url = format!("https://9gag.com/gag/{post_id}");
            Err(Redirect::permanent(&url).into_response())
        }
    }
}
