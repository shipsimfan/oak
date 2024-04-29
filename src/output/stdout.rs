use crate::{LogFormatter, LogOutput, ReadableLogFormatter, SerializedLogRecord};
use std::{
    borrow::Cow,
    io::{stdout, Stdout},
};

// rustdoc imports
#[allow(unused_imports)]
use crate::LogRecord;

/// Writes [`LogRecord`]s to [`stdout`]
pub struct StdoutLogOutput<F: LogFormatter> {
    /// The name of this output
    name: Cow<'static, str>,

    /// Standard out itself
    output: Stdout,

    /// The formatter styling the output
    formatter: F,
}

impl<F: LogFormatter> StdoutLogOutput<F> {
    /// Creates a new [`StdoutLogOutput`]
    pub fn new<S: Into<Cow<'static, str>>>(formatter: F, name: S) -> Self {
        StdoutLogOutput {
            output: stdout(),
            formatter,
            name: name.into(),
        }
    }
}

impl<F: LogFormatter> LogOutput for StdoutLogOutput<F> {
    fn name(&self) -> &str {
        &self.name
    }

    fn output(&mut self, record: &SerializedLogRecord) {
        self.formatter.format(&mut self.output.lock(), record).ok();
    }
}

impl Default for StdoutLogOutput<ReadableLogFormatter> {
    fn default() -> Self {
        StdoutLogOutput::new(ReadableLogFormatter::new(), "stdout")
    }
}
