use std::{
  ffi::OsStr,
  fs,
  path::{Path, PathBuf},
  process::Command,
};

use anyhow::{format_err, Context, Error, Result};
use clap::{ArgGroup, Parser};

#[derive(Parser, Debug)]
#[clap(author, version, about)]
#[command(group(ArgGroup::new("mode").required(true)))]
struct Args {
  #[arg(short = 'x', long, value_name = "FILE", group = "mode")]
  extract: Option<PathBuf>,

  #[arg(short, long, value_name = "FILE", group = "mode")]
  compress: Option<PathBuf>,

  #[arg(short, long, value_name = "FILE", group = "mode")]
  list: Option<PathBuf>,

  files: Vec<PathBuf>,
}

/// Supported archive formats.
enum Format {
  TarGz,
  Zip,
}

impl TryFrom<&Path> for Format {
  type Error = Error;

  fn try_from(path: &Path) -> Result<Self, Self::Error> {
    match extension(path).as_deref() {
      Some("zip") => Ok(Format::Zip),
      Some("tar.gz") => Ok(Format::TarGz),
      Some(ext) => Err(format_err!("Unsupported extension: {:?}", ext)),

      None => Err(format_err!("No extension for file: {:?}", path)),
    }
  }
}

/// Returns extension of path, specifically everything after the second to last dot.
///
/// foo.zip -> zip
/// foo.tar.gz -> tar.gz
/// foo.x.tar.gz -> tar.gz
fn extension(path: &Path) -> Option<String> {
  let first = path.with_extension("");
  let first = first.extension().and_then(OsStr::to_str);
  let last = path.extension().and_then(OsStr::to_str);

  match (first, last) {
    (Some(first), Some(last)) => Some(format!("{}.{}", first, last)),
    (_, Some(last)) => Some(last.to_owned()),
    (_, _) => None,
  }
}

fn extract(src: PathBuf, dst: Option<PathBuf>) -> Result<()> {
  // Create a destination directory if none exists
  let dst = if let Some(dst) = dst {
    dst
  } else {
    let dst = src.with_extension("extracted");

    fs::create_dir(&dst).with_context(|| format!("Failed to create destination directory: {:?}", &dst))?;

    dst
  };

  match Format::try_from(src.as_path())? {
    Format::TarGz => {
      Command::new("tar").arg("-xf").arg(src).arg("--directory").arg(dst).spawn()?.wait()?;
    }

    Format::Zip => {
      Command::new("unzip").arg(src).arg("-d").arg(dst).spawn()?.wait()?;
    }
  }

  Ok(())
}

fn compress(srcs: &[PathBuf], dst: PathBuf) -> Result<()> {
  match Format::try_from(dst.as_path())? {
    Format::TarGz => {
      Command::new("tar").arg("-czf").arg(dst).args(srcs).spawn()?.wait()?;
    }

    Format::Zip => {
      Command::new("zip").arg("--recurse-paths").arg(dst).args(srcs).spawn()?.wait()?;
    }
  }

  Ok(())
}

fn list(src: PathBuf) -> Result<()> {
  match Format::try_from(src.as_path())? {
    Format::TarGz => {
      Command::new("tar").arg("-tvf").arg(src).spawn()?.wait()?;
    }

    Format::Zip => {
      Command::new("unzip").arg("-l").arg(src).spawn()?.wait()?;
    }
  }

  Ok(())
}

fn main() -> Result<()> {
  let args = Args::parse();

  if let Some(file) = args.extract {
    extract(file, args.files.first().cloned()).context("Failed to extract file")?;
  } else if let Some(file) = args.compress {
    compress(&args.files, file).context("Failed to compress file(s)")?;
  } else if let Some(file) = args.list {
    list(file).context("Failed to list contents of file")?;
  }

  Ok(())
}
