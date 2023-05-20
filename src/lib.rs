use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub async fn greet() -> String {
    // web_sys::console::log_1(&"Hello, my_project!".into());

    let result = get_html().await.unwrap();

    result
}

use reqwest::Error;

async fn get_html() -> Result<String, Error> {
    let body = reqwest::get("https://docs.rs/v8/0.71.2/v8")
        .await?
        .text()
        .await?;

    Ok(body)
}
