use crate::LogRecordMetadata;

// rustdoc imports
#[allow(unused_imports)]
use crate::LogRecord;

/// A [`LogRecord`] which has been serialized to send to the output thread
pub(crate) struct SerializedLogRecord {
    /// The metadata about this record
    metadata: LogRecordMetadata,

    /// The serialized message describing the event
    message: String,
}
