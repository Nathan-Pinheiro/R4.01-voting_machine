use clap::Parser;
use clap::ValueEnum;

#[derive(Clone, Copy, ValueEnum)]
pub enum StorageType {
    Memory,
    File,
}

#[derive(Parser)]
pub struct Configuration {
    #[arg(short = 'c', long, required = true, num_args = 1..)]
    pub candidates: Vec<String>,

    #[arg(short = 's', long, default_value = "memory")]
    pub storage_type: StorageType,
}