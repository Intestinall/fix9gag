use crate::ResponseResult;
use crate::errors::http::{bad_gateway, simple_bad_gateway};
use crate::page_manipulation::broken_json::get_interest;
use crate::page_manipulation::broken_json::{BrokenJson, HtmlNineGagBrokenJson};
use scraper::{Html, Selector};
use std::sync::LazyLock;

static SELECTOR_JSON_DATA: LazyLock<Selector> =
    LazyLock::new(|| Selector::parse("script[type=\"text/javascript\"]").unwrap());

fn to_parsed_json(script_content: &str) -> ResponseResult<BrokenJson> {
    let mut script_content = script_content.to_string();
    script_content = script_content.replace(r#"\""#, r#"""#);
    script_content = script_content.replace(r#"\\/"#, r#"/"#);
    script_content = script_content.replace(r#"\\""#, r#"\""#);
    let script_content = script_content
        .trim_end()
        .strip_suffix("\");")
        .unwrap_or(&script_content);
    serde_json::from_str(script_content).map_err(|e| bad_gateway(e, "Could not parse JSON"))
}

pub fn extract_data_from_broken_json(document: &Html) -> ResponseResult<HtmlNineGagBrokenJson> {
    for script_elem in document.select(&SELECTOR_JSON_DATA) {
        let script_content = script_elem.text().collect::<Vec<_>>().join("");
        if let Some(script_content) = script_content
            .trim_start()
            .strip_prefix("window._config = JSON.parse(\"")
        {
            let parsed_json = to_parsed_json(script_content)?;

            return Ok(HtmlNineGagBrokenJson {
                interest: get_interest(&parsed_json)?,
                thumbnail_url: parsed_json.data.post.images.image700.url,
                video_url: parsed_json
                    .data
                    .post
                    .images
                    .image460sv
                    .map(|images| images.url),
                polling: parsed_json.data.post.polling,
            });
        }
    }
    Err(simple_bad_gateway("No interests found"))
}
