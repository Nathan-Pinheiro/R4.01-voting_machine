use clap::Parser;
use v050_use_cases::configuration::Configuration;
use v050_use_cases::app_builder::run_app;
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let configuration = Configuration::parse();
    run_app(configuration).await?;
    Ok(())
}