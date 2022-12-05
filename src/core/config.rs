use std::collections::HashMap;

use anyhow::{Ok, Result};
use serde::Deserialize;
use tokio::{fs::File, io::AsyncReadExt};

#[derive(Deserialize, Debug)]
pub struct SwaggerConfig {
    pub openapi: String,
    pub info: Info,
    pub servers: Vec<Server>,
    pub paths: HashMap<String, Path>,
}

#[derive(Deserialize, Debug)]
pub struct Path {
    pub post: Option<RequestMethod>,
    pub get: Option<RequestMethod>,
}

#[derive(Deserialize, Debug)]
pub struct RequestMethod {
    pub tags: Vec<String>,
    pub summary: Option<String>,
}

impl SwaggerConfig {
    pub async fn from_url(url: &str) -> Result<Self> {
        let resp = reqwest::get(url).await?;
        let config = resp.json().await?;
        Ok(config)
    }

    pub async fn from_test() -> Result<Self> {
        // use tokio read swagger.json
        let mut f = File::open("swagger.json").await?;
        let mut content = String::new();
        f.read_to_string(&mut content).await?;
        let config: SwaggerConfig = serde_json::from_str(&content)?;
        Ok(config)
    }
}

#[derive(Deserialize, Debug)]
pub struct Info {
    pub title: String,
    pub version: String,
    pub description: String,
}

#[derive(Deserialize, Debug)]
pub struct Server {
    pub url: String,
    pub description: String,
}
