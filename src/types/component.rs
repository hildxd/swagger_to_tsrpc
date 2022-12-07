use std::collections::HashMap;

use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Components {
    pub schemas: HashMap<String, ComponentSchema>,
    #[serde(rename = "securitySchemes")]
    pub security_schemes: HashMap<String, SecurityScheme>,
}

impl Components {
    fn get_schema_for_key(&self, key: &str) -> Option<&ComponentSchema> {
        self.schemas.get(key)
    }

    pub fn get_type_map(&self, key: &str) -> Option<HashMap<String, String>> {
        let schema = self.get_schema_for_key(key)?;
        let mut map = HashMap::new();
        for (key, value) in &schema.properties {
            let type_ = value.type_.clone().unwrap_or_else(|| "string".to_string());
            // todo! component 转 ts string
            map.insert(key.clone(), type_);
        }
        Some(map)
    }
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
