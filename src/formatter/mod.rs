use crate::LogRecordMetadata;
use common::{write_id, write_time};
use std::io::Write;

mod common;
mod json_compact;
mod json_pretty;
mod message_only;
mod readable;

pub use json_compact::CompactJSONLogFormatter;
pub use json_pretty::PrettyJSONLogFormatter;
pub use message_only::MessageOnlyLogFormatter;
pub use readable::ReadableLogFormatter;

// rustdoc imports
#[allow(unused_imports)]
use crate::{LogOutput, LogRecord};

/// Formats [`LogRecord`]s for [`LogOutput`]s that require formatting
pub trait LogFormatter: 'static + Send {
    /// Write a `message` and `metadata` to `output` in a formatted way
    fn format(
        &mut self,
        output: &mut dyn Write,
        metadata: &LogRecordMetadata,
        message: &str,
    ) -> std::io::Result<()>;
}
