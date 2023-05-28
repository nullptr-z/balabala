use regex::Regex;
use wasm_bindgen::prelude::*;

/// 判断路由是否是`/`开头
#[wasm_bindgen]
pub fn validate_router(route: &str) -> bool {
    let reg = Regex::new(r"^(http|file|#|/|\.).*$").unwrap();
    reg.is_match(route)
}

#[test]
fn test_validate_router() {
    let valid = validate_router("file:://abc/bbc/abcv");
    assert_eq!(valid, true);
}
