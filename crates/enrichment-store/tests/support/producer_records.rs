//! Bounded fixture collection from the actual native producer plans. There is no test copy
//! of public reachability, identity construction or evidence association policy.
use super::native_ingest::EvidenceRows;
use enrichment_core::evidence::{
    ArtifactKind,
    ingest::{DocumentBatch, IngestContext},
    relational::{FactSource, Locator},
};
use enrichment_store::{runtime::QueryRuntime, rust_normalize::Header};

pub fn collect(
    payload: &str,
    context: IngestContext,
    documents: DocumentBatch,
) -> Result<(Header, EvidenceRows), String> {
    let payload = payload.to_owned();
    let mut evidence = EvidenceRows::default();
    std::thread::spawn(move || -> Result<(Header, EvidenceRows), String> {
        let executor = tokio::runtime::Builder::new_multi_thread()
            .worker_threads(2)
            .enable_all()
            .build()
            .map_err(|e| e.to_string())?;
        executor.block_on(async {
            let root = tempfile::tempdir().map_err(|e| e.to_string())?;
            enrichment_store::leases::initialize(root.path()).map_err(|e| e.to_string())?;
            let blobs =
                enrichment_store::BlobStore::open(root.path()).map_err(|e| e.to_string())?;
            let descriptor = context
                .artifacts
                .iter()
                .find(|a| a.kind == ArtifactKind::RustdocJson)
                .ok_or("fixture rustdoc missing")?
                .clone();
            let artifact = blobs
                .put(payload.as_bytes(), |_| descriptor)
                .map_err(|e| e.to_string())?
                .acquired;
            let runtime = QueryRuntime::new(&root.path().join("spill"), Default::default())
                .map_err(|e| e.to_string())?;
            let facts = enrichment_store::native_rustdoc::from_artifact(
                &runtime,
                blobs,
                artifact.clone(),
                240,
            )
            .await
            .map_err(|e| e.to_string())?;
            let run = context
                .producer_runs
                .last()
                .ok_or("fixture producer missing")?;
            let source = FactSource {
                producer_binding_id: run.semantic_binding_id(),
                extractor: run.producer.clone(),
                extractor_version: run.producer_version.clone(),
                artifact_id: artifact.artifact_id,
                source_uri: Some(artifact.source_uri),
                source_version_match: context.source_version_match,
                locator: Locator::Artifact,
                evidence_class: enrichment_core::wire::EvidenceClass::StaticallyExtracted,
            };
            for (relation, plan) in facts
                .evidence(&context, &source)
                .await
                .map_err(|e| e.to_string())?
            {
                for batch in runtime
                    .execute(plan)
                    .await
                    .map_err(|e| e.to_string())?
                    .batches
                {
                    evidence.collect(relation, &batch)?;
                }
            }
            let base = super::native_ingest::normalize(context, documents)?;
            evidence.extend(base);
            runtime
                .close_diagnostics()
                .await
                .map_err(|e| e.to_string())?;
            Ok((facts.header, evidence))
        })
    })
    .join()
    .map_err(|_| "native Rust fixture panicked".to_owned())?
}
