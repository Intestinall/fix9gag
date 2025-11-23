use axum::response::IntoResponse;
use axum::response::Response;

use crate::ResponseResult;
use crate::config::Config;
use crate::core::HtmlNineGagInfo;
use crate::page_manipulation::{
    build_html, extract_data_from_broken_json, extract_data_from_ld_json, get_ninegag_html,
};
use scraper::Html;

pub async fn extract_ninegag_info(post_id: &str) -> ResponseResult<HtmlNineGagInfo> {
    let html_str = get_ninegag_html(post_id).await?;
    let document = Html::parse_document(&html_str);

    Ok(HtmlNineGagInfo {
        ld_json: extract_data_from_ld_json(&document)?,
        broken_json: extract_data_from_broken_json(&document)?,
    })
}

pub async fn generate_embed_html(post_id: &str, config: Config) -> Result<String, Response> {
    let ninegag_info = extract_ninegag_info(post_id)
        .await
        .map_err(|e| e.into_response())?;

    build_html(ninegag_info, &config)
        .await
        .map_err(|e| e.into_response())
}

#[cfg(test)]
mod tests {
    use super::*;
    use mock_instant::thread_local::MockClock;
    use rstest::rstest;
    use std::io::Read;
    use std::path::PathBuf;
    use std::time::Duration;

    #[rstest]
    #[tokio::test]
    async fn test_generate_embed_html_image(#[files("tests/mocks/*.html")] path: PathBuf) {
        MockClock::set_system_time(Duration::from_secs(1763499718));

        let filename = path.leak().file_name().unwrap().to_str().unwrap();
        let output_html = generate_embed_html(filename, Config::default()).await;
        assert!(output_html.is_ok());
        let mut file = std::fs::File::open(format!("tests/expected/{filename}")).unwrap();
        let mut expected_html = String::new();
        file.read_to_string(&mut expected_html).unwrap();
        assert_eq!(expected_html, output_html.unwrap());
    }
}
