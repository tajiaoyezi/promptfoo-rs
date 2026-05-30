use std::io::{self, Write};

use serde::Serialize;
use serde_json::json;

use crate::output::{OutputError, RunSummary};

const OUTPUT_SCHEMA_VERSION: &str = "promptfoo-rs.output.v1";

pub fn write_json(summary: &RunSummary, writer: impl Write) -> Result<(), OutputError> {
    let payload = output_payload(summary);
    serde_json::to_writer(writer, &payload)?;
    Ok(())
}

pub fn write_jsonl(summary: &RunSummary, mut writer: impl Write) -> Result<(), OutputError> {
    for record in &summary.records {
        serde_json::to_writer(&mut writer, record)?;
        writer.write_all(b"\n")?;
    }
    Ok(())
}

pub fn write_yaml(summary: &RunSummary, mut writer: impl Write) -> Result<(), OutputError> {
    let payload = output_payload(summary);
    let yaml = serde_yaml::to_string(&payload)?;
    writer.write_all(yaml.as_bytes())?;
    Ok(())
}

pub fn output_payload(summary: &RunSummary) -> impl Serialize {
    json!({
        "schema_version": OUTPUT_SCHEMA_VERSION,
        "eval_id": summary.eval_id,
        "summary": summary.counts(),
        "results": summary.records,
    })
}

impl From<io::ErrorKind> for OutputError {
    fn from(value: io::ErrorKind) -> Self {
        Self::new(value.to_string())
    }
}
