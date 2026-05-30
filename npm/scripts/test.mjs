import { readFileSync } from 'node:fs';

const index = readFileSync(new URL('../src/index.ts', import.meta.url), 'utf8');

if (index.includes('run_eval(')) {
  throw new Error('npm wrapper must not duplicate Rust eval business logic');
}
for (const expected of ['evaluate', 'createPromptfooClient', 'callRustCore']) {
  if (!index.includes(expected)) {
    throw new Error(`npm wrapper source missing ${expected}`);
  }
}
