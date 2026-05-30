use serde_json::Value;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RedactionPolicy {
    pub replacement: String,
    pub secret_key_fragments: Vec<String>,
}

impl Default for RedactionPolicy {
    fn default() -> Self {
        Self {
            replacement: "[REDACTED]".to_string(),
            secret_key_fragments: vec![
                "apikey".to_string(),
                "api_key".to_string(),
                "authorization".to_string(),
                "providerheader".to_string(),
                "secret".to_string(),
                "token".to_string(),
            ],
        }
    }
}

pub fn redact_secrets(value: &mut Value, policy: &RedactionPolicy) {
    match value {
        Value::Object(map) => {
            for (key, child) in map.iter_mut() {
                if policy.matches_key(key) {
                    *child = Value::String(policy.replacement.clone());
                } else {
                    redact_secrets(child, policy);
                }
            }
        }
        Value::Array(items) => {
            for item in items {
                redact_secrets(item, policy);
            }
        }
        _ => {}
    }
}

impl RedactionPolicy {
    fn matches_key(&self, key: &str) -> bool {
        let normalized = key
            .chars()
            .filter(|ch| *ch != '-' && *ch != '_')
            .collect::<String>()
            .to_lowercase();
        self.secret_key_fragments.iter().any(|fragment| {
            let fragment = fragment
                .chars()
                .filter(|ch| *ch != '-' && *ch != '_')
                .collect::<String>()
                .to_lowercase();
            normalized.contains(&fragment)
        })
    }
}
