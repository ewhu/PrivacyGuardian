// src/main.rs
/*
 * Main executable for PrivacyGuardian
 */

use clap::Parser;
use privacyguardian::{Result, run};

#[derive(Parser)]
#[command(version, about = "PrivacyGuardian - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose)
}
