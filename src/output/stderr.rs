use crate::{LogFormatter, LogOutput, SerializedLogRecord};
use std::{
    borrow::Cow,
    io::{stderr, Stderr},
};

// rustdoc imports
#[allow(unused_imports)]
use crate::LogRecord;

/// Writes [`LogRecord`]s to [`stderr`]
pub struct StderrLogOutput<F: LogFormatter> {
    /// The name of this output
    name: Cow<'static, str>,

    /// Standard error itself
    output: Stderr,

    /// The formatter styling the output
    formatter: F,
}

impl<F: LogFormatter> StderrLogOutput<F> {
    /// Creates a new [`StderrLogOutput`]
    pub fn new<S: Into<Cow<'static, str>>>(formatter: F, name: S) -> Self {
        StderrLogOutput {
            output: stderr(),
            formatter,
            name: name.into(),
        }
    }
}

impl<F: LogFormatter> LogOutput for StderrLogOutput<F> {
    fn name(&self) -> &str {
        &self.name
    }

    fn output(&mut self, record: &SerializedLogRecord) {
        self.formatter.format(&mut self.output.lock(), record).ok();
    }
}
