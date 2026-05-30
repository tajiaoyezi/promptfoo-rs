import { callRustCore, type NodeRpcResponse, type RustCoreTransport } from "./rpc";

export type EvalConfig = Record<string, unknown>;
export type EvalOptions = Record<string, unknown>;
export type EvalResult = NodeRpcResponse["result"];

export interface ClientOptions {
  transport: RustCoreTransport;
}

export interface PromptfooClient {
  evaluate(config: EvalConfig, options?: EvalOptions): Promise<EvalResult>;
}

export async function evaluate(
  config: EvalConfig,
  options: EvalOptions = {},
  clientOptions: ClientOptions,
): Promise<EvalResult> {
  const response = await callRustCore(
    {
      jsonrpc: "2.0",
      id: "evaluate",
      method: "evaluate",
      params: { config, options },
    },
    clientOptions.transport,
  );
  return response.result;
}

export function createPromptfooClient(options: ClientOptions): PromptfooClient {
  return {
    evaluate(config: EvalConfig, evalOptions: EvalOptions = {}) {
      return evaluate(config, evalOptions, options);
    },
  };
}
