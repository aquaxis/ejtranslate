use anyhow::{Context, Result, bail};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
struct GenerateRequest<'a> {
    model: &'a str,
    system: &'a str,
    prompt: &'a str,
    stream: bool,
}

#[derive(Debug, Deserialize)]
struct GenerateResponse {
    response: String,
}

pub async fn translate(host: &str, model: &str, system: &str, prompt: &str) -> Result<String> {
    let url = format!("{}/api/generate", host.trim_end_matches('/'));
    let client = reqwest::Client::builder()
        .build()
        .context("failed to build HTTP client")?;
    let body = GenerateRequest {
        model,
        system,
        prompt,
        stream: false,
    };
    let resp = client
        .post(&url)
        .json(&body)
        .send()
        .await
        .with_context(|| format!("request to Ollama failed: {}", url))?;
    let status = resp.status();
    if !status.is_success() {
        let text = resp.text().await.unwrap_or_default();
        bail!("Ollama returned HTTP {}: {}", status, text);
    }
    let parsed: GenerateResponse = resp
        .json()
        .await
        .context("failed to decode Ollama response")?;
    Ok(parsed.response)
}
