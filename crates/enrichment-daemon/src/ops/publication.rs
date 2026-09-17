//! One Rust producer-to-Arrow publication boundary for every acquisition path.
use super::resolve::Acquisition;
use crate::service::Service;
use enrichment_core::{
    evidence::{
        Artifact, EvidenceKind,
        ingest::{DocumentSource, IngestContext},
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

pub(super) enum NativeFacts {
    Python(enrichment_store::python_normalize::PythonFacts),
    Rust(enrichment_store::rust_normalize::RustFacts),
}
impl NativeFacts {
    fn input(&self) -> &'static str {
        match self {
            Self::Python(_) => "worker",
            Self::Rust(_) => "rustdoc_json",
        }
    }
    async fn evidence(
        &self,
        context: &IngestContext,
        source: &FactSource,
    ) -> Result<enrichment_store::repository::EvidencePlans, String> {
        match self {
            Self::Python(f) => Box::pin(f.evidence(context, source)).await,
            Self::Rust(f) => Box::pin(f.evidence(context, source)).await,
        }
        .map_err(|e| e.to_string())
    }
}

pub(super) async fn publish(
    service: &Service,
    acq: &mut Acquisition<'_>,
    metadata: SnapshotMetadata,
    input: (impl DocumentSource + 'static, Option<NativeFacts>),
    mut components: BTreeMap<String, String>,
    indexed: Vec<EvidenceKind>,
    missing: Vec<EvidenceKind>,
    details: Option<MetadataInput>,
) -> Result<EvidenceManifest, String> {
    let (input, native) = input;
    components.insert(
        "native-definition".into(),
        enrichment_store::runtime::DEFINITION_REVISION.into(),
    );
    components.insert(
        "evidence-normalizer".into(),
        enrichment_core::evidence::ingest::VERSION.into(),
    );
    {
        let run = acq
            .runs
            .last_mut()
            .ok_or("normalization has no producing attempt")?;
        run.config_digest = enrichment_core::native_key::Key::NormalizationConfiguration
            .hex_digest(
                &enrichment_core::operation::identities::NormalizationConfiguration {
                    configuration_digest: run.config_digest.clone(),
                    components: components.clone(),
                },
            )
            .map_err(|error| error.to_string())?;
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
        release_id: metadata.release.release_id.clone(),
        environment_id: metadata.environment.environment_id.clone(),
        source_version_match,
        producing_attempt: attempt,
        producer_runs: acq.runs.clone(),
        artifacts: acq.artifacts.clone(),
        indexed,
        missing,
        gaps: acq.gaps.clone(),
    };
    let native_plans = if let Some(native) = native {
        let digest = run
            .inputs
            .get(native.input())
            .ok_or("native producer input missing")?;
        let mut artifacts = context.artifacts.iter().filter(|a| &a.sha256 == digest);
        let artifact = artifacts.next().ok_or("native producer artifact missing")?;
        if artifacts.any(|a| a.source_uri != artifact.source_uri) {
            return Err("native producer acquisition provenance is ambiguous".into());
        }
        let source = FactSource {
            producer_binding_id: run.semantic_binding_id(),
            extractor: run.producer.clone(),
            extractor_version: run.producer_version.clone(),
            artifact_id: artifact.artifact_id.clone(),
            source_uri: Some(artifact.source_uri.clone()),
            source_version_match,
            locator: Locator::Artifact,
            evidence_class: EvidenceClass::StaticallyExtracted,
        };
        native
            .evidence(&context, &source)
            .await
            .map_err(|e| e.to_string())?
    } else {
        Default::default()
    };
    let release_id = metadata.release.release_id.clone();
    let details = details
        .map(|details| {
            Ok::<_, String>(ReleaseMetadata::new(
                release_id,
                details.details,
                detail_source.ok_or("metadata source missing")?,
            ))
        })
        .transpose()?;
    let (mut plans, attempts) = service
        .repository
        .prepare_documents(context, input, details)
        .await
        .map_err(|e| e.to_string())?;
    for (relation, native) in native_plans {
        let other = plans.remove(&relation).ok_or("producer relation missing")?;
        plans.insert(relation, other.union(native).map_err(|e| e.to_string())?);
    }
    match completion {
        Some((stage, completion)) => {
            let work = acq.work.ok_or("durable resolution owner disappeared")?;
            work.check().map_err(|e| e.to_string())?;
            service
                .jobs
                .pin_resolution(&work.id, stage)
                .await
                .map_err(|e| e.to_string())?;
            let published = service
                .repository
                .publish_native(metadata, plans, attempts, None, Some(completion))
                .await
                .map_err(|e| e.to_string())?;
            let manifest = published.manifest;
            let result = published
                .result
                .ok_or("committed publication lacks prepared native result")?;
            let state = match result.status() {
                enrichment_core::wire::Status::Ok => enrichment_core::wire::JobState::Succeeded,
                enrichment_core::wire::Status::Partial => enrichment_core::wire::JobState::Partial,
                enrichment_core::wire::Status::Error => enrichment_core::wire::JobState::Failed,
                enrichment_core::wire::Status::Pending => {
                    return Err("committed acquisition cannot remain pending".into());
                }
            };
            let _ = work
                .committed
                .set((state, manifest.snapshot_id.clone(), result));
            Ok(manifest)
        }
        None => service
            .repository
            .publish_native(metadata, plans, attempts, None, None)
            .await
            .map(|published| published.manifest)
            .map_err(|e| e.to_string()),
    }
}
