use std::future::Future;
use std::path::{Path, PathBuf};

use serde_json::Value;
use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use sqlx::Row;

use crate::results::schema::{AssertionResultRecord, ResultRecord, ResultStatus, StoreError};

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ResultQuery {
    eval_id: Option<String>,
    case_id: Option<String>,
    provider_id: Option<String>,
    assertion_type: Option<String>,
}

impl ResultQuery {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn eval_id(mut self, eval_id: impl Into<String>) -> Self {
        self.eval_id = Some(eval_id.into());
        self
    }

    pub fn case_id(mut self, case_id: impl Into<String>) -> Self {
        self.case_id = Some(case_id.into());
        self
    }

    pub fn provider_id(mut self, provider_id: impl Into<String>) -> Self {
        self.provider_id = Some(provider_id.into());
        self
    }

    pub fn assertion_type(mut self, assertion_type: impl Into<String>) -> Self {
        self.assertion_type = Some(assertion_type.into());
        self
    }

    fn matches(&self, record: &ResultRecord) -> bool {
        self.eval_id
            .as_deref()
            .is_none_or(|eval_id| eval_id == record.eval_id)
            && self
                .case_id
                .as_deref()
                .is_none_or(|case_id| case_id == record.case_id)
            && self
                .provider_id
                .as_deref()
                .is_none_or(|provider_id| provider_id == record.provider_id)
            && self.assertion_type.as_deref().is_none_or(|assertion_type| {
                record
                    .assertion_results
                    .iter()
                    .any(|assertion| assertion.assertion_type == assertion_type)
            })
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SqliteResultStore {
    path: PathBuf,
}

impl SqliteResultStore {
    pub fn open(path: impl AsRef<Path>) -> Result<Self, StoreError> {
        let path = path.as_ref().to_path_buf();
        let init_path = path.clone();
        run_blocking(async move {
            let pool = open_pool(&init_path, true).await?;
            initialize_schema(&pool).await?;
            pool.close().await;
            Ok(())
        })?;
        Ok(Self { path })
    }

    pub fn insert(&self, record: &ResultRecord) -> Result<(), StoreError> {
        let path = self.path.clone();
        let record = record.clone();
        run_blocking(async move {
            let pool = open_pool(&path, false).await?;
            let mut tx = pool.begin().await?;
            let result_json = optional_json_to_string(record.result.as_ref())?;
            let metadata_json = serde_json::to_string(&record.metadata)?;
            sqlx::query(
                "INSERT INTO result_records
                 (eval_id, case_id, provider_id, status, result_json, error, metadata_json, latency_ms)
                 VALUES (?, ?, ?, ?, ?, ?, ?, ?)",
            )
            .bind(&record.eval_id)
            .bind(&record.case_id)
            .bind(&record.provider_id)
            .bind(record.status.as_str())
            .bind(result_json)
            .bind(&record.error)
            .bind(metadata_json)
            .bind(record.latency_ms as i64)
            .execute(&mut *tx)
            .await?;

            let record_id: i64 = sqlx::query_scalar("SELECT last_insert_rowid()")
                .fetch_one(&mut *tx)
                .await?;
            for assertion in &record.assertion_results {
                sqlx::query(
                    "INSERT INTO assertion_results
                     (result_id, assertion_type, status, message)
                     VALUES (?, ?, ?, ?)",
                )
                .bind(record_id)
                .bind(&assertion.assertion_type)
                .bind(assertion.status.as_str())
                .bind(&assertion.message)
                .execute(&mut *tx)
                .await?;
            }

            tx.commit().await?;
            pool.close().await;
            Ok(())
        })
    }

    pub fn query(&self, query: ResultQuery) -> Result<Vec<ResultRecord>, StoreError> {
        let path = self.path.clone();
        run_blocking(async move {
            let pool = open_pool(&path, false).await?;
            let rows = sqlx::query(
                "SELECT id, eval_id, case_id, provider_id, status, result_json, error, metadata_json, latency_ms
                 FROM result_records
                 ORDER BY id",
            )
            .fetch_all(&pool)
            .await?;

            let mut records = Vec::with_capacity(rows.len());
            for row in rows {
                let result_id: i64 = row.get("id");
                let assertion_rows = sqlx::query(
                    "SELECT assertion_type, status, message
                     FROM assertion_results
                     WHERE result_id = ?
                     ORDER BY id",
                )
                .bind(result_id)
                .fetch_all(&pool)
                .await?;

                let assertions = assertion_rows
                    .into_iter()
                    .map(|assertion_row| {
                        Ok(AssertionResultRecord {
                            assertion_type: assertion_row.get("assertion_type"),
                            status: ResultStatus::parse(
                                assertion_row.get::<String, _>("status").as_str(),
                            )?,
                            message: assertion_row.get("message"),
                        })
                    })
                    .collect::<Result<Vec<_>, StoreError>>()?;

                let result_json: Option<String> = row.get("result_json");
                let metadata_json: String = row.get("metadata_json");
                let record = ResultRecord {
                    eval_id: row.get("eval_id"),
                    case_id: row.get("case_id"),
                    provider_id: row.get("provider_id"),
                    status: ResultStatus::parse(row.get::<String, _>("status").as_str())?,
                    result: optional_json_from_string(result_json)?,
                    assertion_results: assertions,
                    latency_ms: row.get::<i64, _>("latency_ms") as u64,
                    metadata: serde_json::from_str(&metadata_json)?,
                    error: row.get("error"),
                };
                if query.matches(&record) {
                    records.push(record);
                }
            }

            pool.close().await;
            Ok(records)
        })
    }

    pub fn delete_eval(&self, eval_id: &str) -> Result<u64, StoreError> {
        let path = self.path.clone();
        let eval_id = eval_id.to_string();
        run_blocking(async move {
            let pool = open_pool(&path, false).await?;
            let mut tx = pool.begin().await?;
            sqlx::query(
                "DELETE FROM assertion_results
                 WHERE result_id IN (
                     SELECT id FROM result_records WHERE eval_id = ?
                 )",
            )
            .bind(&eval_id)
            .execute(&mut *tx)
            .await?;
            let result = sqlx::query("DELETE FROM result_records WHERE eval_id = ?")
                .bind(&eval_id)
                .execute(&mut *tx)
                .await?;
            tx.commit().await?;
            pool.close().await;
            Ok(result.rows_affected())
        })
    }
}

async fn open_pool(path: &Path, create_if_missing: bool) -> Result<sqlx::SqlitePool, StoreError> {
    let options = SqliteConnectOptions::new()
        .filename(path)
        .create_if_missing(create_if_missing);
    Ok(SqlitePoolOptions::new()
        .max_connections(1)
        .connect_with(options)
        .await?)
}

async fn initialize_schema(pool: &sqlx::SqlitePool) -> Result<(), StoreError> {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS result_records (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            eval_id TEXT NOT NULL,
            case_id TEXT NOT NULL,
            provider_id TEXT NOT NULL,
            status TEXT NOT NULL,
            result_json TEXT,
            error TEXT,
            metadata_json TEXT NOT NULL,
            latency_ms INTEGER NOT NULL
        )",
    )
    .execute(pool)
    .await?;
    sqlx::query(
        "CREATE INDEX IF NOT EXISTS idx_result_records_eval_case_provider
         ON result_records (eval_id, case_id, provider_id)",
    )
    .execute(pool)
    .await?;
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS assertion_results (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            result_id INTEGER NOT NULL,
            assertion_type TEXT NOT NULL,
            status TEXT NOT NULL,
            message TEXT,
            FOREIGN KEY(result_id) REFERENCES result_records(id) ON DELETE CASCADE
        )",
    )
    .execute(pool)
    .await?;
    sqlx::query(
        "CREATE INDEX IF NOT EXISTS idx_assertion_results_type
         ON assertion_results (assertion_type)",
    )
    .execute(pool)
    .await?;
    Ok(())
}

fn optional_json_to_string(value: Option<&Value>) -> Result<Option<String>, StoreError> {
    value
        .map(serde_json::to_string)
        .transpose()
        .map_err(StoreError::from)
}

fn optional_json_from_string(value: Option<String>) -> Result<Option<Value>, StoreError> {
    value
        .map(|json| serde_json::from_str(&json))
        .transpose()
        .map_err(StoreError::from)
}

fn run_blocking<T, F>(future: F) -> Result<T, StoreError>
where
    T: Send + 'static,
    F: Future<Output = Result<T, StoreError>> + Send + 'static,
{
    if tokio::runtime::Handle::try_current().is_ok() {
        return std::thread::spawn(move || run_on_new_runtime(future))
            .join()
            .map_err(|_| StoreError::new("SQLite result store thread panicked"))?;
    }
    run_on_new_runtime(future)
}

fn run_on_new_runtime<T, F>(future: F) -> Result<T, StoreError>
where
    F: Future<Output = Result<T, StoreError>>,
{
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .map_err(StoreError::from)?;
    runtime.block_on(future)
}
