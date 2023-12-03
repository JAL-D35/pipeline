use clap::Parser;

#[derive(Parser)]
#[command(version)]
struct Args {}

pub fn run() {
    let _args = Args::parse();
}
