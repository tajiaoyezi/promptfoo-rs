export type JsonRpcId = string | number;

export interface NodeRpcRequest {
  jsonrpc: "2.0";
  id: JsonRpcId;
  method: "evaluate";
  params: Record<string, unknown>;
}

export interface NodeRpcResponse<T = unknown> {
  jsonrpc: "2.0";
  id: JsonRpcId;
  result: T;
}

export interface RustCoreTransport {
  (request: NodeRpcRequest): Promise<NodeRpcResponse>;
}

export async function callRustCore(
  request: NodeRpcRequest,
  transport: RustCoreTransport,
): Promise<NodeRpcResponse> {
  return transport(request);
}
