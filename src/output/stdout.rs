use crate::LogOutput;

// rustdoc imports
#[allow(unused_imports)]
use crate::LogRecord;
#[allow(unused_imports)]
use std::io::stdout;

/// Writes [`LogRecord`]s to [`stdout`]
pub struct StdoutLogOutput {}

impl LogOutput for StdoutLogOutput {}
