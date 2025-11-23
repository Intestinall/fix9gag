mod embed_html_generator;
mod oembed_json_generator;
mod structs;

pub use embed_html_generator::generate_embed_html;
pub use oembed_json_generator::generate_oembed_json;
pub use oembed_json_generator::{OembedResponse, QueryParams};
pub use structs::HtmlNineGagInfo;
