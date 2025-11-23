use axum::http::HeaderMap;

pub async fn is_discord_bot(headers: &HeaderMap) -> bool {
    match headers
        .get("user-agent")
        .and_then(|value| value.to_str().ok())
    {
        Some(user_agent) => user_agent.contains("Discordbot"),
        None => false,
    }
}
