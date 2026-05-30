use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use promptfoo_rs::results::{
    AssertionResultRecord, JsonlResultWriter, ResultQuery, ResultRecord, ResultStatus,
    SqliteResultStore,
};
use serde_json::{json, Value};

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

fn result_record(eval_id: &str, case_id: &str, provider_id: &str) -> ResultRecord {
    ResultRecord {
        eval_id: eval_id.to_string(),
        case_id: case_id.to_string(),
        provider_id: provider_id.to_string(),
        status: ResultStatus::Passed,
        result: Some(json!({
            "output": "hello from provider",
            "score": 1.0
        })),
        assertion_results: vec![AssertionResultRecord {
            assertion_type: "equals".to_string(),
            status: ResultStatus::Passed,
            message: Some("matched expected output".to_string()),
        }],
        latency_ms: 42,
        metadata: json!({
            "promptfooVersion": "0.121.13",
            "fixture": "TEST-5.1"
        }),
        error: None,
    }
}

#[test]
fn test_5_1_1_jsonl_append_schema_covers_result_error_metadata_and_latency() {
    let path = temp_path("jsonl-schema", "jsonl");
    let mut writer = JsonlResultWriter::create(&path).expect("jsonl writer opens");

    let passing = result_record("eval-jsonl", "case-pass", "openai:gpt-4.1-mini");
    let error = ResultRecord {
        status: ResultStatus::Error,
        result: None,
        error: Some("provider timeout".to_string()),
        metadata: json!({ "retryable": true, "source": "SCEN-5.1.1" }),
        latency_ms: 5_000,
        assertion_results: vec![AssertionResultRecord {
            assertion_type: "provider-response".to_string(),
            status: ResultStatus::Error,
            message: Some("request did not complete".to_string()),
        }],
        ..result_record("eval-jsonl", "case-error", "openai:gpt-4.1-mini")
    };

    writer.append(&passing).expect("passing record is appended");
    writer.append(&error).expect("error record is appended");
    drop(writer);

    let lines = fs::read_to_string(&path).expect("jsonl can be read");
    let parsed: Vec<Value> = lines
        .lines()
        .map(|line| serde_json::from_str(line).expect("each line is valid json"))
        .collect();

    assert_eq!(parsed.len(), 2);
    assert_eq!(parsed[0]["eval_id"], "eval-jsonl");
    assert_eq!(parsed[0]["case_id"], "case-pass");
    assert_eq!(parsed[0]["provider_id"], "openai:gpt-4.1-mini");
    assert_eq!(parsed[0]["status"], "passed");
    assert!(parsed[0]["result"]["output"].is_string());
    assert_eq!(parsed[0]["metadata"]["promptfooVersion"], "0.121.13");
    assert_eq!(parsed[0]["latency_ms"], 42);
    assert_eq!(
        parsed[0]["assertion_results"][0]["assertion_type"],
        "equals"
    );

    assert_eq!(parsed[1]["status"], "error");
    assert_eq!(parsed[1]["error"], "provider timeout");
    assert!(parsed[1]["result"].is_null());
    assert_eq!(parsed[1]["metadata"]["retryable"], true);
    assert_eq!(parsed[1]["latency_ms"], 5_000);
}

#[test]
fn test_5_1_2_sqlite_store_queries_by_eval_case_provider_and_assertion() {
    let path = temp_path("sqlite-schema", "db");
    let store = SqliteResultStore::open(&path).expect("sqlite store opens");

    let mut openai = result_record("eval-sqlite", "case-1", "openai:gpt-4.1-mini");
    openai.assertion_results = vec![AssertionResultRecord {
        assertion_type: "contains-json".to_string(),
        status: ResultStatus::Passed,
        message: Some("valid JSON block".to_string()),
    }];
    let anthropic = result_record("eval-sqlite", "case-2", "anthropic:claude-3.5-sonnet");

    store.insert(&openai).expect("openai record inserts");
    store.insert(&anthropic).expect("anthropic record inserts");

    let by_eval = store
        .query(ResultQuery::new().eval_id("eval-sqlite"))
        .expect("query by eval");
    assert_eq!(by_eval.len(), 2);

    let by_case_provider = store
        .query(
            ResultQuery::new()
                .case_id("case-1")
                .provider_id("openai:gpt-4.1-mini"),
        )
        .expect("query by case and provider");
    assert_eq!(by_case_provider, vec![openai.clone()]);

    let by_assertion = store
        .query(ResultQuery::new().assertion_type("contains-json"))
        .expect("query by assertion");
    assert_eq!(by_assertion, vec![openai]);
}

#[test]
fn test_5_1_3_jsonl_writer_streams_10k_cases_without_retaining_result_set() {
    let path = temp_path("streaming", "jsonl");
    let mut writer = JsonlResultWriter::create(&path).expect("jsonl writer opens");

    for case_index in 0..10_000 {
        let record = result_record(
            "eval-streaming",
            &format!("case-{case_index:05}"),
            "ollama:llama3.1",
        );
        writer.append(&record).expect("record appends");
        assert_eq!(writer.buffered_records(), 0);
    }

    assert_eq!(writer.records_written(), 10_000);
    drop(writer);

    let line_count = fs::read_to_string(&path)
        .expect("jsonl can be read")
        .lines()
        .count();
    assert_eq!(line_count, 10_000);
}
