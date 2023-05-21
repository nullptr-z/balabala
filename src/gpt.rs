use anyhow::Result;
use reqwest::{Client, Response};
use serde_json::json;
use tokio::fs;

const OPEN_AI_API_KEY: &str = "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";

async fn connect() -> Result<Response> {
    let client = Client::new();

    let body = serde_json::to_string(&json!({
          "model": "gpt-3.5-turbo",
        "messages":[
            {"role": "user", "content": "hi?"}
        ],
        // "model":"text-davinci-002",
        // "prompt": "Hello, who are you?"

        "max_tokens": 5,
    }))
    .unwrap();

    let req_build = client
        .post("https://api.openai.com/v1/chat/completions")
        .header("Authorization", format!("Bearer {}", OPEN_AI_API_KEY)) // 个人API密钥；.header("OpenAI-Organization", "org-gqQWrGIVxkdQOjgvQUVSG0UK"); // 组织ID
        .header("Content-Type", "application/json")
        .body(body);

    println!("【 req_build 】==> {:?}", req_build);

    let res = req_build.send().await.unwrap_or_else(|err| {
        eprintln!("{}", err);
        std::process::exit(1);
    });

    // println!(" res ==> {:?}", res.text().await?);
    Ok(res)
}

async fn write_to_file(res: Response, file_name: &str) -> Result<()> {
    let body = res.text().await?;
    fs::write(file_name, body).await?;

    Ok(())
}

async fn get_models() -> Result<Response> {
    let client = Client::new();

    let req_build = client
        .get("https://api.openai.com/v1/models")
        .header("Authorization", format!("Bearer {}", OPEN_AI_API_KEY)); // 个人API密钥；.header("OpenAI-Organization", "org-gqQWrGIVxkdQOjgvQUVSG0UK"); // 组织ID

    println!("【 req_build 】==> {:?}", req_build);

    let res = req_build.send().await.unwrap_or_else(|err| {
        eprintln!("{}", err);
        std::process::exit(1);
    });

    Ok(res)
}

#[cfg(test)]
mod tests {
    use anyhow::{Ok, Result};

    use crate::gpt::write_to_file;

    #[tokio::test]
    async fn test_connect() {
        let res = super::connect().await.unwrap();

        write_to_file(res, "result.json").await.unwrap();
    }

    #[tokio::test]
    async fn write_models() {
        let res = super::get_models().await.unwrap();
        write_to_file(res, "models.json").await.unwrap();
    }
}
