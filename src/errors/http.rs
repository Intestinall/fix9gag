use crate::smart_println;
use axum::http::StatusCode;
use std::fmt::Debug;

pub fn bad_gateway(error: impl Debug, stringable: impl ToString) -> (StatusCode, String) {
    smart_println!("[BAD_GATEWAY] {error:?}");
    (StatusCode::BAD_GATEWAY, stringable.to_string())
}

pub fn simple_bad_gateway(stringable: impl ToString) -> (StatusCode, String) {
    let s = stringable.to_string();
    smart_println!("[BAD_GATEWAY] {s}");
    (StatusCode::BAD_GATEWAY, s)
}

pub fn internal_server_error(stringable: impl ToString, error: impl Debug) -> (StatusCode, String) {
    smart_println!("[INTERNAL_SERVER_ERROR] {error:?}");
    (StatusCode::INTERNAL_SERVER_ERROR, stringable.to_string())
}

pub fn not_found() -> (StatusCode, String) {
    (StatusCode::NOT_FOUND, "not_found".to_string())
}
