use std::collections::HashMap;

use serde::Deserialize;

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
