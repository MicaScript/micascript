use clap::{Args, Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "MicaScript")]
#[command(about = "MicaScript: a JavaScript compiler and runtime", long_about = None)]
#[command(version, arg_required_else_help = true)]
pub struct Cli {
  // pub file_path: Option<PathBuf>,
  #[command(subcommand)]
  pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
  #[command(about = "Run a JavaScript file with MicaScript", long_about = None)]
  Run(RunArgs),
  // TODO: add init command
}

#[derive(Args)]
pub struct RunArgs {
  pub file_path: PathBuf,

  #[arg(long, visible_alias = "esm")]
  pub es_module: bool,
}

pub fn parse_args() -> Cli {
  Cli::parse()
}
