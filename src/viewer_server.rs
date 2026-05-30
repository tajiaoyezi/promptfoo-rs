use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};

use serde::Serialize;

use crate::results::{ResultQuery, ResultRecord, ResultStatus, SqliteResultStore, StoreError};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ResultSource {
    Jsonl(PathBuf),
    Sqlite(PathBuf),
}

impl ResultSource {
    pub fn jsonl(path: impl AsRef<Path>) -> Self {
        Self::Jsonl(path.as_ref().to_path_buf())
    }

    pub fn sqlite(path: impl AsRef<Path>) -> Self {
        Self::Sqlite(path.as_ref().to_path_buf())
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ViewerFilter {
    status: Option<ResultStatus>,
    provider_id: Option<String>,
    case_id: Option<String>,
    assertion_type: Option<String>,
}

impl ViewerFilter {
    pub fn failed() -> Self {
        Self::default().status(ResultStatus::Failed)
    }

    pub fn status(mut self, status: ResultStatus) -> Self {
        self.status = Some(status);
        self
    }

    pub fn provider_id(mut self, provider_id: impl Into<String>) -> Self {
        self.provider_id = Some(provider_id.into());
        self
    }

    pub fn case_id(mut self, case_id: impl Into<String>) -> Self {
        self.case_id = Some(case_id.into());
        self
    }

    pub fn assertion_type(mut self, assertion_type: impl Into<String>) -> Self {
        self.assertion_type = Some(assertion_type.into());
        self
    }

    fn matches(&self, record: &ResultRecord) -> bool {
        self.status
            .as_ref()
            .is_none_or(|status| status == &record.status)
            && self
                .provider_id
                .as_deref()
                .is_none_or(|provider_id| provider_id == record.provider_id)
            && self
                .case_id
                .as_deref()
                .is_none_or(|case_id| case_id == record.case_id)
            && self.assertion_type.as_deref().is_none_or(|assertion_type| {
                record
                    .assertion_results
                    .iter()
                    .any(|assertion| assertion.assertion_type == assertion_type)
            })
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct ViewerResultsTable {
    pub columns: Vec<&'static str>,
    pub rows: Vec<ViewerResultRow>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct ViewerResultRow {
    pub case_id: String,
    pub provider_id: String,
    pub status: String,
    pub assertion_types: Vec<String>,
    pub eval_id: String,
    pub error: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ExportFormat {
    Json,
    Csv,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ViewerContract {
    pub schema_version: &'static str,
    pub upstream_pixel_parity_required: bool,
    pub parity_boundary: &'static str,
    pub test_ids: Vec<&'static str>,
}

pub type ViewerError = StoreError;

pub fn load_viewer_records(source: ResultSource) -> Result<Vec<ResultRecord>, ViewerError> {
    match source {
        ResultSource::Jsonl(path) => load_jsonl_records(path),
        ResultSource::Sqlite(path) => {
            let store = SqliteResultStore::open(path)?;
            store.query(ResultQuery::new())
        }
    }
}

pub fn build_results_table(records: &[ResultRecord], filter: ViewerFilter) -> ViewerResultsTable {
    let rows = records
        .iter()
        .filter(|record| filter.matches(record))
        .map(|record| ViewerResultRow {
            case_id: record.case_id.clone(),
            provider_id: record.provider_id.clone(),
            status: record.status.as_str().to_string(),
            assertion_types: record
                .assertion_results
                .iter()
                .map(|assertion| assertion.assertion_type.clone())
                .collect(),
            eval_id: record.eval_id.clone(),
            error: record.error.clone(),
        })
        .collect();

    ViewerResultsTable {
        columns: vec![
            "case_id",
            "provider_id",
            "status",
            "assertion_types",
            "eval_id",
            "error",
        ],
        rows,
    }
}

pub fn export_viewer_records(
    table: &ViewerResultsTable,
    format: ExportFormat,
) -> Result<String, ViewerError> {
    match format {
        ExportFormat::Json => serde_json::to_string(&table.rows).map_err(StoreError::from),
        ExportFormat::Csv => export_csv(table),
    }
}

pub fn viewer_contract() -> ViewerContract {
    ViewerContract {
        schema_version: "promptfoo-rs.viewer.v1",
        upstream_pixel_parity_required: false,
        parity_boundary: "stable-result-schema-and-table-actions",
        test_ids: vec!["TEST-10.1.1", "TEST-10.1.2", "TEST-10.1.3"],
    }
}

fn load_jsonl_records(path: PathBuf) -> Result<Vec<ResultRecord>, ViewerError> {
    let file = File::open(path)?;
    let mut records = Vec::new();
    for (line_index, line) in BufReader::new(file).lines().enumerate() {
        let line = line?;
        if line.trim().is_empty() {
            continue;
        }
        let record = serde_json::from_str(&line).map_err(|error| {
            StoreError::new(format!("invalid JSONL record {}: {error}", line_index + 1))
        })?;
        records.push(record);
    }
    Ok(records)
}

fn export_csv(table: &ViewerResultsTable) -> Result<String, ViewerError> {
    let mut writer = csv::Writer::from_writer(Vec::new());
    writer
        .write_record(["case_id", "provider_id", "status", "assertion_types"])
        .map_err(|error| StoreError::new(error.to_string()))?;

    for row in &table.rows {
        writer
            .write_record([
                row.case_id.as_str(),
                row.provider_id.as_str(),
                row.status.as_str(),
                row.assertion_types.join("|").as_str(),
            ])
            .map_err(|error| StoreError::new(error.to_string()))?;
    }

    let bytes = writer
        .into_inner()
        .map_err(|error| StoreError::new(error.to_string()))?;
    String::from_utf8(bytes).map_err(|error| StoreError::new(error.to_string()))
}
