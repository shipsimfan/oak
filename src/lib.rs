//! A logging library

#![deny(missing_docs)]
#![deny(rustdoc::private_intra_doc_links)]
#![deny(rustdoc::unescaped_backticks)]
#![deny(rustdoc::redundant_explicit_links)]
#![warn(rustdoc::broken_intra_doc_links)]

mod controller;
mod formatter;
mod logger;
mod output;
mod record;
mod writer;

pub use controller::LogController;
pub use formatter::{
    CompactJSONLogFormatter, LogFormatter, MessageOnlyLogFormatter, PrettyJSONLogFormatter,
    ReadableLogFormatter,
};
pub use logger::Logger;
pub use output::{FileLogOutput, LogOutput, OTLPLogOutput, StderrLogOutput, StdoutLogOutput};
pub use record::{LogLevel, LogRecord, LogRecordMetadata};

use record::SerializedLogRecord;
use writer::LogWriter;
