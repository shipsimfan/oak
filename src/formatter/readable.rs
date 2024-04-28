use super::write_time;
use crate::{LogFormatter, LogRecordMetadata};
use std::io::Write;

// rustdoc imports
#[allow(unused_imports)]
use crate::LogRecord;

/// Formats [`LogRecord`]s in a human readable way, including metadata
pub struct ReadableLogFormatter;

impl LogFormatter for ReadableLogFormatter {
    fn format(
        &mut self,
        output: &mut dyn Write,
        metadata: &LogRecordMetadata,
        message: &str,
    ) -> std::io::Result<()> {
        write!(output, "[{}][{}][", metadata.level(), metadata.scope())?;

        write_time(output, metadata.timestamp())?;

        write!(output, "] {}", message)
    }
}
