use crate::config::Config;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct OembedResponse {
    author_url: String,
    author_name: String,
    provider_url: String,
    provider_name: String,
}

#[derive(Deserialize, Debug)]
pub struct QueryParams {
    post_url: String,
    title: String,
}

pub async fn generate_oembed_json(query_params: QueryParams, config: Config) -> OembedResponse {
    println!("[OEMBED] {query_params:?}");
    OembedResponse {
        author_url: query_params.post_url,
        author_name: query_params.title,
        provider_url: config.provider_url,
        provider_name: config.provider_name,
    }
}
