use std::sync::Arc;
use arrow_array::{Array, Int64Array, RecordBatch};
use arrow_schema::{DataType, Field, Schema};
use datafusion::prelude::*;
use deltalake::DeltaTable;
use deltalake::delta_datafusion::{DeltaCdfTableProvider, SessionFallbackPolicy};
use deltalake::delta_datafusion::engine::AsObjectStoreUrl;
use deltalake::kernel::Transaction;
use deltalake::kernel::transaction::CommitProperties;
use deltalake::protocol::SaveMode;
use futures::TryStreamExt;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;
fn batch(ids: &[i64], values: &[i64]) -> RecordBatch {
    RecordBatch::try_new(Arc::new(Schema::new(vec![
        Field::new("id", DataType::Int64, false),
        Field::new("value", DataType::Int64, false),
    ])), vec![Arc::new(Int64Array::from(ids.to_vec())), Arc::new(Int64Array::from(values.to_vec()))]).unwrap()
}
async fn rows(table: &DeltaTable) -> Result<Vec<(i64, i64)>> {
    let (_, mut stream) = table.scan_table().await?;
    let mut out = Vec::new();
    while let Some(b) = stream.try_next().await? {
        let ids = b.column(0).as_any().downcast_ref::<Int64Array>().unwrap();
        let values = b.column(1).as_any().downcast_ref::<Int64Array>().unwrap();
        out.extend((0..b.num_rows()).map(|i| (ids.value(i), values.value(i))));
    }
    out.sort();
    Ok(out)
}
async fn query_count(ctx: &SessionContext) -> Result<i64> {
    let b = ctx.sql("select count(*) as n from dashboard").await?.collect().await?;
    Ok(b[0].column(0).as_any().downcast_ref::<Int64Array>().unwrap().value(0))
}

#[tokio::test]
async fn e01b_refresh_and_replace_provider_in_reused_context() -> Result<()> {
    let mut table = DeltaTable::new_in_memory().write([batch(&[1], &[10])]).await?;
    let ctx = SessionContext::new();
    table.update_datafusion_session(&ctx.state())?;
    ctx.register_table("dashboard", table.table_provider().await?)?;
    assert_eq!(query_count(&ctx).await?, 1);
    let old_query = ctx.sql("select count(*) from dashboard").await?;
    let written = table.clone().write([batch(&[2], &[20])]).with_save_mode(SaveMode::Append).await?;
    assert_eq!(written.version(), Some(1));
    assert_eq!(query_count(&ctx).await?, 1, "provider remains pinned before replacement");
    table.update_state().await?;
    assert_eq!(table.version(), Some(1));
    ctx.deregister_table("dashboard")?;
    ctx.register_table("dashboard", table.table_provider().await?)?;
    assert_eq!(query_count(&ctx).await?, 2);
    let old = old_query.collect().await?;
    assert_eq!(old[0].column(0).as_any().downcast_ref::<Int64Array>().unwrap().value(0), 1);
    Ok(())
}

#[tokio::test]
async fn e03a_projected_incremental_stream() -> Result<()> {
    let table = DeltaTable::new_in_memory().write([batch(&[1, 2, 3], &[10, 20, 30])]).await?;
    let ctx = SessionContext::new_with_config(SessionConfig::new().with_batch_size(1));
    table.update_datafusion_session(&ctx.state())?;
    let (loaded, mut stream) = table.scan_table().with_columns(["id"])
        .with_session_state(Arc::new(ctx.state())).await?;
    assert_eq!(loaded.version(), table.version());
    assert_eq!(stream.schema().fields().len(), 1);
    assert_eq!(stream.schema().field(0).name(), "id");
    let mut ids = Vec::new();
    let mut batches = 0;
    while let Some(b) = stream.try_next().await? {
        batches += 1;
        assert_eq!(b.num_columns(), 1);
        let a = b.column(0).as_any().downcast_ref::<Int64Array>().unwrap();
        ids.extend(a.values().iter().copied());
    }
    ids.sort();
    assert_eq!(ids, vec![1, 2, 3]);
    assert!(batches >= 2, "the consumer actually processes multiple batches");
    assert!(table.scan_table().with_columns(["missing"]).await.is_err());
    Ok(())
}

#[tokio::test]
async fn e05b_logical_plan_uses_caller_udf_and_object_store() -> Result<()> {
    use datafusion::logical_expr::{create_udf, Volatility};
    let source = DeltaTable::new_in_memory().write([batch(&[1, 2], &[10, 20])]).await?;
    let ctx = SessionContext::new_with_config(SessionConfig::new().with_batch_size(1));
    source.update_datafusion_session(&ctx.state())?;
    ctx.register_table("source", source.table_provider().await?)?;
    let udf = create_udf("keep_value", vec![DataType::Int64], DataType::Int64,
        Volatility::Volatile, Arc::new(|args| Ok(args[0].clone())));
    ctx.register_udf(udf);
    let frame = ctx.sql("select id, keep_value(value) + 7 as value from source where id = 2").await?;
    let plan = frame.logical_plan().clone();
    let state = Arc::new(ctx.state());
    let before_runtime = state.runtime_env().clone();
    let destination = DeltaTable::new_in_memory().write([batch(&[0], &[0])]).await?;
    let incompatible = destination.clone().write(Vec::<RecordBatch>::new())
        .with_input_plan(plan.clone()).with_save_mode(SaveMode::Append)
        .with_session_state(state.clone())
        .with_session_fallback_policy(SessionFallbackPolicy::RequireSessionState).await;
    let message = incompatible.expect_err("plain DataFusion planner lacks Delta logical extensions").to_string();
    assert!(message.contains("MetricObserver"), "unexpected failure: {message}");
    let state = Arc::new(datafusion::execution::session_state::SessionStateBuilder::new_from_existing((*state).clone())
        .with_query_planner(deltalake::delta_datafusion::planner::DeltaPlanner::new()).build());
    let result = destination.write(Vec::<RecordBatch>::new())
        .with_input_plan(plan).with_save_mode(SaveMode::Append)
        .with_session_state(state.clone())
        .with_session_fallback_policy(SessionFallbackPolicy::RequireSessionState).await?;
    assert_eq!(rows(&result).await?, vec![(0, 0), (2, 27)]);
    assert!(Arc::ptr_eq(&before_runtime, state.runtime_env()));
    assert_eq!(state.config().batch_size(), 1);
    Ok(())
}

#[tokio::test]
async fn e06b_predicate_overwrite_preserves_other_rows_and_rejects_invalid_input() -> Result<()> {
    let table = DeltaTable::new_in_memory().write([batch(&[1, 2, 3], &[10, 20, 30])]).await?;
    let good = table.write([batch(&[2, 4], &[200, 400])]).with_save_mode(SaveMode::Overwrite)
        .with_replace_where("id >= 2").await?;
    assert_eq!(rows(&good).await?, vec![(1, 10), (2, 200), (4, 400)]);
    let version = good.version();
    let failure = good.clone().write([batch(&[0, 5], &[0, 500])])
        .with_save_mode(SaveMode::Overwrite).with_replace_where("id >= 2").await;
    assert!(failure.is_err(), "out-of-predicate replacement rows must be rejected");
    let mut reloaded = good.clone();
    reloaded.update_state().await?;
    assert_eq!(reloaded.version(), version, "validation failure publishes no new version");
    assert_eq!(rows(&reloaded).await?, vec![(1, 10), (2, 200), (4, 400)]);
    Ok(())
}

async fn serialized_append_once(mut table: DeltaTable, marker: i64) -> Result<DeltaTable> {
    table.update_state().await?;
    let seen = table.snapshot()?.transaction_version(table.log_store().as_ref(), "baseline-app").await?;
    if seen.is_some_and(|value| value >= marker) { return Ok(table); }
    Ok(table.write([batch(&[1], &[10])]).with_save_mode(SaveMode::Append)
        .with_commit_properties(CommitProperties::default().with_application_transaction(Transaction::new("baseline-app", marker)))
        .await?)
}
#[tokio::test]
async fn e09b_serialized_replay_guard_and_unguarded_marker_replay() -> Result<()> {
    let table = DeltaTable::new_in_memory().write([batch(&[0], &[0])]).await?;
    let committed = serialized_append_once(table, 5).await?;
    assert_eq!(committed.snapshot()?.transaction_version(committed.log_store().as_ref(), "baseline-app").await?, Some(5));
    let version = committed.version();
    let replayed = serialized_append_once(committed, 5).await?;
    assert_eq!(replayed.version(), version);
    assert_eq!(rows(&replayed).await?, vec![(0, 0), (1, 10)]);
    let unguarded = replayed.write([batch(&[1], &[10])]).with_save_mode(SaveMode::Append)
        .with_commit_properties(CommitProperties::default().with_application_transaction(Transaction::new("baseline-app", 5))).await?;
    assert_eq!(unguarded.version(), version.map(|v| v + 1));
    assert_eq!(rows(&unguarded).await?, vec![(0, 0), (1, 10), (1, 10)], "a marker alone is not sequential replay suppression");
    Ok(())
}

#[tokio::test]
async fn e11b_cdf_explicit_window_has_insert_update_images_and_delete() -> Result<()> {
    let table = DeltaTable::new_in_memory().write([batch(&[1], &[10])])
        .with_configuration([("delta.enableChangeDataFeed", Some("true"))]).await?;
    let table = table.write([batch(&[2], &[20])]).with_save_mode(SaveMode::Append).await?;
    let (table, _) = table.update().with_predicate("id = 1").with_update("value", "value + 1").await?;
    let (table, _) = table.delete().with_predicate("id = 2").await?;
    assert_eq!(table.version(), Some(3));
    let ctx = SessionContext::new();
    table.update_datafusion_session(&ctx.state())?;
    let builder = table.clone().scan_cdf().with_starting_version(1).with_ending_version(3);
    ctx.register_table("changes", Arc::new(DeltaCdfTableProvider::try_new(builder)?))?;
    let batches = ctx.sql("select id, value, _change_type, _commit_version from changes").await?.collect().await?;
    let mut actual = Vec::new();
    for b in batches {
        let ids = b.column(0).as_any().downcast_ref::<Int64Array>().unwrap();
        let values = b.column(1).as_any().downcast_ref::<Int64Array>().unwrap();
        for i in 0..b.num_rows() {
            actual.push((ids.value(i), values.value(i), deltalake::arrow::util::display::array_value_to_string(b.column(2).as_ref(), i)?, deltalake::arrow::util::display::array_value_to_string(b.column(3).as_ref(), i)?.parse::<i64>()?));
        }
    }
    actual.sort();
    let mut expected = vec![(2, 20, "insert".to_string(), 1), (1, 10, "update_preimage".to_string(), 2), (1, 11, "update_postimage".to_string(), 2), (2, 20, "delete".to_string(), 3)];
    expected.sort();
    assert_eq!(actual, expected);
    assert_eq!(rows(&table).await?, vec![(1, 11)]);
    Ok(())
}

#[tokio::test]
async fn e14b_restore_publishes_new_version_and_preserves_previous_history() -> Result<()> {
    let original = DeltaTable::new_in_memory().write([batch(&[1], &[10])]).await?;
    assert_eq!(original.version(), Some(0));
    let changed = original.clone().write([batch(&[2], &[20])]).with_save_mode(SaveMode::Overwrite).await?;
    assert_eq!(changed.version(), Some(1));
    let (restored, metrics) = changed.clone().restore().with_version_to_restore(0).await?;
    assert_eq!(restored.version(), Some(2));
    assert_eq!(rows(&restored).await?, vec![(1, 10)]);
    assert_eq!(rows(&changed).await?, vec![(2, 20)]);
    assert_eq!(metrics.num_restored_file, 1);
    assert_eq!(metrics.num_removed_file, 1);
    let history: Vec<_> = restored.history(None).await?.collect();
    assert_eq!(history.len(), 3);
    assert_eq!(history[0].operation.as_deref(), Some("RESTORE"));
    let mut time_travel = restored.clone();
    time_travel.load_version(1).await?;
    assert_eq!(rows(&time_travel).await?, vec![(2, 20)]);
    Ok(())
}

#[tokio::test]
async fn e16b_existing_wrong_store_requires_explicit_replacement() -> Result<()> {
    let table = DeltaTable::new_in_memory().write([batch(&[1], &[10])]).await?;
    let ctx = SessionContext::new();
    let runtime = ctx.runtime_env();
    let root = table.log_store().root_url().as_object_store_url();
    let wrong = DeltaTable::new_in_memory().log_store().root_object_store(None);
    runtime.register_object_store(root.as_ref(), wrong.clone());
    table.update_datafusion_session(&ctx.state())?;
    assert!(Arc::ptr_eq(&runtime.object_store(&root)?, &wrong), "ensure must leave an existing mapping alone");
    ctx.register_table("dashboard", table.table_provider().await?)?;
    let bad = ctx.sql("select id, value from dashboard").await?.collect().await;
    assert!(bad.is_err(), "stale mapping cannot fetch this table's files");
    let correct = table.log_store().root_object_store(None);
    runtime.register_object_store(root.as_ref(), correct.clone());
    assert!(Arc::ptr_eq(&runtime.object_store(&root)?, &correct));
    let fetched = ctx.sql("select id, value from dashboard").await?.collect().await?;
    assert_eq!(fetched.iter().map(RecordBatch::num_rows).sum::<usize>(), 1);
    assert_eq!(fetched[0].column(1).as_any().downcast_ref::<Int64Array>().unwrap().value(0), 10);
    Ok(())
}
