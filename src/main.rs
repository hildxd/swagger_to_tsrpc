use anyhow::Result;
use spinners::{Spinner, Spinners};

mod core;

#[tokio::main]
async fn main() -> Result<()> {
    let mut sp = Spinner::new(Spinners::Dots9, "Loading Config".into());
    let config =
        core::config::SwaggerConfig::from_url("http://192.168.0.124:12001/api/system/v3/api-docs")
            .await?;
    // let config = core::config::SwaggerConfig::from_test().await?;
    sp.stop();
    println!("Config: {:#?}", config);
    Ok(())
}
