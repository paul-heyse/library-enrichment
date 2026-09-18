//! Native ordered alternative-prefix selection and exact escaped-byte accounting.
use crate::{native_catalog, runtime::QueryRuntime};
use datafusion::{
    common::{DataFusionError, Result},
    dataframe::DataFrame,
    prelude::*,
};
use enrichment_core::native_union::{NativeStruct, Rule};

pub(crate) const ITEMS: usize = 32;
const INLINE_BYTES: u64 = 4096;
enrichment_core::native_struct! { struct Policy {
    offset: u64 => Rule::Text,
    artifact_bytes: u64 => Rule::Text,
} }
enrichment_core::native_struct! { pub(crate) struct Boundary {
    observed: bool => Rule::Text,
    returned: u64 => Rule::Text,
    has_more: bool => Rule::Text,
    next_offset: u64 => Rule::Text,
    remaining: u64 => Rule::Text,
} }
pub(crate) struct Selected {
    pub frame: DataFrame,
    pub boundary: Boundary,
}

/// An unrequested detail side contributes observed presence only. Its alternatives are
/// neither rehydrated nor counted; the native aggregate preserves absence versus empty.
pub(crate) async fn presence(
    runtime: &QueryRuntime,
    input: DataFrame,
) -> Result<Option<Vec<enrichment_core::compare::Alternative>>> {
    use enrichment_core::evidence::arrow_model::expressions::literal;
    enrichment_core::native_struct! { struct Presence {
        values:Option<Vec<enrichment_core::compare::Alternative>> => Rule::Sequence,
    } }
    let count = datafusion::functions_aggregate::expr_fn::count(lit(1));
    let observed = input
        .limit(0, Some(1))?
        .aggregate(vec![], vec![count.alias("count")])?;
    let values = datafusion::logical_expr::when(
        col("count").gt(lit(0i64)),
        literal(&Some(Vec::<enrichment_core::compare::Alternative>::new()))?,
    )
    .otherwise(literal(
        &Option::<Vec<enrichment_core::compare::Alternative>>::None,
    )?)?;
    runtime
        .records::<Presence>(observed.select(vec![values.alias("values")])?, 1)
        .await?
        .pop()
        .map(|row| row.values)
        .ok_or_else(|| DataFusionError::Internal("comparison presence missing".into()))
}

pub(crate) async fn select(
    runtime: &QueryRuntime,
    input: DataFrame,
    offset: usize,
    artifact_bytes: usize,
) -> Result<Selected> {
    let field = input.schema().field_with_unqualified_name("value")?.clone();
    let ordered = input
        .sort(vec![
            col("value").sort(true, true),
            col("source").sort(true, true),
        ])?
        .limit(offset, Some(ITEMS + 1))?;
    let measured = ordered.with_column(
        "encoded_bytes",
        enrichment_core::native_transport::value_bytes(
            runtime.session().runtime_env().memory_pool.clone(),
            field,
            crate::result::MAX_BYTES as usize,
        )
        .call(vec![col("value")]),
    )?;
    let session = runtime.session();
    native_catalog::input(
        &session,
        "alternative_policy",
        Policy::batch(&[Policy {
            offset: offset as u64,
            artifact_bytes: artifact_bytes as u64,
        }])?,
    )?;
    native_catalog::work(&session, "alternative_measured", measured.into_view())?;
    let candidates=session.sql(&format!(r#"
        WITH ranked AS (SELECT *,row_number() OVER (ORDER BY value ASC NULLS FIRST,source ASC NULLS FIRST) AS ordinal,
          encoded_bytes<={INLINE_BYTES} AS inline,
          CASE WHEN encoded_bytes<={INLINE_BYTES} THEN CAST(0 AS BIGINT UNSIGNED) ELSE encoded_bytes END AS artifact_bytes
          FROM alternative_measured), costs AS (
          SELECT *,sum(artifact_bytes) OVER (ORDER BY ordinal ROWS BETWEEN UNBOUNDED PRECEDING AND CURRENT ROW) AS cumulative FROM ranked
        ) SELECT c.*, c.ordinal<={ITEMS} AND c.cumulative<=p.artifact_bytes AS selected FROM costs c CROSS JOIN alternative_policy p
    "#)).await?;
    native_catalog::work(&session, "alternative_candidates", candidates.into_view())?;
    let summary=session.sql("SELECT count(*)>0 AS observed,CAST(count(*) FILTER (WHERE selected) AS BIGINT UNSIGNED) AS returned,count(*)>count(*) FILTER (WHERE selected) AS has_more,coalesce(sum(artifact_bytes) FILTER (WHERE selected),CAST(0 AS BIGINT UNSIGNED)) AS used FROM alternative_candidates").await?;
    native_catalog::work(&session, "alternative_summary", summary.into_view())?;
    runtime.require_empty_with_cause(session.sql("SELECT 'alternative_capacity' AS witness FROM alternative_summary WHERE observed AND returned=0").await?,"comparison_alternative_capacity","comparison_page",enrichment_core::wire::DiagnosticCause::Capacity).await?;
    runtime.require_empty_with_cause(session.sql("SELECT 'alternative_cursor' AS witness FROM alternative_summary s CROSS JOIN alternative_policy p WHERE (p.offset>0 AND NOT s.observed) OR p.offset+CAST(s.returned AS DECIMAL(20,0))>18446744073709551615").await?,"comparison_alternative_cursor","comparison_page",enrichment_core::wire::DiagnosticCause::InvalidInput).await?;
    runtime.require_empty(session.sql("SELECT 'alternative_accounting' AS witness FROM alternative_summary s CROSS JOIN alternative_policy p WHERE s.used>p.artifact_bytes").await?,"comparison_alternative_accounting","comparison_page").await?;
    let boundary=runtime.records(session.sql("SELECT s.observed,s.returned,s.has_more,p.offset+s.returned AS next_offset,p.artifact_bytes-s.used AS remaining FROM alternative_summary s CROSS JOIN alternative_policy p").await?,1).await?.pop().ok_or_else(||DataFusionError::Internal("alternative boundary missing".into()))?;
    let frame=session.sql("SELECT value,source,encoded_bytes,inline FROM alternative_candidates WHERE selected ORDER BY ordinal").await?;
    Ok(Selected { frame, boundary })
}

#[cfg(test)]
mod tests {
    use super::*;
    enrichment_core::native_struct! { struct Value { value:String=>Rule::Text, source:Option<String> =>Rule::Text } }
    enrichment_core::native_struct! { struct Projected { value:String=>Rule::Text, encoded_bytes:u64=>Rule::Text, inline:bool=>Rule::Text } }
    #[tokio::test]
    async fn plan19_alternative_prefix_is_native_and_counts_exact_escaped_bytes() -> Result<()> {
        let root = tempfile::tempdir()?;
        let runtime = QueryRuntime::new(root.path(), Default::default())?;
        let values = vec![
            Value {
                value: "a😀\"\n".into(),
                source: None,
            },
            Value {
                value: format!("b{}", "\u{0001}".repeat(800)),
                source: None,
            },
            Value {
                value: "c".into(),
                source: None,
            },
        ];
        let frame = crate::native_catalog::batch(
            &runtime.session(),
            "comparison_page",
            Value::batch(&values)?,
        )?;
        let expected = serde_json::to_vec(&values[1].value).unwrap().len();
        let bounded = select(&runtime, frame.clone(), 0, expected - 1).await?;
        assert_eq!(bounded.boundary.returned, 1);
        assert!(bounded.boundary.has_more && bounded.boundary.observed);
        assert_eq!(bounded.boundary.remaining, (expected - 1) as u64);
        let exact = select(&runtime, frame.clone(), 0, expected).await?;
        assert_eq!(exact.boundary.returned, 3);
        assert!(!exact.boundary.has_more);
        assert_eq!(exact.boundary.remaining, 0);
        let projected = runtime
            .records::<Projected>(
                exact
                    .frame
                    .select_columns(&["value", "encoded_bytes", "inline"])?,
                3,
            )
            .await?;
        for (row, input) in projected.iter().zip(&values) {
            assert_eq!(row.value, input.value);
            assert_eq!(
                row.encoded_bytes,
                serde_json::to_vec(&input.value).unwrap().len() as u64
            );
        }
        assert!(projected[0].inline && !projected[1].inline && projected[2].inline);
        let error = select(&runtime, frame.clone(), 1, expected - 1)
            .await
            .err()
            .expect("never skip an unfit first alternative");
        let diagnostic = crate::query::QueryError::from(error).diagnostic();
        assert_eq!(
            diagnostic.cause,
            enrichment_core::wire::DiagnosticCause::Capacity
        );
        assert!(matches!(
            diagnostic.actions.as_slice(),
            [enrichment_core::wire::RecoveryAction::ChangeRequest { .. }]
        ));
        assert!(
            select(&runtime, frame, 4, expected).await.is_err(),
            "past-end continuation refuses"
        );
        let empty = crate::native_catalog::batch(
            &runtime.session(),
            "comparison_page",
            Value::batch(&[])?,
        )?;
        assert_eq!(presence(&runtime, empty.clone()).await?, None);
        let empty = select(&runtime, empty, 0, 0).await?;
        assert!(!empty.boundary.observed && !empty.boundary.has_more);
        let rows = (0..40)
            .map(|n| Value {
                value: format!("{n:03}"),
                source: None,
            })
            .collect::<Vec<_>>();
        let frame = crate::native_catalog::batch(
            &runtime.session(),
            "comparison_page",
            Value::batch(&rows)?,
        )?;
        assert_eq!(presence(&runtime, frame.clone()).await?, Some(vec![]));
        let bounded = select(&runtime, frame.clone(), 0, 0).await?;
        assert_eq!(bounded.boundary.returned, 32);
        assert_eq!(bounded.boundary.next_offset, 32);
        assert!(bounded.boundary.has_more);
        let end = select(&runtime, frame, 32, 0).await?;
        assert_eq!(end.boundary.returned, 8);
        assert_eq!(end.boundary.next_offset, 40);
        assert!(!end.boundary.has_more);
        Ok(())
    }
}
