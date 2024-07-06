mod level;
mod metadata;
mod serialized;

use std::fmt::Display;

pub use level::LogLevel;
pub use metadata::LogRecordMetadata;
pub use serialized::SerializedLogRecord;

/// A message to be logged
pub(crate) struct LogRecord<D: Display> {
    /// The metadata about this record
    metadata: LogRecordMetadata,

    /// The message describing the event
    message: D,
}

impl<D: Display> LogRecord<D> {
    /// Creates a new [`LogRecord`]
    pub(crate) fn new(metadata: LogRecordMetadata, message: D) -> Self {
        LogRecord { metadata, message }
    }

    /// Serializes this log record
    pub(crate) fn serialize(self) -> SerializedLogRecord {
        SerializedLogRecord::new(self.metadata, self.message.to_string())
    }
}
