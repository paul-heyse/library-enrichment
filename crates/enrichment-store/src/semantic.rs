//! Typed native semantic projection, deduplication, ordered aggregation and SHA-256.
//!
//! Encoding v1: domain and field names are length-prefixed UTF-8; each value has a
//! validity byte and type tag. Integers use fixed-width little-endian bytes, strings/binary
//! use a u64 byte length, ordered lists use a u64 element count, and structs retain field
//! order. Null and empty differ. String offset/view encodings have identical semantics.
//! Set normalization is a native distinct/order operation before aggregation.
use crate::{admission::Relation, projection, runtime::QueryRuntime};
use arrow::array::*;
use datafusion::{
    dataframe::DataFrame,
    error::{DataFusionError, Result},
    functions::{
        core::expr_fn::coalesce, crypto::expr_fn::sha256, encoding::expr_fn::encode,
        string::expr_fn::concat,
    },
    functions_aggregate::{expr_fn::count, string_agg::string_agg},
    logical_expr::expr_fn::ExprFunctionExt,
    prelude::{col, lit},
};

fn invalid(message: &str) -> DataFusionError {
    DataFusionError::Execution(message.into())
}
fn hex_digest(expr: datafusion::logical_expr::Expr) -> datafusion::logical_expr::Expr {
    encode(sha256(expr), lit("hex"))
}

fn keys(relation: Relation, plan: DataFrame) -> Result<DataFrame> {
    let input = match relation {
        Relation::Definitions | Relation::Symbols => {
            let schema = relation.schema()?;
            enrichment_core::native_identity::canonical_bytes(
                format!("enrichment/arrow-row/1/{}", relation.name()),
                schema.fields().clone(),
            )
            .call(schema.fields().iter().map(|f| col(f.name())).collect())
        }
        Relation::ProducerRuns => col("producer_binding_id"),
        _ => col(relation.key()),
    };
    plan.select(vec![hex_digest(input).alias("semantic_key")])?
        .distinct()
}

pub(crate) fn digest_plan(relation: Relation, plan: DataFrame) -> Result<DataFrame> {
    // One row per relation, retaining the count for native admission before reuse.
    let aggregate = keys(relation, plan)?.aggregate(
        vec![],
        vec![
            string_agg(col("semantic_key"), lit(""))
                .order_by(vec![col("semantic_key").sort(true, false)])
                .build()?
                .alias("keys"),
            count(col("semantic_key")).alias("rows"),
        ],
    )?;
    aggregate.select(vec![
        hex_digest(concat(vec![
            lit(format!("enrichment/relation/1/{}/", relation.name())),
            coalesce(vec![col("keys"), lit("")]),
        ]))
        .alias("digest"),
        col("rows"),
    ])
}

pub(crate) async fn digest(
    relation: Relation,
    plan: DataFrame,
    runtime: &QueryRuntime,
    max_rows: usize,
) -> Result<String> {
    // StringAgg owns ordered multi-partition merging and native accumulator memory accounting.
    // Fixed-width hexadecimal row digests need no ambiguous delimiter or object reconstruction.
    let frame = digest_plan(relation, plan)?;
    let output = runtime.execute(frame).await?;
    let batch = output
        .batches
        .first()
        .ok_or_else(|| invalid("digest produced no row"))?;
    let rows = batch
        .column(1)
        .as_any()
        .downcast_ref::<Int64Array>()
        .ok_or_else(|| invalid("digest count type"))?
        .value(0);
    if usize::try_from(rows).map_err(|_| invalid("negative digest count"))? > max_rows {
        return Err(DataFusionError::ResourcesExhausted(
            "semantic relation exceeds row bound".into(),
        ));
    }
    Ok(projection::TextColumn::new(batch.column(0).as_ref())?
        .required(0)?
        .to_owned())
}

#[cfg(test)]
mod tests {
    use super::*;
    use arrow::{
        datatypes::{DataType, Field, Schema},
        record_batch::RecordBatch,
    };
    use datafusion::datasource::MemTable;
    use std::sync::Arc;
    #[tokio::test]
    async fn ordered_native_digest_is_partition_and_duplicate_independent() {
        let directory = tempfile::tempdir().unwrap();
        let mut reference = None;
        for partitions in [1, 4] {
            let runtime = QueryRuntime::new(
                directory.path(),
                crate::runtime::QueryLimits {
                    partitions,
                    ..Default::default()
                },
            )
            .unwrap();
            let schema = Arc::new(Schema::new(vec![Field::new(
                "fragment_id",
                DataType::Utf8,
                false,
            )]));
            let batch = RecordBatch::try_new(
                schema.clone(),
                vec![Arc::new(StringArray::from(vec!["z", "a", "b", "a"]))],
            )
            .unwrap();
            let session = runtime.session();
            let frame = session
                .read_table(Arc::new(
                    MemTable::try_new(schema, vec![vec![batch]; partitions]).unwrap(),
                ))
                .unwrap();
            let actual = digest(Relation::Fragments, frame.clone(), &runtime, 100)
                .await
                .unwrap();
            if let Some(expected) = &reference {
                assert_eq!(expected, &actual);
            } else {
                reference = Some(actual);
            }
            let changed = digest(
                Relation::Fragments,
                frame.filter(col("fragment_id").not_eq(lit("b"))).unwrap(),
                &runtime,
                100,
            )
            .await
            .unwrap();
            assert_ne!(reference.as_ref().unwrap(), &changed);
        }
    }
}
