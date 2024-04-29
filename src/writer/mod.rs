use crate::SerializedLogRecord;
use std::sync::{mpsc::Sender, Arc};

// rustdoc imports
#[allow(unused_imports)]
use crate::{LogOutput, LogRecord};

mod thread;

/// A thread which writes [`LogRecord`]s to [`LogOutput`]s
pub(crate) struct LogWriter {
    /// The queue of logs to the threads
    senders: Vec<Sender<Arc<SerializedLogRecord>>>,
}

impl LogWriter {
    /// Creates a new [`LogWriter`] thread
    pub(crate) fn new(outputs: Vec<Box<dyn LogOutput>>) -> std::io::Result<Self> {
        let mut senders = Vec::with_capacity(outputs.len());

        for output in outputs {
            let (sender, receiver) = std::sync::mpsc::channel();

            std::thread::Builder::new()
                .name(format!("oak log writer ({})", output.name()))
                .spawn(move || thread::run(receiver, output))?;
            senders.push(sender);
        }

        Ok(LogWriter { senders })
    }

    /// Sends `record` to the writer thread
    pub(crate) fn write(&self, record: Arc<SerializedLogRecord>) {
        for sender in &self.senders {
            sender.send(record.clone()).ok();
        }
    }
}
