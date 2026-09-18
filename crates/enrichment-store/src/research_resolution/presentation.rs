//! One native presentation policy for Rust hosted, revision and Python acquisition.
use crate::{native_catalog, runtime::QueryRuntime};
use datafusion::{common::Result, functions::core::expr_ext::FieldAccessor, prelude::*};
use enrichment_core::{
    evidence::{
        Artifact, EvidenceKind, Gap,
        arrow_model::expressions::{literal, record},
    },
    identity::{Context, Environment, Release},
    native_union::{Cell, NativeStruct, Rule},
    registry::UpstreamCheck,
    wire::{
        ArtifactHandle, Coverage,
        data::{HostedJsonReport, ResolveData},
    },
};

enrichment_core::native_struct! { pub struct LocalCompiler {
    toolchain:String => Rule::NonEmpty,
    identity:String => Rule::NonEmpty,
} }
enrichment_core::native_struct! { struct Scope {
    release:Release => Rule::Text,
    environment:Environment => Rule::Text,
    context:Context => Rule::Text,
    upstream:Option<UpstreamCheck> => Rule::Text,
    hosted:Option<HostedJsonReport> => Rule::Text,
    filename:Option<String> => Rule::Text,
    local:Option<LocalCompiler> => Rule::Text,
    indexed:Vec<EvidenceKind> => Rule::Set,
    gaps:Vec<Gap> => Rule::Sequence,
} }
enrichment_core::native_struct! { struct Receipt {
    ordinal:usize => Rule::Coordinate(enrichment_core::native_union::Unit::Ordinal),
    artifact:Artifact => Rule::Text,
} }
enrichment_core::native_struct! { pub struct Presentation {
    summary:String => Rule::Text,
    coverage:Coverage => Rule::Text,
    artifacts:Vec<ArtifactHandle> => Rule::Sequence,
    partial:bool => Rule::Text,
} }

/// This selects the prepublication template. `result_plan` later binds its exact manifest,
/// requested-scope coverage and terminal disposition atomically with the publication.
pub async fn acquisition_presentation(
    runtime: &QueryRuntime,
    data: &ResolveData,
    indexed: &[EvidenceKind],
    filename: Option<&str>,
    local: Option<LocalCompiler>,
) -> Result<Presentation> {
    if data.artifacts.len() > 8192 || data.gaps.len() > 4096 {
        return datafusion::common::resources_err!("acquisition presentation input bound");
    }
    let session = runtime.session();
    native_catalog::input(
        &session,
        "acquisition_scope",
        Scope::batch(&[Scope {
            release: data.release.clone(),
            environment: data.environment.clone(),
            context: data.context.clone(),
            upstream: data.upstream.clone(),
            hosted: data.hosted_rustdoc_json.clone(),
            filename: filename.map(str::to_owned),
            local,
            indexed: indexed.to_vec(),
            gaps: data.gaps.clone(),
        }])?,
    )?;
    native_catalog::input(
        &session,
        "acquisition_receipts",
        Receipt::batch(
            &data
                .artifacts
                .iter()
                .cloned()
                .enumerate()
                .map(|(ordinal, artifact)| Receipt { ordinal, artifact })
                .collect::<Vec<_>>(),
        )?,
    )?;
    runtime
        .require_empty(
            session
                .sql(
                    r#"
      SELECT artifact.artifact_id AS witness FROM acquisition_receipts
        WHERE NOT regexp_like(artifact.sha256,'^[0-9a-f]{64}$')
           OR artifact.artifact_id<>concat('art_',artifact.sha256)
      UNION ALL SELECT 'python_distribution_name' FROM acquisition_scope
        WHERE release.key.ecosystem='python' AND (filename IS NULL OR filename='')
    "#,
                )
                .await?,
            "acquisition_presentation_inputs",
            "resolution",
        )
        .await?;
    let frame=session.sql(r#"
      WITH selected AS (SELECT *,release.key.ecosystem='python' AS python,
        release.key.ecosystem='rust' AND context.mode='revision' AS rust_revision,
        release.key.package AS package,release.key.version AS version FROM acquisition_scope)
      SELECT *,
        CASE WHEN python THEN concat(package,' ',version,': static distribution evidence')
          WHEN rust_revision THEN concat(package,' at ',version,': source and declarations; compiled API unobserved')
          ELSE concat('Resolved ',package,' ',version,' on crates.io',
            CASE WHEN upstream.newest_stable IS NOT NULL AND NOT upstream.resolved_is_newest_stable
              THEN concat('; the newest stable release is ',upstream.newest_stable) ELSE '' END,'; ',
            CASE hosted.state WHEN 'available' THEN 'hosted rustdoc JSON was stored; normalized publication is pending'
              WHEN 'missing' THEN 'docs.rs has no rustdoc JSON for this release'
              WHEN 'unsupported' THEN 'docs.rs rustdoc JSON is in a format this build cannot read'
              ELSE 'hosted rustdoc JSON could not be checked' END,'.') END AS summary,
        CASE WHEN python THEN concat('Static contents of ',package,' ',version,' (',filename,')')
          WHEN rust_revision THEN concat('Immutable repository source ',package,' ',version)
          ELSE concat('release identity, registry metadata, crate source, hosted documentation and the normalized public API of ',package,' ',version) END AS scope,
        array_sort(array_distinct(indexed)) AS indexed_kinds,
        CASE WHEN rust_revision THEN ['public_api']
          ELSE array_sort(array_distinct(array_transform(gaps,g -> get_field(g,'kind')))) END AS missing_kinds,
        CASE WHEN python THEN ['Static source/stub declarations are not executed or typechecker observations; dependencies and namespace contributions are not complete environments.']
          WHEN rust_revision THEN array_transform(gaps,g -> get_field(g,'detail'))
          ELSE array_concat([
            'Public API facts describe declarations in the selected rustdoc build. External trait definitions and their inherited method details are not expanded.',
            'Hosted documentation reflects the maintainer''s docs.rs build configuration (observed_configuration), not the calling project''s features or target.'
          ],CASE WHEN environment.resolution='unspecified' THEN [
            'No project environment was declared; availability claims are about the documented build only.'
          ] ELSE CAST([] AS VARCHAR[]) END,
          CASE WHEN local IS NOT NULL THEN [concat('This API was compiled locally by ',local.toolchain,' (',
            replace(trim(local.identity),chr(10),'; '),'), not downloaded from docs.rs. A successful nightly build is not evidence that this crate compiles on the project''s stable compiler; use verify_usage for that.')]
          ELSE CAST([] AS VARCHAR[]) END) END AS limitations,
        rust_revision OR cardinality(gaps)>0 OR (NOT python AND coalesce(hosted.state,'not_attempted')<>'available') AS partial
      FROM selected
    "#).await?;
    native_catalog::work(&session, "acquisition_selected", frame.into_view())?;
    let receipts=session.sql(r#"
      SELECT r.ordinal,r.artifact,
        concat('library-evidence://artifacts/',r.artifact.artifact_id) AS uri,
        CASE WHEN s.python THEN 'Distribution evidence' WHEN s.rust_revision THEN 'Revision source evidence'
          ELSE concat(r.artifact.kind,' for ',s.package,' ',s.version) END AS description
      FROM acquisition_receipts r CROSS JOIN acquisition_selected s
    "#).await?;
    let handle = record(
        &ArtifactHandle::data_type(),
        &[
            ("receipt", col("artifact")),
            ("uri", col("uri")),
            ("description", col("description")),
        ],
    )?;
    native_catalog::work(
        &session,
        "acquisition_handles",
        receipts
            .select(vec![col("ordinal"), handle.alias("handle")])?
            .into_view(),
    )?;
    let handles = session
        .sql("SELECT array_agg(handle ORDER BY ordinal) AS artifacts FROM acquisition_handles")
        .await?
        .with_column(
            "artifacts",
            datafusion::functions::core::expr_fn::coalesce(vec![
                col("artifacts"),
                literal(&Vec::<ArtifactHandle>::new())?,
            ]),
        )?;
    native_catalog::work(&session, "acquisition_artifacts", handles.into_view())?;
    let frame = session
        .sql("SELECT * FROM acquisition_selected CROSS JOIN acquisition_artifacts")
        .await?;
    let default = literal(&Coverage::unassessed(""))?;
    let coverage = record(
        &Coverage::data_type(),
        &[
            ("details", default.clone().field("details")),
            ("assessments", default.field("assessments")),
            ("scope", col("scope")),
            ("indexed", col("indexed_kinds")),
            ("missing", col("missing_kinds")),
            ("limitations", col("limitations")),
        ],
    )?;
    runtime
        .records(
            frame.select(vec![
                col("summary"),
                coverage.alias("coverage"),
                col("artifacts"),
                col("partial"),
            ])?,
            1,
        )
        .await?
        .pop()
        .ok_or_else(|| datafusion::common::exec_datafusion_err!("acquisition presentation missing"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use enrichment_core::{
        evidence::{ArtifactKind, GapReason},
        identity::{Ecosystem, ReleaseKey, ResearchMode},
        native_time::AcquisitionTime,
        wire::data::HostedJsonState,
    };
    #[tokio::test]
    async fn native_acquisition_presentation_keeps_source_scope_receipts_and_gaps() -> Result<()> {
        let root = tempfile::tempdir()?;
        let runtime = QueryRuntime::new(root.path(), Default::default())?;
        let release = Release::new(ReleaseKey {
            ecosystem: Ecosystem::Rust,
            registry: "crates.io".into(),
            package: "sample".into(),
            version: "1".into(),
            artifact_digest: None,
        });
        let environment = Environment::unspecified();
        let context = Context::new(
            release.release_id.clone(),
            environment.environment_id.clone(),
            ResearchMode::Upstream,
        );
        let artifact = Artifact::describe(
            b"source",
            ArtifactKind::CrateTarball,
            "application/gzip",
            "source://archive",
            AcquisitionTime::from_micros(1)?,
        );
        let mut later = artifact.clone();
        later.retrieved_at = AcquisitionTime::from_micros(2)?;
        let mut data = ResolveData {
            release,
            environment,
            context,
            upstream: None,
            observed_configuration: None,
            hosted_rustdoc_json: Some(HostedJsonReport {
                state: HostedJsonState::Available,
                format_version: Some(61),
                supported_formats: vec![61],
                target: "target".into(),
                url: "source://hosted".into(),
                declared_crate_version: Some("1".into()),
            }),
            python: None,
            snapshot: None,
            artifacts: vec![artifact.clone(), later],
            gaps: vec![],
            producer_runs: vec![],
            answered_from_cache: false,
        };
        let selected =
            acquisition_presentation(&runtime, &data, &[EvidenceKind::CrateSource], None, None)
                .await?;
        assert!(!selected.partial);
        assert_eq!(
            selected.artifacts.len(),
            2,
            "acquisition records preserve order and clocks"
        );
        assert_eq!(selected.artifacts[0].receipt, artifact);
        assert_eq!(
            selected.artifacts[1].receipt.retrieved_at,
            AcquisitionTime::from_micros(2)?
        );
        assert_eq!(
            selected.artifacts[0].uri.as_str(),
            format!("library-evidence://artifacts/{}", artifact.artifact_id)
        );
        assert_eq!(
            selected.coverage.indexed,
            std::collections::BTreeSet::from(["crate_source".into()])
        );
        assert!(selected.coverage.missing.is_empty());
        assert_eq!(selected.coverage.limitations.len(), 3);
        assert!(
            selected
                .summary
                .contains("normalized publication is pending")
        );
        let selected = acquisition_presentation(
            &runtime,
            &data,
            &[],
            None,
            Some(LocalCompiler {
                toolchain: "dated-nightly".into(),
                identity: "rustc 1\nhost: target\n".into(),
            }),
        )
        .await?;
        assert!(selected.coverage.limitations[3].contains("rustc 1; host: target"));
        assert!(
            selected.coverage.limitations[3]
                .contains("not evidence that this crate compiles on the project's stable compiler")
        );
        data.gaps = vec![Gap {
            kind: EvidenceKind::PublicApi,
            reason: GapReason::NotAttempted,
            detail: "compiled API unobserved".into(),
            planned_fallback: None,
        }];
        data.context.mode = ResearchMode::Revision;
        let revision = acquisition_presentation(&runtime, &data, &[], None, None).await?;
        assert!(revision.partial);
        assert_eq!(
            revision.coverage.limitations,
            vec!["compiled API unobserved"]
        );
        assert_eq!(
            revision.coverage.missing,
            std::collections::BTreeSet::from(["public_api".into()])
        );
        assert!(
            revision
                .summary
                .contains("source and declarations; compiled API unobserved")
        );
        data.release.key.ecosystem = Ecosystem::Python;
        data.gaps.clear();
        let python =
            acquisition_presentation(&runtime, &data, &[], Some("source.whl"), None).await?;
        assert!(!python.partial);
        assert!(python.coverage.scope.ends_with("(source.whl)"));
        assert_eq!(python.coverage.limitations.len(), 1);
        assert!(
            acquisition_presentation(&runtime, &data, &[], None, None)
                .await
                .is_err()
        );
        data.artifacts.clear();
        let empty =
            acquisition_presentation(&runtime, &data, &[], Some("source.whl"), None).await?;
        assert!(empty.artifacts.is_empty());
        let mut damaged = artifact;
        damaged.artifact_id = "unscoped".into();
        data.artifacts = vec![damaged];
        assert!(
            acquisition_presentation(&runtime, &data, &[], Some("source.whl"), None)
                .await
                .is_err()
        );
        runtime.close_diagnostics().await
    }
}
