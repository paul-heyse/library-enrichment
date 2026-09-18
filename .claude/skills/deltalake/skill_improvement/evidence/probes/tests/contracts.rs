use std::sync::Arc;

use arrow_array::{Int64Array, RecordBatch};
use arrow_schema::{DataType, Field, Schema};
use async_trait::async_trait;
use datafusion::prelude::{ParquetReadOptions, SessionContext};
use deltalake::kernel::Transaction;
use deltalake::kernel::transaction::CommitProperties;
use deltalake::logstore::LogStoreRef;
use deltalake::operations::CustomExecuteHandler;
use deltalake::{DeltaResult, DeltaTable, DeltaTableError};
use futures::TryStreamExt;
use uuid::Uuid;

fn batch(values: &[i64]) -> RecordBatch {
    RecordBatch::try_new(
        Arc::new(Schema::new(vec![Field::new("id", DataType::Int64, false)])),
        vec![Arc::new(Int64Array::from(values.to_vec()))],
    )
    .unwrap()
}

async fn rows(table: &DeltaTable) -> usize {
    let (_, stream) = table.scan_table().with_columns(["id"]).await.unwrap();
    let batches: Vec<RecordBatch> = stream.try_collect().await.unwrap();
    batches.iter().map(RecordBatch::num_rows).sum()
}

#[tokio::test]
async fn provider_snapshot_and_unnameable_load_builder() {
    let table = DeltaTable::new_in_memory()
        .write([batch(&[1, 2])])
        .await
        .unwrap();
    let mut stale = table.clone();
    let ctx = SessionContext::new();
    ctx.register_table("old", table.table_provider().await.unwrap())
        .unwrap();
    let table = table.write([batch(&[3])]).await.unwrap();
    assert_eq!(table.version(), Some(1));
    assert_eq!(rows(&stale).await, 2);
    stale.update_incremental(None).await.unwrap();
    assert_eq!(rows(&stale).await, 3);
    let old = ctx.table("old").await.unwrap().collect().await.unwrap();
    assert_eq!(old.iter().map(RecordBatch::num_rows).sum::<usize>(), 2);
    ctx.register_table("fresh", stale.table_provider().await.unwrap())
        .unwrap();
    let fresh = ctx.table("fresh").await.unwrap().collect().await.unwrap();
    assert_eq!(fresh.iter().map(RecordBatch::num_rows).sum::<usize>(), 3);
    // An already supplied snapshot takes precedence over the builder's version option.
    let selected = table.table_provider().with_table_version(0).await.unwrap();
    let selected_rows = ctx.read_table(selected).unwrap().collect().await.unwrap();
    assert_eq!(
        selected_rows
            .iter()
            .map(RecordBatch::num_rows)
            .sum::<usize>(),
        3
    );
    let mut historical = table.clone();
    historical.load_version(0).await.unwrap();
    assert_eq!(rows(&historical).await, 2);
    println!(
        "snapshot: old provider=2, refreshed table and replacement provider=3; inferred LoadBuilder await returns (DeltaTable, stream)"
    );
    println!(
        "loaded table at v1 plus provider with_table_version(0) still returns 3; explicit load_version(0) returns 2"
    );
}

#[tokio::test]
async fn repeated_transaction_marker_does_not_suppress_sequential_append() {
    let props = || {
        CommitProperties::default()
            .with_application_transaction(Transaction::new("planning-probe", 7))
    };
    let table = DeltaTable::new_in_memory()
        .write([batch(&[1])])
        .with_commit_properties(props())
        .await
        .unwrap();
    let table = table
        .write([batch(&[1])])
        .with_commit_properties(props())
        .await
        .unwrap();
    assert_eq!(rows(&table).await, 2);
    assert_eq!(table.version(), Some(1));
    assert_eq!(
        table
            .snapshot()
            .unwrap()
            .transaction_version(&table.log_store(), "planning-probe")
            .await
            .unwrap(),
        Some(7)
    );
    // A caller can check the persisted marker; this control is not a concurrency protocol.
    let before = table.version();
    let already_written = table
        .snapshot()
        .unwrap()
        .transaction_version(&table.log_store(), "planning-probe")
        .await
        .unwrap()
        == Some(7);
    assert!(already_written);
    assert_eq!(table.version(), before);
    println!(
        "sequential same (app_id, version): two rows, table version 1, transaction marker 7; caller-side marker check can identify replay"
    );
}

#[derive(Debug)]
struct FailAfterCommit;

#[async_trait]
impl CustomExecuteHandler for FailAfterCommit {
    async fn pre_execute(&self, _: &LogStoreRef, _: Uuid) -> DeltaResult<()> {
        Ok(())
    }
    async fn post_execute(&self, _: &LogStoreRef, _: Uuid) -> DeltaResult<()> {
        Ok(())
    }
    async fn before_post_commit_hook(&self, _: &LogStoreRef, _: bool, _: Uuid) -> DeltaResult<()> {
        Err(DeltaTableError::Generic(
            "injected after durable commit".into(),
        ))
    }
    async fn after_post_commit_hook(&self, _: &LogStoreRef, _: bool, _: Uuid) -> DeltaResult<()> {
        Ok(())
    }
}

#[tokio::test]
async fn operation_error_can_follow_a_durable_commit() {
    let table = DeltaTable::new_in_memory()
        .write([batch(&[1])])
        .await
        .unwrap();
    let mut observer = table.clone();
    let result = table
        .write([batch(&[2])])
        .with_custom_execute_handler(Arc::new(FailAfterCommit))
        .await;
    let error = result.unwrap_err().to_string();
    assert!(error.contains("injected after durable commit"));
    observer.update_incremental(None).await.unwrap();
    assert_eq!(observer.version(), Some(1));
    assert_eq!(rows(&observer).await, 2);
    println!(
        "operation returned error but refreshed snapshot sees durable version 1 and both rows"
    );
}

#[tokio::test]
async fn delta_snapshot_excludes_removed_parquet_files() {
    let dir = tempfile::tempdir().unwrap();
    let uri = url::Url::from_directory_path(dir.path()).unwrap();
    let table = DeltaTable::try_from_url(uri)
        .await
        .unwrap()
        .write([batch(&[1, 2])])
        .await
        .unwrap();
    let (table, _) = table.delete().await.unwrap();
    assert_eq!(rows(&table).await, 0);
    let ctx = SessionContext::new();
    let raw = ctx
        .read_parquet(dir.path().to_str().unwrap(), ParquetReadOptions::default())
        .await
        .unwrap()
        .collect()
        .await
        .unwrap();
    assert_eq!(raw.iter().map(RecordBatch::num_rows).sum::<usize>(), 2);
    println!(
        "Delta after delete=0 rows; raw Parquet directory scan=2 obsolete rows (before vacuum)"
    );
}
