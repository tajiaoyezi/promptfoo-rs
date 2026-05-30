use std::io::Write;

use crate::output::json::output_payload;
use crate::output::{OutputError, RunSummary};

pub fn write_html(summary: &RunSummary, mut writer: impl Write) -> Result<(), OutputError> {
    let payload = serde_json::to_string(&output_payload(summary))?;
    write!(
        writer,
        r#"<!doctype html><html><head><meta charset="utf-8"><title>promptfoo-rs results</title></head><body data-contract-version="promptfoo-rs.html.v1"><main id="promptfoo-rs-viewer"></main><script id="promptfoo-rs-data" type="application/json">{}</script></body></html>"#,
        payload
    )?;
    Ok(())
}
