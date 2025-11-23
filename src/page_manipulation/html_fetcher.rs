use crate::errors::http::{bad_gateway, internal_server_error, not_found, simple_bad_gateway};
use crate::{ResponseResult, smart_println};

pub async fn get_ninegag_html(post_id: &str) -> ResponseResult<String> {
    let url = format!("https://9gag.com/gag/{post_id}");
    smart_println!("Fetching {}", url);

    // User agent from : https://9gag.com/robots.txt
    let client = reqwest::Client::builder()
        .user_agent("Mediapartners-Google")
        .build()
        .map_err(|e| internal_server_error(e, "Could not create client"))?;

    let response = client.get(url).send().await.map_err(|e| {
        bad_gateway(
            e,
            "Something wrong happened while sending the request to 9gag.com",
        )
    })?;

    if response.status() == reqwest::StatusCode::NOT_FOUND {
        Err(not_found())
    } else if !response.status().is_success() {
        Err(simple_bad_gateway(format!(
            "9gag.com did not return a successful HTTP response: {}",
            response.status()
        )))
    } else {
        response.text().await.map_err(|e| {
            bad_gateway(
                e,
                "Something wrong happened while receiving the request from 9gag.com",
            )
        })
    }
}

#[cfg(test)]
pub async fn mock_get_ninegag_html(post_id: &str) -> ResponseResult<String> {
    use std::io::Read;
    let mut file = std::fs::File::open(format!("tests/mocks/{post_id}")).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    Ok(contents)
}
