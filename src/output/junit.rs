use std::io::Write;

use crate::output::{OutputError, RunSummary};
use crate::results::ResultStatus;

pub fn write_junit(summary: &RunSummary, mut writer: impl Write) -> Result<(), OutputError> {
    let counts = summary.counts();
    write!(
        writer,
        r#"<testsuite name="{}" tests="{}" failures="{}" errors="{}">"#,
        xml_escape(&summary.eval_id),
        counts.total,
        counts.failed,
        counts.errors
    )?;

    for record in &summary.records {
        write!(
            writer,
            r#"<testcase classname="{}" name="{}" time="{}">"#,
            xml_escape(&record.provider_id),
            xml_escape(&record.case_id),
            record.latency_ms as f64 / 1000.0
        )?;
        match record.status {
            ResultStatus::Failed => {
                let message = record.error.as_deref().unwrap_or("assertion failed");
                let details = record
                    .assertion_results
                    .iter()
                    .find_map(|assertion| assertion.message.as_deref())
                    .unwrap_or(message);
                write!(
                    writer,
                    r#"<failure message="{}">{}</failure>"#,
                    xml_escape(message),
                    xml_escape(details)
                )?;
            }
            ResultStatus::Error => {
                let message = record.error.as_deref().unwrap_or("provider error");
                write!(
                    writer,
                    r#"<error message="{}">{}</error>"#,
                    xml_escape(message),
                    xml_escape(message)
                )?;
            }
            ResultStatus::Skipped => {
                writer.write_all(b"<skipped/>")?;
            }
            ResultStatus::Passed => {}
        }
        writer.write_all(b"</testcase>")?;
    }

    writer.write_all(b"</testsuite>")?;
    Ok(())
}

fn xml_escape(value: &str) -> String {
    quick_xml::escape::escape(value)
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}
