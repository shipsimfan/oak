mod level;
mod metadata;
mod serialized;

pub use level::LogLevel;
pub use metadata::LogRecordMetadata;
pub use serialized::SerializedLogRecord;

/// A message to be logged
pub(crate) struct LogRecord<'a> {
    /// The metadata about this record
    metadata: LogRecordMetadata,

    /// The message describing the event
    message: &'a dyn std::fmt::Display,
}

impl<'a> LogRecord<'a> {
    /// Creates a new [`LogRecord`]
    pub(crate) fn new(metadata: LogRecordMetadata, message: &'a dyn std::fmt::Display) -> Self {
        LogRecord { metadata, message }
    }

    /// Serializes this log record
    pub(crate) fn serialize(self) -> SerializedLogRecord {
        SerializedLogRecord::new(self.metadata, self.message.to_string())
    }
}
