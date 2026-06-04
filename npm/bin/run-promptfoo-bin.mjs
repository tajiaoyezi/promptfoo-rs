import { spawnSync } from 'node:child_process';
import { existsSync } from 'node:fs';
import { delimiter, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';

const packageBinDir = fileURLToPath(new URL('.', import.meta.url));
const workspaceRoot = resolve(packageBinDir, '..', '..');

export function runPromptfooBin(argv, env = process.env, options = {}) {
  const commandName = options.commandName ?? 'promptfoo';
  const executable = resolveRustBinary(commandName, env);
  const result = spawnSync(executable, argv, {
    env,
    stdio: options.stdio ?? 'inherit',
    windowsHide: true,
  });
  if (result.error) {
    throw result.error;
  }
  return result.status ?? 1;
}

export function resolveRustBinary(commandName, env = process.env) {
  const envVar = commandName === 'promptfoo-rs' ? 'PROMPTFOO_RS_BIN' : 'PROMPTFOO_BIN';
  const explicit = env[envVar] ?? env.PROMPTFOO_BIN;
  if (explicit) {
    return explicit;
  }

  for (const candidate of candidateBinaries(commandName)) {
    if (existsSync(candidate)) {
      return candidate;
    }
  }

  throw new Error(
    `Unable to find ${commandName} Rust binary. Build with cargo first or set ${envVar}.`,
  );
}

function candidateBinaries(commandName) {
  const names = process.platform === 'win32' ? [`${commandName}.exe`, commandName] : [commandName];
  const profiles = ['release', 'debug'];
  const candidates = [];
  for (const profile of profiles) {
    for (const name of names) {
      candidates.push(resolve(workspaceRoot, 'target', profile, name));
    }
  }
  for (const directory of (process.env.PATH ?? '').split(delimiter)) {
    if (!directory) {
      continue;
    }
    for (const name of names) {
      candidates.push(resolve(directory, name));
    }
  }
  return candidates;
}
