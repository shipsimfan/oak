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

/// An error that can occur while opening a standard log output
#[derive(Debug)]
pub struct OpenStdLogOutputError(std::io::Error, PathBuf);

impl StdLogOutput {
    /// Converts a [`Vec`] of this into a [`Vec`] of `Box<dyn LogOutput>`s
    pub fn convert_vec(
        outputs: &[StdLogOutput],
    ) -> Result<Vec<Box<dyn LogOutput>>, OpenStdLogOutputError> {
        outputs.iter().map(StdLogOutput::into_log_output).collect()
    }

    /// Get open a [`LogOutput`] from this
    pub fn into_log_output(&self) -> Result<Box<dyn LogOutput>, OpenStdLogOutputError> {
        match self {
            StdLogOutput::Stdout => Ok(StdoutLogOutput::default()),
            StdLogOutput::Stderr => Ok(StderrLogOutput::default()),
            StdLogOutput::File(path) => {
                FileLogOutput::open(path, ReadableLogFormatter::new(), "file")
                    .map_err(|error| OpenStdLogOutputError(error, path.clone()))
            }
        }
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

impl std::error::Error for OpenStdLogOutputError {}

impl std::fmt::Display for OpenStdLogOutputError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "unable to open log output \"{}\" - {}",
            self.1.display(),
            self.0
        )
    }
}
