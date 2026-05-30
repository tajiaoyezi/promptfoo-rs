use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use promptfoo_rs::results::{
    AssertionResultRecord, JsonlResultWriter, ResultRecord, ResultStatus, SqliteResultStore,
};
use promptfoo_rs::viewer_server::{
    build_results_table, export_viewer_records, load_viewer_records, viewer_contract, ExportFormat,
    ResultSource, ViewerFilter,
};
use serde_json::json;

fn temp_path(name: &str, extension: &str) -> PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time is after epoch")
        .as_nanos();
    let path = std::env::temp_dir().join(format!(
        "promptfoo-rs-{name}-{}-{nanos}.{extension}",
        std::process::id()
    ));
    let _ = fs::remove_file(&path);
    path
}

fn result_record(
    case_id: &str,
    provider_id: &str,
    status: ResultStatus,
    assertion_type: &str,
) -> ResultRecord {
    ResultRecord {
        eval_id: "eval-viewer".to_string(),
        case_id: case_id.to_string(),
        provider_id: provider_id.to_string(),
        status: status.clone(),
        result: Some(json!({
            "output": format!("output for {case_id}"),
            "score": if status == ResultStatus::Passed { 1.0 } else { 0.0 }
        })),
        assertion_results: vec![AssertionResultRecord {
            assertion_type: assertion_type.to_string(),
            status,
            message: Some(format!("{assertion_type} assertion for {case_id}")),
        }],
        latency_ms: 37,
        metadata: json!({
            "promptfooVersion": "0.121.13",
            "scenario": "SCEN-10.1"
        }),
        error: None,
    }
}

#[test]
fn test_10_1_1_viewer_reads_stable_result_schema_from_jsonl_and_sqlite() {
    let records = vec![
        result_record(
            "case-pass",
            "openai:gpt-4.1-mini",
            ResultStatus::Passed,
            "equals",
        ),
        result_record(
            "case-fail",
            "anthropic:claude-3.5-sonnet",
            ResultStatus::Failed,
            "contains",
        ),
    ];

    let jsonl_path = temp_path("viewer-schema", "jsonl");
    let mut writer = JsonlResultWriter::create(&jsonl_path).expect("jsonl writer opens");
    for record in &records {
        writer.append(record).expect("record appends");
    }
    drop(writer);

    let sqlite_path = temp_path("viewer-schema", "db");
    let store = SqliteResultStore::open(&sqlite_path).expect("sqlite store opens");
    for record in &records {
        store.insert(record).expect("record inserts");
    }

    let jsonl_records =
        load_viewer_records(ResultSource::jsonl(&jsonl_path)).expect("jsonl records load");
    let sqlite_records =
        load_viewer_records(ResultSource::sqlite(&sqlite_path)).expect("sqlite records load");

    assert_eq!(jsonl_records, records);
    assert_eq!(sqlite_records, records);
    assert_eq!(jsonl_records[0].eval_id, "eval-viewer");
    assert_eq!(jsonl_records[0].metadata["promptfooVersion"], "0.121.13");
    assert_eq!(jsonl_records[1].assertion_results[0].assertion_type, "contains");
}

#[test]
fn test_10_1_2_eval_table_filters_failed_provider_and_assertion_rows() {
    let records = vec![
        result_record(
            "case-pass",
            "openai:gpt-4.1-mini",
            ResultStatus::Passed,
            "equals",
        ),
        result_record(
            "case-fail",
            "openai:gpt-4.1-mini",
            ResultStatus::Failed,
            "contains",
        ),
        result_record(
            "case-other-provider",
            "anthropic:claude-3.5-sonnet",
            ResultStatus::Failed,
            "contains",
        ),
    ];

    let table = build_results_table(
        &records,
        ViewerFilter::failed()
            .provider_id("openai:gpt-4.1-mini")
            .assertion_type("contains"),
    );

    assert_eq!(table.rows.len(), 1);
    assert_eq!(table.rows[0].eval_id, "eval-viewer");
    assert_eq!(table.rows[0].case_id, "case-fail");
    assert_eq!(table.rows[0].provider_id, "openai:gpt-4.1-mini");
    assert_eq!(table.rows[0].status, "failed");
    assert_eq!(table.rows[0].assertion_types, vec!["contains"]);
    assert!(table.columns.contains(&"provider_id"));
    assert!(table.columns.contains(&"case_id"));
    assert!(table.columns.contains(&"status"));
    assert!(table.columns.contains(&"assertion_types"));
}

#[test]
fn test_10_1_3_viewer_contract_is_data_parity_not_pixel_replication() {
    let contract = viewer_contract();

    assert_eq!(contract.schema_version, "promptfoo-rs.viewer.v1");
    assert!(!contract.upstream_pixel_parity_required);
    assert_eq!(
        contract.parity_boundary,
        "stable-result-schema-and-table-actions"
    );
    assert!(contract.test_ids.contains(&"TEST-10.1.1"));
    assert!(contract.test_ids.contains(&"TEST-10.1.2"));
    assert!(contract.test_ids.contains(&"TEST-10.1.3"));

    let rows = build_results_table(
        &[result_record(
            "case-fail",
            "openai:gpt-4.1-mini",
            ResultStatus::Failed,
            "contains",
        )],
        ViewerFilter::failed(),
    );
    let json_export =
        export_viewer_records(&rows, ExportFormat::Json).expect("json export succeeds");
    let csv_export = export_viewer_records(&rows, ExportFormat::Csv).expect("csv export succeeds");

    assert!(json_export.contains("\"case_id\":\"case-fail\""));
    assert!(csv_export.contains("case_id,provider_id,status,assertion_types"));
    assert!(csv_export.contains("case-fail,openai:gpt-4.1-mini,failed,contains"));
}
