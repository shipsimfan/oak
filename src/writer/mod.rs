use crate::SerializedLogRecord;
use std::sync::mpsc::Sender;

// rustdoc imports
#[allow(unused_imports)]
use crate::{LogOutput, LogRecord};

mod thread;

/// A thread which writes [`LogRecord`]s to [`LogOutput`]s
pub(crate) struct LogWriter {
    /// The queue of logs to the thread
    sender: Sender<SerializedLogRecord>,
}
