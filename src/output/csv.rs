use std::io::Write;

use crate::output::{OutputError, RunSummary};

pub fn write_csv(summary: &RunSummary, writer: impl Write) -> Result<(), OutputError> {
    let mut csv = ::csv::Writer::from_writer(writer);
    csv.write_record([
        "eval_id",
        "case_id",
        "provider_id",
        "status",
        "latency_ms",
        "error",
    ])?;
    for record in &summary.records {
        csv.write_record([
            record.eval_id.as_str(),
            record.case_id.as_str(),
            record.provider_id.as_str(),
            record.status.as_str(),
            &record.latency_ms.to_string(),
            record.error.as_deref().unwrap_or(""),
        ])?;
    }
    csv.flush()?;
    Ok(())
}
