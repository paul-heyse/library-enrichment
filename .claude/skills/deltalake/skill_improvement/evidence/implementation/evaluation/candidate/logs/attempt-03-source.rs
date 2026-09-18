use std::sync::Arc;

use arrow_array::{Array, Int64Array, RecordBatch, StringArray};
use arrow_schema::{DataType, Field, Schema};
use datafusion::logical_expr::{ColumnarValue, Volatility, create_udf};
use datafusion::prelude::SessionContext;
use deltalake::delta_datafusion::{DeltaCdfTableProvider, create_session};
use deltalake::kernel::{Transaction, transaction::CommitProperties};
use deltalake::protocol::SaveMode;
use deltalake::{DeltaTable, DeltaTableBuilder};
use futures::TryStreamExt;

type TestResult = Result<(), Box<dyn std::error::Error>>;

fn batch(ids: &[i64]) -> RecordBatch {
    RecordBatch::try_new(
        Arc::new(Schema::new(vec![Field::new("id", DataType::Int64, true)])),
        vec![Arc::new(Int64Array::from(ids.to_vec()))],
    ).unwrap()
}

fn empty_table() -> DeltaTable {
    DeltaTableBuilder::from_url(
        url::Url::parse(&format!("memory:///candidate_{}", uuid::Uuid::new_v4())).unwrap(),
    ).unwrap().build().unwrap()
}

fn ids(batches: &[RecordBatch]) -> Vec<i64> {
    let mut rows = Vec::new();
    for b in batches {
        let a = b.column_by_name("id").unwrap().as_any().downcast_ref::<Int64Array>().unwrap();
        rows.extend(a.iter().map(|v| v.unwrap()));
    }
    rows.sort_unstable();
    rows
}

async fn table_ids(table: &DeltaTable) -> Vec<i64> {
    let ctx = SessionContext::new();
    let frame = ctx.read_table(table.table_provider().await.unwrap()).unwrap();
    ids(&frame.collect().await.unwrap())
}

#[tokio::test]
async fn e01b_refresh_and_replace_provider_in_reused_context() -> TestResult {
    let table = empty_table().write([batch(&[1])]).await?;
    let mut polling_handle = table.clone();
    let ctx = SessionContext::new();
    ctx.register_table("dashboard", table.table_provider().await?)?;
    assert_eq!(ids(&ctx.table("dashboard").await?.collect().await?), vec![1]);
    let written = table.write([batch(&[2])]).await?;
    assert_eq!(written.version(), Some(1));
    assert_eq!(ids(&ctx.table("dashboard").await?.collect().await?), vec![1]);
    polling_handle.update_incremental(None).await?;
    assert_eq!(polling_handle.version(), Some(1));
    ctx.deregister_table("dashboard")?;
    ctx.register_table("dashboard", polling_handle.table_provider().await?)?;
    assert_eq!(ids(&ctx.table("dashboard").await?.collect().await?), vec![1, 2]);
    Ok(())
}

#[tokio::test]
async fn e03a_partitioned_name_projection_streams_only_id() -> TestResult {
    // Partition-first input makes a positional projection error observable.
    let input = RecordBatch::try_new(
        Arc::new(Schema::new(vec![Field::new("label", DataType::Utf8, false), Field::new("id", DataType::Int64, true)])),
        vec![Arc::new(StringArray::from(vec!["a", "b", "a"])), Arc::new(Int64Array::from(vec![11, 22, 33]))],
    )?;
    let table = empty_table().write([input]).with_partition_columns(["label"]).await?;
    let ctx = SessionContext::new();
    let mut stream = ctx.read_table(table.table_provider().await?)?.select_columns(&["id"])?.execute_stream().await?;
    assert_eq!(stream.schema().fields().len(), 1);
    assert_eq!(stream.schema().field(0).name(), "id");
    assert_eq!(stream.schema().field(0).data_type(), &DataType::Int64);
    let mut observed = Vec::new();
    while let Some(b) = stream.try_next().await? {
        assert_eq!(b.num_columns(), 1);
        observed.extend(ids(&[b]));
    }
    observed.sort_unstable();
    assert_eq!(observed, vec![11, 22, 33]);
    Ok(())
}

#[tokio::test]
async fn e05b_logical_plan_write_preserves_caller_udf_and_runtime() -> TestResult {
    let ctx = create_session().into_inner();
    ctx.register_udf(create_udf(
        "candidate_bump", vec![DataType::Int64], DataType::Int64, Volatility::Immutable,
        Arc::new(|args| {
            let arrays = ColumnarValue::values_to_arrays(args)?;
            let input = arrays[0].as_any().downcast_ref::<Int64Array>().unwrap();
            let output = Int64Array::from(input.iter().map(|x| x.map(|n| n + 100)).collect::<Vec<_>>());
            Ok(ColumnarValue::Array(Arc::new(output)))
        }),
    ));
    ctx.register_batch("incoming", batch(&[3, 4]))?;
    let frame = ctx.sql("select candidate_bump(id) as id from incoming where id >= 4").await?;
    let state = ctx.state();
    assert!(Arc::ptr_eq(state.runtime_env(), &ctx.runtime_env()));
    let existing = empty_table().write([batch(&[1])]).await?;
    let table = existing.write(Vec::<RecordBatch>::new())
        .with_input_plan(frame.into_optimized_plan()?)
        .with_session_state(Arc::new(state)).await?;
    assert_eq!(table_ids(&table).await, vec![1, 104]);
    Ok(())
}

#[tokio::test]
async fn e06b_predicate_replacement_rejects_outside_rows_and_preserves_rest() -> TestResult {
    let table = empty_table().write([batch(&[1, 2, 3])]).await?;
    let mut observer = table.clone();
    let invalid = table.clone().write([batch(&[0, 20])]).with_save_mode(SaveMode::Overwrite)
        .with_replace_where("id >= 2").await;
    assert!(invalid.is_err());
    observer.update_incremental(None).await?;
    assert_eq!(observer.version(), Some(0));
    assert_eq!(table_ids(&observer).await, vec![1, 2, 3]);
    let replaced = table.write([batch(&[20])]).with_save_mode(SaveMode::Overwrite)
        .with_replace_where("id >= 2").await?;
    assert_eq!(replaced.version(), Some(1));
    assert_eq!(table_ids(&replaced).await, vec![1, 20]);
    Ok(())
}

fn marker() -> CommitProperties {
    CommitProperties::default().with_application_transaction(Transaction::new("candidate_ingestion", 7))
}

#[tokio::test]
async fn e09b_reconcile_marker_before_retry_without_claiming_atomic_dedup() -> TestResult {
    let table = empty_table().write([batch(&[1])]).await?;
    let mut recovering = table.clone();
    let committed = table.write([batch(&[70])]).with_commit_properties(marker()).await?;
    // Simulate losing the append result; recovery uses its older independent handle.
    recovering.update_incremental(None).await?;
    let observed_marker = recovering.snapshot()?.transaction_version(recovering.log_store().as_ref(), "candidate_ingestion").await?;
    assert_eq!(observed_marker, Some(7));
    assert_eq!(table_ids(&recovering).await, vec![1, 70]);
    let should_retry = observed_marker != Some(7);
    assert!(!should_retry);
    assert_eq!(recovering.version(), Some(1));
    // A deliberately naive sequential retry demonstrates why the marker alone is insufficient.
    let replayed = committed.write([batch(&[70])]).with_commit_properties(marker()).await?;
    assert_eq!(table_ids(&replayed).await, vec![1, 70, 70]);
    assert_eq!(replayed.version(), Some(2));
    Ok(())
}

#[tokio::test]
async fn e11b_cdf_preserves_insert_update_delete_and_explicit_versions() -> TestResult {
    let table = empty_table().write([batch(&[1, 2])])
        .with_configuration([("delta.enableChangeDataFeed", Some("true"))]).await?;
    let table = table.write([batch(&[3])]).await?;
    let (table, _) = table.update().with_predicate("id = 1").with_update("id", "10").await?;
    let (table, _) = table.delete().with_predicate("id = 2").await?;
    assert_eq!(table.version(), Some(3));
    let cdf = DeltaCdfTableProvider::try_new(table.scan_cdf().with_starting_version(1).with_ending_version(3))?;
    let ctx = SessionContext::new();
    let batches = ctx.read_table(Arc::new(cdf))?.collect().await?;
    let mut changes = Vec::new();
    for b in &batches {
        let values = b.column_by_name("id").unwrap().as_any().downcast_ref::<Int64Array>().unwrap();
        let kinds = b.column_by_name("_change_type").unwrap();
        let versions = b.column_by_name("_commit_version").unwrap().as_any().downcast_ref::<Int64Array>().unwrap();
        assert!(b.column_by_name("_commit_timestamp").is_some());
        for i in 0..b.num_rows() {
            let kind = datafusion::common::ScalarValue::try_from_array(kinds, i)?.to_string();
            changes.push((versions.value(i), kind, values.value(i)));
        }
    }
    changes.sort();
    assert_eq!(changes, vec![(1,"insert".into(),3),(2,"update_postimage".into(),10),(2,"update_preimage".into(),1),(3,"delete".into(),2)]);
    Ok(())
}

#[tokio::test]
async fn e14b_restore_publishes_new_head_preserving_intermediate_history() -> TestResult {
    let table = empty_table().write([batch(&[1])]).await?;
    let table = table.write([batch(&[2])]).await?;
    let mut history_reader = table.clone();
    let (restored, _) = table.restore().with_version_to_restore(0).await?;
    assert_eq!(restored.version(), Some(2));
    assert_eq!(table_ids(&restored).await, vec![1]);
    history_reader.load_version(1).await?;
    assert_eq!(table_ids(&history_reader).await, vec![1, 2]);
    assert_eq!(restored.history(None).await?.count(), 3);
    Ok(())
}

#[tokio::test]
async fn e16b_stale_store_mapping_requires_explicit_runtime_replacement() -> TestResult {
    let table = empty_table().write([batch(&[8])]).await?;
    let wrong = empty_table();
    let ctx = SessionContext::new();
    let log = table.log_store();
    ctx.runtime_env().register_object_store(log.root_url(), wrong.log_store().root_object_store(None));
    ctx.register_table("wrong_mapping", table.table_provider().await?)?;
    assert!(ctx.table("wrong_mapping").await?.collect().await.is_err());
    ctx.runtime_env().register_object_store(log.root_url(), log.root_object_store(None));
    assert_eq!(ids(&ctx.table("wrong_mapping").await?.collect().await?), vec![8]);
    Ok(())
}
