// rustdoc imports
#[allow(unused_imports)]
use crate::LogRecord;

/// The severity of a [`LogRecord`]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum LogLevel {
    /// A fine-grained debugging event
    Trace,

    /// A debugging event
    Debug,

    /// An informational event
    Info,

    /// A warning event
    Warning,

    /// An error event
    Error,

    /// A fatal error
    Fatal,
}

impl LogLevel {
    /// Gets a string representation of a [`LogLevel`]
    pub const fn as_str(&self) -> &'static str {
        match self {
            LogLevel::Trace => "trace",
            LogLevel::Debug => "debug",
            LogLevel::Info => "info",
            LogLevel::Warning => "warning",
            LogLevel::Error => "error",
            LogLevel::Fatal => "fatal",
        }
    }
}

impl std::fmt::Display for LogLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
