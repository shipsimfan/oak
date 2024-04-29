use crate::SerializedLogRecord;
use std::sync::mpsc::Sender;

// rustdoc imports
#[allow(unused_imports)]
use crate::{LogOutput, LogRecord};

mod thread;

/// A thread which writes [`LogRecord`]s to [`LogOutput`]s
pub(crate) struct LogWriter {
    /// The queue of logs to the threads
    senders: Vec<Sender<SerializedLogRecord>>,
}

impl LogWriter {
    /// Creates a new [`LogWriter`] thread
    pub(crate) fn new(outputs: Vec<Box<dyn LogOutput>>) -> std::io::Result<Self> {
        let mut senders = Vec::with_capacity(outputs.len());

        for output in outputs {
            let (sender, receiver) = std::sync::mpsc::channel();

            std::thread::Builder::new()
                .name("oak log writer".to_owned())
                .spawn(move || thread::run(receiver, output))?;
            senders.push(sender);
        }

        Ok(LogWriter { senders })
    }
}
