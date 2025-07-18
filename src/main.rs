// src/main.rs
/*
 * Main executable for ArtificialEaselFrameworkUltra
 */

use clap::Parser;
use artificialeaselframeworkultra::{Result, run};

#[derive(Parser)]
#[command(version, about = "ArtificialEaselFrameworkUltra - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose)
}
