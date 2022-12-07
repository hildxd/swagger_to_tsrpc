use anyhow::Result;
use serde::Deserialize;
use tokio::{fs::File, io::AsyncWriteExt};

use super::swagger::SwaggerConfig;

#[derive(Deserialize, Debug)]
pub struct Tag {
    pub name: String,
    pub description: String,
}

impl Tag {
    pub async fn write_file(&self) -> Result<()> {
        let mut f = File::create(format!("api/{}1.rs", self.name)).await?;
        f.write_all(format!("// Path: src/{}.rs", self.name).as_bytes())
            .await?;
        Ok(())
    }

    pub fn get_all_schema(&self, config: &SwaggerConfig) -> Vec<String> {
        let mut schemas = Vec::new();
        for (_key, method) in &config.paths {
            if let Some(method) = method.post.as_ref() {
                if method.tags.contains(&self.name) {
                    if let Some(request_body) = &method.requestBody {
                        if let Some(media_type) = request_body.content.get("application/json") {
                            let schema = &media_type.schema;
                            let schema_name = schema.ref_.split("/").last().unwrap();
                            schemas.push(schema_name.to_string());
                        }
                    }
                }
            }
            if let Some(method) = method.get.as_ref() {
                if method.tags.contains(&self.name) {
                    if let Some(parameters) = &method.parameters {
                        for parameter in parameters {
                            if parameter.in_ == "query" {
                                if let Some(schema) = &parameter.schema {
                                    schemas.push(schema.type_.clone());
                                }
                            }
                        }
                    }
                }
            }
        }
        schemas
    }
}
