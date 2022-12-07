use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Server {
    pub url: String,
    pub description: String,
}
