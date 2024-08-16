use std::env;

use anyhow::Result;
use gloo_net::http::Request;
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct ShaderContent {
    pub title: String,
    pub description: ContentI18ned,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct ContentI18ned {
    pub ja: String,
    pub en: String,
}

pub async fn fetch_shader_content<'a>(slug: impl zoon::IntoCowStr<'a> + std::fmt::Display) -> Result<ShaderContent> {
    let url = format!("{}shaders/{}", env!("MICROCMS_API_ENDPOINT"), slug);
    let response = Request::get(&url)
        .header("X-MICROCMS-API-KEY", env!("MICROCMS_API_KEY"))
        .send()
        .await?;
    let json: serde_json::Value = response.json().await?;
    let shader_content: ShaderContent = serde_json::from_value(json)?;

    Ok(shader_content)
}
