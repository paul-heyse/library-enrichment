use native_ingest::EvidenceRows;
pub mod native_ingest;
mod producer_records;
use enrichment_core::{
    evidence::{
        Artifact, ArtifactKind, EvidenceKind,
        ingest::{DocumentBatch, IngestContext},
    },
    identity::Ecosystem,
    policy::ExecutionProfile,
    producer::{ProducerRun, RunOutcome},
    wire::SourceVersionMatch,
};

pub fn rust_evidence_for(
    release_id: enrichment_core::identity::ReleaseId,
    environment_id: enrichment_core::identity::EnvironmentId,
) -> EvidenceRows {
    let payload = include_str!("../../../../tests/fixtures/rustdoc/enr-fixture-0.1.0-default.json");
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
    producer_records::collect(
        payload,
        IngestContext {
            ecosystem: Ecosystem::Rust,
            symbol_package: "enr_fixture".into(),
            release_id,
            environment_id,
            source_version_match: SourceVersionMatch::Exact,
            producing_attempt: "attempt_00112233445566778899aabbccddeeff"
                .to_owned()
                .try_into()
                .unwrap(),
            producer_runs: vec![ProducerRun {
                attempt_id: "attempt_00112233445566778899aabbccddeeff"
                    .to_owned()
                    .try_into()
                    .unwrap(),
                producer: "rustdoc-json".into(),
                producer_version: "2".into(),
                config_digest: enrichment_core::canonical::sha256_hex(b"configuration"),
                inputs: [("rustdoc_json".into(), artifact.sha256.clone())]
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
        },
        DocumentBatch::default(),
    )
    .expect("typed normalization")
    .1
}
