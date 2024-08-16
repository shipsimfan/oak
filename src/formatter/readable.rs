use crate::{LogFormatter, SerializedLogRecord};
use std::io::Write;
use time::{DateTime, SimpleTimeZone, TimeZone, Timestamp};

// rustdoc imports
#[allow(unused_imports)]
use crate::LogRecord;

/// Formats [`LogRecord`]s in a human readable way, including metadata
pub struct ReadableLogFormatter {
    /// The time zone
    time_zone: SimpleTimeZone,
}

impl ReadableLogFormatter {
    /// Creates a new [`CompactJSONLogFormatter`]
    pub fn new() -> Self {
        ReadableLogFormatter {
            time_zone: SimpleTimeZone::local(),
        }
    }
}

impl LogFormatter for ReadableLogFormatter {
    fn format(
        &mut self,
        output: &mut dyn Write,
        record: &SerializedLogRecord,
    ) -> std::io::Result<()> {
        let mut timestamp: Timestamp = record.timestamp().into();
        timestamp.change_timezone(self.time_zone);

        write!(
            output,
            "[{}][{}][{}] {}",
            record.level(),
            record.scope(),
            DateTime::from(timestamp).iso8601(),
            record.message()
        )
    }
}
