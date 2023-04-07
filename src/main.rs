#![allow(unused)]

use anyhow::{Context, Result};
use clap::Parser;
use log::{info, trace, warn};
use std::fs::File;
use std::io::{BufRead, BufReader};

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
    let content = File::open(&args.path)
        .with_context(|| format!("Could not read file `{}`", &args.path.display()))?;
    trace!("starting up");
    let reader = BufReader::new(content);
    info!("Display lines warn {:?}", reader);
    for line in reader.lines().map(|l| l.unwrap()) {
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }
    Ok(())
}
