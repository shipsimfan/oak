use super::{write_id, write_time};
use crate::{LogFormatter, LogRecordMetadata};
use std::io::Write;

// rustdoc imports
#[allow(unused_imports)]
use crate::LogRecord;

/// Formats [`LogRecord`]s as JSON using no extra spacing or newlines
pub struct CompactJSONLogFormatter;

impl LogFormatter for CompactJSONLogFormatter {
    fn format(
        &mut self,
        output: &mut dyn Write,
        metadata: &LogRecordMetadata,
        message: &str,
    ) -> std::io::Result<()> {
        output.write_all(b"{")?;

        output.write_all(b"timestamp:\"")?;
        write_time(output, metadata.timestamp())?;
        output.write_all(b"\",")?;

        if let Some(trace_id) = metadata.trace_id() {
            output.write_all(b"trace_id:\"")?;
            write_id(output, trace_id)?;
            output.write_all(b"\",")?;
        }

        if let Some(span_id) = metadata.span_id() {
            output.write_all(b"span_id:\"")?;
            write_id(output, span_id)?;
            output.write_all(b"\",")?;
        }

        write!(output, "level:\"{}\",", metadata.level())?;

        write!(output, "resource:\"{}\",", metadata.resource())?;

        write!(output, "scope:\"{}\",", metadata.scope())?;

        write!(output, "message:\"{}\"", message)?;

        output.write_all(b"}")
    }
}
