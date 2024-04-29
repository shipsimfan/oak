mod level;
mod metadata;
mod serialized;

pub use level::LogLevel;
pub use metadata::LogRecordMetadata;
pub use serialized::SerializedLogRecord;

/// A message to be logged
pub struct LogRecord<'a> {
    /// The metadata about this record
    metadata: LogRecordMetadata,

    /// The message describing the event
    message: &'a dyn std::fmt::Display,
}
