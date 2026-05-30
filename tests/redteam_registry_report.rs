use promptfoo_rs::compatibility::harness::{
    Artifact, ArtifactEngine, BaselineReference,
};
use promptfoo_rs::compatibility::normalize::{
    normalize_artifact, NormalizationRules,
};
use promptfoo_rs::redteam::registry::{CompatibilityLevel, RedteamRegistry};
use promptfoo_rs::redteam::report::write_redteam_report;
use promptfoo_rs::redteam::risk::{score_risk, Severity};
use promptfoo_rs::redteam::{
    RedteamFinding, RedteamReport, RedteamStageRecord,
};
use serde_json::Value;

fn finding(case_id: &str, severity: &str) -> RedteamFinding {
    RedteamFinding {
        case_id: case_id.to_string(),
        plugin: "prompt-injection".to_string(),
        strategy: "jailbreak".to_string(),
        severity: severity.to_string(),
        message: format!("{severity} risk finding"),
    }
}

fn report_with_findings(findings: Vec<RedteamFinding>) -> RedteamReport {
    RedteamReport {
        status: "completed".to_string(),
        target_id: "mock-target".to_string(),
        stages: vec![RedteamStageRecord {
            stage: "report".to_string(),
            status: "completed".to_string(),
        }],
        findings,
        errors: Vec::new(),
    }
}

#[test]
fn test_7_2_1_core_plugin_strategy_registry_records_p0_p1_p2() {
    let registry = RedteamRegistry::core_defaults();

    assert!(registry
        .plugins_by_level(CompatibilityLevel::P0)
        .iter()
        .any(|plugin| plugin.id == "prompt-injection"));
    assert!(registry
        .plugins_by_level(CompatibilityLevel::P1)
        .iter()
        .any(|plugin| plugin.id == "harmful-content"));
    assert!(registry
        .plugins_by_level(CompatibilityLevel::P2)
        .iter()
        .any(|plugin| plugin.id == "custom-policy"));
    assert!(registry
        .strategies_by_level(CompatibilityLevel::P0)
        .iter()
        .any(|strategy| strategy.id == "jailbreak"));
    assert!(registry
        .strategies_by_level(CompatibilityLevel::P1)
        .iter()
        .any(|strategy| strategy.id == "multi-turn"));
    assert!(registry
        .strategies_by_level(CompatibilityLevel::P2)
        .iter()
        .any(|strategy| strategy.id == "agentic-chain"));
}

#[test]
fn test_7_2_2_risk_scoring_fields_are_stable_snapshot() {
    let summary = score_risk(&[
        finding("case-high", "high"),
        finding("case-medium", "medium"),
        finding("case-low", "low"),
    ]);

    assert_eq!(summary.total_findings, 3);
    assert_eq!(summary.high, 1);
    assert_eq!(summary.medium, 1);
    assert_eq!(summary.low, 1);
    assert_eq!(summary.max_severity, Severity::High);
    assert_eq!(summary.weighted_score, 160);
}

#[test]
fn test_7_2_3_redteam_report_output_enters_compatibility_harness() {
    let report = report_with_findings(vec![finding("case-high", "high")]);
    let mut output = Vec::new();
    write_redteam_report(&report, &mut output).expect("report writes");
    let payload: Value = serde_json::from_slice(&output).expect("report is json");

    assert_eq!(
        payload["schema_version"],
        "promptfoo-rs.redteam.report.v1"
    );
    assert_eq!(payload["risk"]["total_findings"], 1);
    assert_eq!(payload["risk"]["high"], 1);
    assert_eq!(payload["report"]["findings"][0]["case_id"], "case-high");

    let artifact = Artifact {
        engine: ArtifactEngine::PromptfooRs,
        fixture_name: "redteam-report".to_string(),
        baseline: BaselineReference::npm("promptfoo@0.121.13"),
        payload,
    };
    let normalized =
        normalize_artifact(&artifact, &NormalizationRules::default_promptfoo_0_121_13());

    assert_eq!(
        normalized.payload["schema_version"],
        "promptfoo-rs.redteam.report.v1"
    );
    assert_eq!(normalized.fixture_name, "redteam-report");
}
