use std::collections::BTreeMap;
use std::time::Duration;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct RateLimitHeaderState {
    pub remaining: Option<u32>,
    pub reset_after: Option<Duration>,
}

impl RateLimitHeaderState {
    pub fn parse(headers: &BTreeMap<String, String>) -> Self {
        let normalized = headers
            .iter()
            .map(|(key, value)| (key.to_ascii_lowercase(), value.trim().to_string()))
            .collect::<BTreeMap<_, _>>();

        let remaining = [
            "x-ratelimit-remaining-requests",
            "x-ratelimit-remaining-tokens",
            "ratelimit-remaining",
            "rate-limit-remaining",
        ]
        .iter()
        .find_map(|key| normalized.get(*key).and_then(|value| value.parse().ok()));

        let reset_after = [
            "x-ratelimit-reset-requests",
            "x-ratelimit-reset-tokens",
            "x-ratelimit-reset",
            "retry-after",
            "ratelimit-reset",
            "rate-limit-reset",
        ]
        .iter()
        .find_map(|key| {
            normalized
                .get(*key)
                .and_then(|value| parse_duration(value, *key == "retry-after"))
        });

        Self {
            remaining,
            reset_after,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct RateLimitKey(String);

impl RateLimitKey {
    pub fn new(
        provider_id: impl Into<String>,
        model: impl Into<String>,
        scope: impl Into<String>,
    ) -> Self {
        Self(format!(
            "{}:{}:{}",
            slug(&provider_id.into()),
            slug(&model.into()),
            slug(&scope.into())
        ))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RateLimitDecision {
    Ready,
    Delay(Duration),
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ProviderRateLimitRegistry {
    states: BTreeMap<RateLimitKey, RateLimitHeaderState>,
}

impl ProviderRateLimitRegistry {
    pub fn delay_for(&self, key: &RateLimitKey) -> RateLimitDecision {
        let Some(state) = self.states.get(key) else {
            return RateLimitDecision::Ready;
        };
        if state.remaining == Some(0) {
            if let Some(delay) = state.reset_after.filter(|delay| !delay.is_zero()) {
                return RateLimitDecision::Delay(delay);
            }
        }
        RateLimitDecision::Ready
    }

    pub fn record_headers(&mut self, key: RateLimitKey, headers: &BTreeMap<String, String>) {
        let state = RateLimitHeaderState::parse(headers);
        if state.remaining.is_some() || state.reset_after.is_some() {
            self.states.insert(key, state);
        }
    }

    pub fn call_with_policy<F>(
        &mut self,
        context: ProviderCallExecutionContext,
        call: F,
    ) -> WrappedProviderCallResult
    where
        F: FnOnce(&ProviderCallExecutionContext) -> ProviderCallOutcome,
    {
        let pre_call_decision = self.delay_for(&context.rate_limit_key);
        let outcome = call(&context);
        self.record_headers(context.rate_limit_key.clone(), &outcome.headers);
        WrappedProviderCallResult {
            context,
            pre_call_decision,
            output: outcome.output,
            error: outcome.error,
            observed_headers: outcome.headers,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AdaptiveObservation {
    Success,
    Failure,
    RateLimited,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AdaptiveConcurrencyController {
    min: usize,
    max: usize,
    current: usize,
}

impl AdaptiveConcurrencyController {
    pub fn new(min: usize, max: usize, initial: usize) -> Self {
        let min = min.max(1);
        let max = max.max(min);
        let current = initial.clamp(min, max);
        Self { min, max, current }
    }

    pub fn current_limit(&self) -> usize {
        self.current
    }

    pub fn observe(&mut self, observation: AdaptiveObservation) -> usize {
        self.current = match observation {
            AdaptiveObservation::Success => (self.current + 1).min(self.max),
            AdaptiveObservation::Failure => self.current.saturating_sub(1).max(self.min),
            AdaptiveObservation::RateLimited => (self.current / 2).max(self.min),
        };
        self.current
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProviderCallExecutionContext {
    pub provider_id: String,
    pub model: String,
    pub scope: String,
    pub attempt: usize,
    pub rate_limit_key: RateLimitKey,
}

impl ProviderCallExecutionContext {
    pub fn new(
        provider_id: impl Into<String>,
        model: impl Into<String>,
        scope: impl Into<String>,
        attempt: usize,
    ) -> Self {
        let provider_id = provider_id.into();
        let model = model.into();
        let scope = scope.into();
        let rate_limit_key = RateLimitKey::new(provider_id.clone(), model.clone(), scope.clone());
        Self {
            provider_id,
            model,
            scope,
            attempt,
            rate_limit_key,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProviderCallOutcome {
    pub output: Option<String>,
    pub error: Option<String>,
    pub headers: BTreeMap<String, String>,
}

impl ProviderCallOutcome {
    pub fn success(output: impl Into<String>, headers: BTreeMap<String, String>) -> Self {
        Self {
            output: Some(output.into()),
            error: None,
            headers,
        }
    }

    pub fn error(error: impl Into<String>, headers: BTreeMap<String, String>) -> Self {
        Self {
            output: None,
            error: Some(error.into()),
            headers,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WrappedProviderCallResult {
    pub context: ProviderCallExecutionContext,
    pub pre_call_decision: RateLimitDecision,
    pub output: Option<String>,
    pub error: Option<String>,
    pub observed_headers: BTreeMap<String, String>,
}

fn parse_duration(value: &str, bare_number_is_seconds: bool) -> Option<Duration> {
    let trimmed = value.trim();
    if let Some(ms) = trimmed.strip_suffix("ms") {
        return ms.trim().parse().ok().map(Duration::from_millis);
    }
    if let Some(seconds) = trimmed.strip_suffix('s') {
        return seconds.trim().parse().ok().map(Duration::from_secs);
    }
    let parsed = trimmed.parse::<u64>().ok()?;
    Some(if bare_number_is_seconds {
        Duration::from_secs(parsed)
    } else {
        Duration::from_millis(parsed)
    })
}

fn slug(value: &str) -> String {
    let slug = value
        .chars()
        .map(|ch| {
            if ch.is_ascii_alphanumeric() {
                ch.to_ascii_lowercase()
            } else {
                '-'
            }
        })
        .collect::<String>()
        .split('-')
        .filter(|part| !part.is_empty())
        .collect::<Vec<_>>()
        .join("-");
    if slug.is_empty() {
        "unknown".to_string()
    } else {
        slug
    }
}
