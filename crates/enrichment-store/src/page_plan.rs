//! Native bounded-page selection. The sentinel is counted in DataFusion, never decoded and
//! truncated by a handler. Count and selected rows share the same immutable input plan.
use crate::{native_catalog, runtime::QueryRuntime};
use datafusion::{
    common::{DataFusionError, Result},
    dataframe::DataFrame,
    prelude::*,
};
use enrichment_core::native_union::{NativeStruct, Rule};

enrichment_core::native_struct! { pub struct Policy {
    page_size: u64 => Rule::UnsignedRange {min:1,max:1024},
    offset: u64 => Rule::Text,
    total: Option<u64> => Rule::Text,
    detail: bool => Rule::Text,
} }
enrichment_core::native_struct! { pub struct Boundary {
    returned: u64 => Rule::Text,
    total: Option<u64> => Rule::Text,
    has_more: bool => Rule::Text,
    next_offset: u64 => Rule::Text,
} }
pub struct Selected {
    pub frame: DataFrame,
    pub boundary: Boundary,
}

pub async fn select(runtime: &QueryRuntime, frame: DataFrame, policy: Policy) -> Result<Selected> {
    let session = runtime.session();
    native_catalog::input(
        &session,
        "page_policy",
        Policy::batch(std::slice::from_ref(&policy))?,
    )?;
    runtime.require_empty_with_cause(session.sql("SELECT 'invalid_page_policy' AS witness FROM page_policy WHERE page_size<1 OR page_size>1024 OR (total IS NOT NULL AND offset>total)").await?,"native_page_policy","page_selection", enrichment_core::wire::DiagnosticCause::InvalidInput).await?;
    let limit =
        usize::try_from(policy.page_size).map_err(|e| DataFusionError::Plan(e.to_string()))?;
    let count = frame.clone().limit(0, Some(limit + 1))?.aggregate(
        vec![],
        vec![
            datafusion::logical_expr::expr_fn::cast(
                datafusion::functions_aggregate::expr_fn::count(lit(1_i32)),
                arrow::datatypes::DataType::UInt64,
            )
            .alias("observed"),
        ],
    )?;
    native_catalog::work(&session, "page_candidates", count.into_view())?;
    runtime.require_empty_with_cause(session.sql("SELECT 'invalid_page_position' AS witness FROM page_policy p CROSS JOIN page_candidates c WHERE (p.detail AND c.observed<>1) OR (p.total IS NOT NULL AND p.offset+CAST(least(c.observed,p.page_size) AS DECIMAL(20,0))>p.total) OR p.offset+CAST(least(c.observed,p.page_size) AS DECIMAL(20,0))>18446744073709551615").await?,"native_page_position","page_selection", enrichment_core::wire::DiagnosticCause::InvalidInput).await?;
    let boundary=runtime.records(session.sql("SELECT least(c.observed,p.page_size) AS returned, CASE WHEN p.detail THEN least(c.observed,p.page_size) ELSE p.total END AS total, NOT p.detail AND (c.observed>p.page_size OR coalesce(p.offset+least(c.observed,p.page_size)<p.total,false)) AS has_more, p.offset+least(c.observed,p.page_size) AS next_offset FROM page_policy p CROSS JOIN page_candidates c").await?,1).await?.pop().ok_or_else(|| DataFusionError::Internal("native page boundary missing".into()))?;
    // An empty keyset cannot promise another nonempty page merely from an inconsistent cursor.
    let boundary: Boundary = boundary;
    let verified = crate::native_catalog::batch(
        &session,
        "page_plan",
        Boundary::batch(std::slice::from_ref(&boundary))?,
    )?;
    runtime
        .require_empty_with_cause(
            verified
                .filter(col("has_more").and(col("returned").eq(lit(0_u64))))?
                .select(vec![lit("empty_continuation").alias("witness")])?,
            "native_page_progress",
            "page_selection",
            enrichment_core::wire::DiagnosticCause::InvalidInput,
        )
        .await?;
    Ok(Selected {
        frame: frame.limit(0, Some(limit))?,
        boundary,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn plan19_native_pages_preserve_order_progress_exact_counts_and_detail_scope()
    -> Result<()> {
        let root = tempfile::tempdir()?;
        let runtime = QueryRuntime::new(root.path(), Default::default())?;
        let all = runtime
            .session()
            .sql("SELECT * FROM (VALUES (3),(1),(2)) AS data(value) ORDER BY value")
            .await?;
        let policy = Policy {
            page_size: 2,
            offset: 0,
            total: Some(3),
            detail: false,
        };
        let selected = select(&runtime, all.clone(), policy.clone()).await?;
        assert_eq!(
            selected.boundary,
            Boundary {
                returned: 2,
                total: Some(3),
                has_more: true,
                next_offset: 2
            }
        );
        enrichment_core::native_struct! { struct Value {value:i64=>Rule::Text} }
        assert_eq!(
            runtime.records::<Value>(selected.frame, 2).await?,
            vec![Value { value: 1 }, Value { value: 2 }]
        );
        let last = select(
            &runtime,
            all.clone().filter(col("value").gt(lit(2_i64)))?,
            Policy {
                offset: 2,
                ..policy.clone()
            },
        )
        .await?;
        assert_eq!(
            last.boundary,
            Boundary {
                returned: 1,
                total: Some(3),
                has_more: false,
                next_offset: 3
            }
        );
        let empty = all.clone().filter(lit(false))?;
        assert_eq!(
            select(
                &runtime,
                empty.clone(),
                Policy {
                    total: None,
                    ..policy.clone()
                }
            )
            .await?
            .boundary,
            Boundary {
                returned: 0,
                total: None,
                has_more: false,
                next_offset: 0
            }
        );
        let detail = select(
            &runtime,
            all.clone().filter(col("value").eq(lit(2_i64)))?,
            Policy {
                detail: true,
                ..policy.clone()
            },
        )
        .await?;
        assert_eq!(
            detail.boundary,
            Boundary {
                returned: 1,
                total: Some(1),
                has_more: false,
                next_offset: 1
            }
        );
        for invalid in [
            Policy {
                page_size: 0,
                ..policy.clone()
            },
            Policy {
                page_size: 1025,
                ..policy.clone()
            },
            Policy {
                offset: 4,
                ..policy.clone()
            },
            Policy {
                total: Some(1),
                ..policy.clone()
            },
            Policy {
                detail: true,
                ..policy.clone()
            },
            Policy {
                offset: u64::MAX,
                total: None,
                ..policy.clone()
            },
        ] {
            assert!(select(&runtime, all.clone(), invalid).await.is_err());
        }
        assert!(
            select(
                &runtime,
                empty,
                Policy {
                    offset: 2,
                    ..policy
                }
            )
            .await
            .is_err(),
            "an empty continuation cannot claim progress"
        );
        runtime.close_diagnostics().await
    }
}
