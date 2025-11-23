use crate::ResponseResult;
use crate::config::Config;
use crate::core::HtmlNineGagInfo;
use crate::errors::http::bad_gateway;
use crate::page_manipulation::get_interest_emoji;
use url::Url;

pub async fn build_oembed_url(info: &HtmlNineGagInfo, config: &Config) -> ResponseResult<Url> {
    let title_param = &format!(
        "[{}{}] {}",
        get_interest_emoji(&info.broken_json.interest),
        &info.broken_json.interest,
        &info
            .ld_json
            .headline
            .strip_suffix(" - 9GAG")
            .unwrap_or(&info.ld_json.headline)
    );
    let url = Url::parse_with_params(
        &format!("{}/{}", &config.hostname, &config.oembed_endpoint),
        &[("post_url", &info.ld_json.post_url), ("title", title_param)],
    );
    url.map_err(|e| bad_gateway(e, "Could not create the Oembed url"))
}
