//! Test-local collection over the production native staging and distinct plans.
use enrichment_core::evidence::ingest::{DocumentBatch, IngestContext};
use enrichment_store::{
    admission::Relation,
    dataset::WriteLimits,
    projection,
    runtime::{QueryLimits, QueryRuntime},
};

use enrichment_core::evidence::{
    Artifact,
    relational::{
        ApiObservation, CoverageFact, Definition, InputArtifact, PublicBinding,
        RelationshipObservation, TextFragment,
    },
};
use std::collections::BTreeMap;

/// Decoded expected facts for fixtures. No normalization or admission policy lives here.
#[derive(Debug, Clone, Default)]
pub struct EvidenceRows {
    pub definitions: Vec<Definition>,
    pub symbols: Vec<PublicBinding>,
    pub api_observations: Vec<ApiObservation>,
    pub execution_observations: Vec<enrichment_core::evidence::execution::ExecutionObservation>,
    pub relationships: Vec<RelationshipObservation>,
    pub fragments: Vec<TextFragment>,
    pub producer_runs: Vec<enrichment_core::producer::ProducerRun>,
    pub input_artifacts: Vec<InputArtifact>,
    pub coverage: Vec<CoverageFact>,
    pub release_metadata: Vec<enrichment_core::evidence::metadata::ReleaseMetadata>,
    pub attempt_artifacts: BTreeMap<String, Vec<Artifact>>,
}

impl EvidenceRows {
    pub fn collect(
        &mut self,
        relation: Relation,
        batch: &arrow::record_batch::RecordBatch,
    ) -> Result<(), String> {
        match relation {
            Relation::Definitions => self
                .definitions
                .extend(projection::decode::definitions(batch).map_err(|e| e.to_string())?),
            Relation::Symbols => self
                .symbols
                .extend(projection::decode::bindings(batch).map_err(|e| e.to_string())?),
            Relation::ApiObservations => self
                .api_observations
                .extend(projection::decode::observations(batch).map_err(|e| e.to_string())?),
            Relation::ExecutionObservations => self
                .execution_observations
                .extend(projection::execution::decode(batch).map_err(|e| e.to_string())?),
            Relation::Relationships => self
                .relationships
                .extend(projection::relationships_from_batch(batch).map_err(|e| e.to_string())?),
            Relation::Fragments => self
                .fragments
                .extend(projection::fragments_from_batch(batch).map_err(|e| e.to_string())?),
            Relation::ProducerRuns => self
                .producer_runs
                .extend(projection::producer_runs_from_batch(batch).map_err(|e| e.to_string())?),
            Relation::InputArtifacts => self
                .input_artifacts
                .extend(projection::input_artifacts_from_batch(batch).map_err(|e| e.to_string())?),
            Relation::Coverage => self
                .coverage
                .extend(projection::coverage_from_batch(batch).map_err(|e| e.to_string())?),
            Relation::ReleaseMetadata => self
                .release_metadata
                .extend(projection::metadata::decode(batch).map_err(|e| e.to_string())?),
        }
        Ok(())
    }
    pub fn extend(&mut self, other: Self) {
        self.definitions.extend(other.definitions);
        self.symbols.extend(other.symbols);
        self.api_observations.extend(other.api_observations);
        self.execution_observations
            .extend(other.execution_observations);
        self.relationships.extend(other.relationships);
        self.fragments.extend(other.fragments);
        self.producer_runs.extend(other.producer_runs);
        self.input_artifacts.extend(other.input_artifacts);
        self.coverage.extend(other.coverage);
        self.release_metadata.extend(other.release_metadata);
        self.attempt_artifacts.extend(other.attempt_artifacts);
    }
}

/// Encode fixture facts to native plans. Production entry points accept native plans or raw
/// producer facts; this helper neither chooses nor validates semantic results.
pub async fn publish_rows(
    repository: &enrichment_store::repository::EvidenceRepository,
    metadata: enrichment_core::evidence::snapshot::SnapshotMetadata,
    mut rows: EvidenceRows,
    expected: Option<enrichment_core::identity::SnapshotId>,
    completion: Option<enrichment_store::repository::JobCompletion>,
) -> datafusion::error::Result<enrichment_core::evidence::snapshot::EvidenceManifest> {
    let session = repository.runtime.session();
    let plans = [
        (
            Relation::Definitions,
            session.read_batch(projection::definitions(&rows.definitions)?)?,
        ),
        (
            Relation::Symbols,
            session.read_batch(projection::bindings(&rows.symbols)?)?,
        ),
        (
            Relation::ApiObservations,
            session.read_batch(projection::observations(&rows.api_observations)?)?,
        ),
        (
            Relation::ExecutionObservations,
            session.read_batch(projection::execution::encode(&rows.execution_observations)?)?,
        ),
        (
            Relation::Relationships,
            session.read_batch(projection::relationships(&rows.relationships)?)?,
        ),
        (
            Relation::Fragments,
            session.read_batch(projection::fragments(&rows.fragments)?)?,
        ),
        (
            Relation::ProducerRuns,
            session.read_batch(projection::producer_runs(&rows.producer_runs)?)?,
        ),
        (
            Relation::InputArtifacts,
            session.read_batch(projection::input_artifacts(&rows.input_artifacts)?)?,
        ),
        (
            Relation::Coverage,
            session.read_batch(projection::coverage(&rows.coverage)?)?,
        ),
        (
            Relation::ReleaseMetadata,
            session.read_batch(projection::metadata::encode(&rows.release_metadata)?)?,
        ),
    ]
    .into_iter()
    .collect();
    let attempts = rows
        .producer_runs
        .into_iter()
        .map(|run| {
            let inputs = rows
                .attempt_artifacts
                .remove(&run.attempt_id)
                .unwrap_or_default();
            (run, inputs)
        })
        .collect();
    repository
        .publish_native(metadata, plans, attempts, expected, completion)
        .await
        .map(|published| published.manifest)
}

pub fn normalize(context: IngestContext, input: DocumentBatch) -> Result<EvidenceRows, String> {
    std::thread::spawn(move || -> Result<EvidenceRows, String> {
        let executor = tokio::runtime::Builder::new_multi_thread()
            .worker_threads(2)
            .enable_all()
            .build()
            .map_err(|e| e.to_string())?;
        let root = tempfile::tempdir().map_err(|e| e.to_string())?;
        let runtime = QueryRuntime::new(&root.path().join("spill"), QueryLimits::default())
            .map_err(|e| e.to_string())?;
        let limits = WriteLimits::default();
        let mut result = EvidenceRows::default();
        executor.block_on(async {
            let (plans, attempts) = enrichment_store::ingest::prepare(
                context,
                input,
                None,
                root.path(),
                &runtime,
                &limits,
            )
            .await
            .map_err(|e| e.to_string())?;
            result.attempt_artifacts = attempts
                .into_iter()
                .map(|(run, artifacts)| (run.attempt_id, artifacts))
                .collect();
            for (relation, frame) in plans {
                let output = runtime.execute(frame).await.map_err(|e| e.to_string())?;
                for batch in output.batches {
                    result.collect(relation, &batch)?;
                }
            }
            runtime
                .close_diagnostics()
                .await
                .map_err(|e| e.to_string())?;
            Ok::<_, String>(())
        })?;
        Ok(result)
    })
    .join()
    .map_err(|_| "native fixture collection panicked".to_owned())?
}
