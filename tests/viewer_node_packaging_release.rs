use std::path::{Path, PathBuf};

use promptfoo_rs::release::{
    run_release_packaging_smoke, verify_npm_wrapper_package, verify_viewer_package,
    PackagingSmokeConfig,
};

#[test]
fn test_15_1_1_viewer_package_has_lockfile_scripts_and_browser_smoke() {
    /* TEST-15.1.1 */
    let check = verify_viewer_package(Path::new("viewer")).expect("viewer package should verify");

    assert_eq!(check.package_name, "@promptfoo-rs/viewer");
    assert!(check.has_lockfile, "{check:#?}");
    for script in ["typecheck", "test", "build", "smoke:browser"] {
        assert!(check.scripts.contains_key(script), "{check:#?}");
    }
    assert!(
        check
            .scripts
            .get("build")
            .expect("build script exists")
            .contains("smoke:browser"),
        "{check:#?}"
    );
    assert!(check.entrypoints.contains(&"src/App.tsx".to_string()));
    assert!(check.entrypoints.contains(&"src/results.ts".to_string()));
}

#[test]
fn test_15_1_2_npm_wrapper_package_is_thin_node_api_transport() {
    /* TEST-15.1.2 */
    let check = verify_npm_wrapper_package(Path::new("npm")).expect("npm wrapper should verify");

    assert_eq!(check.package_name, "@promptfoo-rs/node");
    assert!(check.has_lockfile, "{check:#?}");
    for script in ["typecheck", "test", "build", "smoke:node"] {
        assert!(check.scripts.contains_key(script), "{check:#?}");
    }
    assert!(check.thin_wrapper, "{check:#?}");
    assert_eq!(check.transport.as_deref(), Some("json-rpc-stdio"));
    assert!(check.exported_api.contains(&"evaluate".to_string()));
    assert!(check
        .exported_api
        .contains(&"createPromptfooClient".to_string()));
}

#[test]
fn test_15_1_3_release_packaging_smoke_records_dry_run_artifacts_without_publish() {
    /* TEST-15.1.3 */
    let report = run_release_packaging_smoke(&PackagingSmokeConfig {
        root: PathBuf::from("."),
        dry_run: true,
        publish: false,
    })
    .expect("release packaging smoke should run");

    assert!(report.dry_run, "{report:#?}");
    assert!(!report.published, "{report:#?}");
    assert_eq!(report.package_names.viewer, "@promptfoo-rs/viewer");
    assert_eq!(report.package_names.npm_wrapper, "@promptfoo-rs/node");
    assert!(report.no_publish_evidence.contains("publish=false"));
    for artifact in ["viewer-dist", "npm-wrapper-dist"] {
        let record = report
            .artifacts
            .iter()
            .find(|candidate| candidate.name == artifact)
            .unwrap_or_else(|| panic!("missing artifact {artifact}: {report:#?}"));
        assert!(!record.checksum_sha256.is_empty(), "{record:#?}");
        assert!(
            record.path.starts_with("target/package-smoke"),
            "{record:#?}"
        );
    }

    let workflow = std::fs::read_to_string(".github/workflows/release.yml")
        .expect("release workflow should exist");
    assert!(workflow.contains("Viewer package smoke"), "{workflow}");
    assert!(workflow.contains("npm wrapper package smoke"), "{workflow}");
}
