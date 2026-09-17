//! One native coverage summary used by publication and read admission.
use crate::runtime::QueryRuntime;
use datafusion::{
    error::{DataFusionError, Result},
    prelude::SessionContext,
};
use enrichment_core::evidence::EvidenceKind;

pub(crate) struct Summary {
    pub indexed: Vec<EvidenceKind>,
    pub missing: Vec<EvidenceKind>,
}

pub(crate) async fn summarize(runtime: &QueryRuntime, session: &SessionContext) -> Result<Summary> {
    // Enum order is a contract, not lexical order. Both parsing and this native expression
    // derive from the same finite vocabulary; no independently maintained SQL list exists.
    let order = EvidenceKind::VALUES
        .iter()
        .enumerate()
        .map(|(index, kind)| format!("WHEN '{}' THEN {index}", kind))
        .collect::<Vec<_>>()
        .join(" ");
    let plan = session.sql(&format!("WITH kinds AS (SELECT kind, bool_or(outcome <> 'missing') AS present FROM snapshot.evidence.coverage GROUP BY kind) SELECT array_agg(kind ORDER BY CASE kind {order} END) FILTER (WHERE present) AS indexed, array_agg(kind ORDER BY CASE kind {order} END) FILTER (WHERE NOT present) AS missing FROM kinds")).await?;
    #[derive(serde::Deserialize)]
    struct Row {
        indexed: Option<Vec<EvidenceKind>>,
        missing: Option<Vec<EvidenceKind>>,
    }
    let mut rows = crate::registry::rows::<Row>(runtime, plan, 1).await?;
    let row = rows
        .pop()
        .ok_or_else(|| DataFusionError::Internal("coverage aggregate has no row".into()))?;
    Ok(Summary {
        indexed: row.indexed.unwrap_or_default(),
        missing: row.missing.unwrap_or_default(),
    })
}
