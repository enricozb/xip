use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about)]
struct Args {
  #[clap(short, long, conflicts_with = "extract")]
  output: Option<String>,
  #[clap(short, long, conflicts_with = "output")]
  extract: Option<String>,
  #[clap(required = true)]
  files: Vec<String>,
}

fn main() {
  let _ = Args::parse();
}
