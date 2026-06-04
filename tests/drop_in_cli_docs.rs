use std::fs;

const USER_DOCS: &[&str] = &[
    "README.md",
    "README.en.md",
    "docs/QUICKSTART.md",
    "docs/QUICKSTART.en.md",
];

const RELEASE_BOUNDARY_DOCS: &[&str] = &["README.md", "README.en.md", "docs/release.md"];

const CLAIM_DOCS: &[&str] = &[
    "README.md",
    "README.en.md",
    "docs/QUICKSTART.md",
    "docs/QUICKSTART.en.md",
    "docs/PROJECT-OVERVIEW.md",
    "docs/release.md",
    "NOTICE",
];

fn read_doc(path: &str) -> String {
    fs::read_to_string(path).unwrap_or_else(|error| panic!("{path} should be readable: {error}"))
}

fn assert_contains(path: &str, contents: &str, needle: &str) {
    assert!(
        contents.contains(needle),
        "{path} should contain `{needle}`"
    );
}

#[test]
fn test_45_3_1_docs_prefer_promptfoo_command_examples() {
    // TEST-45.3.1
    for path in USER_DOCS {
        let contents = read_doc(path);
        assert_contains(path, &contents, "promptfoo --help");
        assert_contains(path, &contents, "promptfoo eval -c promptfooconfig.yaml");
        assert_contains(path, &contents, "promptfoo view");
    }
}

#[test]
fn test_45_3_2_docs_explain_supported_aliases() {
    // TEST-45.3.2
    for path in USER_DOCS {
        let contents = read_doc(path);
        assert_contains(path, &contents, "promptfoo-rs");
        assert_contains(path, &contents, "`pf`");
    }

    let release = read_doc("docs/release.md");
    assert_contains("docs/release.md", &release, "`promptfoo`");
    assert_contains("docs/release.md", &release, "`promptfoo-rs`");
    assert_contains("docs/release.md", &release, "`pf`");
}

#[test]
fn test_45_3_3_docs_keep_publication_boundary_explicit() {
    // TEST-45.3.3
    for path in RELEASE_BOUNDARY_DOCS {
        let contents = read_doc(path);
        assert_contains(path, &contents, "local build/package smoke");
        assert_contains(path, &contents, "public registry publication");
    }
}

#[test]
fn test_45_3_4_docs_do_not_make_forbidden_release_or_parity_claims() {
    // TEST-45.3.4
    let forbidden_claims = [
        "bug-free",
        "no potential bugs",
        "zero possible bugs",
        "upstream endorsed",
        "official promptfoo distribution",
        "public stable publication is complete",
        "public registry publication is complete",
    ];

    for path in CLAIM_DOCS {
        let normalized = read_doc(path).to_ascii_lowercase();
        for claim in forbidden_claims {
            assert!(
                !normalized.contains(claim),
                "{path} should not contain forbidden claim `{claim}`"
            );
        }
    }
}
