//! Native qualification of the selected Python declaration for semantic execution.
use crate::{SnapshotReader, query::QueryError, runtime::QueryRuntime};
use arrow::array::{Array, Int64Array};
use datafusion::{error::DataFusionError, prelude::SessionContext};

const CLASS_SCOPE: &str = r#"
    WITH observed AS (
        SELECT o.payload FROM snapshot.evidence.api_observations o LEFT SEMI JOIN snapshot.domain.bound_observations b
        ON o.observation_id = b.observation_id AND b.binding_id = $1
    ), declarations AS (
        SELECT count(*) AS observations,
            count(*) FILTER (WHERE payload.declared_kind != 'class'
                OR payload.python IS NULL OR payload.python.bases IS NULL) AS unsupported_observations
        FROM observed
    ), bases AS (
        SELECT unnest(payload.python.bases) AS base FROM observed
    ), unsupported AS (
        SELECT count(*) AS unsupported_bases FROM bases
        WHERE base IS NULL OR base.rendering NOT IN ('object', 'builtins.object')
    )
    SELECT observations, unsupported_observations, unsupported_bases
    FROM declarations CROSS JOIN unsupported
"#;

/// Bind semantic producer provenance to the native qualification implementation it uses.
#[must_use]
pub fn identity() -> String {
    enrichment_core::canonical::digest_hex(&serde_json::json!([
        include_str!("semantic_scope.rs"),
        crate::projection::VERSION,
    ]))
}

impl SnapshotReader {
    /// A base-free or object-only class must be established by every qualified observation.
    /// No observations, a missing Python payload or an unresolved base stays conservative.
    pub async fn is_nominal_python_class(&self, symbol: &str) -> Result<bool, QueryError> {
        assess(self.session(), self.runtime(), symbol).await
    }
}

async fn assess(
    session: &SessionContext,
    runtime: &QueryRuntime,
    symbol: &str,
) -> Result<bool, QueryError> {
    let frame = session
        .sql(CLASS_SCOPE)
        .await?
        .with_param_values(vec![datafusion::common::ScalarValue::from(symbol)])?;
    let output = runtime
        .execute_family(
            frame,
            Some(crate::preparation::QueryFamily::PythonClassScope),
        )
        .await?;
    if output.rows != 1 {
        return Err(DataFusionError::Internal("class scope aggregate cardinality".into()).into());
    }
    let batch = output
        .batches
        .iter()
        .find(|batch| batch.num_rows() == 1)
        .ok_or_else(|| DataFusionError::Internal("class scope aggregate row missing".into()))?;
    let count = |name: &str| -> Result<i64, QueryError> {
        let values = batch
            .column_by_name(name)
            .and_then(|column| column.as_any().downcast_ref::<Int64Array>())
            .filter(|values| !values.is_null(0))
            .ok_or_else(|| DataFusionError::Internal(format!("class scope count {name}")))?;
        Ok(values.value(0))
    };
    Ok(count("observations")? > 0
        && count("unsupported_observations")? == 0
        && count("unsupported_bases")? == 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn every_qualified_declaration_must_establish_nominal_scope() {
        let dir = tempfile::tempdir().unwrap();
        let runtime = QueryRuntime::new(&dir.path().join("spill"), Default::default()).unwrap();
        let session = runtime.session();
        let rows = session
            .sql(
                "SELECT * FROM (VALUES
            ('empty', 'empty', 'class', false, true),
            ('object', 'object', 'class', true, true),
            ('unknown', 'unknown', 'class', true, true),
            ('absent', 'absent', 'class', true, false),
            ('conflict-a', 'conflict', 'class', false, true),
            ('conflict-b', 'conflict', 'function', false, true)
        ) AS t(observation_id, binding_id, declared_kind, has_base, has_python)",
            )
            .await
            .unwrap();
        crate::native_catalog::work(&session, "fixtures", rows.into_view()).unwrap();
        let observations = session.sql("SELECT observation_id,
            named_struct('declared_kind', declared_kind, 'python',
                CASE WHEN has_python THEN named_struct('bases',
                    CASE WHEN has_base THEN make_array(named_struct('ordinal', 0, 'rendering', CASE WHEN binding_id = 'unknown' THEN 'Unknown' ELSE 'object' END))
                    ELSE array_slice(make_array(named_struct('ordinal', 0, 'rendering', 'object')), 1, 0) END) ELSE NULL END) AS payload
            FROM fixtures").await.unwrap();

        let bindings = session
            .sql("SELECT observation_id, binding_id FROM fixtures")
            .await
            .unwrap();
        let catalog = crate::native_catalog::BoundCatalog::default()
            .with_schema(
                crate::native_catalog::BindingKind::AdmittedEvidence,
                std::collections::BTreeMap::from([(
                    "api_observations".into(),
                    observations.into_view(),
                )]),
            )
            .with_schema(
                crate::native_catalog::BindingKind::AdmittedDomain,
                std::collections::BTreeMap::from([(
                    "bound_observations".into(),
                    bindings.into_view(),
                )]),
            );
        let session = runtime
            .bound_session(std::collections::BTreeMap::from([(
                "snapshot".into(),
                std::sync::Arc::new(catalog)
                    as std::sync::Arc<dyn datafusion::catalog::CatalogProvider>,
            )]))
            .unwrap();
        for symbol in ["empty", "object"] {
            assert!(
                assess(&session, &runtime, symbol).await.unwrap(),
                "{symbol}"
            );
        }
        for symbol in ["unknown", "absent", "conflict", "unobserved"] {
            assert!(
                !assess(&session, &runtime, symbol).await.unwrap(),
                "{symbol}"
            );
        }
    }
}
