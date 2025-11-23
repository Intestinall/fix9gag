use crate::page_manipulation::broken_json::Polling;
use crate::page_manipulation::builders::build_ascii_poll;

pub async fn build_description(polling: Option<Polling>, description: String) -> String {
    match polling {
        Some(polling) => build_ascii_poll(polling.metrics).await,
        None => description.replace(". ", ".\n"),
    }
}
