use datafusion::{
    arrow::{
        array::Int64Array,
        datatypes::{DataType, Field, Schema},
        record_batch::RecordBatch,
    },
    common::{Constraint, Constraints, ScalarValue},
    datasource::MemTable,
    logical_expr::{ColumnarValue, Volatility, create_udf},
    physical_plan::displayable,
    prelude::SessionContext,
};
use deltalake::{DeltaTable, kernel::Transaction, kernel::transaction::CommitProperties};
use std::{
    future::IntoFuture,
    sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    },
};
type R<T> = Result<T, Box<dyn std::error::Error>>;
fn batch(ids: Vec<i64>) -> RecordBatch {
    RecordBatch::try_new(
        Arc::new(Schema::new(vec![Field::new("id", DataType::Int64, false)])),
        vec![Arc::new(Int64Array::from(ids))],
    )
    .unwrap()
}
async fn count(ctx: &SessionContext, sql: &str) -> R<i64> {
    let batches = ctx.sql(sql).await?.collect().await?;
    Ok(batches[0]
        .column(0)
        .as_any()
        .downcast_ref::<Int64Array>()
        .unwrap()
        .value(0))
}
#[tokio::main]
async fn main() -> R<()> {
    let ctx = SessionContext::new();
    let closure=ctx.sql("WITH RECURSIVE edges(a,b) AS (VALUES (1,2),(2,3),(3,1)), walk(node,path,depth) AS (SELECT 1, [1], 0 UNION ALL SELECT e.b, array_append(w.path,e.b), w.depth+1 FROM walk w JOIN edges e ON w.node=e.a WHERE w.depth<64 AND NOT array_has(w.path,e.b)) SELECT node FROM walk ORDER BY node").await?;
    let plan = closure.clone().create_physical_plan().await?;
    assert!(
        displayable(plan.as_ref())
            .indent(true)
            .to_string()
            .contains("RecursiveQuery")
    );
    let rows = closure.collect().await?;
    assert_eq!(rows.iter().map(RecordBatch::num_rows).sum::<usize>(), 3);
    println!(
        "NATIVE_RECURSION: three-node cycle terminates with native array_has path guard; rows=3"
    );
    assert_eq!(
        count(
            &ctx,
            "SELECT count(*) FROM UNNEST([1,2,3]) AS t(x) WHERE x>1"
        )
        .await?,
        2
    );
    println!("NESTED: native UNNEST+filter rows=2");
    let calls = Arc::new(AtomicUsize::new(0));
    let captured = calls.clone();
    ctx.register_udf(create_udf(
        "effect_probe",
        vec![DataType::Int64],
        DataType::Int64,
        Volatility::Volatile,
        Arc::new(move |_| {
            captured.fetch_add(1, Ordering::SeqCst);
            Ok(ColumnarValue::Scalar(ScalarValue::Int64(Some(1))))
        }),
    ));
    ctx.sql("SELECT effect_probe(1) WHERE false")
        .await?
        .collect()
        .await?;
    assert_eq!(calls.load(Ordering::SeqCst), 0);
    let repeat = ctx.sql("SELECT effect_probe(1)").await?;
    repeat.clone().collect().await?;
    repeat.collect().await?;
    assert_eq!(calls.load(Ordering::SeqCst), 2);
    println!("VOLATILITY: dead branch calls=0, executing same frame twice calls=2");
    let initial = batch(vec![1]);
    let table = MemTable::try_new(initial.schema(), vec![vec![initial]])?.with_constraints(
        Constraints::new_unverified(vec![Constraint::PrimaryKey(vec![0])]),
    );
    ctx.register_table("declared_key", Arc::new(table))?;
    ctx.sql("INSERT INTO declared_key VALUES (1)")
        .await?
        .collect()
        .await?;
    assert_eq!(count(&ctx, "SELECT count(*) FROM declared_key").await?, 2);
    println!("DECLARED_KEY: duplicate accepted by MemTable INSERT; rows=2");
    let root = std::path::PathBuf::from(std::env::args().nth(1).expect("fresh scratch table path"));
    std::fs::create_dir_all(&root)?;
    let url = url::Url::from_directory_path(&root).map_err(|_| "absolute scratch path required")?;
    let table = DeltaTable::try_from_url(url.clone())
        .await?
        .write(vec![batch(vec![0])])
        .await?;
    let state = Arc::new(
        datafusion::execution::SessionStateBuilder::from(ctx.state())
            .with_query_planner(deltalake::delta_datafusion::planner::DeltaPlanner::new())
            .build(),
    );
    table.update_datafusion_session(state.as_ref())?;
    let pinned = table.table_provider().with_session(state.clone()).await?;
    ctx.register_table("delta_pinned", pinned.clone())?;
    let table = table
        .add_constraint()
        .with_constraint("positive", "id >= 0")
        .with_session_state(state.clone())
        .await?;
    let plain_error = table
        .clone()
        .write(vec![batch(vec![4])])
        .with_session_state(Arc::new(ctx.state()))
        .await
        .unwrap_err()
        .to_string();
    assert!(plain_error.contains("No installed planner") && plain_error.contains("MetricObserver"));
    println!("DELTA_PLAIN_SESSION: missing MetricObserver planner confirmed on current revision");
    let rejected = table
        .clone()
        .write(vec![batch(vec![-1])])
        .with_session_state(state.clone())
        .await;
    let invalid_error = rejected.unwrap_err().to_string();
    println!("DELTA_WRITE_BUILDER_ERROR: {invalid_error}");
    assert!(!invalid_error.contains("No installed planner"));
    assert!(
        invalid_error.to_lowercase().contains("invariant")
            || invalid_error.to_lowercase().contains("check")
            || invalid_error.contains("id >=")
            || invalid_error.contains("positive")
    );
    println!("DELTA_WRITE_BUILDER_CHECK: negative id rejected by table CHECK");
    let constrained = table.table_provider().with_session(state.clone()).await?;
    ctx.register_table("delta_constrained", constrained.clone())?;
    let sql_write = ctx
        .sql("INSERT INTO delta_constrained VALUES (-1)")
        .await?
        .collect()
        .await;
    assert!(sql_write.is_ok());
    println!(
        "DELTA_PROVIDER_CHECK: SQL INSERT negative id result={}",
        if sql_write.is_ok() {
            "accepted"
        } else {
            "rejected"
        }
    );
    let latest = deltalake::open_table(url.clone()).await?;
    ctx.register_table(
        "delta_latest",
        latest.table_provider().with_session(state.clone()).await?,
    )?;
    println!(
        "DELTA_SNAPSHOT: pinned_rows={} latest_rows={}",
        count(&ctx, "SELECT count(*) FROM delta_pinned").await?,
        count(&ctx, "SELECT count(*) FROM delta_latest").await?
    );
    assert_eq!(count(&ctx, "SELECT count(*) FROM delta_pinned").await?, 1);
    println!(
        "DELTA_PROVIDER_TRUNCATE: {}",
        constrained.truncate(state.as_ref()).await.unwrap_err()
    );
    let a = latest
        .clone()
        .write(vec![batch(vec![11])])
        .with_session_state(state.clone())
        .with_commit_properties(
            CommitProperties::default()
                .with_application_transaction(Transaction::new("claim/job-x", 1)),
        );
    let b = latest
        .clone()
        .write(vec![batch(vec![12])])
        .with_session_state(state.clone())
        .with_commit_properties(
            CommitProperties::default()
                .with_application_transaction(Transaction::new("claim/job-x", 1)),
        );
    let (a, b) = tokio::join!(a.into_future(), b.into_future());
    println!(
        "DELTA_CONCURRENT_DETAILS: a={:?} b={:?}",
        a.as_ref().err(),
        b.as_ref().err()
    );
    assert_ne!(a.is_ok(), b.is_ok());
    assert!(
        a.as_ref()
            .err()
            .or(b.as_ref().err())
            .unwrap()
            .to_string()
            .to_lowercase()
            .contains("concurrent transaction")
    );
    println!("DELTA_CONCURRENT_TXN: same app_id stale snapshots successes=1 failures=1");
    let current = deltalake::open_table(url.clone()).await?;
    let retry = current
        .write(vec![batch(vec![13])])
        .with_session_state(state.clone())
        .with_commit_properties(
            CommitProperties::default()
                .with_application_transaction(Transaction::new("claim/job-x", 1)),
        )
        .await;
    assert!(retry.is_ok());
    println!(
        "DELTA_SEQUENTIAL_TXN_RETRY: repeated app_id/version against fresh snapshot result={}",
        if retry.is_ok() {
            "accepted"
        } else {
            "rejected"
        }
    );
    println!("PROBE_COMPLETE");
    Ok(())
}
