pub mod gpt;

use std::future::IntoFuture;

use anyhow::Result;
use futures::FutureExt;
use js_sys::Promise;
use reqwest::Error;
use wasm_bindgen::{__rt::IntoJsResult, convert::IntoWasmAbi, prelude::*};
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

    pub fn get_host_name(&self) -> String {
        self.host_name.clone()
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

    pub async fn fetch_html_all(&self, string_arr: js_sys::Array) -> js_sys::Array {
        let apis = string_arr.to_vec();

        let futures = apis.into_iter().map(|api| {
            let url = format!("{}{}", self.host_name, api.as_string().unwrap());
            log(&url);

            async move { _get_html(&url).await }
        });

        let results = futures::future::join_all(futures).await;
        // todo 1：使用 js_sys::Promise 来做
        // todo 1：使用 std::future来做 ** 很练技术，有挑战，有助于了解底层：poll, wait, pending

        // 方法一
        // let results = results.into_iter().collect::<Result<Vec<_>, _>>().unwrap();
        // js_sys::Array::from_iter(results)
        // 方法一 end

        // 方法二
        let results = results
            .into_iter()
            .map(|result| {
                let value = result.unwrap();
                log(&format!(
                    "---------------------------------------------------------------------{:?}",
                    value
                ));
                JsValue::from_str(&value)
            })
            .collect::<Vec<_>>();
        js_sys::Array::from_iter(results)
        // 方法二 end

        // for api in apis {
        //     let url = format!("{}{}", self.host_name, api.as_string().unwrap());
        //     let res = get_html(&url).await;
        //     log(&url);
        //     log(&format!(
        //         "---------------------------------------------------------------------{:?}",
        //         res
        //     ));
        //     result.push(JsValue::from_str(&res));
        // }
    }

    pub async fn fetch_html_promise_all(&self, string_arr: js_sys::Array) -> Promise {
        let apis = string_arr.to_vec();
        let array = js_sys::Array::new();

        for api in apis {
            let url = format!("{}{}", self.host_name, api.as_string().unwrap());

            // 方法一
            // 这里使用了FutureExt的map方法，将Future<Output = Result<String, Error>> 转成了 Future<Output = JsValue>
            // let promises = get_html(&url).map(JsValue::from).await; // 简写
            let promises = get_html(&url)
                .map(|value| {
                    log(&format!(
                        "---------------------------------------------------------------------{:?}",
                        value
                    ));
                    let promise = JsValue::from(value);
                    promise
                })
                .await;
            array.push(&promises);
            // 方法一 end

            // 方法二
            let promise = future_to_promise(async move {
                // 这里不使用FutureExt的map，手动转换
                // ---
                // 如果这里不想match，就的吧String 和 Error转成JsValue
                // 因为future_to_promise 接受一个 Future<Output = Result<JsValue, JsValue>>
                match _get_html(&url).await {
                    Ok(value) => {
                        log(&format!("---------------------------------------------------------------------{:?}",value));
                        Ok(JsValue::from(value))
                    }
                    Err(err) => Err(JsValue::from_str(&err.to_string())),
                }
            });
            array.push(&promise);
            // 方法二 end
        }

        let promise = js_sys::Promise::all(&array);

        promise
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
