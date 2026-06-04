#!/usr/bin/env node
import { runPromptfooBin } from './run-promptfoo-bin.mjs';

process.exitCode = runPromptfooBin(process.argv.slice(2), process.env, {
  commandName: 'promptfoo-rs',
});
