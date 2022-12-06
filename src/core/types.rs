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
    pub tags: Vec<Tag>,
    pub components: Components,
}

#[derive(Deserialize, Debug)]
pub struct Components {
    pub schemas: HashMap<String, ComponentSchema>,
    #[serde(rename = "securitySchemes")]
    pub security_schemes: HashMap<String, SecurityScheme>,
}
#[derive(Deserialize, Debug)]
pub struct SecurityScheme {
    #[serde(rename = "type")]
    pub type_: Option<String>,
    pub name: String,
    #[serde(rename = "in")]
    pub in_: String,
}

#[derive(Deserialize, Debug)]
pub struct ComponentSchema {
    pub required: Option<Vec<String>>,
    #[serde(rename = "type")]
    pub type_: String,
    pub properties: HashMap<String, Property>,
    pub description: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct Property {
    #[serde(rename = "type")]
    pub type_: Option<String>,
    pub format: Option<String>,
    pub description: Option<String>,
}
#[derive(Deserialize, Debug)]
pub struct Tag {
    pub name: String,
    pub description: String,
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
    pub operationId: String,
    pub requestBody: Option<RequestBody>,
    pub responses: HashMap<String, ResponseBody>,
    pub parameters: Option<Vec<Parameter>>,
}

#[derive(Deserialize, Debug)]
pub struct Parameter {
    pub name: String,
    #[serde(rename = "in")]
    pub in_: String,
    pub required: bool,
    pub schema: Option<ParameterSchema>,
}

#[derive(Deserialize, Debug)]
pub struct ParameterSchema {
    #[serde(rename = "type")]
    pub type_: String,
    pub format: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct ResponseBody {
    pub description: String,
    pub content: HashMap<String, MediaType>,
}

#[derive(Deserialize, Debug)]
pub struct RequestBody {
    pub content: HashMap<String, MediaType>,
}

#[derive(Deserialize, Debug)]
pub struct MediaType {
    pub schema: Schema,
}

#[derive(Deserialize, Debug)]
pub struct Schema {
    #[serde(rename = "$ref")]
    pub ref_: String,
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
