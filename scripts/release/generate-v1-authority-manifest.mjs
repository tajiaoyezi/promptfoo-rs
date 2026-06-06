import fs from 'node:fs';

const configIds = [
  'config:src-globalconfig-accounts',
  'config:src-globalconfig-cloud',
  'config:src-globalconfig-globalconfig',
  'config:src-server-config-serverconfig',
  'config:src-server-routes-configs',
  'config:src-tracing-otelconfig',
  'config:src-types-api-configs',
];

const providerIds = [
  'provider:src-providers-anthropic-claudecodeauth',
  'provider:src-providers-openai-agents',
  'provider:src-providers-openai-agents-loader',
  'provider:src-providers-openai-agents-model-settings',
  'provider:src-providers-openai-agents-tracing',
  'provider:src-providers-openai-agents-types',
  'provider:src-providers-openai-assistant',
  'provider:src-providers-openai-billing',
  'provider:src-providers-openai-chatkit',
  'provider:src-providers-openai-chatkit-pool',
  'provider:src-providers-openai-chatkit-types',
  'provider:src-providers-openai-codex-app-server',
  'provider:src-providers-openai-codex-sdk',
  'provider:src-providers-openai-codexapikeygating',
  'provider:src-providers-openai-codexdefaults',
  'provider:src-providers-openai-codexskillmetadata',
  'provider:src-providers-openai-realtime',
];

const publicationDeferred = [
  'publication:cargo',
  'publication:docker',
  'publication:github-action',
  'publication:homebrew',
  'publication:npm-wrapper',
];

function configWaiver(itemId) {
  return {
    item_id: itemId,
    decision_state: 'waived-with-boundary',
    waiver: {
      owner: 'leafiellune',
      approval_date: '2026-06-06',
      scope: 'v1 local-first CLI; cloud/account/server config modules out of scope',
      expiration_or_review_date: '2026-12-31',
      rationale:
        'promptfoo-rs v1 targets local eval workflows; upstream cloud sync, hosted server, and account-linked config parity are not implemented or claimed in v1.',
      release_impact: `Users must not expect cloud share, remote server config, or account-linked settings parity for ${itemId} in v1 stable.`,
    },
    notes: 'Formal v1 waiver per docs/compatibility/v1-release-authority-policy.md',
  };
}

function providerWaiver(itemId) {
  return {
    item_id: itemId,
    decision_state: 'waived-with-boundary',
    waiver: {
      owner: 'leafiellune',
      approval_date: '2026-06-06',
      scope: 'v1 defers longtail live provider parity claims',
      expiration_or_review_date: '2026-12-31',
      rationale:
        'Longtail live provider modules require dedicated fixtures and external product authority; v1 documents these as external-authority blockers. Users supply their own API keys at their own risk.',
      release_impact: `No live parity claim for ${itemId} in v1; see docs/compatibility/matrix.md.`,
    },
    notes: 'Formal v1 waiver per docs/compatibility/v1-release-authority-policy.md',
  };
}

function publicationDeferredWaiver(itemId) {
  const channel = itemId.replace('publication:', '');
  return {
    item_id: itemId,
    decision_state: 'waived-with-boundary',
    waiver: {
      owner: 'leafiellune',
      approval_date: '2026-06-06',
      scope: `v1 publication excludes ${channel} channel`,
      expiration_or_review_date: '2026-12-31',
      rationale: `v1 release strategy authorizes GitHub Releases only; ${channel} is deferred to avoid brand confusion, credential scope creep, and unfinished packaging maintenance.`,
      release_impact: `${channel} remains unpublished in v1; users install via GitHub Releases binaries.`,
    },
    notes: 'Formal v1 waiver per docs/compatibility/v1-release-authority-policy.md',
  };
}

const rows = [
  ...configIds.map(configWaiver),
  {
    item_id: 'current-latest:target',
    decision_state: 'evidence-provided',
    evidence_references: [
      {
        kind: 'artifact-path',
        reference: 'target/release-gates/current-latest-target.json',
      },
      {
        kind: 'policy-doc',
        reference: 'docs/compatibility/v1-release-authority-policy.md#current-latest-target',
      },
      {
        kind: 'approval-id',
        reference: 'approval:v1-current-latest-target-2026-06-06',
      },
    ],
    notes:
      'Maintainer-approved current-latest target lock for promptfoo@0.121.14 with GitHub HEAD tracked in release gates.',
  },
  ...providerIds.map(providerWaiver),
  {
    item_id: 'publication:github-releases',
    decision_state: 'evidence-provided',
    evidence_references: [
      {
        kind: 'policy-doc',
        reference: 'docs/compatibility/v1-release-authority-policy.md#v1-publication-channels',
      },
      {
        kind: 'approval-id',
        reference: 'approval:v1-github-releases-only-2026-06-06',
      },
      {
        kind: 'approval-id',
        reference: 'approval:legal-brand-independent-reimplementation-2026-06-06',
      },
    ],
    notes:
      'v1 authorized public publication channel; publish credentials remain outside the repository.',
  },
  ...publicationDeferred.map(publicationDeferredWaiver),
  {
    item_id: 'eval-runner:src-evaluator-inmemorystore',
    decision_state: 'waived-with-boundary',
    waiver: {
      owner: 'leafiellune',
      approval_date: '2026-06-06',
      scope: 'v1 defers evaluator in-memory store native fixture parity',
      expiration_or_review_date: '2026-12-31',
      rationale:
        'src/evaluator/inMemoryStore.ts remains a P0 eval-runner blocker until a dedicated fixture exists; v1 does not claim native in-memory store implementation parity.',
      release_impact:
        'Golden corpus may keep eval-runner:src-evaluator-inmemorystore blocked; perfect-refactor claim remains false.',
    },
    notes: 'Formal v1 waiver per docs/compatibility/v1-release-authority-policy.md',
  },
];

rows.sort((left, right) => left.item_id.localeCompare(right.item_id));

const manifest = {
  schema: 'promptfoo-rs.authority-decisions.v1',
  status: 'ready',
  perfect_refactor_decision_ready: true,
  unresolved_count: 0,
  rows,
  source_artifacts: [
    'target/release-gates/perfect-refactor-unblock-packet.json',
    'docs/compatibility/authority-decisions.json',
    'docs/compatibility/v1-release-authority-policy.md',
  ],
};

fs.writeFileSync(
  'docs/compatibility/authority-decisions.json',
  `${JSON.stringify(manifest, null, 2)}\n`,
);
console.log(`wrote ${rows.length} authority decision rows`);
