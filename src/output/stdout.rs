use crate::{LogFormatter, LogOutput, SerializedLogRecord};
use std::io::{stdout, Stdout};

// rustdoc imports
#[allow(unused_imports)]
use crate::LogRecord;

/// Writes [`LogRecord`]s to [`stdout`]
pub struct StdoutLogOutput<F: LogFormatter> {
    /// Standard out itself
    output: Stdout,

    /// The formatter styling the output
    formatter: F,
}

impl<F: LogFormatter> StdoutLogOutput<F> {
    /// Creates a new [`StdoutLogOutput`]
    pub fn new(formatter: F) -> Self {
        StdoutLogOutput {
            output: stdout(),
            formatter,
        }
    }
}

impl<F: LogFormatter> LogOutput for StdoutLogOutput<F> {
    fn output(&mut self, record: &SerializedLogRecord) {
        self.formatter.format(&mut self.output.lock(), record).ok();
    }
}
