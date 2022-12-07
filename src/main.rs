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
    for (_url, path_item) in config.paths {
        let requests = types::path::generate_requests(&path_item);
        println!("{:#?}", requests);
    }

    sp.stop();
    Ok(())
}

async fn init() -> Result<()> {
    tokio::fs::remove_dir_all("api").await?;
    tokio::fs::create_dir("api").await?;
    Ok(())
}
