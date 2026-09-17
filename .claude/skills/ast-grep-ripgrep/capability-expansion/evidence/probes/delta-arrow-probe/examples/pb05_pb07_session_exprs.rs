//! PB05 + PB07 + PB12 — how far does "one `SessionState` threaded through everything" actually
//! reach into Delta operations?
//!
//! Plan §2.4 and §5 rest on evidence 05 §3: every delta-rs builder takes `Arc<dyn Session>`, so a
//! UDF registered once is available in a merge predicate, a CHECK constraint and a projection
//! alike. Three questions decide whether that holds.
//!
//! PB05  Do `MergeBuilder` predicates see UDFs registered on the supplied session?
//! PB07  Can a Delta `CheckConstraints` expression reference a UDF? And if it can be ADDED, can
//!       it still be ENFORCED by a later writer whose session does not have that UDF?
//! PB12  Can the provider wrapper derive DataFusion `Constraints` from the table's own Delta
//!       `CheckConstraints`, rather than declaring them twice (evidence 05 §4)?
//!
//! Read from the pinned source first, because it shapes every arm:
//!
//! - `operations/constraints.rs:146` — when no session is supplied the builder silently
//!   constructs its own via `create_session()`. A probe that omits the session therefore tests
//!   delta-rs's default, not the caller's registry.
//! - `operations/constraints.rs:164` — the expression is resolved against the SUPPLIED session,
//!   then serialised to a SQL STRING by `fmt_expr_to_sql` and stored in table metadata under
//!   `delta.constraints.<name>`.
//! - `delta_datafusion/data_validation.rs:319` — at WRITE time that string is re-parsed with
//!   `parse_predicate_expression(schema, sql, session)` against the WRITER's session. So add-time
//!   and enforce-time are two different sessions, and a UDF present at one may be absent at the
//!   other. That asymmetry is the whole point of arm PB07c.
//! - `delta_datafusion/session.rs:125` — `SessionFallbackPolicy::InternalDefaults` is the DEFAULT,
//!   and it means "if the provided session is not a `SessionState`, log a warning and use internal
//!   defaults". A caller's UDF registry can be discarded with nothing but a log line.
//!
//!     cargo run --example pb05_pb07_session_exprs
//!
//! CONTROLS throughout: every arm that uses a UDF is paired with the same operation using a plain
//! column predicate, which must succeed. Without that pairing a failure could mean "the merge is
//! malformed" rather than "the UDF did not resolve".

use std::collections::HashMap;
use std::sync::Arc;

use arrow::array::{Int64Array, RecordBatch, StringArray};
use arrow_schema::{DataType, Field, Schema, SchemaRef};
use datafusion::common::Result as DFResult;
use datafusion::common::cast::as_int64_array;
use datafusion::logical_expr::{
    ColumnarValue, ScalarFunctionArgs, ScalarUDF, ScalarUDFImpl, Signature, Volatility,
};
use datafusion::prelude::SessionContext;
use deltalake::DeltaTable;
use deltalake::DeltaTableBuilder;
use deltalake::delta_datafusion::{SessionFallbackPolicy, create_session};
use deltalake::kernel::StructField;
use deltalake::kernel::engine::arrow_conversion::TryFromArrow;
use deltalake::operations::create::CreateBuilder;

/// `is_valid_offset(n)` returns `n >= 0` — a stand-in for the plan's domain predicates
/// (`mechanism_satisfies`, `anchor_contains`). Immutable, because PB06 established that a
/// `Volatile` UDF is silently skipped by the preimage machinery.
#[derive(Debug, PartialEq, Eq, Hash)]
struct IsValidOffset {
    signature: Signature,
}

impl IsValidOffset {
    fn new() -> Self {
        Self {
            signature: Signature::exact(vec![DataType::Int64], Volatility::Immutable),
        }
    }
}

impl ScalarUDFImpl for IsValidOffset {
    fn name(&self) -> &str {
        "is_valid_offset"
    }
    fn signature(&self) -> &Signature {
        &self.signature
    }
    fn return_type(&self, _arg_types: &[DataType]) -> DFResult<DataType> {
        Ok(DataType::Boolean)
    }
    fn invoke_with_args(&self, args: ScalarFunctionArgs) -> DFResult<ColumnarValue> {
        let arrays = ColumnarValue::values_to_arrays(&args.args)?;
        let input = as_int64_array(&arrays[0])?;
        let out: arrow::array::BooleanArray = input.iter().map(|v| v.map(|v| v >= 0)).collect();
        Ok(ColumnarValue::Array(Arc::new(out)))
    }
}

fn schema() -> SchemaRef {
    Arc::new(Schema::new(vec![
        Field::new("entity_id", DataType::Utf8, false),
        Field::new("off", DataType::Int64, true),
    ]))
}

fn batch(ids: Vec<&str>, offs: Vec<i64>) -> DFResult<RecordBatch> {
    Ok(RecordBatch::try_new(
        schema(),
        vec![
            Arc::new(StringArray::from(ids)),
            Arc::new(Int64Array::from(offs)),
        ],
    )?)
}

/// A fresh table at `name`, with two rows, returned loaded.
async fn make_table(
    root: &std::path::Path,
    name: &str,
) -> Result<(DeltaTable, url::Url), Box<dyn std::error::Error>> {
    let location = root.join(name);
    let _ = std::fs::remove_dir_all(&location);
    std::fs::create_dir_all(&location)?;
    let url = url::Url::from_directory_path(&location).map_err(|_| "bad path")?;
    let log_store = DeltaTableBuilder::from_url(url.clone())?.build_storage()?;

    let arrow_schema = schema();
    let delta_fields: Vec<StructField> = arrow_schema
        .fields()
        .iter()
        .map(|f| {
            let dt = deltalake::kernel::DataType::try_from_arrow(f.data_type())?;
            Ok::<StructField, Box<dyn std::error::Error>>(StructField::new(
                f.name().clone(),
                dt,
                f.is_nullable(),
            ))
        })
        .collect::<Result<_, _>>()?;

    let table = CreateBuilder::new()
        .with_log_store(log_store.clone())
        .with_columns(delta_fields)
        .await?;

    let table = table
        .write(vec![batch(vec!["e1", "e2"], vec![10, 20])?])
        .await?;
    Ok((table, url))
}

/// A session with the UDF registered, and one without. The pair is the control.
///
/// Both are built from `create_session()`, NOT `SessionContext::new()`. The first run of this
/// probe used the latter and every write failed with
///
///     No installed planner was able to convert the custom node to an execution plan:
///     MetricObserver
///
/// including the control arm that was supposed to succeed. That is a broken observation channel,
/// not a result: delta-rs writes plan through its own `MetricObserver` node, so a bare DataFusion
/// session cannot execute a Delta write at all and the UDF never enters the question. Using
/// delta-rs's own session constructor is what makes the UDF the only variable.
fn sessions() -> (
    Arc<datafusion::execution::SessionState>,
    Arc<datafusion::execution::SessionState>,
) {
    let with = create_session().into_inner();
    with.register_udf(ScalarUDF::from(IsValidOffset::new()));
    let without = create_session().into_inner();
    (Arc::new(with.state()), Arc::new(without.state()))
}

fn verdict<T, E: std::fmt::Display>(r: &Result<T, E>) -> String {
    match r {
        Ok(_) => "OK".into(),
        Err(e) => {
            let msg = e.to_string();
            let first = msg.lines().next().unwrap_or("").trim();
            format!("ERR: {}", &first[..first.len().min(140)])
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let root = std::env::var("PROBE_ROOT").unwrap_or_else(|_| "/tmp/delta-arrow-probe".into());
    let root = std::path::PathBuf::from(root);

    println!("PB05/PB07/PB12 — how far does one SessionState reach into Delta operations?\n");

    let (with_udf, without_udf) = sessions();

    // ================= PB05 — merge predicates and the session registry =================
    println!("== PB05 · do MergeBuilder predicates see session UDFs? ==\n");

    // The merge source is an ordinary DataFusion DataFrame, per evidence 05 §2.
    let source_ctx = SessionContext::new();
    source_ctx.register_udf(ScalarUDF::from(IsValidOffset::new()));
    let source = source_ctx.read_batch(batch(vec!["e2", "e3"], vec![21, 30])?)?;

    // (a) plain column predicate + session WITH udf -- the control. Must succeed.
    let (t, _) = make_table(&root, "t_pb05a").await?;
    let r = t
        .merge(source.clone(), "target.entity_id = source.entity_id")
        .with_session_state(with_udf.clone())
        .with_source_alias("source")
        .with_target_alias("target")
        .when_matched_update(|u| u.update("off", "source.off"))?
        .when_not_matched_insert(|i| {
            i.set("entity_id", "source.entity_id")
                .set("off", "source.off")
        })?
        .await;
    println!(
        "  a CONTROL plain predicate, session WITH udf      {}",
        verdict(&r)
    );

    // (b) UDF in the merge predicate + session WITH udf. The question.
    let (t, _) = make_table(&root, "t_pb05b").await?;
    let r = t
        .merge(
            source.clone(),
            "target.entity_id = source.entity_id AND is_valid_offset(target.off)",
        )
        .with_session_state(with_udf.clone())
        .with_source_alias("source")
        .with_target_alias("target")
        .when_matched_update(|u| u.update("off", "source.off"))?
        .when_not_matched_insert(|i| {
            i.set("entity_id", "source.entity_id")
                .set("off", "source.off")
        })?
        .await;
    println!(
        "  b UDF in merge predicate, session WITH udf       {}",
        verdict(&r)
    );

    // (c) UDF in the predicate + session WITHOUT udf -- the control that proves (b) was the
    //     session's doing rather than delta-rs resolving the name some other way.
    let (t, _) = make_table(&root, "t_pb05c").await?;
    let r = t
        .merge(
            source.clone(),
            "target.entity_id = source.entity_id AND is_valid_offset(target.off)",
        )
        .with_session_state(without_udf.clone())
        .with_source_alias("source")
        .with_target_alias("target")
        .when_matched_update(|u| u.update("off", "source.off"))?
        .when_not_matched_insert(|i| {
            i.set("entity_id", "source.entity_id")
                .set("off", "source.off")
        })?
        .await;
    println!(
        "  c CONTROL UDF in predicate, session WITHOUT udf  {}",
        verdict(&r)
    );

    // (d) UDF inside a MATCH CLAUSE predicate, which is resolved separately from the join
    //     predicate and is where the merge passes of plan §6.2 would actually use one.
    let (t, _) = make_table(&root, "t_pb05d").await?;
    let r = t
        .merge(source.clone(), "target.entity_id = source.entity_id")
        .with_session_state(with_udf.clone())
        .with_source_alias("source")
        .with_target_alias("target")
        .when_matched_update(|u| {
            u.predicate("is_valid_offset(source.off)")
                .update("off", "source.off")
        })?
        .await;
    println!(
        "  d UDF in when_matched_update predicate           {}",
        verdict(&r)
    );

    // (e) the fallback policy. `InternalDefaults` is the DEFAULT and discards a non-SessionState
    //     session with a log line. `RequireSessionState` turns that into an error.
    let (t, _) = make_table(&root, "t_pb05e").await?;
    let r = t
        .merge(
            source.clone(),
            "target.entity_id = source.entity_id AND is_valid_offset(target.off)",
        )
        .with_session_state(with_udf.clone())
        .with_session_fallback_policy(SessionFallbackPolicy::RequireSessionState)
        .with_source_alias("source")
        .with_target_alias("target")
        .when_matched_update(|u| u.update("off", "source.off"))?
        .await;
    println!(
        "  e UDF predicate + RequireSessionState            {}",
        verdict(&r)
    );
    println!();

    // ================= PB07 — CHECK constraints, add time vs write time =================
    println!("== PB07 · can a CHECK constraint reference a UDF, and survive to write time? ==\n");

    // (a) plain constraint -- the control. Must succeed.
    let (t, url_a) = make_table(&root, "t_pb07a").await?;
    let r = t
        .add_constraint()
        .with_constraint("off_nonneg", "off >= 0")
        .with_session_state(with_udf.clone())
        .await;
    println!(
        "  a CONTROL plain constraint `off >= 0`            {}",
        verdict(&r)
    );

    // (b) UDF constraint, session WITH udf -- can it be ADDED?
    let (t, url_b) = make_table(&root, "t_pb07b").await?;
    let added = t
        .add_constraint()
        .with_constraint("off_valid", "is_valid_offset(off)")
        .with_session_state(with_udf.clone())
        .await;
    println!(
        "  b UDF constraint, session WITH udf (add)         {}",
        verdict(&added)
    );

    // (c) if it was added, can a LATER writer without the UDF still write? This is the
    //     asymmetry the source predicts: the constraint is stored as SQL TEXT and re-parsed
    //     against the writer's session.
    if added.is_ok() {
        let reload = DeltaTableBuilder::from_url(url_b.clone())?.load().await?;
        let r = reload
            .write(vec![batch(vec!["e9"], vec![99])?])
            .with_session_state(without_udf.clone())
            .await;
        println!(
            "  c write by a session WITHOUT the udf             {}",
            verdict(&r)
        );

        let reload = DeltaTableBuilder::from_url(url_b.clone())?.load().await?;
        let r = reload
            .write(vec![batch(vec!["e8"], vec![88])?])
            .with_session_state(with_udf.clone())
            .await;
        println!(
            "  d CONTROL write by a session WITH the udf        {}",
            verdict(&r)
        );
    } else {
        println!("  c skipped — the constraint could not be added");
        println!("  d skipped — the constraint could not be added");
    }

    // (e) can a constraint reference ANOTHER TABLE? This is what decides whether referential
    //     invariants are constraints or scheduled queries (plan risk register).
    // NOTE this arm runs inside `tokio::spawn`. `DeltaContextProvider::get_table_source` is
    // `unimplemented!()` (delta_datafusion/expr.rs:237), so a constraint naming another table
    // PANICS rather than returning an error. A panic on the main task would abort the process
    // before PB12 runs; in a spawned task it surfaces as a JoinError that can be reported.
    let (t, _) = make_table(&root, "t_pb07e").await?;
    let session_e = with_udf.clone();
    let handle = tokio::spawn(async move {
        t.add_constraint()
            .with_constraint("fk", "entity_id IN (SELECT entity_id FROM other)")
            .with_session_state(session_e)
            .await
            .map(|_| ())
    });
    let outcome = match handle.await {
        Ok(inner) => verdict(&inner),
        Err(join_err) if join_err.is_panic() => "PANIC (process would have aborted)".to_string(),
        Err(join_err) => format!("ERR: join {join_err}"),
    };
    println!("  e constraint with a subquery on another table    {outcome}");
    println!();

    // ================= PB12 — can the wrapper read the table's own constraints? ==========
    println!("== PB12 · what does a table expose about its own constraints? ==\n");
    for (label, url) in [
        ("plain `off >= 0`", &url_a),
        ("UDF `is_valid_offset(off)`", &url_b),
    ] {
        let table = DeltaTableBuilder::from_url(url.clone())?.load().await?;
        let config: HashMap<String, String> = table
            .snapshot()?
            .metadata()
            .configuration()
            .iter()
            .filter(|(k, _)| k.starts_with("delta.constraints."))
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect();
        println!("  {label}");
        if config.is_empty() {
            println!("      (no delta.constraints.* keys)");
        }
        for (k, v) in &config {
            println!("      {k} = {v:?}");
        }
        // What the plan's wrapper would need: a DataFusion `Constraints`. That type carries only
        // PrimaryKey/Unique over column INDICES, so an arbitrary CHECK expression has no
        // representation in it. Print the shape so the round-2 decision is made on the facts.
        println!(
            "      stored as: SQL TEXT. DataFusion `Constraints` holds PrimaryKey/Unique over"
        );
        println!("      column indices only — an arbitrary CHECK has no slot in that type.");
    }
    println!();
    Ok(())
}
