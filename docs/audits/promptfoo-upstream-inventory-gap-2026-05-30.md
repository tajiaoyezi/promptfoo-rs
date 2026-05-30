# promptfoo upstream inventory gap audit - 2026-05-30

**Status**: Audit attachment
**Parent audit**: `docs/audits/promptfoo-perfect-refactor-audit-2026-05-30.md`
**Purpose**: make the upstream-vs-local surface gap inspectable without treating the current S2V `Done` state as upstream parity.

## Snapshot

| Item | Value |
|---|---|
| Upstream repository | `https://github.com/promptfoo/promptfoo` |
| Upstream ref | `origin/main` fetched with `--depth=1` |
| Upstream HEAD | `c24aa89804d35d6e4233edad80b38d67257cd508` |
| Upstream package version | `0.121.13` |
| Local branch before this attachment | `master` |
| Local HEAD before this attachment | `fbd2c191a92ced37b8a819a74c84b0a84d289bb2` |

This is a source-inventory audit. It does not prove semantic behavior by itself; it identifies where semantic parity evidence is absent.

## Extraction Method

Upstream evidence was extracted from a fresh local shallow fetch:

```powershell
$up = Join-Path $env:TEMP 'promptfoo-upstream-audit'
git clone --depth 1 https://github.com/promptfoo/promptfoo.git $up
git -C $up fetch --depth=1 origin main
git -C $up rev-parse FETCH_HEAD
git -C $up show FETCH_HEAD:package.json
git -C $up ls-tree -r --name-only FETCH_HEAD
```

Local evidence was extracted from `git ls-files` and targeted reads of `src/cli.rs`, `src/providers/`, `src/assertions/`, `src/redteam/`, and `docs/compatibility/matrix.md`.

## Command Surface

### Upstream command patterns

52 unique `.command(...)` patterns were observed under upstream `src/`:

`assertions`, `auth`, `cache`, `can-create-targets`, `clear`, `code-scans`, `config`, `current`, `dataset`, `dataset <id>`, `datasets`, `debug`, `delete <id>`, `email`, `email <email>`, `eval`, `eval [id]`, `eval <evalId>`, `eval <id>`, `evals`, `export`, `feedback [message]`, `generate`, `get`, `import <file>`, `init [directory]`, `list`, `login`, `logout`, `logs`, `logs [file]`, `mcp`, `optimize`, `plugins`, `poison`, `prompt <id>`, `prompts`, `redteam`, `report [directory]`, `retry <evalId>`, `run`, `scan-model`, `set`, `setup [configDirectory]`, `share [id]`, `show [id]`, `target`, `teams`, `unset`, `validate`, `view [directory]`, `whoami`.

Upstream also has 85 command-related TS/JS files under `src/commands`, `src/redteam/commands`, and `src/codeScan`.

### Local command variants

10 local top-level command variants are present in `src/cli.rs`:

`Eval`, `View`, `Cache`, `Redteam`, `Mcp`, `CodeScans`, `ScanModel`, `ModelAudit`, `Import`, `Export`.

`src/cli.rs` currently handles `View`, `Cache`, `Import`, and `Export` by returning `ExitCode::SUCCESS` without implementing upstream-equivalent behavior.

### Verdict

The local CLI is not a complete promptfoo CLI refactor. It covers a small named subset and contains no-op placeholders for several commands that the PRD treats as compatibility-critical.

## Provider Surface

### Upstream provider inventory

Upstream provider top-level directories:

`anthropic`, `audio`, `azure`, `bedrock`, `elevenlabs`, `github`, `google`, `groq`, `hyperbolic`, `mcp`, `mistral`, `nscale`, `nvidia`, `openai`, `openclaw`, `responses`, `video`, `xai`.

Upstream root provider modules:

`abliteration`, `agentic-utils`, `ai21`, `aimlapi`, `alibaba`, `atlascloud`, `browser`, `cerebras`, `claude-agent-sdk`, `cloudera`, `cloudflare-ai`, `cloudflare-gateway`, `cohere`, `cometapi`, `constants`, `databricks`, `deepseek`, `defaults`, `docker`, `echo`, `envoy`, `fal`, `functionCallbackTypes`, `functionCallbackUtils`, `golangCompletion`, `helicone`, `http`, `httpMultipart`, `httpTransforms`, `huggingface`, `index`, `jfrog`, `litellm`, `llama`, `llamaApi`, `localai`, `manualInput`, `minimax`, `mistral`, `mlflow-gateway`, `modelslab`, `n8n`, `novita`, `nscale`, `ollama`, `opencode-sdk`, `openrouter`, `orcarouter`, `packageParser`, `perplexity`, `portkey`, `promptfoo`, `promptfooModel`, `providerRegistry`, `pythonCompletion`, `quiverai`, `registry`, `registryTypes`, `replicate`, `rubyCompletion`, `sagemaker`, `scriptBasedProvider`, `scriptCompletion`, `scriptContext`, `sequence`, `shared`, `simulatedUser`, `slack`, `snowflake`, `togetherai`, `transformers`, `transformersAvailability`, `transformResult`, `transformUtils`, `truefoundry`, `vercel`, `voyage`, `watsonx`, `webhook`, `webSearchUtils`, `websocket`.

### Local provider inventory

Local provider files:

`src/providers/anthropic.rs`, `src/providers/http.rs`, `src/providers/mod.rs`, `src/providers/ollama.rs`, `src/providers/openai.rs`.

Local provider kinds:

`OpenAiCompatible`, `Http`, `Ollama`, `Anthropic`.

### Verdict

The local provider layer implements the PRD's P0 starter set, not the complete upstream provider surface. The matrix row `Other documented providers` remains aggregate and deferred, so the project cannot claim complete provider parity.

## Assertion Surface

### Upstream assertion modules

56 upstream assertion modules were observed:

`answerRelevance`, `assertionsResult`, `bleu`, `classifier`, `contains`, `contextFaithfulness`, `contextRecall`, `contextRelevance`, `contextUtils`, `cost`, `equals`, `factuality`, `finishReason`, `functionToolCall`, `geval`, `gleu`, `guardrails`, `html`, `index`, `javascript`, `json`, `latency`, `levenshtein`, `llmRubric`, `meteor`, `modelGradedClosedQa`, `moderation`, `ngrams`, `openai`, `perplexity`, `pi`, `python`, `redteam`, `refusal`, `regex`, `rouge`, `ruby`, `scriptResultNormalization`, `searchRubric`, `similar`, `skill`, `sql`, `startsWith`, `synthesis`, `toolCallF1`, `traceErrorSpans`, `traceSpanCount`, `traceSpanDuration`, `traceUtils`, `trajectory`, `trajectoryUtils`, `utils`, `validateAssertions`, `webhook`, `wordCount`, `xml`.

### Local assertion inventory

Local assertion files:

`src/assertions/custom.rs`, `src/assertions/deterministic.rs`, `src/assertions/mod.rs`, `src/assertions/model_graded.rs`.

Local deterministic assertion enum cases:

`Equals`, `Contains`, `Regex`, `JsonPointer`, `JsonSchema`.

Local custom/model-graded support exists as a contract slice, but there is no item-level evidence for most upstream assertion modules.

### Verdict

The local assertion layer is a useful core subset, not a complete assertion refactor.

## Redteam Surface

### Upstream redteam inventory

Upstream redteam top-level directories:

`audio`, `commands`, `constants`, `extraction`, `grading`, `plugins`, `providers`, `shared`, `strategies`, `types`.

Observed upstream counts:

| Surface | Count |
|---|---:|
| Redteam TS/JS files | 218 |
| Plugin files under `src/redteam/plugins` | 126 |
| Strategy files under `src/redteam/strategies` | 32 |

Representative plugin categories include agentic, compliance, ecommerce, financial, harmful, insurance, medical, pharmacy, policy, realestate, telecom, and teen safety.

### Local redteam inventory

Local redteam files:

`src/redteam/config.rs`, `src/redteam/flow.rs`, `src/redteam/mod.rs`, `src/redteam/registry.rs`, `src/redteam/report.rs`, `src/redteam/risk.rs`.

Local registry defaults:

- Plugins: `prompt-injection`, `harmful-content`, `custom-policy`
- Strategies: `jailbreak`, `multi-turn`, `agentic-chain`

### Verdict

The local redteam implementation provides a minimal compatible flow and a small registry, not a full upstream redteam refactor.

## Fixture And Gate Evidence

| Evidence | Local value | Impact |
|---|---:|---|
| Compatibility fixtures excluding `.gitkeep` | 0 | Cannot satisfy the PRD's 50-fixture P0 release gate |
| Adapter lint command | `N/A` | No enforced lint gate |
| Adapter integration command | `N/A` | No global integration gate |
| Adapter E2E command | `N/A` | No global E2E gate |
| Adapter coverage command | `N/A` | No enforced coverage gate |
| Adapter runtime smoke command | `N/A` | No global runtime smoke gate |

## Gap Classification

| Surface | Upstream observable evidence | Local observable evidence | Audit status |
|---|---|---|---|
| CLI commands | 52 command patterns, 85 command-related files | 10 variants, several no-op placeholders | Incomplete |
| Providers | 18 provider directories + 81 root provider modules | 4 provider kinds | Incomplete |
| Assertions | 56 assertion modules | core deterministic/custom/model-graded subset | Incomplete |
| Redteam | 126 plugin files + 32 strategy files | 3 plugin defaults + 3 strategy defaults | Incomplete |
| Compatibility fixtures | upstream examples and test corpus available | 0 tracked compatibility fixtures | Missing |
| Release gates | PRD requires P0 golden diff and complete matrix | several adapter gates still `N/A` | Not proven |

## Audit Decision

This attachment strengthens the parent audit conclusion: the current project is not yet a complete or perfect refactor of `promptfoo/promptfoo`. The next aligned S2V action would be to add a task for item-level upstream inventory and compatibility matrix expansion before implementing more parity work.
