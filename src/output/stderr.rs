use crate::{LogFormatter, LogOutput, ReadableLogFormatter, SerializedLogRecord};
use std::{
    borrow::Cow,
    io::{stderr, Stderr, Write},
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
    pub fn new<S: Into<Cow<'static, str>>>(formatter: F, name: S) -> Box<dyn LogOutput> {
        Box::new(StderrLogOutput {
            output: stderr(),
            formatter,
            name: name.into(),
        })
    }
}

impl StderrLogOutput<ReadableLogFormatter> {
    /// Creates a new [`StderrLogOutput`] with a default name and readable formatter
    pub fn default() -> Box<dyn LogOutput> {
        StderrLogOutput::new(ReadableLogFormatter::new(), "stderr")
    }
}

impl<F: LogFormatter> LogOutput for StderrLogOutput<F> {
    fn name(&self) -> &str {
        &self.name
    }

    fn output(&mut self, record: &SerializedLogRecord) {
        let mut output = self.output.lock();

        self.formatter.format(&mut output, record).ok();
        output.write_all(b"\n").ok();
    }
}
