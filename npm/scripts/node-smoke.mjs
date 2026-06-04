import { createPromptfooClient, evaluate } from '../dist/index.js';
import { mkdtempSync, rmSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import { spawnSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';

const calls = [];
const transport = async (request) => {
  calls.push(request);
  return {
    jsonrpc: '2.0',
    id: request.id,
    result: {
      schema_version: 'promptfoo-rs.node-api.v1',
      method: request.method,
      status: 'ok',
    },
  };
};

const direct = await evaluate({ prompts: ['hello'] }, {}, { transport });
const client = createPromptfooClient({ transport });
const viaClient = await client.evaluate({ prompts: ['hello'] });

if (direct.schema_version !== 'promptfoo-rs.node-api.v1') {
  throw new Error('direct evaluate smoke failed');
}
if (viaClient.method !== 'evaluate') {
  throw new Error('client evaluate smoke failed');
}
if (calls.some((request) => request.method !== 'evaluate')) {
  throw new Error('npm wrapper called an unexpected Rust core method');
}

const fixtureDir = mkdtempSync(join(tmpdir(), 'promptfoo-rs-npm-bin-'));
try {
  const configPath = join(fixtureDir, 'promptfooconfig.yaml');
  writeFileSync(
    configPath,
    `providers:
  - id: echo
prompts:
  - "Hello {{name}}"
tests:
  - vars:
      name: Ada
`,
  );

  for (const binName of ['promptfoo', 'promptfoo-rs', 'pf']) {
    const help = runBinShim(binName, ['--help']);
    if (help.status !== 0) {
      throw new Error(`TEST-45.2.2 ${binName} --help failed:\n${help.stderr}`);
    }
    if (!help.stdout.includes('Usage:')) {
      throw new Error(`TEST-45.2.2 ${binName} --help did not print CLI usage`);
    }

    const evalRun = runBinShim(binName, ['eval', '-c', configPath]);
    if (evalRun.status !== 0) {
      throw new Error(`TEST-45.2.3 ${binName} eval failed:\n${evalRun.stderr}`);
    }
    const envelope = JSON.parse(evalRun.stdout);
    if (envelope.status !== 'ok' || envelope.summary?.total_cases !== 1) {
      throw new Error(`TEST-45.2.3 ${binName} eval returned unexpected envelope`);
    }
  }

  const packCommand = process.env.npm_execpath
    ? [process.execPath, [process.env.npm_execpath, 'pack', '--dry-run']]
    : ['pnpm', ['pack', '--dry-run']];
  const pack = spawnSync(packCommand[0], packCommand[1], {
    cwd: new URL('..', import.meta.url),
    encoding: 'utf8',
    windowsHide: true,
  });
  if (pack.status !== 0) {
    throw new Error(`TEST-45.2.3 pnpm pack --dry-run failed:\n${pack.error ?? pack.stderr}`);
  }
  if (!pack.stdout.includes('bin/promptfoo.mjs') || !pack.stdout.includes('bin/pf.mjs')) {
    throw new Error('TEST-45.2.3 pnpm pack --dry-run did not include promptfoo/pf bin shims');
  }
} finally {
  rmSync(fixtureDir, { recursive: true, force: true });
}

function runBinShim(binName, args) {
  return spawnSync(process.execPath, [fileURLToPath(new URL(`../bin/${binName}.mjs`, import.meta.url)), ...args], {
    encoding: 'utf8',
    windowsHide: true,
  });
}
