use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct HtmlNineGagLdJson {
    #[serde(rename(deserialize = "url"))]
    pub post_url: String,
    pub headline: String,
    pub description: String,
}
