use clap::Parser;
use v030_domain::configuration::Configuration;
use v030_domain::app_builder::run_app;

fn main() -> anyhow::Result<()> {
   
    let configuration = Configuration::parse();
    run_app(configuration)?;
    
    Ok(())
}