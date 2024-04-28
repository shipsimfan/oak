// rustdoc imports
#[allow(unused_imports)]
use crate::LogRecord;

mod file;
mod otlp;
mod stderr;
mod stdout;

pub use file::FileLogOutput;
pub use otlp::OTLPLogOutput;
pub use stderr::StderrLogOutput;
pub use stdout::StdoutLogOutput;

/// An output for [`LogRecord`]s
pub trait LogOutput: 'static + Send {}
