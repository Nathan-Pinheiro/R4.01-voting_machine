use clap::Parser;
use v040_memory::configuration::Configuration;
use v040_memory::app_builder::run_app;
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let configuration = Configuration::parse();
    run_app(configuration).await?;
    Ok(())
}