use crate::{LogFormatter, LogRecordMetadata};
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
        _: &LogRecordMetadata,
        message: &str,
    ) -> std::io::Result<()> {
        output.write_all(message.as_bytes())
    }
}
