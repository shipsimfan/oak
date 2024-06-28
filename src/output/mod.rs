use crate::SerializedLogRecord;

// rustdoc imports
#[allow(unused_imports)]
use crate::LogRecord;

mod file;
mod otlp;
mod standard;
mod stderr;
mod stdout;

pub use file::FileLogOutput;
pub use otlp::OTLPLogOutput;
pub use standard::StdLogOutput;
pub use stderr::StderrLogOutput;
pub use stdout::StdoutLogOutput;

/// An output for [`LogRecord`]s
pub trait LogOutput: 'static + Send {
    /// Gets the name of this output
    fn name(&self) -> &str;

    /// Write `record` to this output
    fn output(&mut self, record: &SerializedLogRecord);
}
