use std::collections::HashMap;

use serde::Deserialize;

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

impl Parameter {
    fn to_ts_string(&self) -> String {
        let mut s = String::new();
        if self.required {
            s.push_str(&format!("{}: ", self.name));
        } else {
            s.push_str(&format!("{}?: ", self.name));
        }
        if let Some(schema) = &self.schema {
            s.push_str(&schema.type_);
        }
        s
    }
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
