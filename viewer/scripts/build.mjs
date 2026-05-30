import { mkdirSync, readFileSync, writeFileSync } from 'node:fs';

const out = new URL('../dist/', import.meta.url);
mkdirSync(out, { recursive: true });

const appSource = readFileSync(new URL('../src/App.tsx', import.meta.url), 'utf8');
const manifest = {
  name: '@promptfoo-rs/viewer',
  schema: 'promptfoo-rs.viewer.package.v1',
  entrypoints: ['src/App.tsx', 'src/results.ts'],
  exportsResultsTable: appSource.includes('ResultsTable'),
};

writeFileSync(
  new URL('index.html', out),
  '<!doctype html><html><head><meta charset="utf-8"><title>promptfoo-rs viewer</title></head><body><main id="root" data-viewer="promptfoo-rs.viewer.v1"></main></body></html>\n',
);
writeFileSync(new URL('manifest.json', out), `${JSON.stringify(manifest, null, 2)}\n`);
