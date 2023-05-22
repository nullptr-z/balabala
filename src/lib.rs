pub mod gpt;

use anyhow::Result;
use reqwest::Error;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::future_to_promise;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub struct BalaBala {
    host_name: String,
}

#[wasm_bindgen]
impl BalaBala {
    #[wasm_bindgen(constructor)]
    pub fn new(host_name: String) -> Self {
        // log(&format!("【 new param 】==> {:?}", host_name));
        Self { host_name }
    }

    pub async fn fetch_html(&self, api: String) -> String {
        let url = format!("{}{}", self.host_name, api);
        log(&url);

        get_html(&url).await
    }

    pub async fn fetch_html_promise(&self, api: String) -> js_sys::Promise {
        let url = format!("{}{}", self.host_name, api);

        future_to_promise(async move {
            match _get_html(&url).await {
                Ok(res) => Ok(JsValue::from_str(&res)),
                Err(err) => Err(JsValue::from_str(&err.to_string())),
            }
        })
    }

    pub fn get_host_name(&self) -> String {
        self.host_name.clone()
    }
}

#[wasm_bindgen]
pub async fn get_html(url: &str) -> String {
    _get_html(url).await.unwrap()
}

pub async fn _get_html(url: &str) -> Result<String, Error> {
    let body = reqwest::get(url).await?.text().await?;

    Ok(body)
}
