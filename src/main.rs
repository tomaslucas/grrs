#![allow(unused)]

use anyhow::{Context, Ok, Result};
use clap::Parser;
use log::{info, trace, warn};
use std::fs::File;

#[derive(Parser, Debug)]
struct Cli {
    #[clap(flatten)]
    verbose: clap_verbosity_flag::Verbosity,
    pattern: String,
    path: std::path::PathBuf,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    env_logger::Builder::new()
        .filter_level(args.verbose.log_level_filter())
        .init();

    info!("Verify if we can open the file `{}`", args.path.display());
    let content = std::fs::read_to_string(&args.path)
        .with_context(|| format!("Could not read file `{}`", &args.path.display()))?;

    info!("Display the content of the file: {:?}", content);

    grrs::find_matches(&content, &args.pattern, &mut std::io::stdout())
        .with_context(|| "Failed to find matches")?;

    Ok(())
}
