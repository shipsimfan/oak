mod json_compact;
mod json_pretty;
mod message_only;
mod readable;

pub use json_compact::CompactJSONLogFormatter;
pub use json_pretty::PrettyJSONLogFormatter;
pub use message_only::MessageOnlyLogFormatter;
pub use readable::ReadableLogFormatter;

// rustdoc imports
#[allow(unused_imports)]
use crate::{LogOutput, LogRecord};

/// Formats [`LogRecord`]s for [`LogOutput`]s that require formatting
pub trait LogFormatter: 'static + Send {}
