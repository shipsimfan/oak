use crate::{writer::LogWriter, LogLevel, LogOutput};
use std::borrow::Cow;

mod list_type;

pub use list_type::FilterListType;

/// The central controller for log settings and output
pub struct LogController {
    /// The type of list for filtering scopes
    filter_list_type: FilterListType,

    /// The list of scopes to filter for
    filter_list: Vec<Cow<'static, [u8]>>,

    /// The minimum [`LogLevel`] to log. Logs must be greater than or equal to this severity to log
    min_level: LogLevel,

    /// The maximum [`LogLevel`] to log. Logs must be less than this severity to log
    max_level: Option<LogLevel>,

    /// The output thread
    writer: LogWriter,
}

impl LogController {
    /// Creates a new [`LogController`]
    pub fn new<S: Into<Cow<'static, [u8]>>>(
        min_level: LogLevel,
        max_level: Option<LogLevel>,
        filter_list_type: FilterListType,
        filter_list: Vec<S>,
        outputs: Vec<Box<dyn LogOutput>>,
    ) -> std::io::Result<Self> {
        if let Some(max_level) = max_level {
            assert!(min_level <= max_level);
        }

        let filter_list = filter_list.into_iter().map(|item| item.into()).collect();

        let writer = LogWriter::new(outputs)?;

        Ok(LogController {
            filter_list_type,
            filter_list,
            min_level,
            max_level,
            writer,
        })
    }
}
