use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Tag {
    pub name: String,
    pub description: String,
}
