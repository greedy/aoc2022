use std::path::PathBuf;
use std::ffi::OsStr;
use clap::Args;
use std::io::{BufRead, BufReader, Read};
use std::fs::File;
use std::io::{self, ErrorKind};

use color_eyre::eyre::Result;

mod cache;

#[derive(Clone)]
pub enum OverrideInputSource {
    Stdin,
    File(PathBuf)
}

impl From<&OsStr> for OverrideInputSource {
    fn from(s: &OsStr) -> Self {
        if s == "-" {
            Self::Stdin
        } else {
            Self::File(s.into())
        }
    }
}

#[derive(Args, Clone)]
pub struct InputCLI<const DAY: u32> {
    /// Do not use cached puzzle inputs
    #[arg(long)]
    refresh: bool,
    source: Option<OverrideInputSource>,
}

impl<const DAY: u32> InputCLI<DAY> {
    pub fn get_input_read(&self) -> Result<Box<dyn Read>> {
        match &self.source {
            Some(OverrideInputSource::Stdin) => Ok(Box::new(std::io::stdin())),
            Some(OverrideInputSource::File(path)) => Ok(Box::new(File::open(path)?)),
            None => {
                let cache = cache::Cache::default().ok_or_else(|| io::Error::new(ErrorKind::Other, "Couldn't create cache"))?;
                if self.refresh {
                    Ok(Box::new(cache.download_input(DAY)?))
                } else {
                    Ok(Box::new(cache.get_input(DAY)?))
                }
            }
        }
    }

    pub fn get_input(&self) -> Result<impl BufRead> {
        Ok(BufReader::new(self.get_input_read()?))
    }
}
