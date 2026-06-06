#!/usr/bin/env bash
set -euo pipefail

GATE_DIR="${CURRENT_LATEST_GATE_DIR:-target/release-gates}"
LOCK_FILE="${CURRENT_LATEST_TARGET_LOCK_FILE:-$GATE_DIR/current-latest-target.json}"
OUT="$GATE_DIR/current-latest-source-inventory.json"
MATRIX_OUT="$GATE_DIR/current-latest-matrix.json"
UPSTREAM_REPO="${CURRENT_LATEST_UPSTREAM_REPO:-https://github.com/promptfoo/promptfoo.git}"

mkdir -p "$GATE_DIR"

if [ ! -f "$LOCK_FILE" ] && [ -f "compatibility/inventory/current-latest-target.json" ]; then
  LOCK_FILE="compatibility/inventory/current-latest-target.json"
fi

tmpdir="$(mktemp -d)"
trap 'rm -rf "$tmpdir"' EXIT

source_root="${CURRENT_LATEST_SOURCE_ROOT:-}"
if [ -z "$source_root" ]; then
  head_sha="$(node -e "const r = JSON.parse(require('fs').readFileSync(process.argv[1], 'utf8')); console.log(r.github.default_branch_head)" "$LOCK_FILE")"
  source_root="$tmpdir/upstream"
  source_root_label="git:${UPSTREAM_REPO}#${head_sha}"
  git init --quiet "$source_root"
  git -C "$source_root" remote add origin "$UPSTREAM_REPO"
  git -C "$source_root" fetch --quiet --depth 1 origin "$head_sha"
  git -C "$source_root" checkout --quiet --detach FETCH_HEAD
else
  source_root_label="$source_root"
fi

node - "$LOCK_FILE" "$source_root" "$OUT" "$MATRIX_OUT" "$source_root_label" <<'NODE'
const fs = require('fs');
const path = require('path');

const [lockPath, sourceRoot, inventoryPath, matrixPath, sourceRootLabel] = process.argv.slice(2);
const lock = JSON.parse(fs.readFileSync(lockPath, 'utf8'));
const head = lock.github && lock.github.default_branch_head;
if (!/^[0-9a-fA-F]{40}$/.test(head || '')) {
  throw new Error('current latest lock missing full default_branch_head SHA');
}

function slug(value) {
  return String(value)
    .trim()
    .toLowerCase()
    .replace(/[^a-z0-9]+/g, '-')
    .replace(/^-+|-+$/g, '');
}

function stableId(category, name) {
  return `${slug(category)}:${slug(name)}`;
}

function normalize(file) {
  return file.replace(/\\/g, '/').replace(/^package\//, '');
}

function withoutExtension(file) {
  return file.replace(/\.[^.]+$/, '');
}

function isTsOrJs(file) {
  return /\.(tsx?|jsx?|mjs|cjs)$/.test(file);
}

function isCommand(file) {
  return (
    (file === 'src/main.ts' ||
      file.startsWith('src/commands/') ||
      file.startsWith('src/redteam/commands/') ||
      file.startsWith('src/codeScan/')) &&
    isTsOrJs(file)
  );
}

function isProvider(file) {
  return file.startsWith('src/providers/') && isTsOrJs(file);
}

function isAssertion(file) {
  return file.startsWith('src/assertions/') && isTsOrJs(file);
}

function isRedteamPlugin(file) {
  return file.startsWith('src/redteam/plugins/') && isTsOrJs(file);
}

function isRedteamStrategy(file) {
  return file.startsWith('src/redteam/strategies/') && isTsOrJs(file);
}

function isOutput(file) {
  return file.startsWith('src/') && /(output|report|csv|junit|sarif|yaml|jsonl)/i.test(file);
}

function isConfig(file) {
  return file.startsWith('src/') && /config/i.test(file) && !isCurrentLatestViewerConfig(file);
}

function isCurrentLatestViewerConfig(file) {
  return file.startsWith('src/app/') && /config/i.test(file);
}

function isCurrentLatestRuntimeConfig(file) {
  const lower = file.toLowerCase();
  return lower === 'src/commands/config.ts' || lower === 'src/configtypes.ts' || lower.startsWith('src/util/config/');
}

function isCurrentLatestRedteamConfig(file) {
  return file.toLowerCase() === 'src/redteam/plugins/policy/evals/promptfooconfig.yaml';
}

function isCurrentLatestAuxiliaryConfig(file) {
  const lower = file.toLowerCase();
  return lower.startsWith('src/codescan/config/') || lower === 'src/commands/mcp/tools/validatepromptfooconfig.ts';
}

function currentLatestAuxiliaryConfigOwner(file) {
  return file.toLowerCase().startsWith('src/codescan/config/') ? 'scan-engine' : 'mcp-runtime';
}

function isCurrentLatestExternalConfig(file) {
  const lower = file.toLowerCase();
  return (
    lower.startsWith('src/globalconfig/') ||
    lower.startsWith('src/server/config/') ||
    lower === 'src/server/routes/configs.ts' ||
    lower === 'src/tracing/otelconfig.ts' ||
    lower === 'src/types/api/configs.ts'
  );
}

function isViewer(file) {
  return file.startsWith('src/app/') || file.startsWith('src/server/') || file.startsWith('src/openapi/');
}

function isNodeApi(file) {
  return (
    (file === 'src/index.ts' ||
      file === 'src/index.js' ||
      file.startsWith('src/node/') ||
      file.startsWith('npm/src/') ||
      file.startsWith('packages/node/')) &&
    isTsOrJs(file)
  );
}

function isExample(file) {
  return file.startsWith('examples/');
}

function isDocs(file) {
  const lower = file.toLowerCase();
  return file.startsWith('docs/') && (lower.endsWith('.md') || lower.endsWith('.mdx'));
}

function isP0Provider(file) {
  return ['src/providers/openai', 'src/providers/http', 'src/providers/ollama', 'src/providers/anthropic'].some(
    (prefix) => file.startsWith(prefix),
  );
}

function currentLatestProviderFixtureIds(id) {
  const mapping = {
    'provider:src-providers-anthropic-completion': ['p0-provider-anthropic-completion'],
    'provider:src-providers-anthropic-defaults': ['p0-provider-anthropic-message'],
    'provider:src-providers-anthropic-generic': ['p0-provider-anthropic-message'],
    'provider:src-providers-anthropic-messages': ['p0-provider-anthropic-message'],
    'provider:src-providers-anthropic-types': ['p0-provider-anthropic-message'],
    'provider:src-providers-anthropic-util': ['p0-provider-anthropic-message'],
    'provider:src-providers-http': ['p0-provider-http-get', 'p0-provider-http-post'],
    'provider:src-providers-httpmultipart': ['p0-provider-http-multipart'],
    'provider:src-providers-httptransforms': ['p0-provider-http-transform'],
    'provider:src-providers-ollama': ['p0-provider-ollama-chat'],
    'provider:src-providers-openai-chat': ['p0-provider-openai-chat'],
    'provider:src-providers-openai-completion': ['p0-provider-openai-completion'],
    'provider:src-providers-openai-defaults': ['p0-provider-openai-env', 'p0-provider-openai-headers'],
    'provider:src-providers-openai-embedding': ['p0-provider-openai-embedding'],
    'provider:src-providers-openai-image': ['p0-provider-openai-image'],
    'provider:src-providers-openai-index': ['p0-provider-openai-chat'],
    'provider:src-providers-openai-moderation': ['p0-provider-openai-moderation'],
    'provider:src-providers-openai-responses': ['p0-provider-openai-responses'],
    'provider:src-providers-openai-transcription': ['p0-provider-openai-transcription'],
    'provider:src-providers-openai-types': ['p0-provider-openai-chat'],
    'provider:src-providers-openai-util': [
      'p0-provider-openai-chat',
      'p0-provider-openai-env',
      'p0-provider-openai-headers',
    ],
    'provider:src-providers-openai-video': ['p0-provider-openai-video'],
  };
  return mapping[id] || [];
}

function isCurrentLatestFixtureProvider(id, file) {
  return isP0Provider(file) && currentLatestProviderFixtureIds(id).length > 0;
}

function isCurrentLatestExternalProvider(file) {
  const lower = file.toLowerCase();
  return (
    lower === 'src/providers/anthropic/claudecodeauth.ts' ||
    lower.startsWith('src/providers/openai/agents') ||
    lower === 'src/providers/openai/assistant.ts' ||
    lower === 'src/providers/openai/billing.ts' ||
    lower.startsWith('src/providers/openai/chatkit') ||
    lower.startsWith('src/providers/openai/codex') ||
    lower === 'src/providers/openai/realtime.ts'
  );
}

function currentLatestProviderExternalReason(id, file) {
  const lower = id.toLowerCase();
  let reason;
  if (lower.includes('claudecodeauth')) {
    reason =
      'Anthropic Claude Code auth requires real local credential flow and product authority before current-latest parity can be claimed';
  } else if (lower.includes('codex')) {
    reason =
      'OpenAI Codex provider modules require external product authority and private SDK/server credential confirmation before current-latest parity can be claimed';
  } else if (lower.includes('billing')) {
    reason =
      'OpenAI billing module requires account-level credentials and billing authority; no local mock may be treated as live parity';
  } else if (lower.includes('chatkit')) {
    reason =
      'OpenAI ChatKit modules require product authority and browser/session fixture confirmation before current-latest parity can be claimed';
  } else if (lower.includes('agents')) {
    reason = 'OpenAI Agents SDK and tracing modules require dedicated SDK/trace fixtures plus product contract review';
  } else if (lower.includes('realtime')) {
    reason = 'OpenAI realtime module requires a dedicated streaming protocol fixture and service contract confirmation';
  } else if (lower.includes('assistant')) {
    reason = 'OpenAI Assistants module requires a stateful API fixture and account-authorized behavior review';
  } else {
    reason = 'Provider module requires external authority before current-latest parity can be claimed';
  }
  return `explicit current-latest external provider blocker: ${reason}; source: ${file}`;
}

function isEvalRuntime(file) {
  return (
    isTsOrJs(file) &&
    (['src/evaluate.ts', 'src/evaluator.ts', 'src/evaluator/inMemoryStore.ts', 'src/evaluator/runtime.ts', 'src/evaluatorHelpers.ts', 'src/testCase.ts'].includes(file) ||
      file.startsWith('src/scheduler/') ||
      file.startsWith('src/testCase/') ||
      file.startsWith('src/optimizer/'))
  );
}

function currentLatestEvalRunnerFixtureIds(id) {
  const mapping = {
    'eval-runner:src-evaluate': ['p0-eval-basic', 'p0-eval-output-json', 'p0-eval-retry-timeout'],
    'eval-runner:src-evaluator': ['p0-eval-basic', 'p0-eval-output-json', 'p0-eval-retry-timeout'],
    'eval-runner:src-evaluator-runtime': ['p0-eval-runtime-execution', 'p0-eval-basic', 'p0-eval-output-json', 'p0-eval-retry-timeout'],
    'eval-runner:src-evaluatorhelpers': ['p0-eval-basic', 'p0-eval-output-json', 'p0-eval-retry-timeout'],
    'eval-runner:src-scheduler-index': [
      'p0-eval-concurrency-limit',
      'p0-eval-delay',
      'p0-eval-partial-failure',
    ],
    'eval-runner:src-scheduler-providercallqueue': [
      'p0-eval-concurrency-limit',
      'p0-eval-delay',
      'p0-eval-partial-failure',
    ],
    'eval-runner:src-scheduler-slotqueue': [
      'p0-eval-concurrency-limit',
      'p0-eval-delay',
      'p0-eval-partial-failure',
    ],
    'eval-runner:src-scheduler-types': [
      'p0-eval-concurrency-limit',
      'p0-eval-delay',
      'p0-eval-partial-failure',
    ],
    'eval-runner:src-scheduler-retrypolicy': ['p0-eval-retry-timeout'],
    'eval-runner:src-scheduler-adaptiveconcurrency': ['p0-eval-adaptive-concurrency'],
    'eval-runner:src-scheduler-headerparser': ['p0-eval-rate-limit-headers'],
    'eval-runner:src-scheduler-providercallexecutioncontext': ['p0-eval-provider-call-context'],
    'eval-runner:src-scheduler-providerratelimitstate': ['p0-eval-rate-limit-state'],
    'eval-runner:src-scheduler-providerwrapper': ['p0-eval-provider-wrapper'],
    'eval-runner:src-scheduler-ratelimitkey': ['p0-eval-rate-limit-key'],
    'eval-runner:src-scheduler-ratelimitregistry': ['p0-eval-rate-limit-registry'],
  };
  return mapping[id] || [];
}

function isCurrentLatestEvalRunnerFixture(id, file) {
  return isEvalRuntime(file) && currentLatestEvalRunnerFixtureIds(id).length > 0;
}

function currentLatestEvalRunnerFixtureReason(id, file) {
  if (id.toLowerCase().includes('evaluator-runtime')) {
    return 'current-latest evaluator runtime source is covered by deterministic evaluator runtime fixture evidence';
  }
  return 'current-latest eval/evaluator/scheduler source is covered by existing deterministic eval runner fixture evidence';
}

function isCurrentLatestEvalRunnerSnapshot(file) {
  return [
    'src/optimizer/promptOptimizer.ts',
    'src/scheduler/events.ts',
    'src/testCase/synthesis.ts',
  ].includes(file);
}

function currentLatestEvalRunnerBlockerReason(id, file) {
  const lower = id.toLowerCase();
  let reason;
  if (lower.includes('adaptiveconcurrency')) {
    reason = 'adaptive concurrency requires dedicated current-latest scheduler fixture evidence';
  } else if (lower.includes('inmemorystore')) {
    reason = 'evaluator in-memory store requires dedicated current-latest eval-runner in-memory store fixture evidence';
  } else if (lower.includes('runtime')) {
    reason = 'evaluator runtime requires dedicated current-latest eval-runner runtime fixture evidence';
  } else if (lower.includes('headerparser')) {
    reason = 'provider rate-limit header parsing requires dedicated current-latest eval-runner evidence';
  } else if (lower.includes('providercallexecutioncontext')) {
    reason = 'provider call execution context requires dedicated current-latest eval-runner evidence';
  } else if (lower.includes('providerratelimitstate')) {
    reason = 'provider rate-limit state requires dedicated current-latest eval-runner evidence';
  } else if (lower.includes('providerwrapper')) {
    reason = 'provider wrapper behavior requires dedicated current-latest eval-runner evidence';
  } else if (lower.includes('ratelimitkey')) {
    reason = 'provider rate-limit key derivation requires dedicated current-latest eval-runner evidence';
  } else if (lower.includes('ratelimitregistry')) {
    reason = 'provider rate-limit registry behavior requires dedicated current-latest eval-runner evidence';
  } else {
    reason = 'dedicated current-latest eval-runner evidence is required';
  }
  return `dedicated current-latest eval-runner evidence is required before this row can be claimed native: ${reason}; source: ${file}`;
}

function isCacheStore(file) {
  return (
    isTsOrJs(file) &&
    (file === 'src/cache.ts' || file.startsWith('src/database/') || file.startsWith('src/storage/'))
  );
}

function isCurrentLatestCacheStoreFixture(id, file) {
  const ids = [
    'cache-store:src-cache',
    'cache-store:src-database-evaldeletion',
    'cache-store:src-database-index',
    'cache-store:src-database-tables',
    'cache-store:src-storage-index',
    'cache-store:src-storage-localfilesystemprovider',
    'cache-store:src-storage-types',
  ];
  const files = [
    'src/cache.ts',
    'src/database/evalDeletion.ts',
    'src/database/index.ts',
    'src/database/tables.ts',
    'src/storage/index.ts',
    'src/storage/localFileSystemProvider.ts',
    'src/storage/types.ts',
  ];
  return isCacheStore(file) && (ids.includes(id) || files.includes(file));
}

function isCurrentLatestCacheStoreSnapshot(file) {
  return isCacheStore(file) && ['src/database/signal.ts', 'src/database/testing.ts'].includes(file);
}

function currentLatestCacheStoreBlockerReason(id, file) {
  const lower = id.toLowerCase();
  const reason =
    lower.includes('evaldeletion') || file === 'src/database/evalDeletion.ts'
      ? 'eval deletion lifecycle semantics require dedicated current-latest cache-store evidence'
      : 'dedicated current-latest cache-store evidence is required';
  return `dedicated current-latest cache-store evidence is required before this row can be claimed native: ${reason}; source: ${file}`;
}

function isPromptProcessing(file) {
  return (
    isTsOrJs(file) &&
    (file.startsWith('src/prompts/') ||
      file.startsWith('src/external/prompts/') ||
      file.startsWith('src/optimizer/'))
  );
}

function currentLatestPromptProcessingFixtureIds(id) {
  const localProcessorFixtures = currentLatestLocalPromptProcessorFixtureIds(id);
  if (localProcessorFixtures.length > 0) {
    return localProcessorFixtures;
  }
  const scriptProcessorFixtures = currentLatestScriptPromptProcessorFixtureIds(id);
  if (scriptProcessorFixtures.length > 0) {
    return scriptProcessorFixtures;
  }

  const mapping = {
    'prompt-processing:src-prompts-index': ['p0-config-file-prompt', 'p0-eval-prompt-vars'],
    'prompt-processing:src-prompts-utils': ['p0-config-file-prompt', 'p0-eval-prompt-vars'],
    'prompt-processing:src-prompts-processors-string': ['p0-eval-prompt-vars'],
    'prompt-processing:src-prompts-processors-text': ['p0-config-file-prompt'],
  };
  return mapping[id] || [];
}

function currentLatestScriptPromptProcessorFixtureIds(id) {
  const mapping = {
    'prompt-processing:src-prompts-processors-executable': [
      'p0-script-prompt-executable-processor',
    ],
    'prompt-processing:src-prompts-processors-javascript': [
      'p0-script-prompt-javascript-processor',
    ],
    'prompt-processing:src-prompts-processors-python': [
      'p0-script-prompt-python-processor',
    ],
  };
  return mapping[id] || [];
}

function currentLatestLocalPromptProcessorFixtureIds(id) {
  const mapping = {
    'prompt-processing:src-prompts-processors-json': ['p0-config-file-prompt'],
    'prompt-processing:src-prompts-processors-markdown': ['p0-config-file-prompt'],
    'prompt-processing:src-prompts-processors-jinja': [
      'p0-config-file-prompt',
      'p0-eval-prompt-vars',
    ],
  };
  return mapping[id] || [];
}

function isCurrentLatestScriptPromptProcessorFixture(id, file) {
  return isPromptProcessing(file) && currentLatestScriptPromptProcessorFixtureIds(id).length > 0;
}

function isCurrentLatestLocalPromptProcessorFixture(id, file) {
  return isPromptProcessing(file) && currentLatestLocalPromptProcessorFixtureIds(id).length > 0;
}

function isCurrentLatestPromptProcessingFixture(id, file) {
  return isPromptProcessing(file) && currentLatestPromptProcessingFixtureIds(id).length > 0;
}

function currentLatestPromptProcessingFixtureOwner(id, file) {
  return isCurrentLatestScriptPromptProcessorFixture(id, file) ? 'script-bridge' : 'config-loader';
}

function currentLatestPromptProcessingFixtureReason(id, file) {
  if (isCurrentLatestScriptPromptProcessorFixture(id, file)) {
    return 'current-latest JavaScript, Python, and executable prompt processor source is covered by deterministic authorized script bridge fixtures';
  }
  if (isCurrentLatestLocalPromptProcessorFixture(id, file)) {
    return 'current-latest JSON, Markdown, and Jinja prompt processor source is covered by deterministic local config and eval prompt fixtures';
  }
  return 'current-latest prompt index/string/text/utils source is covered by existing deterministic config and eval prompt fixtures';
}

function isCurrentLatestPromptProcessingSnapshot(file) {
  return [
    'src/external/prompts/ragas.ts',
    'src/prompts/constants.ts',
    'src/prompts/grading.ts',
  ].includes(file);
}

function currentLatestPromptProcessingBlockerOwner(id, file) {
  const lower = id.toLowerCase();
  if (
    lower.includes('javascript') ||
    lower.includes('python') ||
    lower.includes('executable') ||
    file.includes('/javascript.') ||
    file.includes('/python.') ||
    file.includes('/executable.')
  ) {
    return 'script-bridge';
  }
  return 'config-loader';
}

function currentLatestPromptProcessingBlockerReason(id, file) {
  const lower = id.toLowerCase();
  let reason;
  if (lower.includes('javascript')) {
    reason = 'JavaScript prompt processor requires dedicated current-latest script bridge fixture evidence';
  } else if (lower.includes('python')) {
    reason = 'Python prompt processor requires dedicated current-latest script bridge fixture evidence';
  } else if (lower.includes('executable')) {
    reason = 'executable prompt processor requires dedicated current-latest script bridge subprocess fixture evidence';
  } else if (lower.includes('jinja')) {
    reason = 'Jinja prompt processor requires dedicated current-latest template rendering fixture evidence';
  } else if (lower.includes('json')) {
    reason = 'JSON prompt processor requires dedicated current-latest structured prompt fixture evidence';
  } else if (lower.includes('markdown')) {
    reason = 'Markdown prompt processor requires dedicated current-latest markdown fixture evidence';
  } else {
    reason = 'dedicated current-latest prompt-processing evidence is required';
  }
  return `dedicated current-latest prompt-processing evidence is required before this row can be claimed native: ${reason}; source: ${file}`;
}

function isAssertionSupport(file) {
  return (
    isTsOrJs(file) &&
    (file.startsWith('src/matchers/') ||
      file.startsWith('src/external/matchers/') ||
      file.startsWith('src/external/assertions/') ||
      ['src/remoteGrading.ts', 'src/remoteScoring.ts', 'src/guardrails.ts'].includes(file))
  );
}

function isRedteamSupport(file) {
  return file.startsWith('src/redteam/') && isTsOrJs(file);
}

function isSchema(file) {
  return (
    isTsOrJs(file) &&
    (file === 'src/contracts.ts' ||
      file.startsWith('src/types/') ||
      file.startsWith('src/contracts/') ||
      file.startsWith('src/models/') ||
      file.startsWith('src/validators/'))
  );
}

function isScriptBridge(file) {
  return isTsOrJs(file) && (file.startsWith('src/python/') || file.startsWith('src/ruby/'));
}

function currentLatestPythonScriptBridgeFixtureIds(id) {
  const mapping = {
    'script-bridge:src-python-pythonutils': ['p0-python-bridge-runtime-utils'],
    'script-bridge:src-python-stderr': ['p0-python-bridge-stderr'],
    'script-bridge:src-python-worker': ['p0-python-bridge-worker'],
    'script-bridge:src-python-workerpool': ['p0-python-bridge-worker-pool'],
    'script-bridge:src-python-wrapper': ['p0-python-bridge-wrapper'],
  };
  return mapping[id] || [];
}

function isCurrentLatestPythonScriptBridgeFixture(id, file) {
  return file.startsWith('src/python/') && currentLatestPythonScriptBridgeFixtureIds(id).length > 0;
}

function currentLatestRubyScriptBridgeFixtureIds(id) {
  const mapping = {
    'script-bridge:src-ruby-rubyutils': ['p0-ruby-bridge-runtime-utils'],
    'script-bridge:src-ruby-wrapper': ['p0-ruby-bridge-wrapper'],
  };
  return mapping[id] || [];
}

function isCurrentLatestRubyScriptBridgeFixture(id, file) {
  return file.startsWith('src/ruby/') && currentLatestRubyScriptBridgeFixtureIds(id).length > 0;
}

function currentLatestScriptBridgeBlockerReason(id, file) {
  if (file.startsWith('src/ruby/')) {
    return `current-latest Ruby script bridge surface requires dedicated Ruby runtime fixture evidence before this row can be claimed native: ${id}; source: ${file}`;
  }
  return `current-latest script bridge surface requires authorized subprocess fixture evidence before this row can be claimed native: ${id}; source: ${file}`;
}

function isImportExport(file) {
  return isTsOrJs(file) && (file.startsWith('src/importers/') || file.startsWith('src/util/exportToFile/'));
}

function isIntegration(file) {
  return (
    isTsOrJs(file) &&
    (file.startsWith('src/integrations/') || ['src/googleSheets.ts', 'src/microsoftSharepoint.ts'].includes(file))
  );
}

function isCloudShare(file) {
  return (
    isTsOrJs(file) &&
    ([
      'src/share.ts',
      'src/feedback.ts',
      'src/onboarding.ts',
      'src/suggestions.ts',
      'src/telemetry.ts',
      'src/telemetryEvents.ts',
      'src/updates.ts',
    ].includes(file) ||
      file.startsWith('src/updates/'))
  );
}

function isBlobStore(file) {
  return file.startsWith('src/blobs/') && isTsOrJs(file);
}

function isObservability(file) {
  return file.startsWith('src/tracing/') && isTsOrJs(file);
}

function isRuntimeSupport(file) {
  return (
    isTsOrJs(file) &&
    (file.startsWith('src/util/') ||
      file.startsWith('src/constants/') ||
      file.startsWith('src/__mocks__/') ||
      [
        'src/cliState.ts',
        'src/constants.ts',
        'src/entrypoint.ts',
        'src/envars.ts',
        'src/envOverrides.ts',
        'src/esm.ts',
        'src/logger.ts',
        'src/logger.browser.ts',
        'src/mainUtils.ts',
        'src/migrate.ts',
        'src/table.ts',
        'src/version.ts',
      ].includes(file))
  );
}

function categoriesFor(file) {
  const categories = [];
  if (isCommand(file)) categories.push('command');
  if (isProvider(file)) categories.push('provider');
  if (isAssertion(file)) categories.push('assertion');
  if (isRedteamPlugin(file)) categories.push('redteam-plugin');
  if (isRedteamStrategy(file)) categories.push('redteam-strategy');
  if (isOutput(file)) categories.push('output');
  if (isConfig(file)) categories.push('config');
  if (isViewer(file)) categories.push('viewer');
  if (isNodeApi(file)) categories.push('node-api');
  if (isExample(file)) categories.push('example');
  if (isDocs(file)) categories.push('docs');
  if (categories.length === 0 && isEvalRuntime(file)) categories.push('eval-runner');
  if (categories.length === 0 && isCacheStore(file)) categories.push('cache-store');
  if (categories.length === 0 && isPromptProcessing(file)) categories.push('prompt-processing');
  if (categories.length === 0 && isAssertionSupport(file)) categories.push('assertion-support');
  if (categories.length === 0 && isRedteamSupport(file)) categories.push('redteam-support');
  if (categories.length === 0 && isSchema(file)) categories.push('schema');
  if (categories.length === 0 && isScriptBridge(file)) categories.push('script-bridge');
  if (categories.length === 0 && isImportExport(file)) categories.push('import-export');
  if (categories.length === 0 && isIntegration(file)) categories.push('integration');
  if (categories.length === 0 && isCloudShare(file)) categories.push('cloud-share');
  if (categories.length === 0 && isBlobStore(file)) categories.push('blob-store');
  if (categories.length === 0 && isObservability(file)) categories.push('observability');
  if (categories.length === 0 && isRuntimeSupport(file)) categories.push('runtime-support');
  if (categories.length === 0 && file.startsWith('src/') && isTsOrJs(file)) {
    categories.push('unclassified');
  }
  return categories;
}

function metadata(category, id, file) {
  if (category === 'command') return ['P1', 'later', 'cli', 'snapshot', `current-latest command requires CLI behavior snapshot or fixture evidence; item: ${id}`];
  if (category === 'flag') return ['P1', 'later', 'cli', 'snapshot', `current-latest flag requires CLI parity snapshot or fixture evidence; item: ${id}`];
  if (category === 'provider' && isCurrentLatestFixtureProvider(id, file)) return ['P0', 'native', 'provider-runtime', 'fixture', `current-latest P0 provider source is covered by existing mock/recorded provider request-response fixture evidence; item: ${id}`];
  if (category === 'provider' && isCurrentLatestExternalProvider(file)) return ['P0', 'blocked', 'external-authority', 'blocker', `${currentLatestProviderExternalReason(id, file)}; item: ${id}`];
  if (category === 'provider' && isP0Provider(file)) return ['P0', 'blocked', 'provider-runtime', 'blocker', `current-latest P0 provider requires native or bridge fixture evidence; item: ${id}`];
  if (category === 'provider') return ['P2', 'later', 'provider-runtime', 'registration', `current-latest long-tail provider is registered until fixture evidence promotes it; item: ${id}`];
  if (category === 'assertion') return ['P1', 'later', 'assertion-engine', 'snapshot', `current-latest assertion requires snapshot evidence; item: ${id}`];
  if (category === 'redteam-plugin' || category === 'redteam-strategy') return ['P1', 'later', 'redteam-engine', 'snapshot', `current-latest redteam surface requires snapshot evidence; item: ${id}`];
  if (category === 'output') return ['P1', 'later', 'reporting', 'snapshot', `current-latest output surface requires output contract snapshot; item: ${id}`];
  if (category === 'config' && isCurrentLatestRuntimeConfig(file)) return ['P0', 'native', 'config-loader', 'fixture', `current-latest runtime promptfooconfig/env/file config surface is covered by existing P0 native config fixtures; item: ${id}`];
  if (category === 'config' && isCurrentLatestRedteamConfig(file)) return ['P0', 'native', 'redteam-engine', 'fixture', `current-latest redteam promptfooconfig source is covered by redteam YAML fixture evidence; item: ${id}`];
  if (category === 'config' && isCurrentLatestAuxiliaryConfig(file)) return ['P1', 'later', currentLatestAuxiliaryConfigOwner(file), 'snapshot', `current-latest auxiliary command or scan config source is registered under P1 snapshot evidence; item: ${id}`];
  if (category === 'config' && isCurrentLatestExternalConfig(file)) return ['P0', 'blocked', 'external-authority', 'blocker', `explicit current-latest external cloud/server/telemetry/global config blocker; not counted as local runtime parity without product authority credentials or service contract evidence; item: ${id}`];
  if (category === 'config') return ['P0', 'blocked', 'config-loader', 'blocker', `current-latest config surface requires fixture evidence; item: ${id}`];
  if (category === 'eval-runner' && isCurrentLatestEvalRunnerFixture(id, file)) return ['P0', 'native', 'eval-runner', 'fixture', `${currentLatestEvalRunnerFixtureReason(id, file)}; item: ${id}`];
  if (category === 'eval-runner' && isCurrentLatestEvalRunnerSnapshot(file)) return ['P1', 'later', 'eval-runner', 'snapshot', `current-latest optimizer/event/synthesis eval-runner source is registered under P1 snapshot evidence until dedicated parity work proves native behavior; item: ${id}`];
  if (category === 'eval-runner') return ['P0', 'blocked', 'eval-runner', 'blocker', `${currentLatestEvalRunnerBlockerReason(id, file)}; item: ${id}`];
  if (category === 'cache-store' && isCurrentLatestCacheStoreFixture(id, file)) return ['P0', 'native', 'cache-resume-store', 'fixture', `current-latest cache key, database schema, eval deletion lifecycle, and local filesystem storage source is covered by deterministic cache/resume/result-store fixtures; item: ${id}`];
  if (category === 'cache-store' && isCurrentLatestCacheStoreSnapshot(file)) return ['P1', 'later', 'cache-resume-store', 'snapshot', `current-latest database testing/signal helper source is registered under P1 snapshot evidence until dedicated lifecycle parity work proves native behavior; item: ${id}`];
  if (category === 'cache-store') return ['P0', 'blocked', 'cache-resume-store', 'blocker', `${currentLatestCacheStoreBlockerReason(id, file)}; item: ${id}`];
  if (category === 'prompt-processing' && isCurrentLatestPromptProcessingFixture(id, file)) return ['P0', 'native', currentLatestPromptProcessingFixtureOwner(id, file), 'fixture', `${currentLatestPromptProcessingFixtureReason(id, file)}; item: ${id}`];
  if (category === 'prompt-processing' && isCurrentLatestPromptProcessingSnapshot(file)) return ['P1', 'later', 'config-loader', 'snapshot', `current-latest static/external prompt helper source is registered under P1 snapshot evidence until dedicated parity work proves native behavior; item: ${id}`];
  if (category === 'prompt-processing') return ['P0', 'blocked', currentLatestPromptProcessingBlockerOwner(id, file), 'blocker', `${currentLatestPromptProcessingBlockerReason(id, file)}; item: ${id}`];
  if (category === 'script-bridge' && isCurrentLatestPythonScriptBridgeFixture(id, file)) return ['P0', 'native', 'script-bridge', 'fixture', `current-latest Python script bridge source is covered by deterministic authorized Python subprocess fixtures; item: ${id}`];
  if (category === 'script-bridge' && isCurrentLatestRubyScriptBridgeFixture(id, file)) return ['P0', 'native', 'script-bridge', 'fixture', `current-latest Ruby script bridge source is covered by deterministic authorized Ruby subprocess fixtures; item: ${id}`];
  if (category === 'script-bridge') return ['P0', 'blocked', 'script-bridge', 'blocker', `${currentLatestScriptBridgeBlockerReason(id, file)}; item: ${id}`];
  if (category === 'viewer') return ['P1', 'later', 'web-viewer', 'snapshot', `current-latest viewer surface requires data-contract or browser snapshot; item: ${id}`];
  if (category === 'assertion-support') return ['P1', 'later', 'assertion-engine', 'snapshot', `current-latest assertion support surface requires matcher or grading snapshot evidence; item: ${id}`];
  if (category === 'redteam-support') return ['P1', 'later', 'redteam-engine', 'snapshot', `current-latest redteam support surface requires registry or behavior snapshot evidence; item: ${id}`];
  if (category === 'schema') return ['P1', 'later', 'protocol', 'snapshot', `current-latest schema/model/contract surface requires protocol snapshot evidence; item: ${id}`];
  if (category === 'import-export') return ['P1', 'later', 'output-writers', 'snapshot', `current-latest import/export surface requires conversion snapshot evidence; item: ${id}`];
  if (category === 'blob-store') return ['P1', 'later', 'eval-runner', 'snapshot', `current-latest blob and media storage surface requires data-contract snapshot evidence; item: ${id}`];
  if (category === 'runtime-support') return ['P1', 'later', 'runtime', 'snapshot', `current-latest runtime support surface requires deterministic snapshot evidence; item: ${id}`];
  if (category === 'observability') return ['P1', 'later', 'observability', 'snapshot', `current-latest tracing and observability surface requires telemetry snapshot evidence; item: ${id}`];
  if (category === 'node-api') return ['P1', 'later', 'node-api-wrapper', 'snapshot', `current-latest Node API surface requires wrapper contract snapshot; item: ${id}`];
  if (category === 'example') return ['P2', 'later', 'compatibility', 'registration', `current-latest example is registered unless promoted into P0/P1 corpus; item: ${id}`];
  if (category === 'docs') return ['P2', 'later', 'compatibility', 'registration', `current-latest documented workflow is registered until mapped to executable evidence; item: ${id}`];
  if (category === 'integration') return ['P2', 'later', 'compatibility', 'registration', `current-latest external integration is registered until promoted with fixture or authority evidence; item: ${id}`];
  if (category === 'cloud-share') return ['P2', 'unsupported', 'compatibility', 'registration', `current-latest cloud/share surface remains local-first unsupported unless legal brand and service authority are provided; item: ${id}`];
  return ['P0', 'blocked', 'compatibility', 'blocker', `current-latest source row is unclassified and must be mapped before any perfect-refactor claim; item: ${id}`];
}

function evidenceReference(category, id, evidenceKind) {
  if (evidenceKind === 'fixture') return `fixture:${id}`;
  if (evidenceKind === 'snapshot' || evidenceKind === 'protocol') return `snapshot:${id}`;
  if (evidenceKind === 'registration') return `registration:${id}`;
  if (evidenceKind === 'blocker') return `blocker:${id}`;
  if (['provider', 'config', 'eval-runner', 'cache-store', 'prompt-processing', 'script-bridge', 'unclassified'].includes(category)) return `blocker:${id}`;
  if (['example', 'docs', 'integration', 'cloud-share'].includes(category)) return `registration:${id}`;
  return `snapshot:${id}`;
}

function sourceReference(file, fragment) {
  return `promptfoo@current-latest:${head}:${file}${fragment ? `#${fragment}` : ''}`;
}

function walk(current, out) {
  for (const entry of fs.readdirSync(current, { withFileTypes: true })) {
    if (entry.isDirectory()) {
      if (['.git', 'node_modules', 'target', '.turbo', '.next', 'dist', 'build'].includes(entry.name)) {
        continue;
      }
      walk(path.join(current, entry.name), out);
      continue;
    }
    if (!entry.isFile()) continue;
    out.push(normalize(path.relative(sourceRoot, path.join(current, entry.name))));
  }
}

function flagsFrom(content) {
  return Array.from(new Set(Array.from(content.matchAll(/--([A-Za-z0-9][A-Za-z0-9_-]*)/g)).map((match) => match[1]))).sort();
}

function addRow(rows, category, file, name, fragment) {
  const slugName = slug(name);
  const id = stableId(category, slugName);
  if (rows.has(id)) return;
  const [level, implementationStatus, owner, evidenceKind, reason] = metadata(category, id, file);
  rows.set(id, {
    stable_id: id,
    category,
    name: slugName,
    source_reference: sourceReference(file, fragment),
    source_file: file,
    level,
    implementation_status: implementationStatus,
    verification_owner: owner,
    evidence_kind: evidenceKind,
    evidence_reference: evidenceReference(category, id, evidenceKind),
    blocker_reason: reason,
  });
}

const sourceFiles = [];
walk(sourceRoot, sourceFiles);
sourceFiles.sort();

const rows = new Map();
const sourceCounts = {
  command_related_files: 0,
  provider_files: 0,
  assertion_files: 0,
  redteam_plugin_files: 0,
  redteam_strategy_files: 0,
  viewer_app_files: 0,
  example_files: 0,
  output_files: 0,
  config_files: 0,
};

for (const file of sourceFiles) {
  for (const category of categoriesFor(file)) {
    if (category === 'command') sourceCounts.command_related_files += 1;
    if (category === 'provider') sourceCounts.provider_files += 1;
    if (category === 'assertion') sourceCounts.assertion_files += 1;
    if (category === 'redteam-plugin') sourceCounts.redteam_plugin_files += 1;
    if (category === 'redteam-strategy') sourceCounts.redteam_strategy_files += 1;
    if (category === 'viewer') sourceCounts.viewer_app_files += 1;
    if (category === 'example') sourceCounts.example_files += 1;
    if (category === 'output') sourceCounts.output_files += 1;
    if (category === 'config') sourceCounts.config_files += 1;
    addRow(rows, category, file, withoutExtension(file), null);
  }
  let content = '';
  try {
    content = fs.readFileSync(path.join(sourceRoot, file), 'utf8');
  } catch (_) {
    content = '';
  }
  for (const flag of flagsFrom(content)) {
    addRow(rows, 'flag', file, flag, `--${flag}`);
  }
}

const inventoryRows = Array.from(rows.values()).sort((left, right) => left.stable_id.localeCompare(right.stable_id));
const unclassifiedRows = inventoryRows.filter((row) => row.category === 'unclassified').map((row) => row.stable_id);
const rowsMissingEvidence = inventoryRows
  .filter((row) => !row.evidence_kind || !row.evidence_reference)
  .map((row) => row.stable_id);
const categories = Array.from(new Set(inventoryRows.map((row) => row.category))).sort();
const inventoryStatus = unclassifiedRows.length || rowsMissingEvidence.length ? 'ready-with-blockers' : 'ready';
const extractionTimestamp = `unix:${Math.floor(Date.now() / 1000)}`;

const inventory = {
  schema: 'promptfoo-rs.current-latest-source-inventory.v1',
  status: inventoryStatus,
  target: lock,
  extraction_mode: 'current-latest-locked-source-tree',
  source_root: sourceRootLabel,
  extraction_timestamp: extractionTimestamp,
  source_counts: sourceCounts,
  rows: inventoryRows,
  categories,
  unclassified_rows: unclassifiedRows,
  rows_missing_evidence: rowsMissingEvidence,
  perfect_refactor_claim_allowed: false,
};

const matrixRows = inventoryRows.map((row) => ({
  item_id: row.stable_id,
  category: row.category,
  source_reference: row.source_reference,
  level: row.level,
  implementation_status: row.implementation_status,
  verification_owner: row.verification_owner,
  evidence_kind: row.evidence_kind,
  evidence_reference: row.evidence_reference,
  blocker_reason: row.blocker_reason,
}));
const matrix = {
  schema: 'promptfoo-rs.current-latest-matrix.v1',
  status: unclassifiedRows.length || rowsMissingEvidence.length ? 'ready-with-blockers' : 'ready',
  target_ref: head,
  rows: matrixRows,
  unclassified_rows: unclassifiedRows,
  rows_missing_evidence: rowsMissingEvidence,
  perfect_refactor_claim_allowed:
    unclassifiedRows.length === 0 &&
    rowsMissingEvidence.length === 0 &&
    matrixRows.every((row) => row.implementation_status === 'native' && row.evidence_kind !== 'blocker' && !row.blocker_reason),
};

fs.writeFileSync(inventoryPath, `${JSON.stringify(inventory, null, 2)}\n`);
fs.writeFileSync(matrixPath, `${JSON.stringify(matrix, null, 2)}\n`);

if (process.env.CURRENT_LATEST_WRITE_TRACKED === '1') {
  fs.mkdirSync('compatibility/inventory', { recursive: true });
  fs.mkdirSync('compatibility/matrix', { recursive: true });
  fs.writeFileSync('compatibility/inventory/current-latest-source-inventory.json', `${JSON.stringify(inventory, null, 2)}\n`);
  fs.writeFileSync('compatibility/matrix/current-latest-matrix.json', `${JSON.stringify(matrix, null, 2)}\n`);
}
NODE

node -e "JSON.parse(require('fs').readFileSync(process.argv[1], 'utf8')); JSON.parse(require('fs').readFileSync(process.argv[2], 'utf8'))" "$OUT" "$MATRIX_OUT"
