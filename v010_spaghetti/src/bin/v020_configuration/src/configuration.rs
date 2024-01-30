use clap::Parser;

#[derive(Parser)]
struct Configuration {
    #[args(short, long, required = true, num_args = 1..)]
    candidates: Vec<String>,
}