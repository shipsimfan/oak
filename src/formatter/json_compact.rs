use crate::LogFormatter;

// rustdoc imports
#[allow(unused_imports)]
use crate::LogRecord;

/// Formats [`LogRecord`]s as JSON using no extra spacing or newlines
pub struct CompactJSONLogFormatter {}

impl LogFormatter for CompactJSONLogFormatter {}
