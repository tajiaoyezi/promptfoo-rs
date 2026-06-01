use std::path::Path;
use std::process::Command;

use promptfoo_rs::compatibility::diff::DiffClass;
use promptfoo_rs::compatibility::harness::{
    build_current_latest_golden_corpus, evaluate_current_latest_release_blockers,
    write_current_latest_golden_corpus, GoldenCorpusReport, GoldenDiffFinding,
};
use serde_json::json;

fn fixture_dir(name: &str) -> std::path::PathBuf {
    let dir = std::env::temp_dir().join(format!(
        "promptfoo-rs-current-latest-golden-{name}-{}",
        std::process::id()
    ));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("fixture dir should create");
    dir
}

fn write_matrix(path: &Path, rows: Vec<serde_json::Value>) {
    let value = json!({
        "schema": "promptfoo-rs.current-latest-matrix.v1",
        "status": "ready-with-blockers",
        "target_ref": "1d09dfeb5f0766905409117f923dd5c4b0838d9f",
        "rows": rows,
        "unclassified_rows": [],
        "rows_missing_evidence": [],
        "perfect_refactor_claim_allowed": false
    });
    std::fs::write(
        path,
        serde_json::to_string_pretty(&value).expect("matrix should serialize"),
    )
    .expect("matrix should write");
}

fn matrix_row(
    item_id: &str,
    category: &str,
    level: &str,
    implementation_status: &str,
    evidence_kind: &str,
    evidence_reference: &str,
    blocker_reason: Option<&str>,
) -> serde_json::Value {
    json!({
        "item_id": item_id,
        "category": category,
        "source_reference": format!("promptfoo@current-latest:1d09dfeb5f0766905409117f923dd5c4b0838d9f:src/{item_id}.ts"),
        "level": level,
        "implementation_status": implementation_status,
        "verification_owner": "compatibility",
        "evidence_kind": evidence_kind,
        "evidence_reference": evidence_reference,
        "blocker_reason": blocker_reason
    })
}

#[test]
fn test_24_3_1_every_p0_row_gets_fixture_and_golden_artifacts() {
    /* TEST-24.3.1 */
    let root = fixture_dir("p0-artifacts");
    let matrix_path = root.join("current-latest-matrix.json");
    let fixtures_root = root.join("fixtures");
    let artifacts_root = root.join("artifacts");
    write_matrix(
        &matrix_path,
        vec![
            matrix_row(
                "config:runtime",
                "config",
                "P0",
                "native",
                "fixture",
                "fixture:config:runtime",
                None,
            ),
            matrix_row(
                "provider:blocked",
                "provider",
                "P0",
                "blocked",
                "blocker",
                "blocker:provider:blocked",
                Some("external provider authority is still required"),
            ),
        ],
    );

    let report = build_current_latest_golden_corpus(&matrix_path, &fixtures_root, &artifacts_root)
        .expect("current latest corpus should build");

    assert_eq!(
        report.schema,
        "promptfoo-rs.current-latest-golden-corpus.v1"
    );
    assert_eq!(report.p0_total, 2);
    assert_eq!(report.p0_fixture_coverage_count, 2);
    assert_eq!(report.p0_artifact_coverage_count, 2);
    assert!(report
        .rows
        .iter()
        .filter(|row| row.level == "P0")
        .all(|row| row.executable_fixture
            && row
                .artifact_paths
                .iter()
                .all(|path| Path::new(path).exists())));
    assert!(report.rows.iter().any(|row| {
        row.item_id == "config:runtime"
            && row
                .artifact_paths
                .iter()
                .any(|path| path.ends_with("raw/upstream.json"))
            && row
                .artifact_paths
                .iter()
                .any(|path| path.ends_with("normalized/rs.json"))
            && row
                .artifact_paths
                .iter()
                .any(|path| path.ends_with("diff/findings.json"))
    }));

    let _ = std::fs::remove_dir_all(root);
}

#[test]
fn test_24_3_2_p0_bug_or_unclassified_diff_blocks_current_latest_claim() {
    /* TEST-24.3.2 */
    let root = fixture_dir("p0-blockers");
    let matrix_path = root.join("current-latest-matrix.json");
    write_matrix(
        &matrix_path,
        vec![
            matrix_row(
                "provider:buggy",
                "provider",
                "P0",
                "blocked",
                "blocker",
                "blocker:provider:buggy",
                Some("P0 provider row lacks fixture evidence"),
            ),
            matrix_row(
                "unclassified:surface",
                "unclassified",
                "P0",
                "blocked",
                "blocker",
                "blocker:unclassified:surface",
                Some("source row is unclassified"),
            ),
        ],
    );

    let report = build_current_latest_golden_corpus(
        &matrix_path,
        &root.join("fixtures"),
        &root.join("artifacts"),
    )
    .expect("current latest corpus should build");
    let blockers: Vec<GoldenDiffFinding> = evaluate_current_latest_release_blockers(&report);

    assert!(!report.perfect_refactor_claim_allowed, "{report:#?}");
    assert!(blockers
        .iter()
        .any(|finding| finding.class == DiffClass::Bug));
    assert!(blockers
        .iter()
        .any(|finding| finding.class == DiffClass::Unclassified));

    let _ = std::fs::remove_dir_all(root);
}

#[test]
fn test_24_3_3_p1_and_p2_rows_have_snapshot_or_registration_evidence() {
    /* TEST-24.3.3 */
    let root = fixture_dir("p1-p2");
    let matrix_path = root.join("current-latest-matrix.json");
    write_matrix(
        &matrix_path,
        vec![
            matrix_row(
                "assertion:contains",
                "assertion",
                "P1",
                "later",
                "snapshot",
                "snapshot:assertion:contains",
                Some("P1 assertion requires current-latest snapshot"),
            ),
            matrix_row(
                "docs:workflow",
                "docs",
                "P2",
                "later",
                "registration",
                "registration:docs:workflow",
                Some("documented workflow registered as P2 later evidence"),
            ),
        ],
    );

    let report = build_current_latest_golden_corpus(
        &matrix_path,
        &root.join("fixtures"),
        &root.join("artifacts"),
    )
    .expect("current latest corpus should build");

    assert_eq!(report.p1_total, 1);
    assert_eq!(report.p1_snapshot_coverage_count, 1);
    assert_eq!(report.p2_total, 1);
    assert_eq!(report.p2_registration_coverage_count, 1);
    assert!(report.rows.iter().any(|row| {
        row.item_id == "assertion:contains"
            && row
                .snapshot_path
                .as_deref()
                .is_some_and(|path| Path::new(path).exists())
    }));

    let _ = std::fs::remove_dir_all(root);
}

#[test]
fn test_24_3_4_corpus_scale_and_runtime_smoke_gate_are_wired() {
    /* TEST-24.3.4 */
    let root = fixture_dir("scale");
    let matrix_path = root.join("current-latest-matrix.json");
    let rows = (0..260)
        .map(|index| {
            matrix_row(
                &format!("config:item-{index:03}"),
                "config",
                "P0",
                "native",
                "fixture",
                &format!("fixture:config:item-{index:03}"),
                None,
            )
        })
        .collect();
    write_matrix(&matrix_path, rows);

    let report: GoldenCorpusReport = build_current_latest_golden_corpus(
        &matrix_path,
        &root.join("fixtures"),
        &root.join("artifacts"),
    )
    .expect("current latest corpus should build");
    assert!(report.fixture_case_count >= 250, "{report:#?}");
    assert_eq!(report.fixture_case_count, 260);

    let out = root.join("current-latest-golden-corpus.json");
    write_current_latest_golden_corpus(&report, Path::new(&out)).expect("report should write");
    assert!(out.exists());

    let script = std::fs::read_to_string("scripts/release/current-latest-golden-corpus.sh")
        .unwrap_or_default();
    assert!(script.contains("CURRENT_LATEST_FIXTURES_ROOT"), "{script}");
    let runtime = std::fs::read_to_string("scripts/release/runtime-smoke.sh")
        .expect("runtime smoke script should exist");
    assert!(
        runtime.contains("current-latest-golden-corpus.sh"),
        "{runtime}"
    );

    let _ = std::fs::remove_dir_all(root);
}

#[test]
fn test_24_3_5_script_writes_current_latest_corpus_report() {
    let root = fixture_dir("script");
    let matrix_path = root.join("current-latest-matrix.json");
    let gate_dir = root.join("gate");
    write_matrix(
        &matrix_path,
        vec![matrix_row(
            "config:script",
            "config",
            "P0",
            "native",
            "fixture",
            "fixture:config:script",
            None,
        )],
    );

    let command = format!(
        "CURRENT_LATEST_MATRIX_FILE='{}' CURRENT_LATEST_GATE_DIR='{}' CURRENT_LATEST_FIXTURES_ROOT='{}' bash scripts/release/current-latest-golden-corpus.sh",
        shell_escape(&matrix_path),
        shell_escape(&gate_dir),
        shell_escape(&root.join("fixtures"))
    );
    let output = Command::new(git_bash())
        .args(["-lc", &command])
        .output()
        .expect("current latest golden corpus script should execute");
    assert!(
        output.status.success(),
        "stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    let report_path = gate_dir.join("current-latest-golden-corpus.json");
    let report: serde_json::Value = serde_json::from_str(
        &std::fs::read_to_string(report_path).expect("report should be readable"),
    )
    .expect("report should parse");
    assert_eq!(
        report["schema"],
        "promptfoo-rs.current-latest-golden-corpus.v1"
    );
    assert_eq!(report["p0_fixture_coverage_count"], 1);

    let _ = std::fs::remove_dir_all(root);
}

fn shell_escape(path: &Path) -> String {
    path.to_string_lossy()
        .replace('\\', "/")
        .replace('\'', "'\\''")
}

fn git_bash() -> &'static str {
    if cfg!(windows) {
        r"C:\Program Files\Git\bin\bash.exe"
    } else {
        "bash"
    }
}
