use clap::Parser;
use v041_file::configuration::Configuration;
use v041_file::app_builder::run_app;
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let configuration = Configuration::parse();
    run_app(configuration).await?;
    Ok(())
}