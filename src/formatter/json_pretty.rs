use time::{DateTime, SimpleTimeZone, TimeZone, Timestamp};
use super::write_id;
use crate::{LogFormatter, SerializedLogRecord};
use std::io::Write;

// rustdoc imports
#[allow(unused_imports)]
use crate::LogRecord;

/// Formats [`LogRecord`]s as JSON using spacing and newlines to make it readable
pub struct PrettyJSONLogFormatter {
    /// The number of spaces to tab
    tab_size: usize,

    /// The time zone
    time_zone: SimpleTimeZone,
}

impl PrettyJSONLogFormatter {
    /// Creates a new [`PrettyJSONLogFormatter`]
    pub fn new(tab_size: usize) -> Self {
        PrettyJSONLogFormatter {
            tab_size,
            time_zone: SimpleTimeZone::local(),
        }
    }

    /// Writes a tab of indentation
    fn write_tab(&self, output: &mut dyn Write) -> std::io::Result<()> {
        for _ in 0..self.tab_size {
            output.write_all(b" ")?;
        }
        Ok(())
    }
}

impl LogFormatter for PrettyJSONLogFormatter {
    fn format(
        &mut self,
        output: &mut dyn Write,
        record: &SerializedLogRecord,
    ) -> std::io::Result<()> {
        output.write_all(b"{\n")?;

        self.write_tab(output)?;
        let mut timestamp: Timestamp = record.timestamp().into();
        timestamp.change_timezone(self.time_zone);
        write!(output, "timestamp: \"{}\",\n", DateTime::from(timestamp).iso8601())?;

        if let Some(trace_id) = record.trace_id() {
            self.write_tab(output)?;
            output.write_all(b"trace_id: \"")?;
            write_id(output, trace_id)?;
            output.write_all(b"\",\n")?;
        }

        if let Some(span_id) = record.span_id() {
            self.write_tab(output)?;
            output.write_all(b"span_id: \"")?;
            write_id(output, span_id)?;
            output.write_all(b"\",\n")?;
        }

        self.write_tab(output)?;
        write!(output, "level: \"{}\",\n", record.level())?;

        self.write_tab(output)?;
        write!(output, "resource: \"{}\",\n", record.resource())?;

        self.write_tab(output)?;
        write!(output, "scope: \"{}\",\n", record.scope())?;

        self.write_tab(output)?;
        write!(output, "message: \"{}\"\n", record.message())?;

        output.write_all(b"}")
    }
}

impl Default for PrettyJSONLogFormatter {
    fn default() -> Self {
        PrettyJSONLogFormatter::new(4)
    }
}
