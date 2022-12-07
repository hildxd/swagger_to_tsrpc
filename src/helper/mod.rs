use std::collections::HashMap;

pub fn uppercase_first_letter(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().chain(c).collect(),
    }
}

pub fn map_to_typescript(map: &HashMap<String, String>, requireds: Vec<String>) -> String {
    let mut result = String::new();
    for (key, value) in map {
        let mut type_ = value.clone();
        if type_ == "integer" {
            type_ = "number".to_string();
        }
        if requireds.contains(key) {
            result.push_str(&format!("{}: {};", key, type_));
        } else {
            result.push_str(&format!("{}?: {};", key, type_));
        }
    }
    result
}
