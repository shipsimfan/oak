use super::write_id;
use crate::{LogFormatter, SerializedLogRecord};
use std::io::Write;
use time::{DateTime, SimpleTimeZone, TimeZone, Timestamp};

// rustdoc imports
#[allow(unused_imports)]
use crate::LogRecord;

/// Formats [`LogRecord`]s as JSON using no extra spacing or newlines
pub struct CompactJSONLogFormatter {
    /// The time zone
    time_zone: SimpleTimeZone,
}

impl CompactJSONLogFormatter {
    /// Creates a new [`CompactJSONLogFormatter`]
    pub fn new() -> Self {
        CompactJSONLogFormatter { time_zone: SimpleTimeZone::local() }
    }
}

impl LogFormatter for CompactJSONLogFormatter {
    fn format(
        &mut self,
        output: &mut dyn Write,
        record: &SerializedLogRecord,
    ) -> std::io::Result<()> {
        output.write_all(b"{")?;

        let mut timestamp: Timestamp = record.timestamp().into();
        timestamp.change_timezone(self.time_zone);
        write!(
            output,
            "timestamp: \"{}\",",
            DateTime::from(timestamp).iso8601()
        )?;

        if let Some(trace_id) = record.trace_id() {
            output.write_all(b"trace_id:\"")?;
            write_id(output, trace_id)?;
            output.write_all(b"\",")?;
        }

        if let Some(span_id) = record.span_id() {
            output.write_all(b"span_id:\"")?;
            write_id(output, span_id)?;
            output.write_all(b"\",")?;
        }

        write!(output, "level:\"{}\",", record.level())?;

        write!(output, "resource:\"{}\",", record.resource())?;

        write!(output, "scope:\"{}\",", record.scope())?;

        write!(output, "message:\"{}\"", record.message())?;

        output.write_all(b"}\n")
    }
}

impl Default for CompactJSONLogFormatter {
    fn default() -> Self {
        CompactJSONLogFormatter::new()
    }
}
