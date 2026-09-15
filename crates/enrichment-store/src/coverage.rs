//! One requested-scope relation shared by every research operation.
use crate::{SnapshotReader, projection::render, query::QueryError};
use datafusion::common::ScalarValue;
use enrichment_core::{
    evidence::{EvidenceKind, relational::SubjectRef},
    wire::{Coverage, ScopeAssessment, ScopeState},
};
use std::collections::BTreeSet;

impl SnapshotReader {
    /// Assess selected retained execution results, including their producer qualification.
    pub async fn assess_execution(&self, artifacts: &[String]) -> Result<Coverage, QueryError> {
        assess_execution(self.session(), self.runtime(), self.manifest(), artifacts).await
    }

    /// Acquisition, cached resolution and manifest resources evaluate the same domain.
    pub async fn assess_acquisition(&self) -> Result<Coverage, QueryError> {
        assess(
            self.session(),
            self.runtime(),
            self.manifest(),
            &acquisition_kinds(self.manifest()),
            None,
            format!(
                "requested acquisition evidence in snapshot {}",
                self.manifest().snapshot_id
            ),
        )
        .await
    }
    /// Assess only requested kinds, within this admitted snapshot's environment. A library
    /// fact may establish a symbol's coverage; a symbol fact never establishes library coverage.
    /// Successful replacement producers supersede failed attempts for coverage, while every
    /// attempt remains in the immutable coverage relation for diagnosis.
    pub async fn assess(
        &self,
        kinds: &[EvidenceKind],
        symbol_id: Option<&str>,
        scope: String,
    ) -> Result<Coverage, QueryError> {
        assess(
            self.session(),
            self.runtime(),
            self.manifest(),
            kinds,
            symbol_id,
            scope,
        )
        .await
    }
}

pub(crate) fn acquisition_kinds(
    manifest: &enrichment_core::evidence::snapshot::EvidenceManifest,
) -> Vec<EvidenceKind> {
    let mut kinds: BTreeSet<_> = manifest
        .indexed
        .iter()
        .chain(&manifest.missing)
        .copied()
        .collect();
    if kinds.is_empty() {
        kinds.insert(EvidenceKind::RegistryMetadata);
    }
    kinds.into_iter().collect()
}

/// Also used before publication, over the fully admitted candidate's coverage relation.
/// Candidate delivery and a later retained read therefore apply the same scope semantics.
pub(crate) async fn assess(
    session: &datafusion::prelude::SessionContext,
    runtime: &crate::runtime::QueryRuntime,
    manifest: &enrichment_core::evidence::snapshot::EvidenceManifest,
    kinds: &[EvidenceKind],
    symbol_id: Option<&str>,
    scope: String,
) -> Result<Coverage, QueryError> {
    let requested: BTreeSet<_> = kinds.iter().copied().collect();
    if requested.is_empty() {
        return Err(datafusion::error::DataFusionError::Plan(
            "coverage assessment requires at least one evidence kind".into(),
        )
        .into());
    }
    let subject = symbol_id.map_or_else(
        || SubjectRef::Library {
            release_id: manifest.release_id.to_string(),
        },
        |id| SubjectRef::Symbol {
            symbol_id: id.into(),
        },
    );
    let values = requested
        .iter()
        .map(|kind| format!("('{}', $1, CAST(NULL AS VARCHAR))", kind.as_str()))
        .collect::<Vec<_>>()
        .join(",");
    let mut params = vec![crate::projection::subject_scalar(&subject)?];
    let compatibility = if symbol_id.is_some() {
        params.push(ScalarValue::from(manifest.release_id.as_str()));
        "OR (c.subject.kind = 'library' AND c.subject.release_id = $2)"
    } else {
        ""
    };
    evaluate(
        session,
        runtime,
        manifest,
        format!("VALUES {values}"),
        params,
        compatibility,
        scope,
    )
    .await
}

/// Scope execution publication to its exact retained result identities and producer bindings.
/// A successful unrelated query on the same subject cannot establish this operation's coverage.
pub(crate) async fn assess_execution(
    session: &datafusion::prelude::SessionContext,
    runtime: &crate::runtime::QueryRuntime,
    manifest: &enrichment_core::evidence::snapshot::EvidenceManifest,
    artifacts: &[String],
) -> Result<Coverage, QueryError> {
    if artifacts.is_empty() || artifacts.len() > 1024 {
        return Err(datafusion::error::DataFusionError::Plan(
            "execution coverage requires 1..1024 retained result identities".into(),
        )
        .into());
    }
    let placeholders = (1..=artifacts.len())
        .map(|n| format!("${n}"))
        .collect::<Vec<_>>()
        .join(",");
    let requested = format!(
        "SELECT DISTINCT CASE payload.kind
        WHEN 'semantic_query' THEN 'semantic_queries' WHEN 'runtime_object' THEN 'runtime_api'
        WHEN 'usage_probe' THEN 'usage_probes' END AS kind, subject, source.producer_binding_id
        FROM execution_observations WHERE source.artifact_id IN ({placeholders})"
    );
    evaluate(
        session,
        runtime,
        manifest,
        requested,
        artifacts
            .iter()
            .map(|id| ScalarValue::from(id.as_str()))
            .collect(),
        "",
        "exact retained execution results and their producing bindings".into(),
    )
    .await
}

async fn evaluate(
    session: &datafusion::prelude::SessionContext,
    runtime: &crate::runtime::QueryRuntime,
    manifest: &enrichment_core::evidence::snapshot::EvidenceManifest,
    requested: String,
    params: Vec<ScalarValue>,
    compatibility: &str,
    scope: String,
) -> Result<Coverage, QueryError> {
    let sql = format!(
        "WITH requested(kind, subject, producer_binding_id) AS ({requested}), ranked AS (
           SELECT r.kind, r.subject, r.producer_binding_id,
             COALESCE(MAX(CASE c.outcome WHEN 'indexed' THEN 3 WHEN 'partial' THEN 2
               WHEN 'missing' THEN 1 ELSE 0 END), 0) AS rank,
             MIN(CASE WHEN c.outcome = 'indexed' THEN c.coverage_id END) AS indexed_id,
             MIN(CASE WHEN c.outcome = 'partial' THEN c.coverage_id END) AS partial_id,
             MIN(CASE WHEN c.outcome = 'missing' THEN c.coverage_id END) AS missing_id
           FROM requested r LEFT JOIN coverage c ON r.kind = c.kind
             AND ((r.subject IS NOT DISTINCT FROM c.subject) {compatibility})
             AND (r.producer_binding_id IS NULL OR r.producer_binding_id = c.producer_binding_id)
           GROUP BY r.kind, r.subject, r.producer_binding_id)
         SELECT kind, subject, CASE rank WHEN 3 THEN 'indexed' WHEN 2 THEN 'partial'
           WHEN 1 THEN 'missing' ELSE 'unknown' END AS state,
           COALESCE(CASE rank WHEN 3 THEN indexed_id WHEN 2 THEN partial_id
             WHEN 1 THEN missing_id ELSE NULL END, '') AS witness_id
         FROM ranked ORDER BY kind, subject, producer_binding_id"
    );
    let plan = session.sql(&sql).await?.with_param_values(params)?;
    let output = runtime
        .execute_family(plan, Some(crate::preparation::QueryFamily::Coverage))
        .await?;
    let mut coverage = Coverage {
        details: None,
        assessments: Vec::new(),
        scope,
        indexed: BTreeSet::new(),
        missing: BTreeSet::new(),
        limitations: Vec::new(),
    };
    let kinds = render::strings(&output.batches, "kind")?;
    let states = render::strings(&output.batches, "state")?;
    let witnesses = render::strings(&output.batches, "witness_id")?;
    let subjects = output
        .batches
        .iter()
        .map(crate::projection::decode::subjects)
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .flatten();
    for (((kind, state_name), witness), subject) in
        kinds.iter().zip(states).zip(witnesses).zip(subjects)
    {
        let kind = EvidenceKind::parse(kind)
            .ok_or_else(|| std::io::Error::other("invalid requested coverage kind"))?;
        let state = match state_name.as_str() {
            "indexed" => ScopeState::Indexed,
            "partial" => ScopeState::Partial,
            "missing" => ScopeState::Missing,
            "unknown" => ScopeState::Unknown,
            _ => return Err(std::io::Error::other("invalid coverage state").into()),
        };
        if state != ScopeState::Indexed {
            coverage.limitations.push(format!(
                "{}: {} within requested scope",
                kind.as_str(),
                state_name
            ));
        }
        coverage.assessments.push(ScopeAssessment {
            snapshot_id: manifest.snapshot_id.to_string(),
            subject,
            kind,
            state,
            witness_id: (!witness.is_empty()).then_some(witness),
        });
    }
    coverage.refresh_kinds();
    Ok(coverage)
}

/// Derive revision reachability from typed filesystem observations. Path-domain membership
/// belongs to this native assessment; acquisition merely records declarations and omissions.
pub async fn assess_revision_inputs(
    runtime: &crate::runtime::QueryRuntime,
    inputs: enrichment_core::producer::revision::RevisionInputs,
) -> datafusion::error::Result<enrichment_core::producer::revision::RevisionExtraction> {
    use arrow::{
        array::{ArrayRef, BooleanArray, StringArray},
        record_batch::RecordBatch,
    };
    use datafusion::datasource::MemTable;
    use enrichment_core::producer::revision::{RevisionExtraction, SourceClosure};
    use std::sync::Arc;

    let rows = inputs
        .declared_inputs
        .iter()
        .map(|p| ("declared", p.as_str()))
        .chain(inputs.source_roots.iter().map(|p| ("root", p.as_str())))
        .chain(
            inputs
                .missing_inputs
                .iter()
                .map(|p| ("missing", p.as_str())),
        )
        .chain(
            inputs
                .omissions
                .iter()
                .map(|o| ("omitted", o.path.as_str())),
        )
        .collect::<Vec<_>>();
    let batch = RecordBatch::try_from_iter(vec![
        (
            "kind",
            Arc::new(StringArray::from_iter_values(rows.iter().map(|r| r.0))) as ArrayRef,
        ),
        (
            "path",
            Arc::new(StringArray::from_iter_values(rows.iter().map(|r| r.1))) as ArrayRef,
        ),
    ])?;
    let session = runtime.session();
    session.register_table(
        "revision_inputs",
        Arc::new(MemTable::try_new(batch.schema(), vec![vec![batch]])?),
    )?;
    // Exact path components, not lexical prefixes: pkg-extra is not inside pkg. Omitting an
    // ancestor of a declared input affects that input, including ancestor workspace manifests.
    let plan = session
        .sql(
            r"
        WITH affected AS (
            SELECT o.path FROM revision_inputs o JOIN revision_inputs r
                ON o.path = r.path OR starts_with(o.path, concat(r.path, '/'))
            WHERE o.kind = 'omitted' AND r.kind = 'root'
            UNION
            SELECT o.path FROM revision_inputs o JOIN revision_inputs d
                ON d.path = o.path OR starts_with(d.path, concat(o.path, '/'))
            WHERE o.kind = 'omitted' AND d.kind = 'declared'
        ), failures AS (
            SELECT path FROM affected
            UNION ALL
            SELECT path FROM revision_inputs WHERE kind = 'missing'
        )
        SELECT path, false AS summary, false AS incomplete FROM affected
        UNION ALL
        SELECT CAST(NULL AS VARCHAR) AS path, true AS summary,
            count(*) > 0 AS incomplete FROM failures
    ",
        )
        .await?;
    let output = runtime
        .execute_family(
            plan,
            Some(crate::preparation::QueryFamily::RevisionDisposition),
        )
        .await?;
    let mut affected_omissions = Vec::new();
    let mut source_closure = None;
    for batch in &output.batches {
        let paths = crate::projection::TextColumn::new(batch.column(0).as_ref())?;
        let summary = batch
            .column(1)
            .as_any()
            .downcast_ref::<BooleanArray>()
            .ok_or_else(|| {
                datafusion::error::DataFusionError::Internal(
                    "revision summary field is not Boolean".into(),
                )
            })?;
        let incomplete = batch
            .column(2)
            .as_any()
            .downcast_ref::<BooleanArray>()
            .ok_or_else(|| {
                datafusion::error::DataFusionError::Internal(
                    "revision disposition field is not Boolean".into(),
                )
            })?;
        for row in 0..batch.num_rows() {
            if summary.value(row) {
                if source_closure.is_some() {
                    return Err(datafusion::error::DataFusionError::Internal(
                        "duplicate revision disposition".into(),
                    ));
                }
                source_closure = Some(if incomplete.value(row) {
                    SourceClosure::Incomplete
                } else {
                    SourceClosure::Unknown
                });
            } else {
                affected_omissions.push(
                    paths
                        .get(row)
                        .ok_or_else(|| {
                            datafusion::error::DataFusionError::Internal(
                                "missing affected revision path".into(),
                            )
                        })?
                        .to_owned(),
                );
            }
        }
    }
    affected_omissions.sort();
    Ok(RevisionExtraction {
        archive_sha256: inputs.archive_sha256,
        policy: inputs.policy,
        omissions: inputs.omissions,
        selected_package: inputs.selected_package,
        declared_inputs: inputs.declared_inputs,
        source_roots: inputs.source_roots,
        missing_inputs: inputs.missing_inputs,
        affected_omissions,
        source_closure: source_closure.ok_or_else(|| {
            datafusion::error::DataFusionError::Internal("missing revision disposition".into())
        })?,
    })
}
