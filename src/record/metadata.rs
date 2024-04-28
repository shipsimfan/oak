use crate::LogLevel;
use std::{borrow::Cow, time::SystemTime};

// rustdoc imports
#[allow(unused_imports)]
use crate::LogRecord;

/// The metadata about a [`LogRecord`]
pub struct LogRecordMetadata {
    /// The time when an event occurred
    timestamp: SystemTime,

    /// Request trace ID
    trace_id: [u8; 16],

    /// Request span ID
    span_id: [u8; 16],

    /// The severity
    level: LogLevel,

    /// The source of this log
    resource: Cow<'static, [u8]>,

    /// The scope that emitted this log
    scope: Cow<'static, [u8]>,
}
