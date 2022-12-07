use anyhow::{Ok, Result};
use spinners::{Spinner, Spinners};

mod core;
mod types;

#[tokio::main]
async fn main() -> Result<()> {
    init().await?;
    let mut sp = Spinner::new(Spinners::Dots9, "Loading Config".into());
    let config =
        types::SwaggerConfig::from_url("http://192.168.0.124:12001/api/system/v3/api-docs").await?;
    // let config = core::types::SwaggerConfig::from_test().await?;

    sp.stop();
    // config.components.schemas.iter().for_each(|(k, v)| {
    //     println!("{}: {:#?}", k, v);
    // });
    for tag in config.tags.iter() {
        // tag.write_file().await?;
        let schemas = tag.get_all_schema(&config);
        println!("{}: {:#?}", tag.name, schemas);
    }
    Ok(())
}

async fn init() -> Result<()> {
    tokio::fs::remove_dir_all("api").await?;
    tokio::fs::create_dir("api").await?;
    Ok(())
}
