mod level;
mod metadata;
mod serialized;

pub use level::LogLevel;
pub use metadata::LogRecordMetadata;

pub(crate) use serialized::SerializedLogRecord;

/// A message to be logged
pub struct LogRecord {}
