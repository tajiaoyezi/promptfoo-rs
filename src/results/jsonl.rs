use std::fs::{self, File, OpenOptions};
use std::io::{BufWriter, Write};
use std::path::Path;

use crate::results::schema::{ResultRecord, StoreError};

pub struct JsonlResultWriter {
    writer: BufWriter<File>,
    records_written: usize,
}

impl JsonlResultWriter {
    pub fn create(path: impl AsRef<Path>) -> Result<Self, StoreError> {
        if let Some(parent) = path.as_ref().parent() {
            fs::create_dir_all(parent)?;
        }
        let file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(path)?;
        Ok(Self {
            writer: BufWriter::new(file),
            records_written: 0,
        })
    }

    pub fn append(&mut self, record: &ResultRecord) -> Result<(), StoreError> {
        serde_json::to_writer(&mut self.writer, record)?;
        self.writer.write_all(b"\n")?;
        self.writer.flush()?;
        self.records_written += 1;
        Ok(())
    }

    pub fn records_written(&self) -> usize {
        self.records_written
    }

    pub fn buffered_records(&self) -> usize {
        0
    }
}
