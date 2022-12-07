use std::collections::HashMap;

use crate::core::request::Request;
use serde::Deserialize;

pub type Path = HashMap<String, RequestMethod>;

pub fn generate_requests(path: &Path) -> Vec<Request> {
    let mut requests = Vec::new();
    for (path, method) in path {
        let request = Request::new(path, method);
        requests.push(request);
    }
    requests
}

#[derive(Deserialize, Debug)]
pub struct RequestMethod {
    pub tags: Vec<String>,
    pub summary: Option<String>,
    #[serde(rename = "operationId")]
    pub operation_id: String,
    #[serde(rename = "requestBody")]
    pub request_body: Option<RequestBody>,
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

impl MediaType {
    pub fn get_component_key(&self) -> String {
        self.schema
            .ref_
            .clone()
            .split("/")
            .last()
            .unwrap()
            .to_string()
    }
}

#[derive(Deserialize, Debug)]
pub struct Schema {
    #[serde(rename = "$ref")]
    pub ref_: String,
}
