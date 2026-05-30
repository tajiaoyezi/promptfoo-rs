import { readFileSync } from 'node:fs';

const packageJson = JSON.parse(readFileSync(new URL('../package.json', import.meta.url), 'utf8'));
const index = readFileSync(new URL('../src/index.ts', import.meta.url), 'utf8');
const rpc = readFileSync(new URL('../src/rpc.ts', import.meta.url), 'utf8');

for (const script of ['typecheck', 'test', 'build', 'smoke:node']) {
  if (!packageJson.scripts?.[script]) {
    throw new Error(`npm wrapper missing script ${script}`);
  }
}
if (!index.includes('createPromptfooClient') || !index.includes('callRustCore')) {
  throw new Error('npm wrapper must delegate through callRustCore');
}
if (!rpc.includes('RustCoreTransport')) {
  throw new Error('npm wrapper missing RustCoreTransport contract');
}
