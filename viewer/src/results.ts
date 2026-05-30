export type ResultStatus = 'passed' | 'failed' | 'error' | 'skipped';

export interface AssertionResultRecord {
  assertion_type: string;
  status: ResultStatus;
  message?: string | null;
}

export interface ResultRecord {
  eval_id: string;
  case_id: string;
  provider_id: string;
  status: ResultStatus;
  result?: unknown;
  assertion_results: AssertionResultRecord[];
  latency_ms: number;
  metadata: Record<string, unknown>;
  error?: string | null;
}

export interface ResultSource {
  records?: ResultRecord[];
  jsonl?: string;
}

export async function loadResults(source: ResultSource): Promise<ResultRecord[]> {
  if (source.records) {
    return source.records;
  }

  if (!source.jsonl) {
    return [];
  }

  return source.jsonl
    .split(/\r?\n/)
    .map((line) => line.trim())
    .filter(Boolean)
    .map((line) => JSON.parse(line) as ResultRecord);
}

export function filterFailed(records: ResultRecord[]): ResultRecord[] {
  return records.filter((record) => record.status === 'failed' || record.status === 'error');
}

export function assertionTypes(record: ResultRecord): string[] {
  return record.assertion_results.map((assertion) => assertion.assertion_type);
}
