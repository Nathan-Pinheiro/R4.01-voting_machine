use clap::Parser;
use v040_memory::configuration::Configuration;
use v040_memory::app_builder::run_app;

fn main() -> anyhow::Result<()> {
   
    let configuration = Configuration::parse();
    run_app(configuration)?;
    
    Ok(())
}