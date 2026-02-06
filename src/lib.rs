use clap::Parser;

#[derive(Parser)]
#[command(version, author)]
struct Args {}

pub fn main() {
    Args::parse();
}
