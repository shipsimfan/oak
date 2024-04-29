use crate::LogLevel;
use std::{borrow::Cow, sync::Arc, time::SystemTime};

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
    resource: Arc<Cow<'static, str>>,

    /// The scope that emitted this log
    scope: Arc<Cow<'static, str>>,
}

impl LogRecordMetadata {
    pub(crate) fn new(
        trace_id: [u8; 16],
        span_id: [u8; 16],
        level: LogLevel,
        resource: Arc<Cow<'static, str>>,
        scope: Arc<Cow<'static, str>>,
    ) -> Self {
        LogRecordMetadata {
            timestamp: SystemTime::now(),
            trace_id,
            span_id,
            level,
            resource,
            scope,
        }
    }

    /// Gets the timestamp of the record
    pub fn timestamp(&self) -> SystemTime {
        self.timestamp
    }

    /// Gets the trace ID of the record
    pub fn trace_id(&self) -> Option<[u8; 16]> {
        if self.trace_id == [0; 16] {
            return None;
        }

        Some(self.trace_id)
    }

    /// Gets the span ID of the record
    pub fn span_id(&self) -> Option<[u8; 16]> {
        if self.span_id == [0; 16] {
            return None;
        }

        Some(self.span_id)
    }

    /// Gets the severity of the record
    pub fn level(&self) -> LogLevel {
        self.level
    }

    /// The source of this log
    pub fn resource(&self) -> &str {
        &self.resource
    }

    /// The instrumentation scope of this log
    pub fn scope(&self) -> &str {
        &self.scope
    }
}
