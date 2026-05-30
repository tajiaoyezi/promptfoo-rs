import { mkdirSync, writeFileSync } from 'node:fs';

const out = new URL('../dist/', import.meta.url);
mkdirSync(out, { recursive: true });

writeFileSync(
  new URL('index.js', out),
  `export async function callRustCore(request, transport) {
  return transport(request);
}

export async function evaluate(config, options = {}, clientOptions) {
  const response = await callRustCore({
    jsonrpc: '2.0',
    id: 'evaluate',
    method: 'evaluate',
    params: { config, options },
  }, clientOptions.transport);
  return response.result;
}

export function createPromptfooClient(options) {
  return {
    evaluate(config, evalOptions = {}) {
      return evaluate(config, evalOptions, options);
    },
  };
}
`,
);

writeFileSync(
  new URL('index.d.ts', out),
  `export type EvalConfig = Record<string, unknown>;
export type EvalOptions = Record<string, unknown>;
export interface ClientOptions {
  transport: (request: unknown) => Promise<{ result: unknown }>;
}
export declare function evaluate(config: EvalConfig, options: EvalOptions | undefined, clientOptions: ClientOptions): Promise<unknown>;
export declare function createPromptfooClient(options: ClientOptions): { evaluate(config: EvalConfig, options?: EvalOptions): Promise<unknown> };
`,
);
