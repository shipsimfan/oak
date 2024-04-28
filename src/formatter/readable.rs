use crate::LogFormatter;

// rustdoc imports
#[allow(unused_imports)]
use crate::LogRecord;

/// Formats [`LogRecord`]s in a human readable way, including metadata
pub struct ReadableLogFormatter {}

impl LogFormatter for ReadableLogFormatter {}
