use crate::LogLevel;
use std::{borrow::Cow, time::SystemTime};

// rustdoc imports
#[allow(unused_imports)]
use crate::LogRecord;

/// The metadata about a [`LogRecord`]
pub struct LogRecordMetadata {
    /// The time when an event occurred
    pub timestamp: SystemTime,

    /// Request trace ID
    pub trace_id: [u8; 16],

    /// Request span ID
    pub span_id: [u8; 16],

    /// The severity
    pub level: LogLevel,

    /// The source of this log
    pub resource: Cow<'static, [u8]>,

    /// The scope that emitted this log
    pub scope: Cow<'static, [u8]>,
}

impl LogRecordMetadata {
    /// Creates a new [`LogRecordMetadata`]
    pub(crate) fn new<S1: Into<Cow<'static, [u8]>>, S2: Into<Cow<'static, [u8]>>>(
        level: LogLevel,
        resource: S1,
        scope: S2,
    ) -> Self {
        LogRecordMetadata {
            timestamp: SystemTime::now(),
            trace_id: [0; 16],
            span_id: [0; 16],
            level,
            resource: resource.into(),
            scope: scope.into(),
        }
    }

    /// Sets the source of this log
    pub fn set_resource<S: Into<Cow<'static, [u8]>>>(&mut self, resource: S) {
        self.resource = resource.into();
    }

    /// Sets the scope of this log
    pub fn set_scope<S: Into<Cow<'static, [u8]>>>(&mut self, scope: S) {
        self.scope = scope.into();
    }
}
