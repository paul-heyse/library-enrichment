//! One native set reconciliation over bounded, schema-derived field facts. Whole-value facts
//! preserve alternative correlation and sequence order; paths retain declared item ordinals.
use super::*;
use datafusion::functions::core::expr_ext::FieldAccessor;
use enrichment_core::{compare::ComparisonFieldPath, evidence::arrow_model::expressions::record};

pub(super) async fn select(
    runtime: &crate::runtime::QueryRuntime,
    before: DataFrame,
    after: DataFrame,
) -> Result<Vec<ComparisonFieldPath>> {
    let lower = enrichment_core::native_comparison::fields(
        runtime.session().runtime_env().memory_pool.clone(),
    );
    let input = |frame: DataFrame| -> Result<DataFrame> {
        frame
            .select(vec![lower.call(vec![col("value")]).alias("fact")])?
            .unnest_columns(&["fact"])?
            .select(vec![
                col("fact").field("steps").alias("steps"),
                col("fact").field("value").alias("value"),
            ])
    };
    let before = input(before)?;
    let after = input(after)?;
    let changes = before
        .clone()
        .except_distinct(after.clone())?
        .union(after.except_distinct(before)?)?
        .select(vec![col("steps")])?
        .distinct()?
        .sort(vec![col("steps").sort(true, false)])?;
    let wrapper = record(
        &ComparisonFieldPath::data_type(),
        &[("steps", col("steps"))],
    )?;
    runtime
        .records(
            changes
                .select(vec![wrapper.alias("path")])?
                .select(vec![col("path").field("steps").alias("steps")])?,
            enrichment_core::compare::MAX_COMPARISON_FIELDS,
        )
        .await
}
