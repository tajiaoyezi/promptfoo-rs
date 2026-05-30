# promptfoo-rs Compatibility Matrix

**Status**: Draft
**Baseline**: promptfoo 0.121.13 + commit 4860e99

| Capability | Level | Status | Verification | Owner | Notes |
|---|---|---|---|---|---|
| CLI command/flag inventory | P0 | native | golden diff | <TBD-by-user> | eval/view/cache/redteam/mcp/code-scans/scan-model/import/export |
| promptfooconfig.yaml/json | P0 | native | golden diff | <TBD-by-user> | config normalization |
| P0 providers | P0 | native | request/response snapshot + golden diff | <TBD-by-user> | OpenAI-compatible, HTTP, Ollama, Anthropic |
| Long-tail providers | P1/P2 | native/bridge/later | snapshot or known gap | <TBD-by-user> | expand during Phase 1 |