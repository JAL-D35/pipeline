use clap::Parser;
use std::fs::File;

use crate::config;

#[derive(Parser)]
#[command(version)]
struct Args {
    // The path to configuration file
    #[arg(short, long, default_value = "./pipeline.yaml")]
    config: String,
}

pub struct Command {}

impl Command {
    pub fn run() {
        let args = Args::parse();

        let file = File::open(args.config).expect("Failed to open configuration file");

        let _config = config::Config::new(file);
    }
}
