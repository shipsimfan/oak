use std::borrow::Cow;

/// An instrumentation scope which produces log records
#[derive(Clone)]
pub struct Logger {
    /// The scope name
    scope: Cow<'static, [u8]>,

    /// Has this scope been filtered to not log?
    is_filtered: bool,
}
