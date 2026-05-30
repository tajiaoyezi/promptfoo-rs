use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use promptfoo_rs::eval::scheduler::{
    EvalPlan, EvalTask, Provider, ProviderRequest, ProviderResult, Scheduler, SchedulerOptions,
};

#[test]
fn test_3_1_1_max_concurrency_limits_provider_calls() {
    let provider = Arc::new(CountingProvider::default());
    let plan = EvalPlan::new((0..5).map(|idx| EvalTask::new(format!("case-{idx}"))));

    let summary = Scheduler::new(SchedulerOptions {
        max_concurrency: 2,
        ..SchedulerOptions::default()
    })
    .run(plan, provider.clone())
    .expect("TEST-3.1.1 scheduler should run");

    assert_eq!(summary.completed, 5);
    assert!(provider.max_seen() <= 2, "max seen {}", provider.max_seen());
}

#[test]
fn test_3_1_2_delay_and_cancellation_have_deterministic_behavior() {
    let provider = Arc::new(CountingProvider::default());
    let plan = EvalPlan::new((0..3).map(|idx| EvalTask::new(format!("case-{idx}"))));
    let started = Instant::now();

    let summary = Scheduler::new(SchedulerOptions {
        max_concurrency: 1,
        delay_between_cases: Duration::from_millis(15),
        cancel_after: Some(2),
    })
    .run(plan, provider)
    .expect("TEST-3.1.2 scheduler should run");

    assert_eq!(summary.completed, 2);
    assert!(summary.cancelled);
    assert!(started.elapsed() >= Duration::from_millis(15));
}

#[test]
fn test_3_1_3_partial_failure_keeps_completed_results() {
    let provider = Arc::new(FailingProvider);
    let plan = EvalPlan::new(["ok-1", "fail", "ok-2"].into_iter().map(EvalTask::new));

    let summary = Scheduler::new(SchedulerOptions::default())
        .run(plan, provider)
        .expect("TEST-3.1.3 scheduler should keep running after case failure");

    assert_eq!(summary.completed, 2);
    assert_eq!(summary.failed, 1);
    assert_eq!(summary.results.len(), 2);
    assert_eq!(summary.errors.len(), 1);
    assert_eq!(summary.results[0].output, "ok-1");
    assert_eq!(summary.results[1].output, "ok-2");
}

#[derive(Default)]
struct CountingProvider {
    state: Mutex<CountingState>,
}

#[derive(Default)]
struct CountingState {
    in_flight: usize,
    max_seen: usize,
}

impl CountingProvider {
    fn max_seen(&self) -> usize {
        self.state.lock().unwrap().max_seen
    }
}

impl Provider for CountingProvider {
    fn call(&self, request: ProviderRequest) -> ProviderResult {
        {
            let mut state = self.state.lock().unwrap();
            state.in_flight += 1;
            state.max_seen = state.max_seen.max(state.in_flight);
        }
        let output = request.input;
        {
            let mut state = self.state.lock().unwrap();
            state.in_flight -= 1;
        }
        Ok(output)
    }
}

struct FailingProvider;

impl Provider for FailingProvider {
    fn call(&self, request: ProviderRequest) -> ProviderResult {
        if request.input == "fail" {
            Err("provider failure".to_string())
        } else {
            Ok(request.input)
        }
    }
}
