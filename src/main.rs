#![allow(unused)]

use anyhow::{Context, Result};
use clap::Parser;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Parser, Debug)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    let content = File::open(&args.path)
        .with_context(|| format!("Could not read file `{}`", &args.path.display()))?;
    let reader = BufReader::new(content);

    for line in reader.lines().map(|l| l.unwrap()) {
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }
    Ok(())
}
