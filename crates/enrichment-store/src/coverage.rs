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
            release_id: manifest.release_id.clone(),
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
    let mut params = vec![crate::projection::subject_scalar(&subject)?.into()];
    let compatibility = if symbol_id.is_some() {
        params.push(manifest.release_id.parameter());
        "OR (c.subject.kind = 'library' AND c.subject.library.release_id = $2)"
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
        "SELECT DISTINCT d.evidence_kind AS kind, o.subject, o.source.producer_binding_id
        FROM snapshot.evidence.execution_observations o
        JOIN operation.declarations.execution_payloads d ON o.payload.kind=d.kind
        WHERE o.source.artifact_id IN ({placeholders})"
    );
    evaluate(
        session,
        runtime,
        manifest,
        requested,
        artifacts
            .iter()
            .map(|id| ScalarValue::from(id.as_str()).into())
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
    params: Vec<datafusion::common::metadata::ScalarAndMetadata>,
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
           FROM requested r LEFT JOIN snapshot.evidence.coverage c ON r.kind = c.kind
             AND ((r.subject IS NOT DISTINCT FROM c.subject) {compatibility})
             AND (r.producer_binding_id IS NULL OR r.producer_binding_id = c.producer_binding_id)
           GROUP BY r.kind, r.subject, r.producer_binding_id)
         SELECT kind, subject, CASE rank WHEN 3 THEN 'indexed' WHEN 2 THEN 'partial'
           WHEN 1 THEN 'missing' ELSE 'unknown' END AS state,
           COALESCE(CASE rank WHEN 3 THEN indexed_id WHEN 2 THEN partial_id
             WHEN 1 THEN missing_id ELSE NULL END, '') AS witness_id
         FROM ranked ORDER BY kind, subject, producer_binding_id"
    );
    let plan = session
        .sql(&sql)
        .await?
        .with_param_values(datafusion::common::ParamValues::List(params))?;
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
            snapshot_id: manifest.snapshot_id.clone(),
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
    inputs: datafusion::dataframe::DataFrame,
) -> datafusion::error::Result<enrichment_core::producer::revision::RevisionExtraction> {
    use datafusion::{functions::core::expr_fn::coalesce, prelude::col};
    use enrichment_core::{
        evidence::arrow_model::expressions::literal, native_union::NativeStruct,
        producer::revision::RevisionInputs,
    };
    enrichment_core::native_schema::check_input(
        inputs.schema().as_arrow(),
        &arrow::datatypes::Schema::new(RevisionInputs::fields()),
    )?;
    let session = runtime.session();
    crate::native_catalog::work(&session, "revision_inputs", inputs.into_view())?;
    // Component-aware joins include the repository root and omitted ancestors of declared
    // inputs. Missing/omitted files can establish incompleteness; absence never proves closure.
    let plan = session.sql(r#"
        WITH omissions AS (SELECT unnest(omissions) AS entry FROM revision_inputs),
        roots AS (SELECT unnest(source_roots) AS path FROM revision_inputs),
        declared AS (SELECT unnest(declared_inputs) AS path FROM revision_inputs),
        affected AS (
            SELECT o.entry.path AS path FROM omissions o JOIN roots r
              ON r.path='' OR o.entry.path=r.path OR starts_with(o.entry.path, concat(r.path,'/'))
            UNION
            SELECT o.entry.path AS path FROM omissions o JOIN declared d
              ON d.path=o.entry.path OR starts_with(d.path, concat(o.entry.path,'/'))
        ), summary AS (SELECT array_sort(array_agg(path)) AS affected_omissions, count(*) AS affected_count FROM affected)
        SELECT i.*, s.affected_omissions,
          CASE WHEN s.affected_count>0 OR cardinality(i.missing_inputs)>0 THEN 'incomplete' ELSE 'unknown' END AS source_closure
        FROM revision_inputs i CROSS JOIN summary s
    "#).await?.with_column("affected_omissions", coalesce(vec![col("affected_omissions"), literal(&Vec::<String>::new())?]))?;
    runtime.records(plan, 1).await?.pop().ok_or_else(|| {
        datafusion::common::DataFusionError::Execution(
            "revision capture requires one source record".into(),
        )
    })
}
