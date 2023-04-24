use std::{
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
  #[arg(short = 'x', long, value_name = "ARCHIVE", group = "mode")]
  /// Extracts the ARCHIVE to a ARCHIVE.extracted directory, or to the directory provided the next
  /// positional argument.
  extract: Option<PathBuf>,

  #[arg(short, long, value_name = "ARCHIVE", group = "mode")]
  /// Compress the PATHS into the ARCHIVE.
  compress: Option<PathBuf>,

  #[arg(short, long, value_name = "ARCHIVE", group = "mode")]
  /// List the contents of the ARCHIVE.
  list: Option<PathBuf>,

  /// The files or directories to compress, or the directory to extract to.
  #[arg(value_name = "PATHS")]
  files: Vec<PathBuf>,

  #[arg(long, value_name = "BIN")]
  /// Which tar utility to use.
  tar: Option<String>,

  #[arg(long, value_name = "BIN")]
  /// Which unzip utility to use.
  unzip: Option<String>,

  #[arg(long, value_name = "BIN")]
  /// Which zip utility to use.
  zip: Option<String>,
}

impl Args {
  fn tar_command(&self) -> Command {
    Command::new(self.tar.as_deref().unwrap_or("tar"))
  }

  fn unzip_command(&self) -> Command {
    Command::new(self.unzip.as_deref().unwrap_or("unzip"))
  }

  fn zip_command(&self) -> Command {
    Command::new(self.zip.as_deref().unwrap_or("zip"))
  }

  fn extract<P: AsRef<Path>>(&self, archive: P) -> Result<()> {
    let archive = archive.as_ref();
    // create directory if one doesn't exist
    let dir = if let Some(dir) = self.files.first() {
      dir.clone()
    } else {
      let dir = archive.with_extension("extracted");
      fs::create_dir(&dir).with_context(|| format!("Failed to create destination directory: {:?}", &dir))?;
      dir
    };

    let format = Format::try_from(archive)?;

    match format {
      Format::Tar | Format::TarGz => {
        self.tar_command().arg("-xvf").arg(archive).arg("--directory").arg(dir).spawn()?.wait()?;
      }

      Format::Zip => {
        self.unzip_command().arg(archive).arg("-d").arg(dir).spawn()?.wait()?;
      }
    }

    Ok(())
  }

  fn compress<P: AsRef<Path>>(&self, archive: P) -> Result<()> {
    let srcs = &self.files;
    let archive = archive.as_ref();

    match Format::try_from(archive)? {
      Format::Tar => {
        self.tar_command().arg("-cvf").arg(archive).args(srcs).spawn()?.wait()?;
      }

      Format::TarGz => {
        self.tar_command().arg("-czvf").arg(archive).args(srcs).spawn()?.wait()?;
      }

      Format::Zip => {
        self.zip_command().arg("--recurse-paths").arg(archive).args(srcs).spawn()?.wait()?;
      }
    }

    Ok(())
  }

  fn list<P: AsRef<Path>>(&self, archive: P) -> Result<()> {
    let archive = archive.as_ref();

    match Format::try_from(archive)? {
      Format::Tar | Format::TarGz => {
        self.tar_command().arg("-tvf").arg(archive).spawn()?.wait()?;
      }

      Format::Zip => {
        self.unzip_command().arg("-l").arg(archive).spawn()?.wait()?;
      }
    }

    Ok(())
  }
}

/// Supported archive formats.
enum Format {
  Tar,
  TarGz,
  Zip,
}

impl TryFrom<&Path> for Format {
  type Error = Error;

  fn try_from(path: &Path) -> Result<Self, Self::Error> {
    let path_str = path.to_string_lossy();

    if path_str.ends_with(".zip") {
      Ok(Format::Zip)
    } else if path_str.ends_with(".tar") {
      Ok(Format::Tar)
    } else if path_str.ends_with(".tar.gz") || path_str.ends_with(".tgz") {
      Ok(Format::TarGz)
    } else {
      Err(format_err!("Unsupported extension: {:?}", path))
    }
  }
}

fn main() -> Result<()> {
  let args = Args::parse();

  if let Some(archive) = &args.extract {
    args.extract(archive).context("Failed to extract file")?;
  } else if let Some(archive) = &args.compress {
    args.compress(archive).context("Failed to compress file(s)")?;
  } else if let Some(archive) = &args.list {
    args.list(archive).context("Failed to list contents of file")?;
  }

  Ok(())
}
