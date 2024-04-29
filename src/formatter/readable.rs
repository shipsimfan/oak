use super::write_time;
use crate::{LogFormatter, SerializedLogRecord};
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
        record: SerializedLogRecord,
    ) -> std::io::Result<()> {
        write!(output, "[{}][{}][", record.level(), record.scope())?;

        write_time(output, record.timestamp(), self.offset)?;

        write!(output, "] {}\n", record.message())
    }
}
