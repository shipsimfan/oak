use crate::{LogOutput, SerializedLogRecord};

// rustdoc imports
#[allow(unused_imports)]
use crate::LogRecord;

/// Writes [`LogRecord`]s to an OpenTelemetry server
pub struct OTLPLogOutput {}

impl LogOutput for OTLPLogOutput {
    fn output(&mut self, _: SerializedLogRecord) {
        todo!()
    }
}
