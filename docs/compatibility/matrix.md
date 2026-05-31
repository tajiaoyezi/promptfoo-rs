# promptfoo-rs Compatibility Matrix

**Status**: Ready
**Baseline**: promptfoo 0.121.13 + commit 4860e99

Readiness basis: PRD §Compatibility Matrix, PRD §Compatibility Harness Design, ADR-006, ADR-007, ADR-009. Task 1.2 owns executable validation and §10 completion notes before Phase 1 can be Done.

| Capability | Level | Target Status | Verification | Owner | Notes / Gap Reason |
|---|---|---|---|---|---|
| CLI command/flag inventory | P0 | native | golden diff stdout/stderr/exit code | leafiellune | task 13.1 maps the command inventory and task 16.1 closes view/cache/import/export plus output/concurrency flag status as implemented local compatibility behavior |
| promptfooconfig.yaml/json | P0 | native | config normalization golden diff | leafiellune | vars, prompts, tests, providers, assertions |
| redteam.yaml | P0 | native | redteam fixture golden diff | leafiellune | init/generate/eval/run/report core flow covered by task 7.1 |
| .env and file prompts/tests | P0 | native | Linux/macOS/Windows path/env/newline fixtures | leafiellune | CSV/JSON/YAML tests included |
| Eval runner | P0 | native | mock provider result/error/metadata golden diff | leafiellune | task 13.2 covers output targets, assertion/provider failure exit codes, and resume metadata; latency normalized |
| Cache/resume/retry/concurrency/delay | P0 | native | cache key, resume cursor, partial result, retry fixtures | leafiellune | task 13.2 covers resume-from-cache for remaining cases; Azure/assistant special keys tracked as matrix children |
| OpenAI-compatible provider | P0 | native | request/response snapshot + golden diff | leafiellune | env/header/model/options coverage; task 14.1 verifies P0 fixture coverage through provider/assertion parity report |
| HTTP provider | P0 | native | request template/header/body/transform snapshot | leafiellune | common auth/header cases |
| Ollama provider | P0 | native | local mock server snapshot + golden diff | leafiellune | no real model download required |
| Anthropic provider | P0 | native | request/response snapshot + golden diff | leafiellune | network calls mocked |
| Other documented providers | P1/P2 | native/bridge/later | P1 request/output snapshot; P2 known gap row | leafiellune | P2 reason: task 14.1 keeps dynamic provider registry visible as P2/later with reason; no silent provider omission is allowed |
| Deterministic assertions | P0 | native | assertion result golden diff | leafiellune | equals/contains/regex/json/schema core assertions; task 14.1 verifies P0 assertion fixture coverage |
| Model-graded assertions | P1 | native/bridge | prompt, threshold, score parsing, metadata snapshot | leafiellune | P1 because true LLM output is non-deterministic; mock/recorded grader required |
| JS/TS custom provider/assertion | P0 | bridge | allow-scripts fixture, stdio/env/timeout/error snapshot | leafiellune | Shared sandbox default-deny, stdio, timeout, env allowlist, and redaction covered by task 9.1; task 14.1 records JS/TS boundary policy evidence |
| Python custom provider/assertion | P0 | bridge | subprocess fixture, stdio/env/timeout/error snapshot | leafiellune | Shared sandbox default-deny, stdio, timeout, env allowlist, and redaction covered by task 9.1; Python runtime discovery fixture remains follow-up |
| Shell/Ruby custom scripts | P1 | bridge | subprocess snapshot + security gate | leafiellune | Shell sandbox fixture covered by task 9.1; Ruby support depends on upstream 0.121.13 documentation inventory |
| JSON/JSONL/CSV/YAML output | P0 | native | schema + golden diff | leafiellune | JSONL result store streaming and SQLite query schema covered by task 5.1; JSON/JSONL/CSV/YAML formatter contract covered by task 5.2 |
| HTML/JUnit XML/SARIF output | P0/P1 | native | JUnit/SARIF schema snapshot; HTML data contract snapshot | leafiellune | JUnit/SARIF/HTML snapshots covered by task 5.2; SARIF finding production tied to scan phase |
| Local Web viewer | P1 | native web | result schema read/filter/export smoke | leafiellune | Task 10.1 covers JSONL/SQLite stable result schema loading, failed/provider/assertion table filtering, JSON/CSV export, and `promptfoo-rs.viewer.v1`; P1 because pixel-level upstream UI parity is out of scope |
| Compatibility harness / golden diff gate | P0 | native | baseline lock, upstream/rs artifact snapshot, normalization snapshot, release gate summary | leafiellune | Harness runner locks promptfoo@0.121.13 and normalization rules covered by task 6.1; release gate classification and P0/P1/P2 summary covered by task 6.2; task 10.2 binds gate evidence into release checklist and downgrades blocked stable releases to prerelease/nightly |
| Redteam plugins/strategies | P0/P1/P2 | native/later | full registry; core P0 golden diff; P1/P2 annotated | leafiellune | Core registry, risk score snapshot, and report artifact covered by task 7.2; task 14.2 adds item-level `RedteamInventoryCoverage`, P0 fixture count = 4, missing P0 fixture/blocker = 0; P2 reason required for long-tail plugins deferred after inventory |
| MCP provider / promptfoo mcp | P1 | native | protocol/request/response snapshot | leafiellune | Command skeleton, provider JSON-RPC snapshot, and target materialization error path covered by task 8.1; P1 until protocol coverage is complete |
| code-scans / scan-model / model-audit | P1 | native | CLI protocol, SARIF, finding schema snapshot | leafiellune | Task 8.2 covers `promptfoo-rs.scan.v1`, SARIF finding metadata, and `scan.false-positive-rate` as a known limitation; false positive rate is not a 1.0 gate |
| Node API wrapper | P1 | bridge | JS API contract snapshot and wrapper/core drift test | leafiellune | Task 9.2 covers thin TypeScript wrapper source, Rust JSON-RPC evaluate contract, `promptfoo-rs.node-api.v1`, and release-blocking `node-api-wrapper-drift`; npm package scaffold waits for Corepack-enabled packaging environment |
| promptfoo cloud/share | P2 | unsupported/later | capability registration, no-upload test, user-visible error | leafiellune | P2 reason: 1.0 explicitly does not provide SaaS or default upload behavior; brand/legal copy needs review before public release |

## Item-Level Matrix Artifact

Task 11.3 adds the machine-readable item-level matrix manifest at `compatibility/matrix/items.json`. The manifest expands from `compatibility/inventory/upstream-items.json` through `expand_matrix_from_inventory`, so every inventory item receives a row with level, target status, verification, owner, and fixture/snapshot/gap reference. Aggregate markdown rows remain human summaries only and cannot satisfy `validate_no_silent_omissions`.

Task 14.1 adds `validate_provider_assertion_parity` over the item-level matrix and fixture corpus. Current provider/assertion manual review: P0 provider count = 4, P0 assertion count = 6, P0 missing fixture/blocker count = 0, and P2 provider/assertion missing reason count = 0.

Task 17.1 adds frozen source-tree extraction evidence at `target/release-gates/source-extracted-items.json` and `target/release-gates/source-inventory-evidence.json`. The v2 evidence records promptfoo `0.121.13` / commit `4860e990c7e9a2f8f677173fb92cf9867b34d03f`, npm integrity, source counts, and release blockers for any source-extracted item not yet represented by the item-level matrix. Missing rows are intentionally preserved as blockers for task 17.4 classification rather than hidden from the matrix.

Task 18.1 adds `target/release-gates/source-inventory-ledger.json` and upgrades source inventory evidence from missing-row blocker accounting to explicit ledger accounting. Current evidence records 2549 source-extracted items, 2549 ledger rows, 2116 generated accounting rows, `missing_matrix_rows=0`, `release_blockers=74`, and `p0_accounting_blocker_count=111`. This closes silent missing matrix rows; it does not claim those generated P0 rows are native parity.

Task 17.4 classifies the source-extracted provider/assertion/redteam long tail in tracked inventory and emits `target/release-gates/longtail-classification.json`. The initial artifact records 433 tracked long-tail rows, 0 missing tracked rows, 0 unresolved rows, 0 missing-reason rows, and 37 explicit P0 provider module release blockers that require dedicated fixture or blocker resolution before those per-file modules can be claimed as native parity.

Task 18.2 adds P0 provider module burndown evidence to `target/release-gates/longtail-classification.json`. Current evidence records `p0_provider_module_burndown.initial_blocker_count=37`, `resolved_by_fixture_count=13`, `remaining_blocker_count=24`, and `p0_release_blocker_count=24`; `p0_release_blockers[]` lists every remaining provider module blocker by item id, source reference, reason, verification, and external-authority flag. The 13 fixture-covered rows reuse existing P0 provider fixtures without real provider secrets; the remaining 24 rows stay release-blocking rather than being downgraded to P2/later.

Task 17.5 adds release installability evidence at `target/release-gates/installability.json` and includes it in runtime smoke / `release-candidate.json`. The current local evidence records installability_ready=`true`, publication_ready=`credential-blocked`, credential_blocked=`true`, channel-level published=`false`, release-candidate published=`false`, six local dry-run artifacts with checksums, 50 real P0 upstream corpus fixtures, and real upstream smoke exit code parity. This is not a claim that GitHub Releases, Homebrew, crates.io, Docker registry, or npm have been publicly published; those channels still require real credentials and release authority.

Task 18.4 adds publication authority evidence at `target/release-gates/publication-authority.json` and includes `publication_authority` in `target/release-gates/release-candidate.json`. Current evidence records publication_ready=`credential-blocked`, credential_blocked=true, legal_brand_blocked=true, six channel blockers, channel-level `installability_status` separated from `authority_status`, credential probes, legal/brand requirements, `published=false`, and `published_evidence=null`. This keeps local installability ready while preventing any stable public availability claim without real credentials, authorization, and external URL/digest evidence.
