use std::sync::Arc;

use arrow_array::{Array, Int64Array, RecordBatch, StringArray};
use arrow_schema::{DataType, Field, Schema};
use datafusion::common::ScalarValue;
use datafusion::prelude::{SessionContext, col, lit};
use deltalake::delta_datafusion::DeltaCdfTableProvider;
use deltalake::kernel::transaction::CommitProperties;
use deltalake::operations::vacuum::{Clock, VacuumMode};
use deltalake::operations::write::SchemaMode;
use deltalake::protocol::SaveMode;
use deltalake::writer::{DeltaWriter, RecordBatchWriter};
use deltalake::{DeltaTable, DeltaTableBuilder};
use futures::TryStreamExt;
use object_store::ObjectStoreExt;

fn batch(values: &[Option<i64>]) -> RecordBatch {
    RecordBatch::try_new(
        Arc::new(Schema::new(vec![Field::new("id", DataType::Int64, true)])),
        vec![Arc::new(Int64Array::from(values.to_vec()))],
    )
    .unwrap()
}

async fn values(table: &DeltaTable) -> Vec<Option<i64>> {
    let (_, stream) = table.scan_table().await.unwrap();
    let batches: Vec<RecordBatch> = stream.try_collect().await.unwrap();
    let mut result = Vec::new();
    for b in batches {
        let normalized =
            arrow_cast::cast(b.column_by_name("id").unwrap(), &DataType::Int64).unwrap();
        let a = normalized.as_any().downcast_ref::<Int64Array>().unwrap();
        result.extend(a.iter());
    }
    result.sort();
    result
}

#[tokio::test]
async fn save_modes_replace_where_and_validation_before_commit() {
    let table = DeltaTable::new_in_memory()
        .write([batch(&[Some(1), Some(2), Some(3)])])
        .await
        .unwrap();
    assert!(
        table
            .clone()
            .write([batch(&[Some(4)])])
            .with_save_mode(SaveMode::ErrorIfExists)
            .await
            .is_err()
    );
    let ignored = table
        .clone()
        .write([batch(&[Some(4)])])
        .with_save_mode(SaveMode::Ignore)
        .await
        .unwrap();
    assert_eq!(
        values(&ignored).await,
        vec![Some(1), Some(2), Some(3), Some(4)]
    );
    assert_eq!(ignored.version(), Some(1));
    let table = DeltaTable::new_in_memory()
        .write([batch(&[Some(1), Some(2), Some(3)])])
        .await
        .unwrap();
    let mut observer = table.clone();
    assert!(
        table
            .clone()
            .write([batch(&[Some(0)])])
            .with_save_mode(SaveMode::Overwrite)
            .with_replace_where(col("id").gt_eq(lit(2_i64)))
            .await
            .is_err()
    );
    observer.update_incremental(None).await.unwrap();
    assert_eq!(observer.version(), table.version());
    let replaced = table
        .write([batch(&[Some(20)])])
        .with_save_mode(SaveMode::Overwrite)
        .with_replace_where(col("id").gt_eq(lit(2_i64)))
        .await
        .unwrap();
    assert_eq!(values(&replaced).await, vec![Some(1), Some(20)]);
    let overwritten = replaced
        .write([batch(&[Some(9)])])
        .with_save_mode(SaveMode::Overwrite)
        .await
        .unwrap();
    assert_eq!(values(&overwritten).await, vec![Some(9)]);
}

#[tokio::test]
async fn logical_plan_write_and_staged_writer_visibility() {
    let ctx = SessionContext::new();
    let frame = ctx.read_batch(batch(&[Some(4), Some(5)])).unwrap();
    let unconfigured = DeltaTable::new_in_memory()
        .write(Vec::<RecordBatch>::new())
        .with_input_plan(frame.logical_plan().clone())
        .with_session_state(Arc::new(ctx.state()))
        .await;
    assert!(
        unconfigured
            .unwrap_err()
            .to_string()
            .contains("MetricObserver")
    );
    let delta_session = deltalake::delta_datafusion::create_session();
    let table = DeltaTable::new_in_memory()
        .write(Vec::<RecordBatch>::new())
        .with_input_plan(frame.logical_plan().clone())
        .with_session_state(Arc::new(delta_session.state()))
        .await
        .unwrap();
    assert_eq!(values(&table).await, vec![Some(4), Some(5)]);
    let parquet_count = |objects: Vec<object_store::ObjectMeta>| {
        objects
            .iter()
            .filter(|o| o.location.as_ref().ends_with(".parquet"))
            .count()
    };
    let before = parquet_count(table.object_store().list(None).try_collect().await.unwrap());
    let mut writer = RecordBatchWriter::for_table(&table)
        .unwrap()
        .with_target_file_size(1)
        .with_writer_properties(
            deltalake::parquet::file::properties::WriterProperties::builder()
                .set_max_row_group_row_count(Some(1))
                .build(),
        );
    writer.write(batch(&[Some(6)])).await.unwrap();
    // A size roll schedules background upload; allow that task to finish without flushing.
    tokio::time::timeout(std::time::Duration::from_secs(2), async {
        loop {
            let after = parquet_count(table.object_store().list(None).try_collect().await.unwrap());
            if after > before {
                break;
            }
            tokio::task::yield_now().await;
        }
    })
    .await
    .expect("size roll stages a file before explicit flush");
    let mut observer = table.clone();
    observer.update_incremental(None).await.unwrap();
    assert_eq!(values(&observer).await, vec![Some(4), Some(5)]);
    let mut target = table;
    writer.flush_and_commit(&mut target).await.unwrap();
    observer.update_incremental(None).await.unwrap();
    assert_eq!(values(&observer).await, vec![Some(4), Some(5), Some(6)]);
}

#[tokio::test]
async fn schema_merge_and_cast_error_policy() {
    let table = DeltaTable::new_in_memory()
        .write([batch(&[Some(1)])])
        .await
        .unwrap();
    let wider = RecordBatch::try_new(
        Arc::new(Schema::new(vec![
            Field::new("id", DataType::Int64, true),
            Field::new("label", DataType::Utf8, true),
        ])),
        vec![
            Arc::new(Int64Array::from(vec![2])),
            Arc::new(StringArray::from(vec!["two"])),
        ],
    )
    .unwrap();
    assert!(table.clone().write([wider.clone()]).await.is_err());
    let merged = table
        .write([wider])
        .with_schema_mode(SchemaMode::Merge)
        .await
        .unwrap();
    let (_, stream) = merged.scan_table().await.unwrap();
    let batches: Vec<RecordBatch> = stream.try_collect().await.unwrap();
    assert_eq!(batches.iter().map(RecordBatch::num_rows).sum::<usize>(), 2);
    assert_eq!(
        batches
            .iter()
            .map(|b| b.column_by_name("label").unwrap().null_count())
            .sum::<usize>(),
        1
    );
    let table = DeltaTable::new_in_memory()
        .write([batch(&[Some(1)])])
        .await
        .unwrap();
    let strings = RecordBatch::try_new(
        Arc::new(Schema::new(vec![Field::new("id", DataType::Utf8, true)])),
        vec![Arc::new(StringArray::from(vec![
            Some("bad"),
            Some("2"),
            None,
        ]))],
    )
    .unwrap();
    assert!(
        table
            .clone()
            .write([strings.clone()])
            .with_cast_safety(false)
            .await
            .is_err()
    );
    let table = table.write([strings]).with_cast_safety(true).await.unwrap();
    assert_eq!(values(&table).await, vec![None, None, Some(1), Some(2)]);
}

#[tokio::test]
async fn merge_clause_order_null_keys_and_duplicate_updates() {
    let ctx = SessionContext::new();
    let table = DeltaTable::new_in_memory()
        .write([batch(&[None, Some(1), Some(2)])])
        .await
        .unwrap();
    let source = ctx.read_batch(batch(&[None, Some(2), Some(3)])).unwrap();
    let (table, metrics) = table
        .merge(source, col("t.id").eq(col("s.id")))
        .with_source_alias("s")
        .with_target_alias("t")
        .when_matched_update(|u| {
            u.predicate(col("s.id").eq(lit(2_i64)))
                .update("id", lit(20_i64))
        })
        .unwrap()
        .when_matched_update(|u| u.update("id", lit(30_i64)))
        .unwrap()
        .when_not_matched_insert(|i| i.set("id", col("s.id")))
        .unwrap()
        .when_not_matched_by_source_delete(|d| d.predicate(col("t.id").eq(lit(1_i64))))
        .unwrap()
        .await
        .unwrap();
    assert_eq!(values(&table).await, vec![None, None, Some(3), Some(20)]);
    assert_eq!(metrics.num_target_rows_updated, 1);
    assert_eq!(metrics.num_target_rows_inserted, 2);
    assert_eq!(metrics.num_target_rows_deleted, 1);
    let table = DeltaTable::new_in_memory()
        .write([batch(&[Some(1)])])
        .await
        .unwrap();
    let mut observer = table.clone();
    let duplicate = ctx.read_batch(batch(&[Some(1), Some(1)])).unwrap();
    let result = table
        .merge(duplicate, col("t.id").eq(col("s.id")))
        .with_source_alias("s")
        .with_target_alias("t")
        .when_matched_update(|u| u.update("id", col("s.id") + lit(1_i64)))
        .unwrap()
        .await;
    assert!(
        result.is_err(),
        "multiple matching source updates must not silently duplicate a target"
    );
    observer.update_incremental(None).await.unwrap();
    assert_eq!(observer.version(), Some(0));
    assert_eq!(values(&observer).await, vec![Some(1)]);
}

#[tokio::test]
async fn dml_noop_and_update_delete_metrics() {
    let table = DeltaTable::new_in_memory()
        .write([batch(&[None, Some(1), Some(2)])])
        .await
        .unwrap();
    let version = table.version();
    let (table, _) = table
        .delete()
        .with_predicate(col("id").eq(lit(99_i64)))
        .await
        .unwrap();
    assert_eq!(table.version(), version);
    let (table, m) = table
        .update()
        .with_predicate(col("id").eq(lit(1_i64)))
        .with_update("id", lit(10_i64))
        .await
        .unwrap();
    assert_eq!(m.num_updated_rows, 1);
    let (table, m) = table
        .delete()
        .with_predicate(col("id").is_null())
        .await
        .unwrap();
    assert_eq!(m.num_deleted_rows, Some(1));
    assert_eq!(values(&table).await, vec![Some(2), Some(10)]);
}

#[tokio::test]
async fn cdf_bounds_images_and_residual_filter() {
    let table = DeltaTable::new_in_memory()
        .write([batch(&[Some(1), Some(2)])])
        .with_configuration([("delta.enableChangeDataFeed", Some("true"))])
        .await
        .unwrap();
    let (table, _) = table
        .update()
        .with_predicate(col("id").eq(lit(1_i64)))
        .with_update("id", lit(10_i64))
        .await
        .unwrap();
    let ctx = SessionContext::new();
    let provider = DeltaCdfTableProvider::try_new(
        table
            .clone()
            .scan_cdf()
            .with_starting_version(1)
            .with_ending_version(1),
    )
    .unwrap();
    ctx.register_table("changes", Arc::new(provider)).unwrap();
    let all = ctx
        .sql("select id, _change_type, _commit_version from changes")
        .await
        .unwrap()
        .collect()
        .await
        .unwrap();
    assert_eq!(
        all[0]
            .schema()
            .field_with_name("_commit_version")
            .unwrap()
            .data_type(),
        &DataType::UInt64
    );
    let mut observed = Vec::new();
    for b in &all {
        for r in 0..b.num_rows() {
            observed.push((
                ScalarValue::try_from_array(b.column(0), r)
                    .unwrap()
                    .to_string(),
                ScalarValue::try_from_array(b.column(1), r)
                    .unwrap()
                    .to_string(),
                ScalarValue::try_from_array(b.column(2), r)
                    .unwrap()
                    .to_string(),
            ));
        }
    }
    observed.sort();
    assert_eq!(
        observed,
        vec![
            ("1".into(), "update_preimage".into(), "1".into()),
            ("10".into(), "update_postimage".into(), "1".into())
        ]
    );
    let filtered = ctx
        .sql("select * from changes where id > 5")
        .await
        .unwrap()
        .collect()
        .await
        .unwrap();
    assert_eq!(filtered.iter().map(RecordBatch::num_rows).sum::<usize>(), 1);
    assert!(
        table
            .clone()
            .scan_cdf()
            .with_starting_version(0)
            .with_ending_version(99)
            .build(&ctx.state(), None)
            .await
            .is_ok()
    );
    assert!(
        table
            .clone()
            .scan_cdf()
            .with_starting_version(0)
            .with_ending_version(99)
            .with_allow_out_of_range()
            .build(&ctx.state(), None)
            .await
            .is_ok()
    );
    assert!(
        table
            .clone()
            .scan_cdf()
            .with_starting_version(99)
            .build(&ctx.state(), None)
            .await
            .is_err()
    );
    assert!(
        table
            .clone()
            .scan_cdf()
            .with_starting_version(99)
            .with_allow_out_of_range()
            .build(&ctx.state(), None)
            .await
            .is_ok()
    );
    let timed = table
        .clone()
        .scan_cdf()
        .with_starting_timestamp(chrono::DateTime::UNIX_EPOCH)
        .with_ending_timestamp(chrono::Utc::now() + chrono::Duration::days(1))
        .build(&ctx.state(), None)
        .await
        .unwrap();
    let timed_rows = datafusion::physical_plan::collect(timed, ctx.task_ctx())
        .await
        .unwrap();
    assert_eq!(
        timed_rows.iter().map(RecordBatch::num_rows).sum::<usize>(),
        4
    );
    let ordered = ctx
        .sql("select id, _change_type, _commit_version from changes order by id")
        .await
        .unwrap()
        .collect()
        .await;
    assert!(
        ordered
            .unwrap_err()
            .to_string()
            .contains("UnknownPartitioning(0)")
    );
    let plain = DeltaTable::new_in_memory()
        .write([batch(&[Some(1)])])
        .await
        .unwrap();
    assert!(
        plain
            .scan_cdf()
            .with_starting_version(0)
            .build(&ctx.state(), None)
            .await
            .is_err()
    );
}

#[tokio::test]
async fn optimistic_conflict_and_retry_budget() {
    let table = DeltaTable::new_in_memory()
        .write([batch(&[Some(1)])])
        .await
        .unwrap();
    let stale = table.clone();
    let _committed = table.write([batch(&[Some(2)])]).await.unwrap();
    assert!(
        stale
            .clone()
            .write([batch(&[Some(3)])])
            .with_commit_properties(CommitProperties::default().with_max_retries(0))
            .await
            .is_err()
    );
    let table = stale.write([batch(&[Some(3)])]).await.unwrap();
    assert_eq!(values(&table).await, vec![Some(1), Some(2), Some(3)]);
    let stale = table.clone();
    let (table, _) = table
        .update()
        .with_update("id", col("id") + lit(10_i64))
        .await
        .unwrap();
    assert!(
        stale
            .update()
            .with_update("id", col("id") + lit(20_i64))
            .await
            .is_err()
    );
    assert_eq!(values(&table).await, vec![Some(11), Some(12), Some(13)]);
}

#[derive(Debug)]
struct FutureClock;
impl Clock for FutureClock {
    fn current_timestamp_millis(&self) -> i64 {
        4_000_000_000_000
    }
}

#[tokio::test]
async fn competing_marker_writers_conflict_and_cdf_enablement_is_historical() {
    use deltalake::kernel::Transaction;
    let props = || {
        CommitProperties::default().with_application_transaction(Transaction::new("competing", 8))
    };
    let table = DeltaTable::new_in_memory()
        .write([batch(&[Some(1)])])
        .await
        .unwrap();
    let stale = table.clone();
    let table = table
        .write([batch(&[Some(2)])])
        .with_commit_properties(props())
        .await
        .unwrap();
    let conflict = stale
        .write([batch(&[Some(3)])])
        .with_commit_properties(props())
        .await;
    assert!(conflict.is_err());
    assert_eq!(values(&table).await, vec![Some(1), Some(2)]);
    let table = table
        .set_tbl_properties()
        .with_properties(std::collections::HashMap::from([(
            "delta.enableChangeDataFeed".into(),
            "true".into(),
        )]))
        .await
        .unwrap();
    let start = table.version().unwrap();
    let table = table.write([batch(&[Some(4)])]).await.unwrap();
    let ctx = SessionContext::new();
    assert!(
        table
            .clone()
            .scan_cdf()
            .with_starting_version(0)
            .build(&ctx.state(), None)
            .await
            .is_err()
    );
    let provider =
        DeltaCdfTableProvider::try_new(table.scan_cdf().with_starting_version(start)).unwrap();
    let changes = ctx
        .read_table(Arc::new(provider))
        .unwrap()
        .collect()
        .await
        .unwrap();
    assert_eq!(changes.iter().map(RecordBatch::num_rows).sum::<usize>(), 1);
}

#[tokio::test]
async fn vacuum_preview_keep_versions_and_history_loss() {
    let table = DeltaTable::new_in_memory()
        .write([batch(&[Some(1)])])
        .await
        .unwrap();
    let (table, _) = table.delete().await.unwrap();
    assert!(
        table
            .clone()
            .vacuum()
            .with_retention_period(chrono::Duration::seconds(0))
            .await
            .is_err()
    );
    let (kept, m) = table
        .clone()
        .vacuum()
        .with_keep_versions(&[0])
        .with_clock(Arc::new(FutureClock))
        .with_dry_run(true)
        .await
        .unwrap();
    assert!(m.files_deleted.is_empty());
    let (_, m) = table
        .clone()
        .vacuum()
        .with_clock(Arc::new(FutureClock))
        .with_dry_run(true)
        .await
        .unwrap();
    assert!(!m.files_deleted.is_empty());
    let mut old = kept;
    old.load_version(0).await.unwrap();
    assert_eq!(values(&old).await, vec![Some(1)]);
    let (table, m) = table
        .vacuum()
        .with_clock(Arc::new(FutureClock))
        .with_dry_run(false)
        .await
        .unwrap();
    assert!(!m.files_deleted.is_empty());
    let mut old = table;
    old.load_version(0).await.unwrap();
    let result = old.scan_table().await;
    match result {
        Err(_) => (),
        Ok((_, stream)) => assert!(stream.try_collect::<Vec<_>>().await.is_err()),
    }
}

#[tokio::test]
async fn vacuum_full_finds_orphan_lite_does_not() {
    let table = DeltaTable::new_in_memory()
        .write([batch(&[Some(1)])])
        .await
        .unwrap();
    let orphan = object_store::path::Path::from("orphan.parquet");
    table
        .object_store()
        .put(&orphan, object_store::PutPayload::from_static(b"fixture"))
        .await
        .unwrap();
    let (_, lite) = table
        .clone()
        .vacuum()
        .with_clock(Arc::new(FutureClock))
        .with_mode(VacuumMode::Lite)
        .with_dry_run(true)
        .await
        .unwrap();
    let (_, full) = table
        .vacuum()
        .with_clock(Arc::new(FutureClock))
        .with_mode(VacuumMode::Full)
        .with_dry_run(true)
        .await
        .unwrap();
    assert!(!lite.files_deleted.iter().any(|p| p.contains("orphan")));
    assert!(full.files_deleted.iter().any(|p| p.contains("orphan")));
}

#[tokio::test]
async fn restore_creates_new_version_and_checkpoint_preserves_rows() {
    let table = DeltaTable::new_in_memory()
        .write([batch(&[Some(1)])])
        .await
        .unwrap();
    let table = table.write([batch(&[Some(2)])]).await.unwrap();
    let (table, _) = table.restore().with_version_to_restore(0).await.unwrap();
    assert_eq!(table.version(), Some(2));
    assert_eq!(values(&table).await, vec![Some(1)]);
    deltalake::protocol::checkpoints::create_checkpoint(&table, None)
        .await
        .unwrap();
    assert_eq!(values(&table).await, vec![Some(1)]);
    let mut historic = table;
    historic.load_version(1).await.unwrap();
    assert_eq!(values(&historic).await, vec![Some(1), Some(2)]);
}

#[tokio::test]
async fn optimize_compact_and_zorder_preserve_rows() {
    let mut table = DeltaTable::new_in_memory()
        .write([batch(&[Some(1)])])
        .await
        .unwrap();
    for v in [2, 3] {
        table = table.write([batch(&[Some(v)])]).await.unwrap();
    }
    let (table, m) = table.optimize().await.unwrap();
    assert!(m.num_files_removed >= 2);
    assert_eq!(values(&table).await, vec![Some(1), Some(2), Some(3)]);
    let version = table.version();
    let (table, _) = table.optimize().await.unwrap();
    assert_eq!(table.version(), version);
    let (table, _) = table
        .optimize()
        .with_type(deltalake::operations::optimize::OptimizeType::ZOrder(vec![
            "id".into(),
        ]))
        .await
        .unwrap();
    assert_eq!(values(&table).await, vec![Some(1), Some(2), Some(3)]);
}

#[tokio::test]
async fn unloaded_handle_explicit_version_and_partition_reconstruction() {
    let dir = tempfile::tempdir().unwrap();
    let url = url::Url::from_directory_path(dir.path()).unwrap();
    fn partition_batch(ids: Vec<i64>) -> RecordBatch {
        let labels = vec!["payload"; ids.len()];
        RecordBatch::try_new(
            Arc::new(Schema::new(vec![
                Field::new("id", DataType::Int64, true),
                Field::new("label", DataType::Utf8, true),
            ])),
            vec![
                Arc::new(Int64Array::from(ids)),
                Arc::new(StringArray::from(labels)),
            ],
        )
        .unwrap()
    }
    let table = DeltaTable::try_from_url(url.clone())
        .await
        .unwrap()
        .write([partition_batch(vec![1, 2])])
        .with_partition_columns(["id"])
        .await
        .unwrap();
    let table = table.write([partition_batch(vec![3])]).await.unwrap();
    // Pin-specific positional projection regression: LoadBuilder computes indices against
    // a different field order from its provider when a partition field is first.
    let projected = table
        .scan_table()
        .with_columns(["id"])
        .await
        .unwrap()
        .1
        .try_collect::<Vec<_>>()
        .await
        .unwrap();
    assert_eq!(projected[0].schema().field(0).name(), "label");
    assert_eq!(values(&table).await, vec![Some(1), Some(2), Some(3)]);
    let unloaded = DeltaTableBuilder::from_url(url).unwrap().build().unwrap();
    assert_eq!(unloaded.version(), None);
    let ctx = SessionContext::new();
    let provider = unloaded
        .table_provider()
        .with_table_version(0)
        .await
        .unwrap();
    let results = ctx
        .read_table(provider)
        .unwrap()
        .select_columns(&["id"])
        .unwrap()
        .collect()
        .await
        .unwrap();
    assert_eq!(results.iter().map(RecordBatch::num_rows).sum::<usize>(), 2);
    assert_eq!(results[0].schema().field(0).name(), "id");
    let mut ids = Vec::new();
    for b in results {
        let a = arrow_cast::cast(b.column(0), &DataType::Int64).unwrap();
        ids.extend(a.as_any().downcast_ref::<Int64Array>().unwrap().iter());
    }
    ids.sort();
    assert_eq!(ids, vec![Some(1), Some(2)]);
}
