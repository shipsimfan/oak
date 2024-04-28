use std::borrow::Cow;

mod level;
mod metadata;
mod serialized;

pub use level::LogLevel;
pub use metadata::LogRecordMetadata;

pub(crate) use serialized::SerializedLogRecord;

/// A message to be logged
pub struct LogRecord<'a> {
    /// The metadata about this record
    metadata: LogRecordMetadata,

    /// The body of the log record
    body: &'a dyn std::fmt::Display,
}

impl<'a> LogRecord<'a> {
    /// Creates a new [`LogRecord`]
    pub(crate) fn new<S1: Into<Cow<'static, [u8]>>, S2: Into<Cow<'static, [u8]>>>(
        level: LogLevel,
        resource: S1,
        scope: S2,
        body: &'a dyn std::fmt::Display,
    ) -> Self {
        LogRecord {
            metadata: LogRecordMetadata::new(level, resource, scope),
            body,
        }
    }
}
