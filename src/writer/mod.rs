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

impl LogWriter {
    /// Creates a new [`LogWriter`] thread
    pub(crate) fn new(outputs: Vec<Box<dyn LogOutput>>) -> std::io::Result<Self> {
        let (sender, receiver) = std::sync::mpsc::channel();

        std::thread::Builder::new()
            .name("oak log writer".to_owned())
            .spawn(move || thread::run(receiver, outputs))?;

        Ok(LogWriter { sender })
    }
}
