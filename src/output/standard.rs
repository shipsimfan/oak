use crate::{FileLogOutput, LogOutput, ReadableLogFormatter, StderrLogOutput, StdoutLogOutput};
use std::{convert::Infallible, path::PathBuf, str::FromStr};

/// The standard log outputs
pub enum StdLogOutput {
    /// Standard out
    Stdout,

    /// Standard error
    Stderr,

    /// A file at the path
    File(PathBuf),
}

impl StdLogOutput {
    /// Converts a [`Vec`] of this into a [`Vec`] of `Box<dyn LogOutput>`s
    pub fn convert_vec(
        outputs: Vec<StdLogOutput>,
    ) -> Result<Vec<Box<dyn LogOutput>>, std::io::Error> {
        outputs.into_iter().map(TryInto::try_into).collect()
    }
}

impl FromStr for StdLogOutput {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "stderr" => StdLogOutput::Stderr,
            "stdout" => StdLogOutput::Stdout,
            _ => StdLogOutput::File(s.into()),
        })
    }
}

impl TryInto<Box<dyn LogOutput>> for StdLogOutput {
    type Error = std::io::Error;

    fn try_into(self) -> std::io::Result<Box<dyn LogOutput>> {
        match self {
            StdLogOutput::Stdout => Ok(StdoutLogOutput::default()),
            StdLogOutput::Stderr => Ok(StderrLogOutput::default()),
            StdLogOutput::File(path) => {
                FileLogOutput::open(path, ReadableLogFormatter::new(), "file")
            }
        }
    }
}
