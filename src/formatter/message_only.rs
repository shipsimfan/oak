use crate::LogFormatter;

// rustdoc imports
#[allow(unused_imports)]
use crate::LogRecord;

/// Formats [`LogRecord`]s so only their messages's are output
pub struct MessageOnlyLogFormatter {}

impl LogFormatter for MessageOnlyLogFormatter {}
