use std::collections::HashMap;

use anyhow::{Ok, Result};
use serde::Deserialize;
use tokio::{fs::File, io::AsyncReadExt};

use super::component::Components;
use super::info::Info;
use super::path::Path;
use super::server::Server;
use super::tag::Tag;

#[derive(Deserialize, Debug)]
pub struct SwaggerConfig {
    pub openapi: String,
    pub info: Info,
    pub servers: Vec<Server>,
    pub paths: HashMap<String, Path>,
    pub tags: Vec<Tag>,
    pub components: Components,
}

impl SwaggerConfig {
    pub async fn from_url(url: &str) -> Result<Self> {
        let resp = reqwest::get(url).await?;
        let config = resp.json().await?;
        Ok(config)
    }

    pub async fn from_test() -> Result<Self> {
        let mut f = File::open("swagger.json").await?;
        let mut content = String::new();
        f.read_to_string(&mut content).await?;
        let config: SwaggerConfig = serde_json::from_str(&content)?;
        Ok(config)
    }
}
