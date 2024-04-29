use crate::{LogOutput, SerializedLogRecord};
use std::sync::mpsc::Receiver;

// rustdoc imports
#[allow(unused_imports)]
use crate::LogWriter;

/// The main entry point for the [`LogWriter`] thread
pub(super) fn run(receiver: Receiver<SerializedLogRecord>, mut output: Box<dyn LogOutput>) {
    while let Ok(record) = receiver.recv() {
        output.output(record);
    }
}
