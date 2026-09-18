//! One requested-scope relation shared by every research operation.
use crate::{SnapshotReader, query::QueryError};
use datafusion::prelude::col;
use datafusion::{
    dataframe::DataFrame,
    prelude::{SessionContext, lit},
};
use enrichment_core::native_union::{NativeStruct, Rule};
use enrichment_core::{
    evidence::{EvidenceKind, relational::SubjectRef},
    wire::{Coverage, ScopeAssessment},
};
use std::collections::BTreeSet;

impl SnapshotReader {
    /// Assess selected retained execution results, including their producer qualification.
    pub async fn assess_execution(&self, artifacts: &[String]) -> Result<Coverage, QueryError> {
        assess_execution(self.session(), self.runtime(), self.manifest(), artifacts).await
    }

    /// Acquisition, cached resolution and manifest resources evaluate the same domain.
    pub async fn assess_acquisition(&self) -> Result<Coverage, QueryError> {
        let kinds = acquisition_kinds(self.runtime(), self.manifest()).await?;
        assess(
            self.session(),
            self.runtime(),
            self.manifest(),
            &kinds,
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

enrichment_core::native_struct! { struct AcquisitionScope {
    indexed: Vec<EvidenceKind> => Rule::Sequence,
    missing: Vec<EvidenceKind> => Rule::Sequence,
} }
enrichment_core::native_struct! { struct RequestedKinds { kinds: Vec<EvidenceKind> => Rule::Sequence } }
enrichment_core::native_struct! { struct EvidenceRequest {
    kinds: Vec<EvidenceKind> => Rule::SequenceBounds { min: 1, max: 1024 },
    symbol: Option<String> => Rule::Text,
    release: enrichment_core::identity::ReleaseId => Rule::Text,
} }
enrichment_core::native_struct! { struct ExecutionRequest {
    artifacts: Vec<String> => Rule::SequenceBounds { min: 1, max: 1024 },
} }

pub(crate) async fn acquisition_kinds(
    runtime: &crate::runtime::QueryRuntime,
    manifest: &enrichment_core::evidence::snapshot::EvidenceManifest,
) -> datafusion::common::Result<Vec<EvidenceKind>> {
    let session = runtime.session();
    crate::native_catalog::input(
        &session,
        "acquisition_scope",
        AcquisitionScope::batch(&[AcquisitionScope {
            indexed: manifest.indexed.clone(),
            missing: manifest.missing.clone(),
        }])?,
    )?;
    select_acquisition_kinds(runtime, &session).await
}
async fn select_acquisition_kinds(
    runtime: &crate::runtime::QueryRuntime,
    session: &SessionContext,
) -> datafusion::common::Result<Vec<EvidenceKind>> {
    let plan = session.sql("WITH requested AS (SELECT array_sort(array_distinct(array_concat(indexed,missing))) AS kinds FROM acquisition_scope) SELECT CASE WHEN cardinality(kinds)=0 THEN ['registry_metadata'] ELSE kinds END AS kinds FROM requested").await?;
    Ok(runtime
        .records::<RequestedKinds>(plan, 1)
        .await?
        .pop()
        .ok_or_else(|| datafusion::common::internal_datafusion_err!("acquisition scope missing"))?
        .kinds)
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
    let session = runtime.combined_session(&[session])?;
    crate::native_catalog::input(
        &session,
        "coverage_scope",
        EvidenceRequest::batch(&[EvidenceRequest {
            kinds: kinds.to_vec(),
            symbol: symbol_id.map(str::to_owned),
            release: manifest.release_id.clone(),
        }])?,
    )?;
    let requested = evidence_requests(runtime, &session).await?;
    evaluate(&session, runtime, &manifest.snapshot_id, requested, scope).await
}

async fn evidence_requests(
    runtime: &crate::runtime::QueryRuntime,
    session: &SessionContext,
) -> datafusion::common::Result<DataFrame> {
    use enrichment_core::{evidence::arrow_model::expressions::variant, native_union::Cell};
    runtime.require_empty(session.sql("SELECT 'coverage_kinds' AS witness FROM coverage_scope WHERE cardinality(kinds)<1 OR cardinality(kinds)>1024 OR symbol=''").await?, "coverage_scope", "coverage_assessment").await?;
    let frame = session
        .sql("SELECT DISTINCT unnest(kinds) AS kind,symbol,release FROM coverage_scope")
        .await?;
    let subject = datafusion::logical_expr::when(
        col("symbol").is_null(),
        variant(
            &SubjectRef::data_type(),
            "library",
            &[("release_id", col("release"))],
        )?,
    )
    .otherwise(variant(
        &SubjectRef::data_type(),
        "symbol",
        &[("symbol_id", col("symbol"))],
    )?)?;
    frame.select(vec![
        col("kind"),
        subject.alias("subject"),
        lit(datafusion::common::ScalarValue::Utf8(None)).alias("producer_binding_id"),
        col("release").alias("library_release"),
        col("symbol").is_not_null().alias("allow_library"),
    ])
}

/// Scope execution publication to its exact retained result identities and producer bindings.
/// A successful unrelated query on the same subject cannot establish this operation's coverage.
pub(crate) async fn assess_execution(
    session: &datafusion::prelude::SessionContext,
    runtime: &crate::runtime::QueryRuntime,
    manifest: &enrichment_core::evidence::snapshot::EvidenceManifest,
    artifacts: &[String],
) -> Result<Coverage, QueryError> {
    let session = runtime.combined_session(&[session])?;
    crate::native_catalog::input(
        &session,
        "execution_scope",
        ExecutionRequest::batch(&[ExecutionRequest {
            artifacts: artifacts.to_vec(),
        }])?,
    )?;
    let candidates = session.sql("SELECT o.source.artifact_id AS artifact_id,d.evidence_kind AS kind,o.subject,o.source.producer_binding_id FROM snapshot.evidence.execution_observations o JOIN operation.declarations.execution_payloads d ON o.payload.kind=d.kind").await?;
    crate::native_catalog::work(&session, "execution_candidates", candidates.into_view())?;
    let requested = execution_requests(runtime, &session).await?;
    evaluate(
        &session,
        runtime,
        &manifest.snapshot_id,
        requested,
        "exact retained execution results and their producing bindings".into(),
    )
    .await
}

async fn execution_requests(
    runtime: &crate::runtime::QueryRuntime,
    session: &SessionContext,
) -> datafusion::common::Result<DataFrame> {
    runtime.require_empty(session.sql("SELECT 'execution_scope' AS witness FROM execution_scope WHERE cardinality(artifacts)<1 OR cardinality(artifacts)>1024").await?, "execution_scope", "coverage_assessment").await?;
    let requested = session
        .sql("SELECT DISTINCT unnest(artifacts) AS artifact_id FROM execution_scope")
        .await?;
    crate::native_catalog::work(session, "requested_execution", requested.into_view())?;
    runtime.require_empty(session.sql("SELECT r.artifact_id AS witness FROM requested_execution r LEFT ANTI JOIN execution_candidates c ON r.artifact_id=c.artifact_id").await?, "execution_coverage_reference", "coverage_assessment").await?;
    session.sql("SELECT DISTINCT c.kind,c.subject,c.producer_binding_id FROM requested_execution r JOIN execution_candidates c ON r.artifact_id=c.artifact_id").await?
        .with_column("library_release", enrichment_core::evidence::arrow_model::expressions::literal(&Option::<enrichment_core::identity::ReleaseId>::None)?)?
        .with_column("allow_library", lit(false))
}

async fn evaluate(
    session: &datafusion::prelude::SessionContext,
    runtime: &crate::runtime::QueryRuntime,
    snapshot: &enrichment_core::identity::SnapshotId,
    requested: DataFrame,
    scope: String,
) -> Result<Coverage, QueryError> {
    crate::native_catalog::work(session, "coverage_requested", requested.into_view())?;
    let sql = "WITH ranked AS (
           SELECT r.kind, r.subject, r.producer_binding_id,
             COALESCE(MAX(CASE c.outcome WHEN 'indexed' THEN 3 WHEN 'partial' THEN 2
               WHEN 'missing' THEN 1 ELSE 0 END), 0) AS rank,
             MIN(CASE WHEN c.outcome = 'indexed' THEN c.coverage_id END) AS indexed_id,
             MIN(CASE WHEN c.outcome = 'partial' THEN c.coverage_id END) AS partial_id,
             MIN(CASE WHEN c.outcome = 'missing' THEN c.coverage_id END) AS missing_id
           FROM coverage_requested r LEFT JOIN snapshot.evidence.coverage c ON r.kind = c.kind
             AND ((r.subject IS NOT DISTINCT FROM c.subject) OR (r.allow_library AND c.subject.kind='library' AND c.subject.library.release_id=r.library_release))
             AND (r.producer_binding_id IS NULL OR r.producer_binding_id = c.producer_binding_id)
           GROUP BY r.kind, r.subject, r.producer_binding_id)
         SELECT kind, subject, CASE rank WHEN 3 THEN 'indexed' WHEN 2 THEN 'partial'
           WHEN 1 THEN 'missing' ELSE 'unknown' END AS state,
           CASE rank WHEN 3 THEN indexed_id WHEN 2 THEN partial_id
             WHEN 1 THEN missing_id ELSE NULL END AS witness_id
         FROM ranked ORDER BY kind, subject, producer_binding_id";
    let plan = session.sql(sql).await?;
    let plan = plan
        .with_column(
            "snapshot_id",
            enrichment_core::evidence::arrow_model::expressions::literal(snapshot)?,
        )?
        .select(
            ScopeAssessment::fields()
                .iter()
                .map(|field| col(field.name()))
                .collect::<Vec<_>>(),
        )?;
    let assessments = runtime.records::<ScopeAssessment>(plan, 1024).await?;
    let summary = summarize(runtime, &assessments).await?;
    Ok(Coverage {
        details: None,
        assessments,
        scope,
        indexed: summary.indexed,
        missing: summary.missing,
        limitations: summary.limitations,
    })
}

enrichment_core::native_struct! {
    /// Derived only from requested assessments, never from the existence of payload rows.
    pub struct Summary {
        complete: bool => Rule::Text,
        indexed: BTreeSet<String> => Rule::Set,
        missing: BTreeSet<String> => Rule::Set,
        limitations: Vec<String> => Rule::Sequence,
    }
}

/// A kind is indexed only when every requested subject/producer scope for it is indexed.
/// Empty requested evidence is unassessed, not complete. Unknown remains distinct from missing.
pub async fn summarize(
    runtime: &crate::runtime::QueryRuntime,
    assessments: &[ScopeAssessment],
) -> datafusion::common::Result<Summary> {
    use datafusion::functions::core::expr_fn::coalesce;
    use enrichment_core::evidence::arrow_model::expressions::literal;
    let session = runtime.session();
    crate::native_catalog::input(
        &session,
        "requested_assessments",
        ScopeAssessment::batch(assessments)?,
    )?;
    let frame = session
        .sql(
            r#"
        WITH kinds AS (
          SELECT kind, bool_and((state='indexed')) AS complete
          FROM requested_assessments GROUP BY kind
        ), summary AS (
          SELECT count(*)>0 AND count(*) FILTER (WHERE complete IS NOT TRUE)=0 AS complete,
            array_sort(array_agg(kind) FILTER (WHERE complete)) AS indexed,
            array_sort(array_agg(kind) FILTER (WHERE NOT complete)) AS missing
          FROM kinds
        ), limitations AS (
          SELECT array_sort(array_agg(DISTINCT concat(kind, ': ',state,' within requested scope'))
            FILTER (WHERE state<>'indexed')) AS limitations FROM requested_assessments
        ) SELECT * FROM summary CROSS JOIN limitations
    "#,
        )
        .await?;
    let frame =
        ["indexed", "missing", "limitations"]
            .into_iter()
            .try_fold(frame, |frame, name| {
                frame.with_column(
                    name,
                    coalesce(vec![col(name), literal(&Vec::<String>::new())?]),
                )
            })?;
    runtime.records(frame, 1).await?.pop().ok_or_else(|| {
        datafusion::common::DataFusionError::Internal("native coverage summary missing".into())
    })
}

/// Update only derived sets; caller-owned scope text and additional limitations are retained.
pub async fn refresh(
    runtime: &crate::runtime::QueryRuntime,
    coverage: &mut Coverage,
) -> datafusion::common::Result<()> {
    let summary = summarize(runtime, &coverage.assessments).await?;
    coverage.indexed = summary.indexed;
    coverage.missing = summary.missing;
    Ok(())
}

pub async fn complete(
    runtime: &crate::runtime::QueryRuntime,
    coverage: &Coverage,
) -> datafusion::common::Result<bool> {
    Ok(summarize(runtime, &coverage.assessments).await?.complete)
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

#[cfg(test)]
mod native_tests {
    use super::*;
    use enrichment_core::wire::ScopeState;

    enrichment_core::native_struct! { struct ExecutionCandidate {
        artifact_id: String => Rule::NonEmpty,
        kind: EvidenceKind => Rule::Text,
        subject: SubjectRef => Rule::Text,
        producer_binding_id: String => Rule::NonEmpty,
    } }

    #[tokio::test]
    async fn native_coverage_scope_preserves_all_requested_artifacts_and_qualifications()
    -> datafusion::common::Result<()> {
        use crate::native_catalog::{self, BindingKind, BoundCatalog};
        use enrichment_core::{
            evidence::relational::{CoverageFact, CoverageOutcome},
            identity::{ReleaseId, SnapshotId},
        };
        use std::{collections::BTreeMap, sync::Arc};
        let root = tempfile::tempdir()?;
        let runtime = crate::runtime::QueryRuntime::new(root.path(), Default::default())?;
        let release = ReleaseId::try_from(format!("rel_{}", "a".repeat(64))).unwrap();
        let snapshot = SnapshotId::try_from(format!("snap_{}", "b".repeat(64))).unwrap();
        let subject = SubjectRef::Symbol {
            symbol_id: "requested".into(),
        };
        let coverage = vec![
            CoverageFact {
                coverage_id: "library-success".into(),
                producer_binding_id: "build".into(),
                subject: SubjectRef::Library {
                    release_id: release.clone(),
                },
                kind: EvidenceKind::PublicApi,
                outcome: CoverageOutcome::Indexed,
                gaps: vec![],
            },
            CoverageFact {
                coverage_id: "requested-missing".into(),
                producer_binding_id: "wanted".into(),
                subject: subject.clone(),
                kind: EvidenceKind::UsageProbes,
                outcome: CoverageOutcome::Missing,
                gaps: vec![],
            },
            CoverageFact {
                coverage_id: "unrelated-success".into(),
                producer_binding_id: "different".into(),
                subject: subject.clone(),
                kind: EvidenceKind::UsageProbes,
                outcome: CoverageOutcome::Indexed,
                gaps: vec![],
            },
        ];
        let provider = native_catalog::batch(
            &runtime.session(),
            "coverage_fixture",
            CoverageFact::batch(&coverage)?,
        )?
        .into_view();
        let catalog = Arc::new(BoundCatalog::default().with_schema(
            BindingKind::AdmittedEvidence,
            BTreeMap::from([("coverage".into(), provider)]),
        ));
        let session = || {
            runtime.bound_session(BTreeMap::from([(
                "snapshot".into(),
                catalog.clone() as Arc<dyn datafusion::catalog::CatalogProvider>,
            )]))
        };
        for (indexed, missing, expected) in [
            (vec![], vec![], vec![EvidenceKind::RegistryMetadata]),
            (
                vec![EvidenceKind::PublicApi; 2],
                vec![EvidenceKind::Documentation, EvidenceKind::PublicApi],
                vec![EvidenceKind::Documentation, EvidenceKind::PublicApi],
            ),
        ] {
            let session = session()?;
            native_catalog::input(
                &session,
                "acquisition_scope",
                AcquisitionScope::batch(&[AcquisitionScope { indexed, missing }])?,
            )?;
            assert_eq!(
                select_acquisition_kinds(&runtime, &session).await?,
                expected
            );
        }
        let normal = session()?;
        native_catalog::input(
            &normal,
            "coverage_scope",
            EvidenceRequest::batch(&[EvidenceRequest {
                kinds: vec![
                    EvidenceKind::PublicApi,
                    EvidenceKind::PublicApi,
                    EvidenceKind::Documentation,
                ],
                symbol: Some("requested".into()),
                release: release.clone(),
            }])?,
        )?;
        let requested = evidence_requests(&runtime, &normal).await?;
        let result = evaluate(&normal, &runtime, &snapshot, requested, "fixture".into())
            .await
            .unwrap();
        assert_eq!(result.assessments.len(), 2);
        assert_eq!(result.indexed, BTreeSet::from(["public_api".into()]));
        assert_eq!(
            result
                .assessments
                .iter()
                .find(|a| a.kind == EvidenceKind::Documentation)
                .unwrap()
                .state,
            ScopeState::Unknown
        );
        let candidate = ExecutionCandidate {
            artifact_id: "artifact-selected".into(),
            kind: EvidenceKind::UsageProbes,
            subject,
            producer_binding_id: "wanted".into(),
        };
        for (artifacts, valid) in [
            (vec!["artifact-selected".into(); 2], true),
            (vec!["artifact-selected".into(), "unmatched".into()], false),
            (vec![], false),
            (vec!["artifact-selected".into(); 1025], false),
        ] {
            let session = session()?;
            native_catalog::input(
                &session,
                "execution_scope",
                ExecutionRequest::batch(&[ExecutionRequest { artifacts }])?,
            )?;
            native_catalog::input(
                &session,
                "execution_candidates",
                ExecutionCandidate::batch(std::slice::from_ref(&candidate))?,
            )?;
            let requested = execution_requests(&runtime, &session).await;
            assert_eq!(requested.is_ok(), valid);
            if let Ok(requested) = requested {
                let result = evaluate(&session, &runtime, &snapshot, requested, "fixture".into())
                    .await
                    .unwrap();
                assert_eq!(result.assessments.len(), 1);
                assert_eq!(result.assessments[0].state, ScopeState::Missing);
                assert_eq!(
                    result.assessments[0].witness_id.as_deref(),
                    Some("requested-missing")
                );
                assert!(result.indexed.is_empty());
            }
        }
        runtime.close_diagnostics().await?;
        Ok(())
    }

    #[tokio::test]
    async fn plan19_coverage_requires_every_requested_subject_and_preserves_unknowns()
    -> datafusion::common::Result<()> {
        let root = tempfile::tempdir()?;
        let runtime = crate::runtime::QueryRuntime::new(root.path(), Default::default())?;
        let first = ScopeAssessment {
            snapshot_id: enrichment_core::identity::SnapshotId::try_from(format!(
                "snap_{}",
                "a".repeat(64)
            ))
            .unwrap(),
            subject: SubjectRef::Symbol {
                symbol_id: "first".into(),
            },
            kind: EvidenceKind::PublicApi,
            state: ScopeState::Indexed,
            witness_id: Some("successful fact".into()),
        };
        let empty = summarize(&runtime, &[]).await?;
        assert!(!empty.complete);
        assert!(empty.indexed.is_empty() && empty.missing.is_empty());
        let indexed = summarize(&runtime, std::slice::from_ref(&first)).await?;
        assert!(indexed.complete);
        assert_eq!(indexed.indexed, BTreeSet::from(["public_api".into()]));
        for state in [
            ScopeState::Partial,
            ScopeState::Missing,
            ScopeState::Unknown,
        ] {
            let mut other = first.clone();
            other.subject = SubjectRef::Symbol {
                symbol_id: "second".into(),
            };
            other.state = state;
            other.witness_id = None;
            let result = summarize(&runtime, &[first.clone(), other.clone()]).await?;
            assert!(!result.complete);
            assert!(result.indexed.is_empty());
            assert_eq!(result.missing, BTreeSet::from(["public_api".into()]));
            assert_eq!(
                result.limitations,
                vec![format!(
                    "public_api: {} within requested scope",
                    state.as_str()
                )]
            );
            other.kind = EvidenceKind::Documentation;
            let result = summarize(&runtime, &[first.clone(), other]).await?;
            assert_eq!(result.indexed, BTreeSet::from(["public_api".into()]));
            assert_eq!(result.missing, BTreeSet::from(["documentation".into()]));
        }
        let mut coverage = Coverage::unassessed("scope is preserved");
        coverage.assessments.push(first);
        coverage.limitations.push("additional qualification".into());
        coverage.missing.insert("stale derived value".into());
        refresh(&runtime, &mut coverage).await?;
        assert!(coverage.missing.is_empty());
        assert_eq!(coverage.scope, "scope is preserved");
        assert_eq!(coverage.limitations, vec!["additional qualification"]);
        assert!(complete(&runtime, &coverage).await?);
        Ok(())
    }
}
