//! Actual Delta providers for evidence view qualification.
use super::support::native_ingest::EvidenceRows;
use arrow::record_batch::RecordBatch;
use enrichment_store::{
    admission::Relation, delta_evidence::EvidenceTables, projection, runtime::QueryRuntime,
};
use std::{collections::BTreeMap, path::Path, sync::Arc};

pub async fn providers(
    root: &Path,
    runtime: &QueryRuntime,
    evidence: &EvidenceRows,
) -> BTreeMap<Relation, Arc<dyn datafusion::catalog::TableProvider>> {
    let tables = EvidenceTables::new(root, runtime.clone()).unwrap();
    let batches: [(Relation, RecordBatch); 10] = [
        (
            Relation::Definitions,
            projection::definitions(&evidence.definitions).unwrap(),
        ),
        (
            Relation::Symbols,
            projection::bindings(&evidence.symbols).unwrap(),
        ),
        (
            Relation::ApiObservations,
            projection::observations(&evidence.api_observations).unwrap(),
        ),
        (
            Relation::ExecutionObservations,
            projection::execution::encode(&evidence.execution_observations).unwrap(),
        ),
        (
            Relation::Relationships,
            projection::relationships(&evidence.relationships).unwrap(),
        ),
        (
            Relation::Fragments,
            projection::fragments(&evidence.fragments).unwrap(),
        ),
        (
            Relation::ProducerRuns,
            projection::producer_runs(&evidence.producer_runs).unwrap(),
        ),
        (
            Relation::InputArtifacts,
            projection::input_artifacts(&evidence.input_artifacts).unwrap(),
        ),
        (
            Relation::Coverage,
            projection::coverage(&evidence.coverage).unwrap(),
        ),
        (
            Relation::ReleaseMetadata,
            projection::metadata::encode(&evidence.release_metadata).unwrap(),
        ),
    ];
    let mut bindings = Vec::new();
    for (relation, batch) in batches {
        let rows = batch.num_rows() as u64;
        let input = runtime.session().read_batch(batch).unwrap();
        bindings.push(
            tables
                .append(
                    relation,
                    &enrichment_core::identity::CohortId::new(),
                    input,
                    rows,
                )
                .await
                .unwrap(),
        );
    }
    let data_root = root.parent().unwrap();
    enrichment_store::leases::initialize(data_root).unwrap();
    let retention = enrichment_store::retention::RetentionStore::new(
        enrichment_store::control::ControlStore::open(data_root, runtime.clone()).unwrap(),
        runtime.clone(),
    );
    let protection = enrichment_store::leases::ReadProtection::Durable(
        retention
            .enroll(
                "fixture".into(),
                enrichment_store::retention::ProtectionKind::Query,
                bindings
                    .iter()
                    .map(|binding| enrichment_store::retention::Dependency::Table {
                        value: binding.selection(),
                    })
                    .collect(),
            )
            .await
            .unwrap(),
    );
    tables.providers(&bindings, protection).await.unwrap()
}
