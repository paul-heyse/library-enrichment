// Syntax-only controls; deliberately not a compilable consumer.
async fn controls(table: DeltaTable, client: AuditClient, batch: RecordBatch) {
    table.write([batch.clone()]).await; // Delta lead, caller policy still unknown.
    client.write(batch.clone()).await; // Unrelated same-name false positive.
    table.write([batch]).with_commit_properties(CommitProperties::default()).await;
    // Above has commit properties but no application marker: the hint excludes it.
}
