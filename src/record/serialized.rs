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
    pub message: String,
}

impl Deref for SerializedLogRecord {
    type Target = LogRecordMetadata;

    fn deref(&self) -> &Self::Target {
        &self.metadata
    }
}
