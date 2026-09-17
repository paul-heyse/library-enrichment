//! Targeted caching probe for the 2026-09-17 DataFusion/Delta caching design review.
//! Characterizes current library and service behavior on small local fixtures. It is
//! evidence for a review, not a product acceptance gate; its source is retained under
//! `docs/design_review/reviews/evidence/datafusion-deltalake-caching-2026-09-17/`.
use std::{
    sync::Arc,
    time::{Duration, Instant},
};

use arrow::{
    array::Int64Array,
    datatypes::{DataType, Field, Schema},
    record_batch::RecordBatch,
};
use datafusion::{
    common::TableReference,
    error::{DataFusionError, Result},
    functions_aggregate::expr_fn::sum,
    prelude::col,
};
use datafusion_proto::logical_plan::LogicalExtensionCodec;
use deltalake::{DeltaTableBuilder, delta_datafusion::DeltaLogicalCodec};
use enrichment_store::{
    native_delta::{DeltaStore, StorageContract},
    runtime::{QueryLimits, QueryRuntime},
};

fn ms(d: Duration) -> f64 {
    (d.as_secs_f64() * 10_000.0).round() / 10.0
}
fn external(e: impl std::error::Error + Send + Sync + 'static) -> DataFusionError {
    DataFusionError::External(Box::new(e))
}
fn row(schema: &Arc<Schema>, id: i64) -> Result<RecordBatch> {
    Ok(RecordBatch::try_new(
        schema.clone(),
        vec![Arc::new(Int64Array::from(vec![id]))],
    )?)
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn cache_probe() -> Result<()> {
    let root = tempfile::tempdir()?;
    let runtime = QueryRuntime::new(&root.path().join("spill"), QueryLimits::default())?;
    let store = DeltaStore::new(&root.path().join("tables"), runtime.clone())?;
    let schema = Arc::new(Schema::new(vec![Field::new("id", DataType::Int64, false)]));
    let contract = StorageContract::new(schema.clone())?;
    let name = "probe";
    let commits: i64 = 60;

    // ---- Fixture: one file per commit, no checkpoint (default delta.checkpointInterval=100)
    let mut table = store.create(name, &contract, false).await?;
    let started = Instant::now();
    for i in 0..commits {
        table = store
            .append(
                table,
                &contract,
                store.session().read_batch(row(&schema, i)?)?,
                vec![],
            )
            .await?;
    }
    println!(
        "PROBE fixture commits={commits} files={} version={:?} append_total_ms={}",
        table.get_file_uris().map_err(external)?.count(),
        table.version(),
        ms(started.elapsed())
    );

    // ---- P-A1: full reload through DeltaStore::load, the path every consumer uses today
    let mut full = Vec::new();
    for _ in 0..5 {
        let t = Instant::now();
        let _ = store.load(name, None).await?;
        full.push(ms(t.elapsed()));
    }
    println!("PROBE A1 full_load_ms={full:?}");

    // ---- P-A2: incremental refresh of a retained DeltaTable after one foreign commit each
    let mut retained = store.load(name, None).await?;
    let mut incremental = Vec::new();
    for i in 0..5 {
        let writer = store.load(name, None).await?;
        store
            .append(
                writer,
                &contract,
                store.session().read_batch(row(&schema, 1000 + i)?)?,
                vec![],
            )
            .await?;
        let t = Instant::now();
        retained.update_incremental(None).await.map_err(external)?;
        incremental.push(ms(t.elapsed()));
    }
    println!(
        "PROBE A2 incremental_update_ms={incremental:?} retained_version={:?}",
        retained.version()
    );

    // ---- P-A3: lazy open without file materialization (protocol/metadata only)
    let location = url::Url::from_directory_path(root.path().join("tables").join(name))
        .map_err(|()| DataFusionError::Plan("url".into()))?;
    let object_store = runtime
        .session()
        .runtime_env()
        .object_store(datafusion::execution::object_store::ObjectStoreUrl::local_filesystem())?;
    let mut lazy = Vec::new();
    for _ in 0..5 {
        let t = Instant::now();
        let _ = DeltaTableBuilder::from_url(location.clone())
            .map_err(external)?
            .with_storage_backend(object_store.clone(), location.clone())
            .without_files()
            .load()
            .await
            .map_err(external)?;
        lazy.push(ms(t.elapsed()));
    }
    println!("PROBE A3 lazy_without_files_load_ms={lazy:?}");

    // ---- P-A4: full reload after a checkpoint at the current version
    deltalake::protocol::checkpoints::create_checkpoint(&retained, None)
        .await
        .map_err(external)?;
    let mut after = Vec::new();
    for _ in 0..5 {
        let t = Instant::now();
        let _ = store.load(name, None).await?;
        after.push(ms(t.elapsed()));
    }
    println!("PROBE A4 full_load_after_checkpoint_ms={after:?}");

    // ---- P-C: immutable provider descriptor: JSON codec bytes/time versus Arrow IPC
    let table = store.load(name, None).await?;
    let provider = table
        .table_provider()
        .with_session(Arc::new(store.session().state()))
        .await
        .map_err(external)?;
    let mut json = Vec::new();
    let t = Instant::now();
    DeltaLogicalCodec {}.encode_immutable_provider(provider.as_ref(), &mut json)?;
    let encode = ms(t.elapsed());
    let t = Instant::now();
    let decoded = DeltaLogicalCodec {}.try_decode_table_provider(
        &json,
        &TableReference::bare("probe"),
        schema.clone(),
        &store.session().task_ctx(),
    )?;
    let decode = ms(t.elapsed());
    drop(decoded);
    let adds = table
        .snapshot()
        .map_err(external)?
        .add_actions_table(true)
        .map_err(external)?;
    let mut ipc = Vec::new();
    {
        let mut writer =
            arrow::ipc::writer::StreamWriter::try_new(&mut ipc, adds.schema().as_ref())?;
        writer.write(&adds)?;
        writer.finish()?;
    }
    let files = table.get_file_uris().map_err(external)?.count();
    println!(
        "PROBE C descriptor files={files} json_bytes={} json_bytes_per_file={} encode_ms={encode} decode_ms={decode} add_actions_ipc_bytes={}",
        json.len(),
        json.len() / files.max(1),
        ipc.len()
    );

    // ---- P-D: shared FileMetadataCache reuse across repeated scans of one Delta table
    let cache = runtime
        .session()
        .runtime_env()
        .cache_manager
        .get_file_metadata_cache();
    // The raw next provider bound to the shared session state: the same DeltaScanNext
    // and CachedParquetFileReaderFactory path that the service's captured provider uses.
    let session = store.session();
    let provider = table
        .table_provider()
        .with_session(Arc::new(session.state()))
        .await
        .map_err(external)?;
    let mut observations = Vec::new();
    for _ in 0..3 {
        let frame = session
            .read_table(provider.clone())?
            .aggregate(vec![], vec![sum(col("id")).alias("total")])?;
        let t = Instant::now();
        runtime.execute(frame).await?;
        let hits: usize = cache.list_entries().values().map(|e| e.hits).sum();
        observations.push((ms(t.elapsed()), cache.len(), hits));
    }
    println!(
        "PROBE D metadata_cache (scan_ms, entries, total_hits)={observations:?} limit_bytes={}",
        cache.cache_limit()
    );

    // ---- P-E: not measured here. The service provider/open paths require a full service
    // schema (their invariant queries refuse this one-column fixture); the registry reload
    // per provider() call is established by source reading in the review.

    // ---- P-B: not measured. ControlStore::pin() reaches the same registry lookup
    // (contract_exists -> native_contract::changes -> require_empty) that refuses every
    // warm reopen of an already registered contract in this tree; see the review.

    runtime.close_diagnostics().await?;
    Ok(())
}
