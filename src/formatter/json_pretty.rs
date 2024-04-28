use super::{write_id, write_time};
use crate::{LogFormatter, LogRecordMetadata};
use std::io::Write;

// rustdoc imports
#[allow(unused_imports)]
use crate::LogRecord;

/// Formats [`LogRecord`]s as JSON using spacing and newlines to make it readable
pub struct PrettyJSONLogFormatter {
    /// The number of spaces to tab
    tab_size: usize,
}

impl PrettyJSONLogFormatter {
    /// Creates a new [`PrettyJSONLogFormatter`]
    pub const fn new(tab_size: usize) -> Self {
        PrettyJSONLogFormatter { tab_size }
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
        metadata: &LogRecordMetadata,
        message: &str,
    ) -> std::io::Result<()> {
        output.write_all(b"{\n")?;

        self.write_tab(output)?;
        output.write_all(b"timestamp: \"")?;
        write_time(output, metadata.timestamp())?;
        output.write_all(b"\",\n")?;

        if let Some(trace_id) = metadata.trace_id() {
            self.write_tab(output)?;
            output.write_all(b"trace_id: \"")?;
            write_id(output, trace_id)?;
            output.write_all(b"\",\n")?;
        }

        if let Some(span_id) = metadata.span_id() {
            self.write_tab(output)?;
            output.write_all(b"span_id: \"")?;
            write_id(output, span_id)?;
            output.write_all(b"\",\n")?;
        }

        self.write_tab(output)?;
        write!(output, "level: \"{}\",\n", metadata.level())?;

        self.write_tab(output)?;
        write!(output, "resource: \"{}\",\n", metadata.resource())?;

        self.write_tab(output)?;
        write!(output, "scope: \"{}\",\n", metadata.scope())?;

        self.write_tab(output)?;
        write!(output, "message: \"{}\"\n", message)?;

        output.write_all(b"}")
    }
}

impl Default for PrettyJSONLogFormatter {
    fn default() -> Self {
        PrettyJSONLogFormatter { tab_size: 4 }
    }
}
