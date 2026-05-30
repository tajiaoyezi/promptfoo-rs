import { createPromptfooClient, evaluate } from '../dist/index.js';

const calls = [];
const transport = async (request) => {
  calls.push(request);
  return {
    jsonrpc: '2.0',
    id: request.id,
    result: {
      schema_version: 'promptfoo-rs.node-api.v1',
      method: request.method,
      status: 'ok',
    },
  };
};

const direct = await evaluate({ prompts: ['hello'] }, {}, { transport });
const client = createPromptfooClient({ transport });
const viaClient = await client.evaluate({ prompts: ['hello'] });

if (direct.schema_version !== 'promptfoo-rs.node-api.v1') {
  throw new Error('direct evaluate smoke failed');
}
if (viaClient.method !== 'evaluate') {
  throw new Error('client evaluate smoke failed');
}
if (calls.some((request) => request.method !== 'evaluate')) {
  throw new Error('npm wrapper called an unexpected Rust core method');
}
