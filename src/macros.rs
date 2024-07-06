/// Logs `arg` to `logger` with `severity`
#[macro_export]
macro_rules! log {
    ($logger: expr, $severity: ident, $arg: literal) => {
        $logger.log($crate::Severity::$severity, $arg)
    };

    ($logger: expr, $severity: ident, $arg: expr) => {
        $logger.log($crate::Severity::$severity, &$arg)
    };

    ($logger: expr, $severity: ident, $fmt: literal, $($arg:tt)*) => {
        $logger.log($crate::Severity::$severity, &::std::format_args!($fmt, $($arg)*))
    };
}

/// Logs a fatal message to `logger`
#[macro_export]
macro_rules! fatal {
    ($logger: expr, $($arg:tt)*) => {
        $crate::log!($logger, Fatal, $($arg)*)
    };
}

/// Logs an error message to `logger`
#[macro_export]
macro_rules! error {
    ($logger: expr, $($arg:tt)*) => {
        $crate::log!($logger, Error, $($arg)*)
    };
}

/// Logs a warning message to `logger`
#[macro_export]
macro_rules! warn {
    ($logger: expr, $($arg:tt)*) => {
        $crate::log!($logger, Warning, $($arg)*)
    };
}

/// Logs an information message to `logger`
#[macro_export]
macro_rules! info {
    ($logger: expr, $($arg:tt)*) => {
        $crate::log!($logger, Info, $($arg)*)
    };
}

/// Logs a debug message to `logger`
#[macro_export]
macro_rules! debug {
    ($logger: expr, $($arg:tt)*) => {
        $crate::log!($logger, Debug, $($arg)*)
    };
}

/// Logs a trace message to `logger`
#[macro_export]
macro_rules! trace {
    ($logger: expr, $($arg:tt)*) => {
        $crate::log!($logger, Trace, $($arg)*)
    };
}
