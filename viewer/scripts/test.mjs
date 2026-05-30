import { readFileSync } from 'node:fs';

const results = readFileSync(new URL('../src/results.ts', import.meta.url), 'utf8');

for (const expected of ['ResultRecord', 'loadResults', 'filterFailed', 'assertionTypes']) {
  if (!results.includes(expected)) {
    throw new Error(`viewer result contract missing ${expected}`);
  }
}
