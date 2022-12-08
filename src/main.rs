use std::collections::HashMap;

use anyhow::{Ok, Result};
use spinners::{Spinner, Spinners};

mod core;
mod helper;
mod types;

#[tokio::main]
async fn main() -> Result<()> {
    init().await?;
    let mut sp = Spinner::new(Spinners::Dots9, "Loading Config".into());
    let config =
        types::SwaggerConfig::from_url("http://192.168.0.124:12001/api/system/v3/api-docs").await?;
    // let config = core::types::SwaggerConfig::from_test().await?;
    let mut category_req: HashMap<String, Vec<core::request::Request>> = HashMap::new();
    for (url, path_item) in config.paths {
        let requests = types::path::generate_requests(&url, &path_item, &config.components);
        for request in requests {
            let category = request.category.clone();
            let mut reqs = category_req.entry(category).or_insert(Vec::new());
            reqs.push(request);
        }
    }
    println!("{:#?}", category_req);
    sp.stop();
    Ok(())
}

async fn init() -> Result<()> {
    tokio::fs::remove_dir_all("api").await?;
    tokio::fs::create_dir("api").await?;
    Ok(())
}
