use crate::page_manipulation::{HtmlNineGagBrokenJson, HtmlNineGagLdJson};

#[derive(Debug)]
pub struct HtmlNineGagInfo {
    pub ld_json: HtmlNineGagLdJson,
    pub broken_json: HtmlNineGagBrokenJson,
}
