import { assertionTypes, filterFailed, type ResultRecord } from './results';

export function ResultsTable({ records }: { records: ResultRecord[] }) {
  const failedRecords = filterFailed(records);

  return (
    <table>
      <thead>
        <tr>
          <th>Case</th>
          <th>Provider</th>
          <th>Status</th>
          <th>Assertions</th>
        </tr>
      </thead>
      <tbody>
        {failedRecords.map((record) => (
          <tr key={`${record.eval_id}:${record.case_id}:${record.provider_id}`}>
            <td>{record.case_id}</td>
            <td>{record.provider_id}</td>
            <td>{record.status}</td>
            <td>{assertionTypes(record).join(', ')}</td>
          </tr>
        ))}
      </tbody>
    </table>
  );
}

export default ResultsTable;
