use std::sync::Arc;

use datafusion::{
    arrow::{
        array::{Int64Array, UInt64Array},
        datatypes::{DataType, Field, Schema},
        record_batch::RecordBatch,
    },
    common::TableReference,
    execution::SessionStateBuilder,
    logical_expr::dml::InsertOp,
    prelude::SessionContext,
};
use datafusion_proto::logical_plan::LogicalExtensionCodec;
use deltalake::{
    DeltaTable,
    delta_datafusion::{DeltaCdfTableProvider, DeltaLogicalCodec, planner::DeltaPlanner},
};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn batch(id: i64) -> RecordBatch {
    RecordBatch::try_new(
        Arc::new(Schema::new(vec![Field::new("id", DataType::Int64, false)])),
        vec![Arc::new(Int64Array::from(vec![id]))],
    )
    .unwrap()
}

async fn count(ctx: &SessionContext, query: &str) -> Result<i64> {
    let batches = ctx.sql(query).await?.collect().await?;
    Ok(batches[0]
        .column(0)
        .as_any()
        .downcast_ref::<Int64Array>()
        .unwrap()
        .value(0))
}

#[tokio::main]
async fn main() -> Result<()> {
    let root = std::path::PathBuf::from(
        std::env::args()
            .nth(1)
            .expect("fresh absolute scratch path"),
    );
    std::fs::create_dir_all(&root)?;
    let url = url::Url::from_directory_path(root).map_err(|_| "absolute scratch path required")?;
    let ctx = SessionContext::new();
    let state = Arc::new(
        SessionStateBuilder::from(ctx.state())
            .with_query_planner(DeltaPlanner::new())
            .build(),
    );
    let table = DeltaTable::try_from_url(url)
        .await?
        .write(vec![batch(1)])
        .with_configuration([("delta.enableChangeDataFeed", Some("true"))])
        .with_session_state(state.clone())
        .await?;
    let table = table
        .write(vec![batch(2)])
        .with_session_state(state.clone())
        .await?;
    assert_eq!(table.version(), Some(1));
    table.update_datafusion_session(state.as_ref())?;
    let provider = table.table_provider().with_session(state.clone()).await?;
    let codec = DeltaLogicalCodec {};
    let name = TableReference::bare("events");
    let mut bytes = Vec::new();
    codec.try_encode_table_provider(&name, provider.clone(), &mut bytes)?;
    let decoded = codec.try_decode_table_provider(
        &bytes,
        &name,
        provider.schema(),
        ctx.task_ctx().as_ref(),
    )?;
    let decoded_wrong_schema = codec.try_decode_table_provider(
        &bytes,
        &name,
        Arc::new(Schema::empty()),
        ctx.task_ctx().as_ref(),
    )?;
    assert_eq!(decoded_wrong_schema.schema(), provider.schema());
    println!(
        "CODEC_SCHEMA: caller-supplied empty schema is ignored; validation remains a binding responsibility"
    );
    let table = table
        .write(vec![batch(3)])
        .with_session_state(state.clone())
        .await?;
    assert_eq!(table.version(), Some(2));
    ctx.register_table("decoded", decoded.clone())?;
    assert_eq!(count(&ctx, "SELECT COUNT(*) FROM decoded").await?, 2);
    println!("CODEC_PIN: decoded version-1 provider remains at 2 rows after version 2 exists");
    let input = ctx
        .sql("SELECT CAST(4 AS BIGINT) AS id")
        .await?
        .create_physical_plan()
        .await?;
    let error = decoded
        .insert_into(state.as_ref(), input, InsertOp::Append)
        .await
        .unwrap_err();
    assert!(error.to_string().contains("log_store"));
    println!("CODEC_MUTATION: decoded provider rejects INSERT without runtime log_store handle");
    let cdf = Arc::new(DeltaCdfTableProvider::try_new(
        table
            .scan_cdf()
            .with_starting_version(1)
            .with_ending_version(1),
    )?);
    let mut cdf_bytes = Vec::new();
    assert!(
        codec
            .try_encode_table_provider(
                &TableReference::bare("changes"),
                cdf.clone(),
                &mut cdf_bytes
            )
            .is_err()
    );
    println!("CODEC_CDF: DeltaLogicalCodec rejects CDF provider encoding");
    ctx.register_table("changes", cdf)?;
    assert_eq!(count(&ctx, "SELECT COUNT(*) FROM changes").await?, 1);
    let batches = ctx
        .sql("SELECT _commit_version FROM changes WHERE id = 2 AND _change_type = 'insert' LIMIT 1")
        .await?
        .collect()
        .await?;
    assert_eq!(batches.iter().map(RecordBatch::num_rows).sum::<usize>(), 1);
    assert_eq!(
        batches[0]
            .column(0)
            .as_any()
            .downcast_ref::<UInt64Array>()
            .unwrap()
            .value(0),
        1
    );
    println!(
        "CDF_PROVIDER: explicit [1,1] window excludes versions 0 and 2; filter-only id/change_type columns survive projection+limit"
    );
    println!("PROBE_COMPLETE");
    Ok(())
}
