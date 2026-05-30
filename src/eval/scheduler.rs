use std::sync::Arc;
use std::thread;
use std::time::Duration;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EvalTask {
    pub input: String,
}

impl EvalTask {
    pub fn new(input: impl Into<String>) -> Self {
        Self {
            input: input.into(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EvalPlan {
    pub tasks: Vec<EvalTask>,
}

impl EvalPlan {
    pub fn new(tasks: impl IntoIterator<Item = EvalTask>) -> Self {
        Self {
            tasks: tasks.into_iter().collect(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SchedulerOptions {
    pub max_concurrency: usize,
    pub delay_between_cases: Duration,
    pub cancel_after: Option<usize>,
}

impl Default for SchedulerOptions {
    fn default() -> Self {
        Self {
            max_concurrency: 1,
            delay_between_cases: Duration::ZERO,
            cancel_after: None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RunSummary {
    pub completed: usize,
    pub failed: usize,
    pub cancelled: bool,
    pub results: Vec<CaseResult>,
    pub errors: Vec<CaseError>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CaseResult {
    pub input: String,
    pub output: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CaseError {
    pub input: String,
    pub message: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProviderRequest {
    pub input: String,
}

pub type ProviderResult = Result<String, String>;

pub trait Provider: Send + Sync {
    fn call(&self, request: ProviderRequest) -> ProviderResult;
}

pub struct Scheduler {
    options: SchedulerOptions,
}

impl Scheduler {
    pub fn new(options: SchedulerOptions) -> Self {
        Self { options }
    }

    pub fn run(&self, plan: EvalPlan, provider: Arc<dyn Provider>) -> Result<RunSummary, String> {
        let max_concurrency = self.options.max_concurrency.max(1);
        let mut summary = RunSummary {
            completed: 0,
            failed: 0,
            cancelled: false,
            results: Vec::new(),
            errors: Vec::new(),
        };

        for (index, task) in plan.tasks.into_iter().enumerate() {
            if self
                .options
                .cancel_after
                .is_some_and(|limit| summary.completed + summary.failed >= limit)
            {
                summary.cancelled = true;
                break;
            }
            if index > 0 && !self.options.delay_between_cases.is_zero() {
                thread::sleep(self.options.delay_between_cases);
            }

            let result = run_bounded_call(provider.clone(), task.clone(), max_concurrency)?;
            match result {
                Ok(output) => {
                    summary.completed += 1;
                    summary.results.push(CaseResult {
                        input: task.input,
                        output,
                    });
                }
                Err(message) => {
                    summary.failed += 1;
                    summary.errors.push(CaseError {
                        input: task.input,
                        message,
                    });
                }
            }
        }

        Ok(summary)
    }
}

fn run_bounded_call(
    provider: Arc<dyn Provider>,
    task: EvalTask,
    _max_concurrency: usize,
) -> Result<ProviderResult, String> {
    Ok(provider.call(ProviderRequest { input: task.input }))
}
