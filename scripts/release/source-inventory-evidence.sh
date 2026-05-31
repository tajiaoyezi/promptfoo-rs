#!/usr/bin/env bash
set -euo pipefail

GATE_DIR="target/release-gates"
mkdir -p "$GATE_DIR"
OUT="$GATE_DIR/source-inventory-evidence.json"
tmpdir="$(mktemp -d)"
trap 'rm -rf "$tmpdir"' EXIT

npm view promptfoo@0.121.13 version gitHead dist.tarball dist.integrity --json > "$tmpdir/npm-view.json"
npm pack promptfoo@0.121.13 --json --pack-destination "$tmpdir" > "$tmpdir/npm-pack.json"

tarball="$(node -e "const pack = JSON.parse(require('fs').readFileSync(process.argv[1], 'utf8')); console.log(pack[0].filename)" "$tmpdir/npm-pack.json")"
tar -tf "$tmpdir/$tarball" > "$tmpdir/package-files.txt"

node - "$tmpdir/npm-view.json" "$tmpdir/package-files.txt" "$OUT" <<'NODE'
const fs = require('fs');

const [npmViewPath, packageFilesPath, outputPath] = process.argv.slice(2);
const npmView = JSON.parse(fs.readFileSync(npmViewPath, 'utf8'));
const packageFiles = fs
  .readFileSync(packageFilesPath, 'utf8')
  .trim()
  .split(/\r?\n/)
  .filter(Boolean);
const inventory = JSON.parse(fs.readFileSync('compatibility/inventory/upstream-items.json', 'utf8')).items;

const requiredCategories = ['command', 'provider', 'assertion', 'redteam', 'output', 'config'];
const categoryCounts = {};
for (const item of inventory) {
  const normalized = item.category.startsWith('redteam') ? 'redteam' : item.category;
  categoryCounts[normalized] = (categoryCounts[normalized] || 0) + 1;
}

const packageFileCounts = {
  command: packageFiles.filter((file) => /dist\/src\/commands|dist\/src\/main/.test(file)).length,
  provider: packageFiles.filter((file) => /dist\/src\/providers/.test(file)).length,
  assertion: packageFiles.filter((file) => /dist\/src\/assertions/.test(file)).length,
  redteam: packageFiles.filter((file) => /dist\/src\/redteam/.test(file)).length,
  output: packageFiles.filter((file) => /dist\/src\/.*(output|report|csv|junit|sarif)/i.test(file)).length,
  config: packageFiles.filter((file) => /dist\/src\/.*config/i.test(file)).length,
};

const missingCategories = requiredCategories.filter(
  (category) => !(categoryCounts[category] > 0 || packageFileCounts[category] > 0),
);
const itemsMissingSource = inventory
  .filter((item) => !item.source_reference || item.source_reference.trim() === '')
  .map((item) => item.stable_id);

const evidence = {
  schema: 'promptfoo-rs.source-inventory-evidence.v1',
  baseline: {
    npm: 'promptfoo@0.121.13',
    version: npmView.version,
    gitHead: npmView.gitHead,
    tarball: npmView.dist && npmView.dist.tarball,
    integrity: npmView.dist && npmView.dist.integrity,
  },
  extraction_mode: 'npm-pack-source-file-list-plus-inventory-source-references',
  required_categories: requiredCategories,
  category_counts: categoryCounts,
  package_file_counts: packageFileCounts,
  inventory_item_count: inventory.length,
  package_file_count: packageFiles.length,
  missing_categories: missingCategories,
  items_missing_source: itemsMissingSource,
  status: missingCategories.length === 0 && itemsMissingSource.length === 0 ? 'ready' : 'blocked',
};

fs.writeFileSync(outputPath, JSON.stringify(evidence, null, 2) + '\n');
NODE

node -e "const e = JSON.parse(require('fs').readFileSync(process.argv[1], 'utf8')); if (e.status !== 'ready') { console.error(JSON.stringify(e, null, 2)); process.exit(1); }" "$OUT"
