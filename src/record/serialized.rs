use crate::LogRecordMetadata;
use std::ops::Deref;

// rustdoc imports
#[allow(unused_imports)]
use crate::LogRecord;

/// A [`LogRecord`] which has been serialized to send to the output thread
pub struct SerializedLogRecord {
    /// The metadata about this record
    metadata: LogRecordMetadata,

    /// The serialized message describing the event
    message: String,
}

impl SerializedLogRecord {
    /// Gets the message describing this record
    pub fn message(&self) -> &str {
        &self.message
    }
}

impl Deref for SerializedLogRecord {
    type Target = LogRecordMetadata;

    fn deref(&self) -> &Self::Target {
        &self.metadata
    }
}
