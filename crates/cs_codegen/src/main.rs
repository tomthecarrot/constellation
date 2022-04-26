use clap::Parser;

#[derive(Parser)]
#[clap(version, about)]
struct Cli {}

fn main() {
    let cli = Cli::parse();
}
