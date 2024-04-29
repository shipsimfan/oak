use super::{write_id, write_time};
use crate::{LogFormatter, SerializedLogRecord};
use std::io::Write;

// rustdoc imports
#[allow(unused_imports)]
use crate::LogRecord;

/// Formats [`LogRecord`]s as JSON using spacing and newlines to make it readable
pub struct PrettyJSONLogFormatter {
    /// The number of spaces to tab
    tab_size: usize,

    /// The timezone offset in minutes
    offset: i16,
}

impl PrettyJSONLogFormatter {
    /// Creates a new [`PrettyJSONLogFormatter`]
    pub const fn new(tab_size: usize) -> Self {
        PrettyJSONLogFormatter {
            tab_size,
            offset: 0,
        }
    }

    /// Sets the timezone `offset` in minutes
    pub const fn set_offset(mut self, offset: i16) -> Self {
        self.offset = offset;
        self
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
        output.write_all(b"timestamp: \"")?;
        write_time(output, record.timestamp(), self.offset)?;
        output.write_all(b"\",\n")?;

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

        output.write_all(b"}\n")
    }
}

impl Default for PrettyJSONLogFormatter {
    fn default() -> Self {
        PrettyJSONLogFormatter::new(4)
    }
}
