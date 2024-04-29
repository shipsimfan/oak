use crate::{LogFormatter, LogOutput, SerializedLogRecord};
use std::{
    borrow::Cow,
    fs::{File, OpenOptions},
    path::Path,
};

// rustdoc imports
#[allow(unused_imports)]
use crate::LogRecord;

/// Writes [`LogRecord`]s to a [`File`]
pub struct FileLogOutput<F: LogFormatter> {
    /// The name of this output
    name: Cow<'static, str>,

    /// The file to write to
    file: File,

    /// The formatter styling the output
    formatter: F,
}

impl<F: LogFormatter> FileLogOutput<F> {
    /// Opens the file at `path` as a log output, creating it if it doesn't exist
    pub fn open<P: AsRef<Path>, S: Into<Cow<'static, str>>>(
        path: P,
        formatter: F,
        name: S,
    ) -> std::io::Result<Self> {
        let file = OpenOptions::new().create(true).append(true).open(path)?;

        Ok(FileLogOutput {
            file,
            formatter,
            name: name.into(),
        })
    }
}

impl<F: LogFormatter> LogOutput for FileLogOutput<F> {
    fn name(&self) -> &str {
        &self.name
    }

    fn output(&mut self, record: &SerializedLogRecord) {
        self.formatter.format(&mut self.file, record).ok();
    }
}
