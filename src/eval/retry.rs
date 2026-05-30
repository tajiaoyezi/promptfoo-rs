use std::future::Future;
use std::time::Duration;

use crate::eval::EvalError;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RetryPolicy {
    pub max_attempts: usize,
    pub retry_errors: Vec<String>,
    pub backoff: BackoffSchedule,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct BackoffSchedule {
    delays: Vec<Duration>,
}

impl BackoffSchedule {
    pub fn from_millis<I>(millis: I) -> Self
    where
        I: IntoIterator<Item = u64>,
    {
        Self {
            delays: millis
                .into_iter()
                .map(Duration::from_millis)
                .collect::<Vec<_>>(),
        }
    }

    fn delay_before_next_attempt(&self, failed_attempt: usize) -> Duration {
        if self.delays.is_empty() {
            return Duration::ZERO;
        }
        self.delays
            .get(failed_attempt.saturating_sub(1))
            .copied()
            .unwrap_or_else(|| *self.delays.last().expect("checked non-empty"))
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RetryOutcome<T> {
    pub value: T,
    pub attempts: usize,
    pub backoff_delays: Vec<Duration>,
    pub errors: Vec<EvalError>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RetryFailure {
    pub message: EvalError,
    pub attempts: usize,
    pub backoff_delays: Vec<Duration>,
    pub errors: Vec<EvalError>,
}

pub async fn retry_with_backoff<T, F, Fut>(
    policy: RetryPolicy,
    mut op: F,
) -> Result<RetryOutcome<T>, RetryFailure>
where
    F: FnMut(usize) -> Fut,
    Fut: Future<Output = Result<T, EvalError>>,
{
    let max_attempts = policy.max_attempts.max(1);
    let mut errors = Vec::new();
    let mut backoff_delays = Vec::new();

    for attempt in 1..=max_attempts {
        match op(attempt).await {
            Ok(value) => {
                return Ok(RetryOutcome {
                    value,
                    attempts: attempt,
                    backoff_delays,
                    errors,
                });
            }
            Err(error) => {
                let retryable = is_retryable(&policy.retry_errors, &error);
                errors.push(error.clone());
                if attempt == max_attempts || !retryable {
                    return Err(RetryFailure {
                        message: error,
                        attempts: attempt,
                        backoff_delays,
                        errors,
                    });
                }
                backoff_delays.push(policy.backoff.delay_before_next_attempt(attempt));
            }
        }
    }

    unreachable!("attempt loop always returns")
}

fn is_retryable(retry_errors: &[String], error: &str) -> bool {
    retry_errors
        .iter()
        .any(|retry_error| error.contains(retry_error))
}
