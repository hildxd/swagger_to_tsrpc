use std::collections::HashMap;

use crate::{helper::uppercase_first_letter, types::path::RequestMethod};

#[derive(Debug)]
pub enum RequestType {
    GET,
    POST,
    PUT,
    DELETE,
    PATCH,
    UNKNOW,
}

#[derive(Debug)]
pub struct Request {
    pub r_type: RequestType,
    pub name: String,
    pub description: Option<String>,
    pub category: String,
    pub body: Option<HashMap<String, String>>,
    pub query: Option<HashMap<String, String>>,
    pub response: Option<HashMap<String, String>>,
}

impl Request {
    pub fn new(path: &str, method: &RequestMethod) -> Request {
        let r_type: RequestType = match path.to_lowercase().as_str() {
            "post" => RequestType::POST,
            "get" => RequestType::GET,
            "put" => RequestType::PUT,
            "delete" => RequestType::DELETE,
            "patch" => RequestType::PATCH,
            _ => RequestType::UNKNOW,
        };
        let mut request = Request {
            r_type,
            name: uppercase_first_letter(&method.operation_id),
            description: method.summary.clone(),
            category: method.tags[0].clone(),
            body: None,
            query: None,
            response: None,
        };
        request.init_query(method);
        request.init_body(method);
        request
    }
    fn init_body(&mut self, request: &RequestMethod) {
        if let Some(request_body) = &request.request_body {
            let mut body = HashMap::new();
            for (_, value) in &request_body.content {
                let componet_key = value.get_component_key();
                // body.insert(key.clone(), value.schema.ref_.clone());
            }
            self.body = Some(body);
        }
    }
    fn init_query(&mut self, request: &RequestMethod) {
        if let Some(parameters) = &request.parameters {
            let mut query = HashMap::new();
            for parameter in parameters {
                let key = parameter.name.clone();
                let mut value = String::new();
                if parameter.in_ == "query" {
                    if let Some(schema) = &parameter.schema {
                        value = schema.type_.clone();
                    } else {
                        value = "string".to_string();
                    }
                }
                query.insert(key, value);
            }
            self.query = Some(query);
        }
    }
}
