#[test]
fn test_16_3_1_source_inventory_evidence_uses_frozen_upstream_package() {
    /* TEST-16.3.1 */
    let script = std::fs::read_to_string("scripts/release/source-inventory-evidence.sh")
        .expect("source inventory evidence script should exist");

    assert!(script.contains("promptfoo@0.121.13"), "{script}");
    assert!(
        script.contains("npm pack") || script.contains("npm view"),
        "source evidence must inspect the frozen npm package: {script}"
    );
    assert!(
        script.contains("source-inventory-evidence.json"),
        "{script}"
    );
    for category in [
        "command",
        "provider",
        "assertion",
        "redteam",
        "output",
        "config",
    ] {
        assert!(
            script.contains(category),
            "missing category evidence for {category}"
        );
    }
}

#[test]
fn test_16_3_2_real_upstream_smoke_script_executes_npx_and_rs_binary() {
    /* TEST-16.3.2 */
    let script = std::fs::read_to_string("scripts/release/real-upstream-smoke.sh")
        .expect("real upstream smoke script should exist");

    assert!(script.contains("npx --yes promptfoo@0.121.13"), "{script}");
    assert!(
        script.contains(" eval "),
        "upstream smoke must execute eval: {script}"
    );
    assert!(script.contains("target/release/promptfoo-rs"), "{script}");
    assert!(script.contains("upstream.json"), "{script}");
    assert!(script.contains("rs.json"), "{script}");
    assert!(script.contains("normalized"), "{script}");
    assert!(script.contains("diff"), "{script}");
    assert!(
        !script.contains("current_exe") && !script.contains("--list"),
        "real upstream smoke must not use the local test-binary substitute: {script}"
    );
}

#[test]
fn test_16_3_3_runtime_smoke_fails_closed_without_real_upstream_artifacts() {
    /* TEST-16.3.3 */
    let script = std::fs::read_to_string("scripts/release/runtime-smoke.sh")
        .expect("runtime smoke script should exist");

    assert!(script.contains("real-upstream-smoke.sh"), "{script}");
    assert!(script.contains("source-inventory-evidence.sh"), "{script}");
    assert!(script.contains("real_upstream_smoke"), "{script}");
    assert!(
        script.contains("real-upstream-smoke/latest/metadata.json"),
        "release candidate artifacts must include real upstream metadata: {script}"
    );
}

#[test]
fn test_16_3_4_integration_gate_runs_real_upstream_smoke_contract_tests() {
    /* TEST-16.3.4 */
    let integration = std::fs::read_to_string("scripts/release/integration.sh")
        .expect("integration script should exist");

    assert!(
        integration.contains("--test real_upstream_smoke_gate"),
        "integration gate must run real upstream smoke contract tests: {integration}"
    );
}
