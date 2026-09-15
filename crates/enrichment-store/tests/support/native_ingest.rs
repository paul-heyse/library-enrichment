//! Test-local collection over the production native staging and distinct plans.
use enrichment_core::evidence::ingest::{
    EvidenceBatch, EvidenceSink, IngestContext, ProducerBatch,
};
use enrichment_store::{
    admission::Relation,
    dataset::WriteLimits,
    projection,
    runtime::{QueryLimits, QueryRuntime},
};

pub fn normalize(context: IngestContext, input: ProducerBatch) -> Result<EvidenceBatch, String> {
    std::thread::spawn(move || -> Result<EvidenceBatch, String> {
        let executor = tokio::runtime::Builder::new_multi_thread()
            .worker_threads(2)
            .enable_all()
            .build()
            .map_err(|e| e.to_string())?;
        let root = tempfile::tempdir().map_err(|e| e.to_string())?;
        let runtime = QueryRuntime::new(&root.path().join("spill"), QueryLimits::default())
            .map_err(|e| e.to_string())?;
        let limits = WriteLimits::default();
        let mut writer = enrichment_store::record_writer::RelationWriter::new(
            &root.path().join("records"),
            &limits,
        )
        .map_err(|e| e.to_string())?;
        let mut result = EvidenceBatch {
            attempt_artifacts: enrichment_store::ingest::normalize_into(
                context,
                input,
                &mut writer,
                root.path(),
                &runtime,
                &limits,
                executor.handle(),
            )
            .map_err(|e| e.to_string())?,
            ..Default::default()
        };
        let files = writer.finish().map_err(|e| e.to_string())?;
        executor.block_on(async {
            let session = runtime.session();
            for file in files {
                let frame = session
                    .read_parquet(
                        file.path.to_str().ok_or("non-UTF8 fixture path")?,
                        Default::default(),
                    )
                    .await
                    .map_err(|e| e.to_string())?
                    .distinct()
                    .map_err(|e| e.to_string())?;
                let output = runtime.execute(frame).await.map_err(|e| e.to_string())?;
                for batch in output.batches {
                    macro_rules! rows {
                        ($decode:path, $sink:ident) => {
                            for row in $decode(&batch).map_err(|e| e.to_string())? {
                                result.$sink(row)?;
                            }
                        };
                    }
                    match file.relation {
                        Relation::Definitions => rows!(projection::decode::definitions, definition),
                        Relation::Symbols => rows!(projection::decode::bindings, binding),
                        Relation::ApiObservations => rows!(projection::decode::observations, api),
                        Relation::ExecutionObservations => {
                            rows!(projection::execution::decode, execution)
                        }
                        Relation::Relationships => {
                            rows!(projection::relationships_from_batch, relationship)
                        }
                        Relation::Fragments => rows!(projection::fragments_from_batch, fragment),
                        Relation::ProducerRuns => {
                            rows!(projection::producer_runs_from_batch, producer)
                        }
                        Relation::InputArtifacts => {
                            rows!(projection::input_artifacts_from_batch, input)
                        }
                        Relation::Coverage => rows!(projection::coverage_from_batch, coverage),
                        Relation::ReleaseMetadata => rows!(projection::metadata::decode, metadata),
                    }
                }
            }
            Ok::<_, String>(())
        })?;
        Ok(result)
    })
    .join()
    .map_err(|_| "native fixture collection panicked".to_owned())?
}
