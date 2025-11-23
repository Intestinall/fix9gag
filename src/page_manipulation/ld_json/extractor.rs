use crate::ResponseResult;
use crate::errors::http::{bad_gateway, simple_bad_gateway};
use crate::page_manipulation::ld_json::structs::HtmlNineGagLdJson;
use scraper::{Html, Selector};
use std::sync::LazyLock;

static SELECTOR: LazyLock<Selector> =
    LazyLock::new(|| Selector::parse(r#"script[type="application/ld+json"]"#).unwrap());

pub fn extract_data_from_ld_json(document: &Html) -> ResponseResult<HtmlNineGagLdJson> {
    let ld_json = document
        .select(&SELECTOR)
        .next()
        .ok_or_else(|| simple_bad_gateway("Could not find the ld+json in the HTML"))?
        .inner_html()
        .to_owned();
    serde_json::from_str(&ld_json).map_err(|e| bad_gateway(e, "Could not parse ld+json"))
}
