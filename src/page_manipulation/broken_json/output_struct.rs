use crate::page_manipulation::broken_json::Polling;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct HtmlNineGagBrokenJson {
    pub interest: String,
    pub thumbnail_url: String,
    pub video_url: Option<String>,
    pub polling: Option<Polling>,
}
