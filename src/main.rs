use std::path::PathBuf;

use clap::{ArgGroup, Parser};

#[derive(Parser, Debug)]
#[clap(author, version, about)]
#[command(group(ArgGroup::new("mode").required(true)))]
struct Args {
  #[arg(short, long, value_name = "FILE", group = "mode")]
  output: Option<PathBuf>,

  #[arg(short = 'x', long, value_name = "FILE", group = "mode")]
  extract: Option<PathBuf>,

  files: Vec<PathBuf>,
}

fn extract(src: PathBuf, dst: PathBuf) {}
fn output(files: Vec<PathBuf>, output: PathBuf) {}

fn main() {
  let args = Args::parse();
}
