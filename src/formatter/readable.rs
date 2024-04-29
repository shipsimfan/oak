use super::write_time;
use crate::{LogFormatter, LogRecordMetadata};
use std::io::Write;

// rustdoc imports
#[allow(unused_imports)]
use crate::LogRecord;

/// Formats [`LogRecord`]s in a human readable way, including metadata
pub struct ReadableLogFormatter {
    /// The timezone offset in minutes
    offset: i16,
}

impl ReadableLogFormatter {
    /// Creates a new [`CompactJSONLogFormatter`]
    pub const fn new() -> Self {
        ReadableLogFormatter { offset: 0 }
    }

    /// Sets the timezone `offset` in minutes
    pub const fn set_offset(mut self, offset: i16) -> Self {
        self.offset = offset;
        self
    }
}

impl LogFormatter for ReadableLogFormatter {
    fn format(
        &mut self,
        output: &mut dyn Write,
        metadata: &LogRecordMetadata,
        message: &str,
    ) -> std::io::Result<()> {
        write!(output, "[{}][{}][", metadata.level(), metadata.scope())?;

        write_time(output, metadata.timestamp(), self.offset)?;

        write!(output, "] {}", message)
    }
}
