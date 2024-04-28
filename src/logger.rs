use crate::LogController;
use std::{borrow::Cow, sync::Arc};

/// An instrumentation scope which produces log records
#[derive(Clone)]
pub struct Logger {
    /// The controller to output logs to
    controller: Arc<LogController>,

    /// The scope name
    scope: Cow<'static, str>,

    /// Has this scope been filtered to not log?
    is_filtered: bool,
}

impl Logger {
    /// Creates a new [`Logger`]
    pub(crate) fn new(
        controller: Arc<LogController>,
        scope: Cow<'static, str>,
        is_filtered: bool,
    ) -> Self {
        Logger {
            controller,
            scope: scope.into(),
            is_filtered,
        }
    }
}
