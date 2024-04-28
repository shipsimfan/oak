use crate::{writer::LogWriter, LogLevel};
use std::borrow::Cow;

mod list_type;

pub use list_type::FilterListType;

/// The central controller for log settings and output
pub struct LogController {
    /// The type of list for filtering scopes
    filter_list_type: FilterListType,

    /// The list of scopes to filter for
    filter_list: Vec<Cow<'static, [u8]>>,

    /// The minimum [`LogLevel`] to log
    min_level: LogLevel,

    /// The maximum [`LogLevel`] to log
    max_level: LogLevel,

    /// The output thread
    writer: LogWriter,
}
