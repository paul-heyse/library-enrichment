//! Isolated DataFusion 55.1 capability probes, not service integration acceptance.
include!("upstream_example.rs");

use datafusion::arrow::{
    array::{Array, Int64Array},
    datatypes::{DataType, Field, Schema},
};
use datafusion::common::ScalarValue;
use datafusion::datasource::MemTable;
use datafusion::execution::{memory_pool::GreedyMemoryPool, runtime_env::RuntimeEnvBuilder};
use datafusion::logical_expr::{Volatility, create_udf};
use datafusion::physical_plan::ExecutionPlanProperties;
use std::sync::atomic::{AtomicUsize, Ordering};

fn calls(counter: &AtomicUsize) -> usize {
    counter.load(Ordering::SeqCst)
}

#[derive(Debug)]
struct DelayedPartition(RecordBatch);
impl datafusion::physical_plan::streaming::PartitionStream for DelayedPartition {
    fn schema(&self) -> &Arc<Schema> {
        self.0.schema_ref()
    }
    fn execute(
        &self,
        _: Arc<datafusion::execution::TaskContext>,
    ) -> datafusion::physical_plan::SendableRecordBatchStream {
        let batch = self.0.clone();
        Box::pin(
            datafusion::physical_plan::stream::RecordBatchStreamAdapter::new(
                batch.schema(),
                futures::stream::once(async move {
                    tokio::time::sleep(std::time::Duration::from_millis(50)).await;
                    Ok(batch)
                }),
            ),
        )
    }
}

fn context(factory: Option<Arc<dyn CacheFactory>>, example_planner: bool) -> SessionContext {
    let mut builder = SessionStateBuilder::new()
        .with_default_features()
        .with_config(SessionConfig::new().with_target_partitions(2))
        .with_cache_factory(factory);
    if example_planner {
        builder = builder.with_query_planner(Arc::new(CacheNodeQueryPlanner::default()));
    }
    SessionContext::new_with_state(builder.build())
}

async fn input(ctx: &SessionContext, delay: bool) -> Result<(DataFrame, Arc<AtomicUsize>)> {
    let counter = Arc::new(AtomicUsize::new(0));
    let observed = counter.clone();
    ctx.register_udf(create_udf(
        "observed",
        vec![DataType::Int64],
        DataType::Int64,
        Volatility::Volatile,
        Arc::new(move |args| {
            observed.fetch_add(1, Ordering::SeqCst);
            if delay {
                std::thread::sleep(std::time::Duration::from_millis(50));
            }
            Ok(args[0].clone())
        }),
    ));
    let schema = Arc::new(Schema::new(vec![
        Field::new("x", DataType::Int64, false).with_metadata(std::collections::HashMap::from([(
            "enrichment.probe".into(),
            "exact-field".into(),
        )])),
    ]));
    let batch = RecordBatch::try_new(
        schema.clone(),
        vec![Arc::new(Int64Array::from(vec![1, 2, 3]))],
    )?;
    if delay {
        ctx.register_table(
            "input",
            Arc::new(datafusion::catalog::streaming::StreamingTable::try_new(
                schema,
                vec![Arc::new(DelayedPartition(batch))],
            )?),
        )?;
    } else {
        ctx.register_table(
            "input",
            Arc::new(MemTable::try_new(schema, vec![vec![batch]])?),
        )?;
    }
    Ok((
        ctx.sql("SELECT observed(x) AS x FROM input").await?,
        counter,
    ))
}

fn values(batches: &[RecordBatch]) -> Vec<i64> {
    batches
        .iter()
        .flat_map(|b| {
            b.column(0)
                .as_any()
                .downcast_ref::<Int64Array>()
                .unwrap()
                .values()
                .to_vec()
        })
        .collect()
}

#[derive(Debug)]
struct PassThrough(Arc<AtomicUsize>);
impl CacheFactory for PassThrough {
    fn create(&self, plan: LogicalPlan, _: &SessionState) -> Result<LogicalPlan> {
        self.0.fetch_add(1, Ordering::SeqCst);
        Ok(plan)
    }
}

#[derive(Debug)]
struct Refuse;
impl CacheFactory for Refuse {
    fn create(&self, _: LogicalPlan, _: &SessionState) -> Result<LogicalPlan> {
        datafusion::common::plan_err!("probe cache eligibility refusal")
    }
}

#[derive(Debug)]
struct Replace;
impl CacheFactory for Replace {
    fn create(&self, _: LogicalPlan, _: &SessionState) -> Result<LogicalPlan> {
        datafusion::logical_expr::LogicalPlanBuilder::empty(true)
            .project([lit(ScalarValue::Utf8(Some("replacement".into()))).alias("changed")])?
            .build()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct EntryKey(u8, Option<datafusion::common::TableReference>);
impl datafusion::execution::cache::CacheKey for EntryKey {
    fn size(&self) -> usize {
        1
    }
    fn table_ref(&self) -> Option<&datafusion::common::TableReference> {
        self.1.as_ref()
    }
}
#[derive(Debug, Clone)]
struct EntryValue(Arc<Vec<u8>>);
impl datafusion::execution::cache::CacheValue for EntryValue {
    fn size(&self) -> usize {
        self.0.len()
    }
}

fn native_cache_probe() -> Result<()> {
    use datafusion::execution::cache::{Cache, SchemaFingerprint, default_cache::DefaultCache};
    let cache = DefaultCache::new(202);
    let table = Some(datafusion::common::TableReference::bare("probe"));
    let first = EntryKey(1, table.clone());
    let second = EntryKey(2, table.clone());
    let third = EntryKey(3, None);
    cache.put(&first, EntryValue(Arc::new(vec![0; 100])));
    cache.put(&second, EntryValue(Arc::new(vec![0; 100])));
    let held = cache.get(&first).unwrap();
    cache.put(&third, EntryValue(Arc::new(vec![0; 100])));
    assert!(!cache.contains_key(&second));
    assert_eq!(cache.list_entries()[&first].hits, 1);
    assert_eq!(cache.memory_used(), 202);
    cache.drop_table_entries(table.as_ref().unwrap())?;
    assert!(!cache.contains_key(&first));
    assert!(cache.contains_key(&third));
    assert_eq!(held.0.len(), 100);
    cache.put(&third, EntryValue(Arc::new(vec![0; 500])));
    assert_eq!(cache.len(), 0);
    assert_eq!(cache.memory_used(), 0);
    println!(
        "P13 native_default_cache lru=true bytes=true hits=true table_invalidation=true oversized_rejected=true held_value_survives_eviction=true"
    );
    let a = Schema::new(vec![Field::new("x", DataType::Int64, false).with_metadata(
        std::collections::HashMap::from([("domain".into(), "A".into())]),
    )]);
    let b = Schema::new(vec![Field::new("x", DataType::Int64, false).with_metadata(
        std::collections::HashMap::from([("domain".into(), "B".into())]),
    )]);
    assert_eq!(
        SchemaFingerprint::from_schema(&a),
        SchemaFingerprint::from_schema(&b)
    );
    println!("P14 statistics_schema_fingerprint_ignores_semantic_metadata=true");
    Ok(())
}

async fn probes() -> Result<()> {
    // P01: default caching is eager and avoids subsequent input execution.
    let ctx = context(None, false);
    let (df, counter) = input(&ctx, false).await?;
    assert_eq!(calls(&counter), 0);
    let cached = df.cache().await?;
    let at_cache = calls(&counter);
    assert!(at_cache > 0);
    assert_eq!(values(&cached.clone().collect().await?), vec![1, 2, 3]);
    assert_eq!(values(&cached.collect().await?), vec![1, 2, 3]);
    assert_eq!(calls(&counter), at_cache);
    println!(
        "P01 default_eager input_calls_at_cache={at_cache} input_calls_after_two_reads={}",
        calls(&counter)
    );

    // P02: factory invocation alone does not supply a cache.
    let factory_calls = Arc::new(AtomicUsize::new(0));
    let ctx = context(Some(Arc::new(PassThrough(factory_calls.clone()))), false);
    let (df, counter) = input(&ctx, false).await?;
    let cached = df.cache().await?;
    assert_eq!(calls(&counter), 0);
    assert_eq!(calls(&factory_calls), 1);
    cached.clone().collect().await?;
    let first = calls(&counter);
    cached.collect().await?;
    assert_eq!(calls(&counter), first * 2);
    println!(
        "P02 factory_is_hook factory_calls={} input_calls_after_two_reads={}",
        calls(&factory_calls),
        calls(&counter)
    );

    // P03: the shipped example materializes during physical planning, before execute.
    let ctx = context(Some(Arc::new(CustomCacheFactory {})), true);
    let (df, counter) = input(&ctx, false).await?;
    let cached = df.cache().await?;
    assert_eq!(calls(&counter), 0);
    let _physical = cached.clone().create_physical_plan().await?;
    let at_plan = calls(&counter);
    assert!(at_plan > 0);
    cached.clone().collect().await?;
    cached
        .clone()
        .filter(col("x").gt(lit(1i64)))?
        .collect()
        .await?;
    assert_eq!(calls(&counter), at_plan);
    println!(
        "P03 upstream_planning_materializes input_calls_after_create_physical_plan={at_plan} after_reads={}",
        calls(&counter)
    );

    // P04: EXPLAIN (without ANALYZE) also exercises that materializing planner.
    let ctx = context(Some(Arc::new(CustomCacheFactory {})), true);
    let (df, counter) = input(&ctx, false).await?;
    let cached = df.cache().await?;
    cached.explain(false, false)?.collect().await?;
    assert!(calls(&counter) > 0);
    println!(
        "P04 upstream_explain_without_analyze input_calls={}",
        calls(&counter)
    );

    // P05: a factory returning an extension needs a composed extension planner.
    let ctx = context(Some(Arc::new(CustomCacheFactory {})), false);
    let (df, counter) = input(&ctx, false).await?;
    let error = df.cache().await?.collect().await.unwrap_err();
    assert_eq!(calls(&counter), 0);
    println!("P05 missing_extension_planner_refuses error={error}");

    // P06: registration is captured in SessionState and retained by state clones/builders.
    let factory_calls = Arc::new(AtomicUsize::new(0));
    let ctx = context(Some(Arc::new(PassThrough(factory_calls.clone()))), false);
    let existing = SessionStateBuilder::new_from_existing(ctx.state()).build();
    assert!(existing.cache_factory().is_some());
    SessionContext::new_with_state(existing)
        .sql("SELECT 1")
        .await?
        .cache()
        .await?;
    assert_eq!(calls(&factory_calls), 1);
    println!(
        "P06 cloned_session_factory_retained calls={}",
        calls(&factory_calls)
    );

    // P07: factory refusal is synchronous before input evaluation.
    let ctx = context(Some(Arc::new(Refuse)), false);
    let (df, counter) = input(&ctx, false).await?;
    let error = df.cache().await.unwrap_err();
    assert_eq!(calls(&counter), 0);
    println!("P07 admission_refusal_before_execution error={error}");

    // P08: CacheFactory does not enforce output-schema equivalence for implementors.
    let ctx = context(Some(Arc::new(Replace)), false);
    let (df, counter) = input(&ctx, false).await?;
    let cached = df.cache().await?;
    assert_eq!(cached.schema().field(0).name(), "changed");
    assert_eq!(calls(&counter), 0);
    println!(
        "P08 factory_can_replace_schema type={}",
        cached.schema().field(0).data_type()
    );

    // P09: default cache retains metadata, but output retention isn't charged to the pool.
    // Source allocation is intentionally external; this tests retained-cache accounting, not RSS.
    let runtime = RuntimeEnvBuilder::new()
        .with_memory_pool(Arc::new(GreedyMemoryPool::new(1024)))
        .build_arc()?;
    let ctx = SessionContext::new_with_config_rt(
        SessionConfig::new().with_target_partitions(1),
        runtime.clone(),
    );
    let schema = Arc::new(Schema::new(vec![
        Field::new("x", DataType::Int64, false).with_metadata(std::collections::HashMap::from([(
            "enrichment.probe".into(),
            "exact-field".into(),
        )])),
    ]));
    let batch = RecordBatch::try_new(
        schema.clone(),
        vec![Arc::new(Int64Array::from_iter_values(0..65536))],
    )?;
    let cached = ctx.read_batch(batch)?.cache().await?;
    assert_eq!(
        cached.schema().field(0).metadata(),
        schema.field(0).metadata()
    );
    let data = cached.clone().collect().await?;
    let retained: usize = data.iter().map(RecordBatch::get_array_memory_size).sum();
    assert!(retained > 1024);
    assert_eq!(runtime.memory_pool.reserved(), 0);
    println!(
        "P09 default_cache_retained_bytes={retained} pool_limit=1024 reserved={} metadata_preserved=true",
        runtime.memory_pool.reserved()
    );

    // P10: concurrent misses in the illustrative example may duplicate work.
    let ctx = context(Some(Arc::new(CustomCacheFactory {})), true);
    let (df, counter) = input(&ctx, true).await?;
    let cached = df.cache().await?;
    let barrier = Arc::new(tokio::sync::Barrier::new(2));
    let spawn = |df: DataFrame| {
        let barrier = barrier.clone();
        tokio::spawn(async move {
            barrier.wait().await;
            df.collect().await
        })
    };
    let (a, b) = tokio::join!(spawn(cached.clone()), spawn(cached));
    assert_eq!(values(&a.unwrap()?), vec![1, 2, 3]);
    assert_eq!(values(&b.unwrap()?), vec![1, 2, 3]);
    assert_eq!(calls(&counter), 2);
    println!(
        "P10 concurrent_example_misses input_calls={}",
        calls(&counter)
    );

    // P11: example reuse is keyed by logical plan, beyond one cache invocation.
    let planner = Arc::new(CacheNodeQueryPlanner::default());
    let state = SessionStateBuilder::new()
        .with_default_features()
        .with_query_planner(planner.clone())
        .with_cache_factory(Some(Arc::new(CustomCacheFactory {})))
        .build();
    let ctx = SessionContext::new_with_state(state);
    let (df, counter) = input(&ctx, false).await?;
    df.cache().await?.collect().await?;
    let after_first = calls(&counter);
    ctx.sql("SELECT observed(x) AS x FROM input")
        .await?
        .cache()
        .await?
        .collect()
        .await?;
    assert_eq!(calls(&counter), after_first);
    let entries = planner.cache_manager.read().unwrap().cache.len();
    assert_eq!(entries, 1);
    println!(
        "P11 independent_cache_calls_share_example_entry input_calls={} entries_after_frames_dropped={entries}",
        calls(&counter)
    );

    // P12: default materialization keeps fields but not original key/order annotations.
    let ctx = context(None, false);
    let schema = Arc::new(Schema::new(vec![Field::new("x", DataType::Int64, false)]));
    let batch = RecordBatch::try_new(
        schema.clone(),
        vec![Arc::new(Int64Array::from(vec![1, 2, 3]))],
    )?;
    let table = MemTable::try_new(schema, vec![vec![batch]])?
        .with_constraints(datafusion::common::Constraints::new_unverified(vec![
            datafusion::common::Constraint::PrimaryKey(vec![0]),
        ]))
        .with_sort_order(vec![vec![col("x").sort(true, false)]]);
    let original = ctx.read_table(Arc::new(table))?;
    let before = original.clone().create_physical_plan().await?;
    let cached = original.cache().await?;
    let after = cached.clone().create_physical_plan().await?;
    let LogicalPlan::TableScan(scan) = cached.logical_plan() else {
        panic!("expected cached table scan")
    };
    let keys = scan
        .source
        .constraints()
        .map(|keys| keys.len())
        .unwrap_or(0);
    assert_eq!(keys, 0);
    assert!(before.output_ordering().is_some());
    assert!(after.output_ordering().is_none());
    println!("P12 default_cache_keys={keys} ordered_before=true ordered_after=false");
    native_cache_probe()?;
    Ok(())
}

fn main() -> Result<()> {
    println!("DataFusion 55.1.0 isolated cache factory characterization");
    tokio::runtime::Builder::new_multi_thread()
        .worker_threads(4)
        .enable_all()
        .build()
        .unwrap()
        .block_on(probes())
}
