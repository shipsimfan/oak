use crate::{LogFormatter, LogOutput, SerializedLogRecord};
use std::io::{stderr, Stderr};

// rustdoc imports
#[allow(unused_imports)]
use crate::LogRecord;

/// Writes [`LogRecord`]s to [`stderr`]
pub struct StderrLogOutput<F: LogFormatter> {
    /// Standard error itself
    output: Stderr,

    /// The formatter styling the output
    formatter: F,
}

impl<F: LogFormatter> StderrLogOutput<F> {
    /// Creates a new [`StderrLogOutput`]
    pub fn new(formatter: F) -> Self {
        StderrLogOutput {
            output: stderr(),
            formatter,
        }
    }
}

impl<F: LogFormatter> LogOutput for StderrLogOutput<F> {
    fn output(&mut self, record: &SerializedLogRecord) {
        self.formatter.format(&mut self.output.lock(), record).ok();
    }
}
