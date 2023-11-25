const VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn run() {
    version();
}
fn version() {
    println!("pipeline {}", VERSION);
}
