//! One native owner for comparison scope, coverage, confounders and disposition.
use crate::{coverage, native_catalog, runtime::QueryRuntime};
use datafusion::common::{DataFusionError, Result};
use enrichment_core::{
    compare::Scope,
    evidence::EvidenceKind,
    identity::ReleaseKey,
    native_union::{NativeStruct, Rule},
    wire::Coverage,
};
use std::collections::BTreeSet;

enrichment_core::native_struct! { struct Requested { scopes: Option<Vec<Scope>> => Rule::Set } }
enrichment_core::native_struct! { pub struct Selection {
    scopes: Vec<Scope> => Rule::Set,
    kinds: Vec<EvidenceKind> => Rule::Set,
} }

pub async fn select(runtime: &QueryRuntime, scopes: Option<Vec<Scope>>) -> Result<Selection> {
    let session = runtime.session();
    native_catalog::input(
        &session,
        "comparison_requested",
        Requested::batch(&[Requested { scopes }])?,
    )?;
    let selected = session.sql("SELECT d.* FROM operation.declarations.comparison_scopes d CROSS JOIN comparison_requested r WHERE r.scopes IS NULL OR array_has(r.scopes,d.scope)").await?;
    native_catalog::work(&session, "comparison_selected", selected.into_view())?;
    runtime.require_empty_with_cause(session.sql("SELECT 'empty_comparison_scope' AS witness FROM comparison_selected HAVING count(*)=0").await?,"comparison_scope","comparison_selection",enrichment_core::wire::DiagnosticCause::InvalidInput).await?;
    runtime.records(session.sql("SELECT coalesce(array_sort(array_agg(scope)),cast([] AS VARCHAR[])) AS scopes, coalesce(array_sort(array_agg(DISTINCT evidence_kind)),cast([] AS VARCHAR[])) AS kinds FROM comparison_selected").await?,1).await?.pop().ok_or_else(|| DataFusionError::Internal("comparison selection missing".into()))
}

enrichment_core::native_struct! { pub struct Inputs {
    before: ReleaseKey => Rule::Text,
    after: ReleaseKey => Rule::Text,
    before_normalizer: String => Rule::Text,
    after_normalizer: String => Rule::Text,
    scopes: Vec<Scope> => Rule::Set,
    confounders: Vec<String> => Rule::Sequence,
} }
enrichment_core::native_struct! { pub struct Assessment {
    same_release: bool => Rule::Text,
    api_complete: bool => Rule::Text,
    scope_complete: bool => Rule::Text,
    comparable: bool => Rule::Text,
    partial: bool => Rule::Text,
    confounders: Vec<String> => Rule::Sequence,
    indexed: BTreeSet<String> => Rule::Set,
    missing: BTreeSet<String> => Rule::Set,
    limitations: Vec<String> => Rule::Sequence,
    incomplete_scopes: Vec<Scope> => Rule::Set,
} }

pub async fn assess(
    runtime: &QueryRuntime,
    inputs: Inputs,
    before: &Coverage,
    after: &Coverage,
) -> Result<Assessment> {
    let session = runtime.session();
    native_catalog::input(&session, "comparison_inputs", Inputs::batch(&[inputs])?)?;
    native_catalog::input(
        &session,
        "before_coverage",
        coverage::Summary::batch(&[coverage::summarize(runtime, &before.assessments).await?])?,
    )?;
    native_catalog::input(
        &session,
        "after_coverage",
        coverage::Summary::batch(&[coverage::summarize(runtime, &after.assessments).await?])?,
    )?;
    runtime.require_empty_with_cause(session.sql("SELECT 'different_package' AS witness FROM comparison_inputs WHERE before.ecosystem<>after.ecosystem OR before.package<>after.package OR before.registry<>after.registry").await?,"comparison_package","comparison_coverage",enrichment_core::wire::DiagnosticCause::InvalidInput).await?;
    let frame=session.sql(r#"
      WITH incomplete AS (
        SELECT array_sort(array_agg(d.scope) FILTER (WHERE
          NOT array_has(b.indexed,d.evidence_kind) OR NOT array_has(a.indexed,d.evidence_kind))) AS scopes
        FROM operation.declarations.comparison_scopes d CROSS JOIN comparison_inputs i
        CROSS JOIN before_coverage b CROSS JOIN after_coverage a WHERE array_has(i.scopes,d.scope)
      ), flags AS (
        SELECT i.*, b.complete AND a.complete AS scope_complete,
          before.version=after.version AS same_release,
          before_normalizer=after_normalizer AND array_has(b.indexed,'public_api')
            AND array_has(a.indexed,'public_api') AND
            (array_has(i.scopes,'api') OR array_has(i.scopes,'relationships')) AS api_complete,
          array_intersect(b.indexed,a.indexed) AS indexed,
          array_union(b.missing,a.missing) AS missing,
          coalesce(c.scopes,cast([] AS VARCHAR[])) AS incomplete_scopes
        FROM comparison_inputs i CROSS JOIN before_coverage b CROSS JOIN after_coverage a CROSS JOIN incomplete c
      ), diagnostics AS (
        SELECT *, array_concat(confounders,
          CASE WHEN NOT scope_complete THEN ['Requested evidence scopes are incomplete; an empty delta does not establish unchanged evidence in those scopes'] ELSE [] END,
          CASE WHEN before_normalizer<>after_normalizer THEN ['Normalizer versions differ; normalization may explain differences'] ELSE [] END,
          CASE WHEN same_release AND (before.artifact_digest IS DISTINCT FROM after.artifact_digest) THEN ['Different artifact variants of the same version were selected'] ELSE [] END
        ) AS reasons FROM flags
      ) SELECT same_release,api_complete,scope_complete,cardinality(reasons)=0 AS comparable,
        cardinality(reasons)>0 OR NOT scope_complete OR (array_has(scopes,'api') AND NOT api_complete) AS partial,
        reasons AS confounders,indexed,
        array_union(missing,CASE WHEN array_has(scopes,'api') AND NOT api_complete THEN ['complete_api_comparison'] ELSE [] END) AS missing,
        array_concat(reasons,
          ['A clean API comparison does not imply unchanged behavior; dependency, runtime and project compatibility require exact-environment verification.'],
          CASE WHEN array_has(scopes,'api') THEN ['Signature deltas compare retained producer representations. Compiler rendering can differ, including Infallible and never-type (!) representations; a rendered difference alone does not establish a breaking source-level change.'] ELSE [] END
        ) AS limitations,incomplete_scopes FROM diagnostics
    "#).await?;
    runtime
        .records(frame, 1)
        .await?
        .pop()
        .ok_or_else(|| DataFusionError::Internal("comparison assessment missing".into()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use enrichment_core::{
        evidence::relational::SubjectRef,
        identity::{Ecosystem, SnapshotId},
        wire::{ScopeAssessment, ScopeState},
    };
    #[tokio::test]
    async fn plan19_comparison_scope_and_disposition_have_one_native_policy() -> Result<()> {
        let root = tempfile::tempdir()?;
        let runtime = QueryRuntime::new(root.path(), Default::default())?;
        let defaults = select(&runtime, None).await?;
        assert_eq!(defaults.scopes.len(), Scope::VALUES.len());
        assert_eq!(defaults.kinds.len(), 5);
        assert!(select(&runtime, Some(vec![])).await.is_err());
        let api = select(
            &runtime,
            Some(vec![Scope::Relationships, Scope::Api, Scope::Api]),
        )
        .await?;
        assert_eq!(api.scopes, vec![Scope::Api, Scope::Relationships]);
        assert_eq!(api.kinds, vec![EvidenceKind::PublicApi]);
        let release = ReleaseKey {
            ecosystem: Ecosystem::Rust,
            registry: "crates.io".into(),
            package: "fixture".into(),
            version: "1.0".into(),
            artifact_digest: Some("a".repeat(64)),
        };
        let mut inputs = Inputs {
            before: release.clone(),
            after: release,
            before_normalizer: "1".into(),
            after_normalizer: "1".into(),
            scopes: api.scopes,
            confounders: vec![],
        };
        let mut before = Coverage::unassessed("before");
        before.assessments.push(ScopeAssessment {
            snapshot_id: SnapshotId::try_from(format!("snap_{}", "a".repeat(64))).unwrap(),
            subject: SubjectRef::Symbol {
                symbol_id: "symbol".into(),
            },
            kind: EvidenceKind::PublicApi,
            state: ScopeState::Indexed,
            witness_id: Some("fact".into()),
        });
        let mut after = before.clone();
        // Derived DTO sets are deliberately stale: assessments are authoritative.
        before.missing.insert("public_api".into());
        let clean = assess(&runtime, inputs.clone(), &before, &after).await?;
        assert!(
            clean.comparable && clean.api_complete && clean.scope_complete && clean.same_release
        );
        assert!(!clean.partial);
        assert!(clean.incomplete_scopes.is_empty());
        after.assessments[0].state = ScopeState::Unknown;
        let gap = assess(&runtime, inputs.clone(), &before, &after).await?;
        assert!(gap.partial && !gap.api_complete && !gap.scope_complete && !gap.comparable);
        assert_eq!(
            gap.incomplete_scopes,
            vec![Scope::Api, Scope::Relationships]
        );
        assert_eq!(
            gap.missing,
            BTreeSet::from(["public_api".into(), "complete_api_comparison".into()])
        );
        after.assessments[0].state = ScopeState::Indexed;
        inputs.after_normalizer = "2".into();
        inputs.after.artifact_digest = None;
        let confounded = assess(&runtime, inputs.clone(), &before, &after).await?;
        assert!(confounded.partial && confounded.scope_complete && !confounded.api_complete);
        assert_eq!(confounded.confounders.len(), 2);
        inputs.after.package = "different".into();
        assert!(assess(&runtime, inputs, &before, &after).await.is_err());
        Ok(())
    }
}
