use clap::Parser;
use std::path::PathBuf;

use anyhow::Result;

#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None)]
pub struct RunConfig {
    #[arg(short, long, required = false, default_value = ".")]
    pub target: PathBuf,
}

pub fn parse_args() -> Result<RunConfig> {
    Ok(RunConfig::parse())
}
