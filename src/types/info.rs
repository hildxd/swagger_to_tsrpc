use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Info {
    pub title: String,
    pub version: String,
    pub description: String,
}
