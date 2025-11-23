mod broken_json;
mod builders;
mod html_fetcher;
mod ld_json;

pub use broken_json::{HtmlNineGagBrokenJson, extract_data_from_broken_json, get_interest_emoji};
pub use builders::build_html;

#[cfg(not(test))]
pub use html_fetcher::get_ninegag_html;
#[cfg(test)]
pub use html_fetcher::mock_get_ninegag_html as get_ninegag_html;

pub use ld_json::{HtmlNineGagLdJson, extract_data_from_ld_json};
