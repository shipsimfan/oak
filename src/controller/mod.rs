use crate::{record::LogRecord, writer::LogWriter, LogLevel, LogOutput, Logger};
use std::{borrow::Cow, sync::Arc};

mod list_type;

pub use list_type::FilterListType;

/// The central controller for log settings and output
pub struct LogController {
    /// The name of this log source
    resource: Arc<Cow<'static, str>>,

    /// The type of list for filtering scopes
    filter_list_type: FilterListType,

    /// The list of scopes to filter for
    filter_list: Vec<Cow<'static, str>>,

    /// The minimum [`LogLevel`] to log. Logs must be greater than or equal to this severity to log
    min_level: LogLevel,

    /// The maximum [`LogLevel`] to log. Logs must be less than this severity to log
    max_level: Option<LogLevel>,

    /// The output thread
    writer: LogWriter,
}

impl LogController {
    /// Creates a new [`LogController`]
    pub fn new<S1: Into<Cow<'static, str>>, S2: Into<Cow<'static, str>>>(
        resource: S1,
        min_level: LogLevel,
        max_level: Option<LogLevel>,
        filter_list_type: FilterListType,
        filter_list: Vec<S2>,
        outputs: Vec<Box<dyn LogOutput>>,
    ) -> std::io::Result<Arc<Self>> {
        if let Some(max_level) = max_level {
            assert!(min_level <= max_level);
        }

        let filter_list = filter_list.into_iter().map(|item| item.into()).collect();

        let writer = LogWriter::new(outputs)?;

        Ok(Arc::new(LogController {
            resource: Arc::new(resource.into()),
            filter_list_type,
            filter_list,
            min_level,
            max_level,
            writer,
        }))
    }

    /// Creates a new [`Logger`]
    pub fn create_logger<S: Into<Cow<'static, str>>>(self: &Arc<Self>, scope: S) -> Logger {
        let scope = scope.into();

        let should_log = self.filter_list_type.filter(&scope, &self.filter_list);

        Logger::new(self.clone(), scope, should_log)
    }

    /// Should a record with `level` severity be logged?
    pub(crate) fn should_log(&self, level: LogLevel) -> bool {
        let mut result = level >= self.min_level;
        if let Some(max_level) = self.max_level {
            result &= level < max_level;
        }
        result
    }

    /// Logs `record` to the outputs
    pub(crate) fn log(&self, record: LogRecord) {
        let serialized_record = Arc::new(record.serialize());
        self.writer.write(serialized_record);
    }

    /// Gets the resource name for logs
    pub(crate) fn resource(&self) -> Arc<Cow<'static, str>> {
        self.resource.clone()
    }
}
