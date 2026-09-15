mod support;

use enrichment_store::{
    admission::{AdmissionCache, AdmissionLimits, Relation},
    dataset::{self, WriteLimits},
    runtime::{QueryLimits, QueryRuntime},
};
use parquet::arrow::arrow_reader::ParquetRecordBatchReaderBuilder;
use std::fs::File;

#[tokio::test]
async fn actual_rustdoc_produces_admitted_relations_in_bounded_row_groups() {
    let dir = tempfile::tempdir().expect("directory");
    let evidence = support::rust_evidence_for("rel_fixture", "env_fixture");
    let files = dataset::write(
        &dir.path().join("evidence"),
        &evidence,
        &WriteLimits {
            batch_rows: 3,
            row_group_rows: 2,
            ..WriteLimits::default()
        },
    )
    .expect("bounded write");
    assert_eq!(files.len(), Relation::ALL.len());
    assert!(
        files
            .iter()
            .filter(|f| !matches!(
                f.relation,
                Relation::ReleaseMetadata | Relation::ExecutionObservations
            ))
            .all(|f| f.rows > 0)
    );
    assert_eq!(
        files
            .iter()
            .find(|f| f.relation == enrichment_store::admission::Relation::ReleaseMetadata)
            .expect("explicit unobserved metadata relation")
            .rows,
        0
    );
    for file in &files {
        let reader =
            ParquetRecordBatchReaderBuilder::try_new(File::open(&file.path).expect("file"))
                .expect("Parquet");
        assert!(
            reader
                .metadata()
                .row_groups()
                .iter()
                .all(|g| g.num_rows() <= 2)
        );
    }
    let runtime =
        QueryRuntime::new(&dir.path().join("spill"), QueryLimits::default()).expect("runtime");
    let cache = AdmissionCache::new(runtime.clone(), AdmissionLimits::default()).expect("cache");
    let admitted = cache
        .admit(
            "fixture-manifest",
            &enrichment_store::admission::EvidenceScope {
                ecosystem: enrichment_core::identity::Ecosystem::Rust,
                symbol_package: evidence.definitions[0].defined_in_package.clone(),
                release_id: "rel_fixture".into(),
                environment_id: "env_fixture".into(),
            },
            &files,
        )
        .await
        .expect("full relational admission");
    let session = admitted.session(&runtime, None).expect("bound catalog");
    let result = runtime.execute(session.sql("SELECT o.observation_id, s.path, d.kind, i.artifact_id FROM snapshot.evidence.api_observations o JOIN snapshot.evidence.symbols s ON o.subject.symbol_id = s.symbol_id JOIN snapshot.evidence.definitions d ON s.definition_id = d.definition_id JOIN snapshot.evidence.input_artifacts i ON o.source.producer_binding_id = i.producer_binding_id AND o.source.artifact_id = i.artifact_id ORDER BY o.observation_id").await.expect("plan")).await.expect("execute");
    assert_eq!(result.rows, evidence.api_observations.len());
}

#[test]
fn semantic_components_ignore_order_attempt_clocks_and_file_layout() {
    let mut evidence = support::rust_evidence_for("rel_fixture", "env_fixture");
    let before = semantic_components(&evidence).expect("components");
    evidence.symbols.reverse();
    evidence.fragments.reverse();
    evidence.api_observations.reverse();
    evidence.producer_runs[0].attempt_id = "later-attempt".into();
    evidence.producer_runs[0].started_at = "2026-09-15T00:00:00Z".into();
    evidence.producer_runs[0].finished_at = "2026-09-15T00:00:01Z".into();
    assert_eq!(before, semantic_components(&evidence).expect("components"));
    let dir = tempfile::tempdir().expect("directory");
    let first =
        dataset::write(&dir.path().join("one"), &evidence, &WriteLimits::default()).expect("write");
    let second = dataset::write(
        &dir.path().join("two"),
        &evidence,
        &WriteLimits {
            batch_rows: 3,
            row_group_rows: 2,
            ..WriteLimits::default()
        },
    )
    .expect("write");
    assert_ne!(
        first
            .iter()
            .find(|f| f.relation == Relation::Symbols)
            .expect("symbols")
            .sha256,
        second
            .iter()
            .find(|f| f.relation == Relation::Symbols)
            .expect("symbols")
            .sha256
    );
    assert_eq!(before, semantic_components(&evidence).expect("components"));
}

#[test]
fn empty_relations_have_schemas_and_resource_excess_is_an_error() {
    let dir = tempfile::tempdir().expect("directory");
    let files = dataset::write(
        &dir.path().join("empty"),
        &Default::default(),
        &WriteLimits::default(),
    )
    .expect("empty");
    assert!(files.iter().all(|f| f.rows == 0));
    for file in &files {
        let reader =
            ParquetRecordBatchReaderBuilder::try_new(File::open(&file.path).expect("file"))
                .expect("read");
        assert_eq!(reader.schema(), &file.relation.schema().expect("schema"));
    }
    let evidence = support::rust_evidence_for("rel_fixture", "env_fixture");
    for (name, limits) in [
        (
            "record",
            WriteLimits {
                record_bytes: 20,
                ..WriteLimits::default()
            },
        ),
        (
            "file",
            WriteLimits {
                file_bytes: 20,
                ..WriteLimits::default()
            },
        ),
        (
            "metadata",
            WriteLimits {
                batch_rows: 1,
                row_group_rows: 1,
                row_groups: 1,
                ..WriteLimits::default()
            },
        ),
    ] {
        assert!(
            dataset::write(&dir.path().join(name), &evidence, &limits).is_err(),
            "{name}"
        );
    }
}

#[tokio::test]
async fn streamed_records_cross_batches_without_retaining_the_corpus() {
    use enrichment_core::evidence::{ingest::EvidenceSink, relational::TextFragment};
    let dir = tempfile::tempdir().unwrap();
    let batch = support::rust_evidence_for("rel_fixture", "env_fixture");
    let template = batch.fragments[0].clone();
    let mut expected = std::collections::BTreeSet::new();
    let limits = WriteLimits {
        batch_rows: 31,
        batch_bytes: 256 * 1024,
        record_bytes: 24 * 1024,
        ..WriteLimits::default()
    };
    let mut sink =
        enrichment_store::record_writer::RelationWriter::new(dir.path(), &limits).unwrap();
    batch.drain_into(&mut sink).unwrap();
    for i in 0..3000 {
        let row = TextFragment::new(
            template.kind,
            template.subject.clone(),
            format!("record {i}"),
            format!("{i:04}:{}", "x".repeat(16 * 1024)),
            template.source.clone(),
        )
        .unwrap();
        expected.insert(row.fragment_id.clone());
        sink.fragment(row).unwrap();
        let (rows, bytes) = sink.buffered();
        assert!(rows <= limits.batch_rows);
        assert!(bytes <= limits.batch_bytes);
    }
    let files = sink.finish().unwrap();
    let fragments = files
        .iter()
        .find(|f| f.relation == Relation::Fragments)
        .unwrap();
    assert!(fragments.rows >= 3000);
    let reader =
        ParquetRecordBatchReaderBuilder::try_new(File::open(&fragments.path).unwrap()).unwrap();
    assert!(reader.metadata().num_row_groups() > 100);
    let mut seen = std::collections::BTreeSet::new();
    for batch in reader.with_batch_size(19).build().unwrap() {
        for row in enrichment_store::projection::fragments_from_batch(&batch.unwrap()).unwrap() {
            if expected.contains(&row.fragment_id) {
                seen.insert(row.fragment_id);
            }
        }
    }
    assert_eq!(seen, expected);
    let runtime = QueryRuntime::new(&dir.path().join("spill"), QueryLimits::default()).unwrap();
    let cache = AdmissionCache::new(runtime, AdmissionLimits::default()).unwrap();
    cache
        .admit(
            "streamed",
            &enrichment_store::admission::EvidenceScope {
                ecosystem: enrichment_core::identity::Ecosystem::Rust,
                symbol_package: "enr_fixture".into(),
                release_id: "rel_fixture".into(),
                environment_id: "env_fixture".into(),
            },
            &files,
        )
        .await
        .unwrap();
}

/// Semantic table digests are independent of row order, attempts, file layout and compression.
/// Call this after record-size validation. API/fragment/coverage IDs already bind their entire
/// qualified payload; binding records additionally bind the selected definition and flags.
///
/// # Errors
/// Typed serialization failure is an error and can never become a default digest.
fn semantic_components(
    evidence: &enrichment_core::evidence::ingest::EvidenceBatch,
) -> std::io::Result<std::collections::BTreeMap<String, String>> {
    Ok(std::collections::BTreeMap::from([
        (
            "release_metadata".into(),
            sorted_digest(
                evidence
                    .release_metadata
                    .iter()
                    .map(|r| r.metadata_id.clone())
                    .collect(),
            ),
        ),
        (
            "definitions".into(),
            sorted_digest(records(&evidence.definitions)?),
        ),
        ("symbols".into(), sorted_digest(records(&evidence.symbols)?)),
        (
            "execution_observations".into(),
            sorted_digest(
                evidence
                    .execution_observations
                    .iter()
                    .map(|r| r.observation_id.clone())
                    .collect(),
            ),
        ),
        (
            "api_observations".into(),
            sorted_digest(
                evidence
                    .api_observations
                    .iter()
                    .map(|o| o.observation_id.clone())
                    .collect(),
            ),
        ),
        (
            "relationships".into(),
            sorted_digest(
                evidence
                    .relationships
                    .iter()
                    .map(|o| o.relationship_id.clone())
                    .collect(),
            ),
        ),
        (
            "fragments".into(),
            sorted_digest(
                evidence
                    .fragments
                    .iter()
                    .map(|o| o.fragment_id.clone())
                    .collect(),
            ),
        ),
        (
            "producer_runs".into(),
            sorted_digest(
                evidence
                    .producer_runs
                    .iter()
                    .map(enrichment_core::producer::ProducerRun::semantic_binding_id)
                    .collect(),
            ),
        ),
        (
            "input_artifacts".into(),
            sorted_digest(
                evidence
                    .input_artifacts
                    .iter()
                    .map(|o| o.input_id.clone())
                    .collect(),
            ),
        ),
        (
            "coverage".into(),
            sorted_digest(
                evidence
                    .coverage
                    .iter()
                    .map(|o| o.coverage_id.clone())
                    .collect(),
            ),
        ),
    ]))
}

fn sorted_digest(mut keys: Vec<String>) -> String {
    keys.sort();
    keys.dedup();
    enrichment_core::canonical::digest_hex(&serde_json::json!(keys))
}
fn records<T: serde::Serialize>(rows: &[T]) -> std::io::Result<Vec<String>> {
    rows.iter()
        .map(|row| {
            Ok(enrichment_core::canonical::digest_hex(
                &serde_json::to_value(row)?,
            ))
        })
        .collect()
}
