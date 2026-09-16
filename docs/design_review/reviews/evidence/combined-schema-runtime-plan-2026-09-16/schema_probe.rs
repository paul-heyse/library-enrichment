use datafusion::{
    arrow::{
        array::{ArrayRef, Int64Array, StructArray},
        buffer::NullBuffer,
        datatypes::{DataType, Field, Schema},
        record_batch::RecordBatch,
    },
    execution::SessionStateBuilder,
    prelude::{SessionContext, col},
};
use deltalake::{
    delta_datafusion::planner::DeltaPlanner,
    kernel::{StructType, engine::arrow_conversion::TryFromArrow},
    operations::create::CreateBuilder,
};
use std::{collections::HashMap, path::Path, sync::Arc};
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn metadata(domain: &str) -> HashMap<String, String> {
    HashMap::from([
        ("ARROW:extension:name".into(), "enrichment.id".into()),
        (
            "ARROW:extension:metadata".into(),
            format!("{{\"domain\":\"{domain}\"}}"),
        ),
    ])
}

async fn plans(ctx: &SessionContext) -> Result<()> {
    for (table, domain) in [("symbols", "symbol"), ("definitions", "definition")] {
        ctx.register_batch(
            table,
            RecordBatch::try_new(
                Arc::new(Schema::new(vec![
                    Field::new("id", DataType::Int64, false).with_metadata(metadata(domain)),
                ])),
                vec![Arc::new(Int64Array::from(vec![7]))],
            )?,
        )?;
    }
    for (name, query) in [
        ("projection", "SELECT id FROM symbols"),
        ("alias", "SELECT id AS renamed FROM symbols"),
        ("cast", "SELECT CAST(id AS BIGINT) AS id FROM symbols"),
        (
            "union_same",
            "SELECT id FROM symbols UNION ALL SELECT id FROM symbols",
        ),
        (
            "union_cross_domain",
            "SELECT id FROM symbols UNION ALL SELECT id FROM definitions",
        ),
        (
            "join_cross_domain",
            "SELECT s.id FROM symbols s JOIN definitions d ON s.id=d.id",
        ),
        ("aggregate", "SELECT min(id) AS id FROM symbols"),
    ] {
        match ctx.sql(query).await {
            Ok(frame) => {
                let logical = frame.schema().field(0).metadata().clone();
                match frame.collect().await {
                    Ok(batches) => println!(
                        "PLAN {name}: logical={logical:?} physical={:?} rows={}",
                        batches
                            .first()
                            .map(|b| b.schema().field(0).metadata().clone()),
                        batches.iter().map(|b| b.num_rows()).sum::<usize>()
                    ),
                    Err(e) => println!("PLAN {name}: execution refused {e}"),
                }
            }
            Err(e) => println!("PLAN {name}: planning refused {e}"),
        }
    }
    for (name, query) in [
        (
            "reorder",
            "SELECT CAST(named_struct('b', CAST(20 AS BIGINT), 'a', CAST(10 AS BIGINT)) AS STRUCT<a BIGINT, b BIGINT>) AS value",
        ),
        (
            "rename",
            "SELECT CAST(named_struct('a', CAST(10 AS BIGINT), 'c', CAST(30 AS BIGINT)) AS STRUCT<a BIGINT, b BIGINT>) AS value",
        ),
    ] {
        let frame = ctx.sql(query).await?;
        println!("CAST {name}: {:?}", frame.collect().await?);
    }
    Ok(())
}

fn input(parent_present: bool, child: Option<i64>) -> Result<RecordBatch> {
    let fields = vec![Arc::new(Field::new("required", DataType::Int64, true))];
    let payload: ArrayRef = Arc::new(StructArray::try_new(
        fields.clone().into(),
        vec![Arc::new(Int64Array::from(vec![child]))],
        Some(NullBuffer::from(vec![parent_present])),
    )?);
    Ok(RecordBatch::try_new(
        Arc::new(Schema::new(vec![
            Field::new("payload", DataType::Struct(fields.into()), true),
            Field::new("id", DataType::Int64, false),
        ])),
        vec![payload, Arc::new(Int64Array::from(vec![1]))],
    )?)
}

async fn delta(ctx: &SessionContext, root: &Path) -> Result<()> {
    let state = Arc::new(ctx.state());
    for (name, parent_present, child) in [
        ("valid", true, Some(9)),
        ("missing_child", true, None),
        ("absent_parent", false, None),
        ("guarded_absent", false, None),
        ("guarded_missing", true, None),
        ("feature_guarded_absent", false, None),
        ("feature_guarded_missing", true, None),
        ("explicit_nested_stats", true, Some(9)),
    ] {
        let location = root.join(name);
        std::fs::create_dir_all(&location)?;
        let schema = Schema::new(vec![
            Field::new(
                "payload",
                DataType::Struct(
                    vec![Arc::new(Field::new(
                        "required",
                        DataType::Int64,
                        name.contains("guarded"),
                    ))]
                    .into(),
                ),
                true,
            ),
            Field::new("id", DataType::Int64, false),
        ]);
        let kernel = StructType::try_from_arrow(&schema)?;
        let mut configuration = HashMap::from([("delta.dataSkippingNumIndexedCols", Some("2"))]);
        if name.starts_with("guarded") {
            configuration.insert(
                "delta.constraints.required_when_present",
                Some("payload IS NULL OR payload.required IS NOT NULL"),
            );
        }
        if name == "explicit_nested_stats" {
            configuration.insert(
                "delta.dataSkippingStatsColumns",
                Some("payload.required,id"),
            );
        }
        let table = CreateBuilder::new()
            .with_location(location.to_str().ok_or("path")?)
            .with_columns(kernel.fields().cloned())
            .with_raise_if_key_not_exists(false)
            .with_configuration(configuration)
            .await?;
        let table = if name.starts_with("feature_guarded") {
            table
                .add_constraint()
                .with_constraint(
                    "required_when_present",
                    "payload IS NULL OR payload.required IS NOT NULL",
                )
                .with_session_state(state.clone())
                .await?
        } else {
            table
        };
        let (_, plan) = ctx.read_batch(input(parent_present, child)?)?.into_parts();
        let outcome = table
            .write(Vec::new())
            .with_input_plan(plan)
            .with_session_state(state.clone())
            .await;
        match outcome {
            Ok(table) => {
                let provider = table.table_provider().with_session(state.clone()).await?;
                let frame = ctx
                    .read_table(provider)?
                    .select(vec![col("payload"), col("id")])?;
                println!(
                    "NULLABILITY {name}: accepted version={:?} batches={:?}",
                    table.version(),
                    frame.collect().await?
                );
            }
            Err(e) => println!("NULLABILITY {name}: refused {e}"),
        }
    }
    Ok(())
}

fn main() -> Result<()> {
    let root = std::path::PathBuf::from(std::env::args().nth(1).ok_or("scratch root")?);
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(2)
        .enable_all()
        .build()?;
    runtime.block_on(async {
        let base = SessionContext::new();
        let state = SessionStateBuilder::from(base.state())
            .with_query_planner(DeltaPlanner::new())
            .build();
        let ctx = SessionContext::new_with_state(state);
        plans(&ctx).await?;
        delta(&ctx, &root).await
    })
}
