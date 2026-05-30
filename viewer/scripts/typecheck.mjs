import { readFileSync } from 'node:fs';

const packageJson = JSON.parse(readFileSync(new URL('../package.json', import.meta.url), 'utf8'));
const app = readFileSync(new URL('../src/App.tsx', import.meta.url), 'utf8');
const results = readFileSync(new URL('../src/results.ts', import.meta.url), 'utf8');

for (const script of ['typecheck', 'test', 'build', 'smoke:browser']) {
  if (!packageJson.scripts?.[script]) {
    throw new Error(`viewer package missing script ${script}`);
  }
}
if (!app.includes('export function ResultsTable')) {
  throw new Error('viewer App.tsx must export ResultsTable');
}
if (!results.includes('export async function loadResults')) {
  throw new Error('viewer results.ts must export loadResults');
}
