#[path = "support/native_ingest.rs"]
pub mod native_ingest;
#[path = "support/producer_records.rs"]
mod producer_records;
use enrichment_core::{
    evidence::{
        Artifact, ArtifactKind, EvidenceKind, FragmentKind,
        document::DocumentFact,
        ingest::{DocumentBatch, IngestContext},
        relational::Locator,
    },
    identity::Ecosystem,
    policy::ExecutionProfile,
    producer::{ProducerRun, RunOutcome},
    wire::{EvidenceClass, SourceVersionMatch},
};
use native_ingest::normalize;
fn context(
    artifact: Artifact,
    ecosystem: Ecosystem,
    owner: &str,
    producer: &str,
    role: &str,
) -> IngestContext {
    IngestContext {
        ecosystem,
        symbol_package: owner.into(),
        release_id: "rel_fixture".into(),
        environment_id: "env_fixture".into(),
        source_version_match: SourceVersionMatch::Exact,
        producing_attempt: "attempt-fixture".into(),
        producer_runs: vec![ProducerRun {
            attempt_id: "attempt-fixture".into(),
            producer: producer.into(),
            producer_version: "2".into(),
            config_digest: enrichment_core::canonical::sha256_hex(b"configuration"),
            inputs: [(role.into(), artifact.sha256.clone())]
                .into_iter()
                .collect(),
            profile: ExecutionProfile::Static,
            started_at: enrichment_core::native_time::ObservationTime::try_from(
                "2026-09-14T00:00:00.000000Z".to_owned(),
            )
            .unwrap(),
            finished_at: enrichment_core::native_time::ObservationTime::try_from(
                "2026-09-14T00:00:01.000000Z".to_owned(),
            )
            .unwrap(),
            outcome: RunOutcome::Succeeded,
            gaps: vec![],
            log: None,
        }],
        artifacts: vec![artifact],
        indexed: vec![EvidenceKind::PublicApi, EvidenceKind::Documentation],
        missing: vec![],
        gaps: vec![],
    }
}

#[test]
fn checked_in_rustdoc_normalizes_shared_definitions_and_qualified_members() {
    let payload = include_str!("../../../tests/fixtures/rustdoc/enr-fixture-0.1.0-default.json");
    let artifact = Artifact::describe(
        payload.as_bytes(),
        ArtifactKind::RustdocJson,
        "application/json",
        "https://docs.rs/enr-fixture/0.1.0.json",
        enrichment_core::native_time::AcquisitionTime::try_from(
            "2026-09-14T00:00:00.000000Z".to_owned(),
        )
        .unwrap(),
    );
    let context = context(
        artifact,
        Ecosystem::Rust,
        "enr_fixture",
        "rustdoc-json",
        "rustdoc_json",
    );
    let result = producer_records::collect(payload, context, DocumentBatch::default())
        .expect("native Rust relations")
        .1;
    assert!(
        result.definitions.len() < result.symbols.len(),
        "aliases share definitions"
    );
    assert!(result.api_observations.len() > result.symbols.len());
    assert!(
        result.symbols.iter().any(|s| s.qualifier.is_some()),
        "trait members retain qualifiers"
    );
    assert!(
        result
            .fragments
            .iter()
            .all(|f| matches!(f.source.locator, Locator::RustdocItem { .. }))
    );
    assert!(result.input_artifacts.iter().all(|a| a.validate().is_ok()));
}

#[test]
fn malformed_or_unbound_producer_evidence_is_rejected() {
    let artifact = Artifact::describe(
        b"readme",
        ArtifactKind::Readme,
        "text/markdown",
        "https://example.org/README.md",
        enrichment_core::native_time::AcquisitionTime::try_from(
            "2026-09-14T00:00:00.000000Z".to_owned(),
        )
        .unwrap(),
    );
    let fragment = DocumentFact::new(
        FragmentKind::ReadmeSection,
        "p",
        &artifact.artifact_id,
        enrichment_core::evidence::relational::Locator::MarkdownSection {
            file: "README.md".into(),
            heading: "p".into(),
            line: 1,
        },
        "Documentation".into(),
        EvidenceClass::Declared,
        "source",
        "1",
    )
    .expect("valid fixture locator");
    let mut context = context(artifact, Ecosystem::Rust, "p", "source", "readme");
    context.producer_runs[0].inputs.clear();
    assert!(
        normalize(
            context,
            DocumentBatch {
                fragments: vec![fragment],
            }
        )
        .is_err()
    );
}
