use crate::{LogController, LogLevel, LogRecord, LogRecordMetadata};
use std::{borrow::Cow, fmt::Display, sync::Arc};

/// An instrumentation scope which produces log records
#[derive(Clone)]
pub struct Logger {
    /// The controller to output logs to
    controller: Arc<LogController>,

    /// The scope name
    scope: Arc<Cow<'static, str>>,

    /// Has this scope been filtered to not log?
    should_log: bool,
}

impl Logger {
    /// Creates a new [`Logger`]
    pub(crate) fn new(
        controller: Arc<LogController>,
        scope: Cow<'static, str>,
        should_log: bool,
    ) -> Self {
        Logger {
            controller,
            scope: scope.into(),
            should_log,
        }
    }

    /// Logs an event
    pub fn log<D: Display>(&self, level: LogLevel, message: D) {
        self.log_trace(level, message, [0; 16])
    }

    /// Logs an event with a `trace_id
    pub fn log_trace<D: Display>(&self, level: LogLevel, message: D, trace_id: [u8; 16]) {
        self.log_span(level, message, trace_id, [0; 16])
    }

    /// Logs an event with a `trace_id` and a `span_id`
    pub fn log_span<D: Display>(
        &self,
        level: LogLevel,
        message: D,
        trace_id: [u8; 16],
        span_id: [u8; 16],
    ) {
        if !self.should_log {
            return;
        }

        if !self.controller.should_log(level) {
            return;
        }

        let record = LogRecord::new(
            LogRecordMetadata::new(
                trace_id,
                span_id,
                level,
                self.controller.resource(),
                self.scope.clone(),
            ),
            message,
        );
        self.controller.log(record);
    }
}
