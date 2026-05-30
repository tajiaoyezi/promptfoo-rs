import { readFileSync } from 'node:fs';

const html = readFileSync(new URL('../dist/index.html', import.meta.url), 'utf8');
const manifest = JSON.parse(readFileSync(new URL('../dist/manifest.json', import.meta.url), 'utf8'));

if (!html.includes('data-viewer="promptfoo-rs.viewer.v1"')) {
  throw new Error('viewer browser smoke missing viewer marker');
}
if (manifest.name !== '@promptfoo-rs/viewer' || manifest.exportsResultsTable !== true) {
  throw new Error('viewer browser smoke manifest mismatch');
}
