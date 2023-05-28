// pub mod aa;
pub mod gpt;
pub mod taskController;
pub mod utils;

// pub use utils::validate_router;

use anyhow::Result;
use js_sys::Promise;
use reqwest::Error;
use taskController::TaskController;
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

enum DirectoryType {
    File,
    Directory,
}

pub struct Directory {
    name: String,
    types: DirectoryType,
}

pub struct resource {
    name: String,
    directory: Vec<Directory>,
}

#[wasm_bindgen]
impl BalaBala {
    #[wasm_bindgen(constructor)]
    pub fn new(host_name: String) -> Self {
        // log(&format!("【 new param 】==> {:?}", host_name));
        Self { host_name }
    }

    pub fn make_dist(&self, resource_name: &str) {
        todo!()
    }

    pub fn get_host_name(&self) -> String {
        self.host_name.clone()
    }

    pub async fn fetch_html(&self, api: String) -> String {
        let url = format!("{}{}", self.host_name, api);
        // log(&url);

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

        // 方法一
        // let results = results.into_iter().collect::<Result<Vec<_>, _>>().unwrap();
        // js_sys::Array::from_iter(results)
        // 方法一 end

        // 方法二
        let results = results
            .into_iter()
            .map(|result| {
                let value = result.unwrap();
                JsValue::from_str(&value)
            })
            .collect::<Vec<_>>();
        js_sys::Array::from_iter(results)
        // 方法二 end
    }

    // 使用 js_sys::Promise 来做
    pub async fn fetch_html_promise_all(&self, apis_js: js_sys::Array) -> Promise {
        let apis = apis_js.to_vec();
        let array = js_sys::Array::new();

        for api in apis {
            let url = format!("{}{}", self.host_name, api.as_string().unwrap());
            let promise = future_to_promise(async move {
                // 如果这里不想match，就的吧String 和 Error转成JsValue
                // 因为future_to_promise 接受一个 Future<Output = Result<JsValue, JsValue>>；见方法二 fetch_html_promise_all2
                // 实际上就是推迟了类型处理
                match _get_html(&url).await {
                    Ok(value) => {
                        // log(&format!("---------------------------------------------------------------------{:?}",value));
                        Ok(JsValue::from(value))
                    }
                    Err(err) => Err(JsValue::from_str(&err.to_string())),
                }
            });
            array.push(&promise);
        }

        log(&format!("array {:?}", array));

        let promise = js_sys::Promise::all(&array);

        promise
    }

    // 使用 js_sys::Promise 来做，但是这里使用_get_html2方法，将_get_html返回值Future<Output = Result<String, Error>> 转成了 Future<Output = JsValue>
    pub async fn fetch_html_promise_all2(&self, apis_js: js_sys::Array) -> Promise {
        let apis = apis_js.to_vec();
        let array = js_sys::Array::new();

        for api in apis {
            let url = format!("{}{}", self.host_name, api.as_string().unwrap());

            // 这里使用了FutureExt的map方法，将Future<Output = Result<String, Error>> 转成了 Future<Output = Result<JsValue, JsValue>》
            // 注意：这里map会导致代码同步，除非使用 async 包裹起来
            // let promises = get_html(&url).map(JsValue::from).await; // 简写
            // array.push(&promises);

            let future = _get_html2(url);
            let promises = future_to_promise(future);
            array.push(&promises);
        }

        log(&format!("array {:?}", array));

        let promise = js_sys::Promise::all(&array);

        promise
    }

    // todo 1：使用 std::future来做 ** 很练技术，有挑战，有助于了解底层：poll, wait, pending
    // pub fn fetch_html_sync(&self, apis_js: js_sys::Array) -> String {
    //     let apis = apis_js.to_vec();
    //     let arr = js_sys::Array::new();

    //     for api in apis {
    //         let url = format!("{}{}", self.host_name, api.as_string().unwrap());
    //         _get_html3(&url);
    //     }

    //     "".to_string()
    // }
}

pub async fn _get_html(url: &str) -> Result<String, Error> {
    let body = reqwest::get(url).await?.text().await?;

    Ok(body)
}

// 也许这种做法也许更应该更自然
pub async fn _get_html2(url: String) -> Result<JsValue, JsValue> {
    let body = reqwest::get(url)
        .await
        .map_err(|err| JsValue::from(format!("{}", err.to_string())))?
        .text()
        .await
        .map_err(|err| JsValue::from(format!("{}", err.to_string())))?;

    Ok(JsValue::from(body))
}

// 使用Poll来做异步任务
pub fn _get_html3(url: String) {
    let mut task_control = TaskController::new();
    let res = reqwest::get(url);
    task_control.spawn_join(Box::pin(res));
    let result = task_control.awaits();

    println!("【 result 】==> {:?}", result);
}

#[wasm_bindgen]
pub async fn get_html(url: &str) -> String {
    _get_html(url).await.unwrap()
}

// 编写 _get_html3 测试用例
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_html3() {
        _get_html3("https://docs.rs/v8/latest/v8".to_string());
    }
}
