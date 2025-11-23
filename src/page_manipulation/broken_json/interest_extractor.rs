use crate::ResponseResult;
use crate::errors::http::simple_bad_gateway;
use crate::page_manipulation::broken_json::BrokenJson;

pub fn get_interest(parsed_json: &BrokenJson) -> ResponseResult<String> {
    let key = parsed_json.data.post.interests[0].as_str();
    Ok(parsed_json
        .config
        .interests
        .get(key)
        .ok_or_else(|| simple_bad_gateway("Interest not in interest list"))?
        .name
        .clone())
}
