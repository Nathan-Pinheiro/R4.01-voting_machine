use clap::Parser;
use v021_app_builder::configuration::Configuration;
use v021_app_builder::app_builder::run_app;

fn main() -> anyhow::Result<()> {
   
    let configuration = Configuration::parse();
    run_app(configuration)?;
    
    Ok(())
}