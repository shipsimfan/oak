use crate::{LogFormatter, SerializedLogRecord};
use std::io::Write;

// rustdoc imports
#[allow(unused_imports)]
use crate::LogRecord;

/// Formats [`LogRecord`]s so only their messages's are output
pub struct MessageOnlyLogFormatter;

impl LogFormatter for MessageOnlyLogFormatter {
    fn format(
        &mut self,
        output: &mut dyn Write,
        record: SerializedLogRecord,
    ) -> std::io::Result<()> {
        output.write_all(record.message.as_bytes())?;
        output.write_all(b"\n")
    }
}
