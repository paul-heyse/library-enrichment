mod native_ingest;
mod producer_records;
use enrichment_core::{
    evidence::{
        Artifact, ArtifactKind, EvidenceKind,
        ingest::{EvidenceBatch, IngestContext, ProducerBatch},
    },
    identity::Ecosystem,
    policy::ExecutionProfile,
    producer::{ProducerRun, RunOutcome, normalize},
    wire::SourceVersionMatch,
};

pub fn rust_evidence_for(release_id: &str, environment_id: &str) -> EvidenceBatch {
    let payload = include_str!("../../../../tests/fixtures/rustdoc/enr-fixture-0.1.0-default.json");
    let artifact = Artifact::describe(
        payload.as_bytes(),
        ArtifactKind::RustdocJson,
        "application/json",
        "https://docs.rs/enr-fixture/0.1.0.json",
        "2026-09-14T00:00:00Z",
    );
    let produced = producer_records::collect(
        normalize::prepare(&normalize::NormalizeInput {
            payload,
            rustdoc_artifact_id: &artifact.artifact_id,
            json_path: None,
            summary_chars: 240,
        })
        .expect("prepared fixture producer"),
    )
    .expect("rustdoc normalization");
    native_ingest::normalize(
        IngestContext {
            ecosystem: Ecosystem::Rust,
            symbol_package: produced.source.crate_name,
            release_id: release_id.into(),
            environment_id: environment_id.into(),
            source_version_match: SourceVersionMatch::Exact,
            producing_attempt: "attempt-fixture".into(),
            producer_runs: vec![ProducerRun {
                attempt_id: "attempt-fixture".into(),
                producer: "rustdoc-json".into(),
                producer_version: "2".into(),
                config_digest: "configuration".into(),
                inputs: [("rustdoc_json".into(), artifact.sha256.clone())]
                    .into_iter()
                    .collect(),
                profile: ExecutionProfile::Static,
                started_at: "2026-09-14T00:00:00Z".into(),
                finished_at: "2026-09-14T00:00:01Z".into(),
                outcome: RunOutcome::Succeeded,
                gaps: vec![],
                log: None,
            }],
            artifacts: vec![artifact],
            component_versions: [("rustdoc-json".into(), "2".into())].into_iter().collect(),
            indexed: vec![EvidenceKind::PublicApi, EvidenceKind::Documentation],
            missing: vec![],
            gaps: vec![],
        },
        ProducerBatch {
            symbols: produced.symbols,
            relationships: produced.relationships,
            fragments: produced.fragments,
        },
    )
    .expect("typed normalization")
}
