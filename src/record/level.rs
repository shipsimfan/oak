use std::str::FromStr;

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

pub struct UnknownLogLevel;

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

impl FromStr for LogLevel {
    type Err = UnknownLogLevel;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        const LEVELS: &[(&str, LogLevel)] = &[
            ("trace", LogLevel::Trace),
            ("debug", LogLevel::Debug),
            ("info", LogLevel::Info),
            ("information", LogLevel::Info),
            ("warn", LogLevel::Warning),
            ("warning", LogLevel::Warning),
            ("err", LogLevel::Error),
            ("error", LogLevel::Error),
            ("fatal", LogLevel::Fatal),
        ];

        fn case_insensitive_compare(a: &str, b: &str) -> bool {
            if a.len() != b.len() {
                return false;
            }

            let mut a = a.chars();
            let mut b = b.chars();
            while let Some(a) = a.next() {
                let b = b.next().unwrap();

                if a.to_ascii_lowercase() != b.to_ascii_lowercase() {
                    return false;
                }
            }

            true
        }

        for (level_str, level) in LEVELS {
            if case_insensitive_compare(s, level_str) {
                return Ok(*level);
            }
        }

        Err(UnknownLogLevel)
    }
}

impl std::error::Error for UnknownLogLevel {}

impl std::fmt::Display for UnknownLogLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("unknown log level")
    }
}

impl std::fmt::Debug for UnknownLogLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self, f)
    }
}
