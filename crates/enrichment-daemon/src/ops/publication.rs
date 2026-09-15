//! One Rust producer-to-Arrow publication boundary for every acquisition path.
use super::resolve::Acquisition;
use crate::service::Service;
use enrichment_core::{
    evidence::{
        Artifact, EvidenceKind,
        ingest::{EvidenceSink, IngestContext, ProducerSource},
        metadata::{ReleaseDetails, ReleaseMetadata},
        relational::{FactSource, Locator},
        snapshot::{EvidenceManifest, SnapshotMetadata},
    },
    wire::{EvidenceClass, SourceVersionMatch},
};
use std::collections::BTreeMap;

pub(super) struct MetadataInput {
    pub details: ReleaseDetails,
    pub artifact: Artifact,
    pub locator: Locator,
}

pub(super) async fn publish(
    service: &Service,
    acq: &mut Acquisition<'_>,
    metadata: SnapshotMetadata,
    input: impl ProducerSource + 'static,
    mut components: BTreeMap<String, String>,
    indexed: Vec<EvidenceKind>,
    missing: Vec<EvidenceKind>,
    details: Option<MetadataInput>,
) -> Result<EvidenceManifest, String> {
    components.insert(
        "evidence-normalizer".into(),
        enrichment_core::evidence::ingest::VERSION.into(),
    );
    {
        let run = acq
            .runs
            .last_mut()
            .ok_or("normalization has no producing attempt")?;
        run.config_digest = enrichment_core::canonical::digest_hex(&serde_json::json!([
            "normalization-components/1",
            run.config_digest,
            components,
        ]));
    }
    let completion = super::resolve_job::prepare(acq, &metadata)?;
    let run = acq.runs.last().ok_or("normalization attempt disappeared")?;
    let attempt = run.attempt_id.clone();
    let source_version_match = if metadata.context.mode
        == enrichment_core::identity::ResearchMode::Revision
        || metadata.crate_version.as_deref() == Some(metadata.release.key.version.as_str())
    {
        SourceVersionMatch::Exact
    } else {
        SourceVersionMatch::Unknown
    };
    let detail_source = details.as_ref().map(|d| FactSource {
        producer_binding_id: run.semantic_binding_id(),
        extractor: run.producer.clone(),
        extractor_version: run.producer_version.clone(),
        artifact_id: d.artifact.artifact_id.clone(),
        source_uri: Some(d.artifact.source_uri.clone()),
        source_version_match,
        locator: d.locator.clone(),
        evidence_class: EvidenceClass::StaticallyExtracted,
    });
    let context = IngestContext {
        ecosystem: metadata.release.key.ecosystem,
        symbol_package: metadata.symbol_package.clone(),
        release_id: metadata.release.release_id.to_string(),
        environment_id: metadata.environment.environment_id.to_string(),
        source_version_match,
        producing_attempt: attempt,
        producer_runs: acq.runs.clone(),
        artifacts: acq.artifacts.clone(),
        component_versions: components,
        indexed,
        missing,
        gaps: acq.gaps.clone(),
    };
    let release_id = metadata.release.release_id.to_string();
    let runtime = service.repository.runtime.clone();
    let executor = tokio::runtime::Handle::current();
    let produce = move |sink: &mut enrichment_store::record_writer::RelationWriter| {
        let runs = context.producer_runs.clone();
        let root = sink.staging_root().to_owned();
        let limits = sink.limits().clone();
        let mut acquisitions = enrichment_store::ingest::normalize_into(
            context, input, sink, &root, &runtime, &limits, &executor,
        )
        .map_err(|e| e.to_string())?;
        if let Some(details) = details {
            sink.metadata(ReleaseMetadata::new(
                release_id,
                details.details,
                detail_source.ok_or("metadata source missing")?,
            ))?;
        }
        runs.into_iter()
            .map(|run| {
                let artifacts = acquisitions
                    .remove(&run.attempt_id)
                    .ok_or("attempt inventory missing")?;
                Ok((run, artifacts))
            })
            .collect()
    };
    match completion {
        Some((stage, completion, delivery)) => {
            let work = acq.work.ok_or("durable resolution owner disappeared")?;
            work.check().map_err(|e| e.to_string())?;
            service
                .jobs
                .pin_resolution(&work.id, stage)
                .map_err(|e| e.to_string())?;
            let state = completion.state;
            let manifest = service
                .repository
                .publish_records(metadata, produce, None, Some(completion))
                .await
                .map_err(|e| e.to_string())?;
            let _ = work.committed.set((
                state,
                manifest.snapshot_id.to_string(),
                delivery
                    .get(manifest.snapshot_id.as_str())
                    .map_err(|e| e.to_string())?,
            ));
            Ok(manifest)
        }
        None => service
            .repository
            .publish_records(metadata, produce, None, None)
            .await
            .map_err(|e| e.to_string()),
    }
}
