# promptfoo-rs Compatibility Matrix

**Status**: Ready
**Baseline**: promptfoo 0.121.13 + commit 4860e99

Readiness basis: PRD §Compatibility Matrix, PRD §Compatibility Harness Design, ADR-006, ADR-007, ADR-009. Task 1.2 owns executable validation and §10 completion notes before Phase 1 can be Done.

| Capability | Level | Target Status | Verification | Owner | Notes / Gap Reason |
|---|---|---|---|---|---|
| CLI command/flag inventory | P0 | native | golden diff stdout/stderr/exit code | leafiellune | eval/view/cache/redteam/mcp/code-scans/scan-model/import/export and common flags |
| promptfooconfig.yaml/json | P0 | native | config normalization golden diff | leafiellune | vars, prompts, tests, providers, assertions |
| redteam.yaml | P0 | native | redteam fixture golden diff | leafiellune | init/generate/eval/run/report core flow covered by task 7.1 |
| .env and file prompts/tests | P0 | native | Linux/macOS/Windows path/env/newline fixtures | leafiellune | CSV/JSON/YAML tests included |
| Eval runner | P0 | native | mock provider result/error/metadata golden diff | leafiellune | latency normalized |
| Cache/resume/retry/concurrency/delay | P0 | native | cache key, resume cursor, partial result, retry fixtures | leafiellune | Azure/assistant special keys tracked as matrix children |
| OpenAI-compatible provider | P0 | native | request/response snapshot + golden diff | leafiellune | env/header/model/options coverage |
| HTTP provider | P0 | native | request template/header/body/transform snapshot | leafiellune | common auth/header cases |
| Ollama provider | P0 | native | local mock server snapshot + golden diff | leafiellune | no real model download required |
| Anthropic provider | P0 | native | request/response snapshot + golden diff | leafiellune | network calls mocked |
| Other documented providers | P1/P2 | native/bridge/later | P1 request/output snapshot; P2 known gap row | leafiellune | P2 reason: long-tail provider scope requires Phase 1 inventory before implementation commitment |
| Deterministic assertions | P0 | native | assertion result golden diff | leafiellune | equals/contains/regex/json/schema core assertions |
| Model-graded assertions | P1 | native/bridge | prompt, threshold, score parsing, metadata snapshot | leafiellune | P1 because true LLM output is non-deterministic; mock/recorded grader required |
| JS/TS custom provider/assertion | P0 | bridge | allow-scripts fixture, stdio/env/timeout/error snapshot | leafiellune | default disabled behavior is P0 |
| Python custom provider/assertion | P0 | bridge | subprocess fixture, stdio/env/timeout/error snapshot | leafiellune | default disabled behavior is P0 |
| Shell/Ruby custom scripts | P1 | bridge | subprocess snapshot + security gate | leafiellune | Ruby support depends on upstream 0.121.13 documentation inventory |
| JSON/JSONL/CSV/YAML output | P0 | native | schema + golden diff | leafiellune | JSONL result store streaming and SQLite query schema covered by task 5.1; JSON/JSONL/CSV/YAML formatter contract covered by task 5.2 |
| HTML/JUnit XML/SARIF output | P0/P1 | native | JUnit/SARIF schema snapshot; HTML data contract snapshot | leafiellune | JUnit/SARIF/HTML snapshots covered by task 5.2; SARIF finding production tied to scan phase |
| Local Web viewer | P1 | native web | result schema read/filter/export smoke | leafiellune | P1 because pixel-level upstream UI parity is out of scope |
| Compatibility harness / golden diff gate | P0 | native | baseline lock, upstream/rs artifact snapshot, normalization snapshot, release gate summary | leafiellune | Harness runner locks promptfoo@0.121.13 and normalization rules covered by task 6.1; release gate classification and P0/P1/P2 summary covered by task 6.2 |
| Redteam plugins/strategies | P0/P1/P2 | native/later | full registry; core P0 golden diff; P1/P2 annotated | leafiellune | Core P0/P1/P2 registry, risk score snapshot, and report artifact covered by task 7.2; P2 reason required for long-tail plugins deferred after inventory |
| MCP provider / promptfoo mcp | P1 | native | protocol/request/response snapshot | leafiellune | P1 until protocol coverage is complete |
| code-scans / scan-model / model-audit | P1 | native | CLI protocol, SARIF, finding schema snapshot | leafiellune | false positive rate is not a 1.0 gate |
| Node API wrapper | P1 | bridge | JS API contract snapshot and wrapper/core drift test | leafiellune | wrapper must not reimplement eval logic |
| promptfoo cloud/share | P2 | unsupported/later | capability registration, no-upload test, user-visible error | leafiellune | P2 reason: 1.0 explicitly does not provide SaaS or default upload behavior; brand/legal copy needs review before public release |
