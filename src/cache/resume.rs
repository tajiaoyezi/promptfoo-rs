use std::collections::BTreeSet;
use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

use serde::{Deserialize, Serialize};
use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use sqlx::Row;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResumeRecord {
    pub case_id: String,
    pub cache_key: String,
    pub status: String,
    pub output: Option<String>,
}

impl ResumeRecord {
    pub fn completed(
        case_id: impl Into<String>,
        cache_key: impl Into<String>,
        output: impl Into<String>,
    ) -> Self {
        Self {
            case_id: case_id.into(),
            cache_key: cache_key.into(),
            status: "completed".to_string(),
            output: Some(output.into()),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CorruptRecord {
    pub line: usize,
    pub message: String,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ResumeState {
    pub records: Vec<ResumeRecord>,
    pub corrupt_records: Vec<CorruptRecord>,
}

impl ResumeState {
    pub fn completed_case_ids(&self) -> Vec<&str> {
        self.completed_set().into_iter().collect()
    }

    pub fn remaining_cases<I, S>(&self, expected_cases: I) -> Vec<String>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let completed = self.completed_set();
        expected_cases
            .into_iter()
            .filter_map(|case_id| {
                let case_id = case_id.as_ref();
                (!completed.contains(case_id)).then(|| case_id.to_string())
            })
            .collect()
    }

    fn completed_set(&self) -> BTreeSet<&str> {
        self.records
            .iter()
            .filter(|record| record.status == "completed")
            .map(|record| record.case_id.as_str())
            .collect()
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StoreError {
    message: String,
}

impl StoreError {
    fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl fmt::Display for StoreError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.message)
    }
}

impl std::error::Error for StoreError {}

impl From<std::io::Error> for StoreError {
    fn from(value: std::io::Error) -> Self {
        Self::new(value.to_string())
    }
}

impl From<sqlx::Error> for StoreError {
    fn from(value: sqlx::Error) -> Self {
        Self::new(value.to_string())
    }
}

pub struct ResumeStore;

impl ResumeStore {
    pub fn load(path: &Path) -> Result<ResumeState, StoreError> {
        match path.extension().and_then(|extension| extension.to_str()) {
            Some("sqlite" | "sqlite3" | "db") => Self::load_sqlite_blocking(path),
            _ => Self::load_jsonl(path),
        }
    }

    pub fn load_jsonl(path: &Path) -> Result<ResumeState, StoreError> {
        let file = File::open(path)?;
        let mut state = ResumeState::default();

        for (index, line) in BufReader::new(file).lines().enumerate() {
            let line_number = index + 1;
            let line = line?;
            if line.trim().is_empty() {
                continue;
            }
            match serde_json::from_str::<ResumeRecord>(&line) {
                Ok(record) => state.records.push(record),
                Err(error) => state.corrupt_records.push(CorruptRecord {
                    line: line_number,
                    message: error.to_string(),
                }),
            }
        }

        state
            .records
            .sort_by(|left, right| left.case_id.cmp(&right.case_id));
        Ok(state)
    }

    fn load_sqlite_blocking(path: &Path) -> Result<ResumeState, StoreError> {
        if tokio::runtime::Handle::try_current().is_ok() {
            let path = path.to_path_buf();
            return std::thread::spawn(move || Self::load_sqlite_on_new_runtime(&path))
                .join()
                .map_err(|_| StoreError::new("SQLite resume loader thread panicked"))?;
        }
        Self::load_sqlite_on_new_runtime(path)
    }

    fn load_sqlite_on_new_runtime(path: &Path) -> Result<ResumeState, StoreError> {
        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .map_err(StoreError::from)?;
        runtime.block_on(Self::load_sqlite(path))
    }

    pub async fn load_sqlite(path: &Path) -> Result<ResumeState, StoreError> {
        let options = SqliteConnectOptions::new()
            .filename(path)
            .create_if_missing(false);
        let pool = SqlitePoolOptions::new()
            .max_connections(1)
            .connect_with(options)
            .await?;

        let rows = sqlx::query(
            "SELECT case_id, cache_key, status, output
             FROM results
             ORDER BY case_id",
        )
        .fetch_all(&pool)
        .await?;
        pool.close().await;

        let records = rows
            .into_iter()
            .map(|row| ResumeRecord {
                case_id: row.get("case_id"),
                cache_key: row.get("cache_key"),
                status: row.get("status"),
                output: row.get("output"),
            })
            .collect();

        Ok(ResumeState {
            records,
            corrupt_records: Vec::new(),
        })
    }
}
