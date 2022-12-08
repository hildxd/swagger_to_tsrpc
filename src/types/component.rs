use std::collections::HashMap;

use serde::Deserialize;

use crate::helper;

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

    pub fn get_type_map(&self, key: &str, is_response: bool) -> Option<String> {
        let schema = self.get_schema_for_key(key)?;
        let mut map = HashMap::new();
        for (key, value) in &schema.properties {
            if (value.ref_.is_some() || value.type_.is_none()) && is_response {
                let ref_ = value.ref_.clone().unwrap();
                let ref_key = ref_.split("/").last().unwrap();
                let ref_schema = self.get_schema_for_key(ref_key)?;
                let ref_type = ref_schema.properties;
                map.insert(key.clone(), ref_type);
                continue;
            }
            let type_ = value.type_.clone().unwrap_or_else(|| "string".to_string());
            map.insert(key.clone(), type_);
        }

        let result = helper::map_to_typescript(
            &map,
            schema.required.as_ref().unwrap_or(&Vec::new()),
            is_response,
        );
        Some(result)
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
    #[serde(rename = "$ref")]
    pub ref_: Option<String>,
}
