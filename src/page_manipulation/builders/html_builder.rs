use crate::ResponseResult;
use crate::config::Config;
use crate::core::HtmlNineGagInfo;
use crate::page_manipulation::builders::build_description;
use crate::page_manipulation::builders::build_oembed_url;
use url::Url;

fn get_common_elements(description: String, thumbnail_url: String, oembed_url: Url) -> Vec<String> {
    vec![
        r#"<meta property="og:type" content="rich"/>"#.to_string(),
        r#"<meta property="og:site_name" content="9GAG"/>"#.to_string(),
        format!(r#"<meta property="og:description" content="{description}"/>"#),
        format!(r#"<meta property="og:image" content="{thumbnail_url}"/>"#),
        r#"<meta property="og:image:alt" content="<9gag_thumbnail>"/>"#.to_string(),
        r#"<meta name="theme-color" content="\#FFFFFF"/>"#.to_string(),
        r#"<meta name="twitter:card" content="summary_large_image"/>"#.to_string(),
        format!(r#"<link type="application/json+oembed" href="{oembed_url}"/>"#),
    ]
}

pub async fn build_html(info: HtmlNineGagInfo, config: &Config) -> ResponseResult<String> {
    let oembed_url = build_oembed_url(&info, config).await?;
    let description = build_description(info.broken_json.polling, info.ld_json.description)
        .await
        .replace("\"", "&quot;");
    let mut head_elements =
        get_common_elements(description, info.broken_json.thumbnail_url, oembed_url);

    if let Some(video_url) = info.broken_json.video_url {
        head_elements.extend([
            format!(r#"<meta property="og:video" content="{video_url}"/>"#),
            format!(r#"<meta property="og:video:secure_url" content="{video_url}"/>"#),
        ]);
    }

    let html_string = format!(
        "<!DOCTYPE html><html><head>{}</head></html>",
        head_elements.join("")
    );
    Ok(html_string)
}
