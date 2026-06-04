import { existsSync, readFileSync } from 'node:fs';
import { dirname, join } from 'node:path';
import { fileURLToPath } from 'node:url';

const index = readFileSync(new URL('../src/index.ts', import.meta.url), 'utf8');
const packageJson = JSON.parse(readFileSync(new URL('../package.json', import.meta.url), 'utf8'));
const packageRoot = dirname(fileURLToPath(new URL('../package.json', import.meta.url)));

if (index.includes('run_eval(')) {
  throw new Error('npm wrapper must not duplicate Rust eval business logic');
}
for (const expected of ['evaluate', 'createPromptfooClient', 'callRustCore']) {
  if (!index.includes(expected)) {
    throw new Error(`npm wrapper source missing ${expected}`);
  }
}

const expectedBins = {
  promptfoo: './bin/promptfoo.mjs',
  'promptfoo-rs': './bin/promptfoo-rs.mjs',
  pf: './bin/pf.mjs',
};

for (const [binName, relativePath] of Object.entries(expectedBins)) {
  if (packageJson.bin?.[binName] !== relativePath) {
    throw new Error(`TEST-45.2.1 package.json bin.${binName} should be ${relativePath}`);
  }
  const shimPath = join(packageRoot, relativePath);
  if (!existsSync(shimPath)) {
    throw new Error(`TEST-45.2.2 bin shim missing for ${binName}: ${relativePath}`);
  }
  const shim = readFileSync(shimPath, 'utf8');
  for (const forbidden of ['run_eval(', 'providers:', 'OPENAI_API_KEY', 'npm publish']) {
    if (shim.includes(forbidden)) {
      throw new Error(`TEST-45.2.4 bin shim ${binName} contains forbidden business logic or credential surface: ${forbidden}`);
    }
  }
}
