use crate::LogFormatter;

// rustdoc imports
#[allow(unused_imports)]
use crate::LogRecord;

/// Formats [`LogRecord`]s as JSON using spacing and newlines to make it readable
pub struct PrettyJSONLogFormatter {}

impl LogFormatter for PrettyJSONLogFormatter {}
