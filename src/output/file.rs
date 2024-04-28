use crate::LogOutput;

// rustdoc imports
#[allow(unused_imports)]
use crate::LogRecord;
#[allow(unused_imports)]
use std::fs::File;

/// Writes [`LogRecord`]s to a [`File`]
pub struct FileLogOutput {}

impl LogOutput for FileLogOutput {}
