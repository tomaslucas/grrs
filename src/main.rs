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

fn find_matches(content: &str, pattern: &str, mut writer: impl std::io::Write) -> Result<()> {
    for line in content.lines() {
        if line.contains(pattern) {
            writeln!(writer, "{}", line)
                .with_context(|| format!("Failed to write line: {}", line))?;
        }
    }
    Ok(())
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

    find_matches(&content, &args.pattern, &mut std::io::stdout())
        .with_context(|| "Failed to find matches")?;

    Ok(())
}

#[test]
fn find_a_match() {
    let mut result = Vec::new();
    find_matches("lorem ipsum\ndolor sit amet", "lorem", &mut result);
    assert_eq!(result, b"lorem ipsum\n");
}
