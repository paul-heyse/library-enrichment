#[path = "support/write_parquet.rs"]
mod parquet_fixture;
use std::sync::Arc;

use arrow::array::{
    Array, ArrayRef, Int8DictionaryArray, LargeStringArray, StringArray, StringViewArray,
    StructArray,
};
use arrow::record_batch::RecordBatch;
use datafusion::prelude::{ParquetReadOptions, SessionContext};
use enrichment_core::evidence::{
    Deprecated,
    path::PublicPath,
    relational::{
        ApiObservation, ApiOrigin, ApiPayload, FactSource, Locator, PublicBinding, PythonDetails,
        SubjectRef,
    },
};
use enrichment_core::identity::Ecosystem;
use enrichment_core::producer::python::Publicness;
use enrichment_core::wire::{EvidenceClass, SourceVersionMatch};
use enrichment_store::projection;

fn observations() -> Vec<ApiObservation> {
    let locators = vec![
        Locator::Artifact,
        Locator::Lines {
            file: Some("pkg/mod.py".into()),
            start: 1,
            end: 12,
        },
        Locator::Bytes { start: 0, end: 0 },
        Locator::ArchiveMember {
            path: "pkg/β.py".into(),
        },
        Locator::Heading {
            heading: String::new(),
            ordinal: 0,
        },
        Locator::ProducerItem {
            producer: "rustdoc".into(),
            item: "42".into(),
        },
        Locator::RustdocItem {
            item: 42,
            reported_file: Some("/build/pkg/src/lib.rs".into()),
            reported_line: Some(12),
        },
        Locator::PythonDeclaration {
            file: "pkg/mod.pyi".into(),
            declaration: "pkg.mod.f".into(),
            line: None,
            origin: ApiOrigin::Stub,
            overload: Some(2),
        },
        Locator::ManifestKey {
            file: "Cargo.toml".into(),
            table: "features".into(),
            key: "default".into(),
        },
        Locator::MarkdownSection {
            file: "README.md".into(),
            heading: "Usage".into(),
            line: 8,
        },
        Locator::SourceStart {
            file: "examples/main.rs".into(),
            line: 1,
        },
        Locator::SphinxInventory {
            uri: "https://docs.example.org/v1/api.html#pkg.f".into(),
            role: "py:function".into(),
            project: "Example".into(),
            inventory_version: "1.0".into(),
        },
        Locator::WebDocument {
            uri: "https://docs.example.org/v1/api.html#pkg.f".into(),
            inventory_version: "1.0".into(),
        },
        Locator::Extension {
            format: "test".into(),
            version: "1".into(),
            value: serde_json::json!([1, null, {"v":true}]),
        },
        Locator::Extension {
            format: "test".into(),
            version: "1".into(),
            value: serde_json::json!(null),
        },
        Locator::Extension {
            format: "test".into(),
            version: "1".into(),
            value: serde_json::json!("text"),
        },
    ];
    locators
        .into_iter()
        .enumerate()
        .map(|(i, locator)| {
            ApiObservation::new(
                SubjectRef::Symbol {
                    symbol_id: "sym_fixture".into(),
                },
                if i % 2 == 0 {
                    ApiOrigin::Source
                } else {
                    ApiOrigin::Stub
                },
                format!("env_{}", "a".repeat(64)).try_into().unwrap(),
                ApiPayload {
                    declared_kind: enrichment_core::evidence::SymbolKind::Function,
                    signature: Some(format!("def f{i:02}(x: str) -> bytes")),
                    doc_summary: (i % 2 == 0).then(String::new),
                    docs: Some("preserve:\r\n    indentation β".into()),
                    deprecated: match i % 3 {
                        0 => None,
                        1 => Some(Deprecated {
                            since: None,
                            note: None,
                        }),
                        _ => Some(Deprecated {
                            since: Some("1.0".into()),
                            note: Some(String::new()),
                        }),
                    },
                    cfg_hints: vec!["cfg(unix)".into(), "cfg(feature = \"β\")".into()],
                    rust: None,
                    python: (i % 2 == 1).then(|| PythonDetails {
                        callable: None,
                        overloads: ["f(x: int)", "f(x: str)"]
                            .into_iter()
                            .enumerate()
                            .map(|(ordinal, signature)| {
                                enrichment_core::evidence::declarations::PythonOverload {
                                    ordinal: ordinal as u32,
                                    signature: signature.into(),
                                    callable:
                                        enrichment_core::evidence::declarations::PythonCallable {
                                            parameters: vec![],
                                            returns: None,
                                            labels: vec![],
                                        },
                                }
                            })
                            .collect(),
                        alias_target: Some("pkg.f".into()),
                        bases: vec![],
                        publicness: Publicness {
                            exported: Some(false),
                            underscore: true,
                            reexport: true,
                            docstring: false,
                            declared_exports: vec!["f".into()],
                            unresolved_exports: vec!["more_names".into()],
                        },
                    }),
                },
                FactSource {
                    producer_binding_id: "producer_fixture".into(),
                    extractor: "griffe-static".into(),
                    extractor_version: "1".into(),
                    artifact_id: "art_fixture".into(),
                    source_uri: Some("https://example.org/pkg/1.0".into()),
                    source_version_match: SourceVersionMatch::Exact,
                    locator,
                    evidence_class: EvidenceClass::StaticallyExtracted,
                },
            )
            .expect("typed observation")
        })
        .collect()
}

#[tokio::test]
async fn nested_observations_round_trip_through_parquet_and_datafusion() {
    let records = observations();
    let batch = projection::observations(&records).expect("encode");
    assert_eq!(
        projection::observations(&[])
            .expect("empty schema")
            .schema(),
        batch.schema()
    );
    let dir = tempfile::tempdir().expect("directory");
    let file = dir.path().join("observations.parquet");
    parquet_fixture::write_parquet(&file, &batch, &[]).expect("write");
    let ctx = SessionContext::new();
    ctx.register_parquet(
        "observations",
        file.to_str().expect("path"),
        ParquetReadOptions::default().skip_metadata(false),
    )
    .await
    .expect("register");
    let batches = ctx
        .sql("SELECT * FROM observations ORDER BY payload.signature")
        .await
        .expect("SQL")
        .collect()
        .await
        .expect("scan");
    assert!(batches.iter().any(|batch| matches!(
        batch.column(0).data_type(),
        arrow_schema::DataType::Utf8View
    )));
    let decoded = batches
        .iter()
        .flat_map(|batch| projection::decode::observations(batch).expect("decode"))
        .collect::<Vec<_>>();
    assert_eq!(decoded, records);
    let payload = batches[0]
        .schema()
        .field_with_name("payload")
        .expect("payload")
        .clone();
    assert_eq!(
        payload
            .metadata()
            .get("enrichment.role")
            .map(String::as_str),
        Some("api-payload")
    );
    let distinct = ctx
        .sql("SELECT count(*) AS n FROM observations WHERE payload.deprecated IS NOT NULL")
        .await
        .expect("typed predicate")
        .collect()
        .await
        .expect("aggregate");
    assert_eq!(
        distinct[0]
            .column(0)
            .as_any()
            .downcast_ref::<arrow::array::Int64Array>()
            .expect("count")
            .value(0),
        10
    );
}

#[test]
fn text_encodings_preserve_null_empty_and_dictionary_values() {
    let values = vec![Some("β"), None, Some(""), Some("β")];
    let arrays: Vec<ArrayRef> = vec![
        Arc::new(StringArray::from(values.clone())),
        Arc::new(LargeStringArray::from(values.clone())),
        Arc::new(StringViewArray::from(values.clone())),
        Arc::new(Int8DictionaryArray::from_iter(values.clone())),
    ];
    for array in arrays {
        let reader = projection::TextColumn::new(array.as_ref()).expect("text representation");
        assert_eq!((0..4).map(|i| reader.get(i)).collect::<Vec<_>>(), values);
        assert!(reader.required(1).is_err());
    }
    assert!(projection::TextColumn::new(&arrow::array::UInt32Array::from(vec![1])).is_err());
}

#[test]
fn corrupt_tags_and_observation_ids_cannot_disappear() {
    let batch = projection::observations(&observations()).expect("batch");
    let mut columns = batch.columns().to_vec();
    columns[0] = Arc::new(StringArray::from_iter_values(
        (0..batch.num_rows()).map(|_| "obs_corrupt"),
    ));
    let bad = RecordBatch::try_new(batch.schema(), columns).expect("same shape");
    assert!(projection::decode::observations(&bad).is_err());

    let mut columns = batch.columns().to_vec();
    let subject = columns[1]
        .as_any()
        .downcast_ref::<StructArray>()
        .expect("subject");
    let mut children = subject.columns().to_vec();
    children[0] = Arc::new(StringArray::from_iter_values(
        (0..batch.num_rows()).map(|_| "unknown"),
    ));
    columns[1] = Arc::new(
        StructArray::try_new(subject.fields().clone(), children, None).expect("same shape"),
    );
    let bad = RecordBatch::try_new(batch.schema(), columns).expect("same shape");
    assert!(projection::decode::observations(&bad).is_err());
}

#[test]
fn paths_with_equal_displays_retain_distinct_component_identity() {
    let paths = [
        PublicPath::parse(Ecosystem::Python, "pkg.a.b").expect("nested"),
        PublicPath::new(Ecosystem::Python, vec!["pkg".into(), "a.b".into()]).expect("literal"),
    ];
    let records: Vec<_> = paths
        .into_iter()
        .enumerate()
        .map(|(i, path)| PublicBinding {
            symbol_id: format!("sym_{i}"),
            definition_id: "def_shared".into(),
            path,
            name: "b".into(),
            is_reexport: true,
            qualifier: None,
        })
        .collect();
    assert_eq!(records[0].path.display(), records[1].path.display());
    assert_ne!(records[0].path.id(), records[1].path.id());
    let batch = projection::bindings(&records).expect("batch");
    assert_eq!(
        projection::decode::bindings(&batch).expect("decode"),
        records
    );
}

#[tokio::test]
async fn attempts_inputs_and_coverage_retain_structured_provenance_through_datafusion() {
    use enrichment_core::evidence::{
        Artifact, ArtifactKind, EvidenceKind, Gap, GapReason, PlannedFallback,
        relational::{CoverageFact, CoverageOutcome, InputArtifact},
    };
    use enrichment_core::policy::ExecutionProfile;
    use enrichment_core::producer::{ProducerRun, RunOutcome};
    let artifact = Artifact::describe(
        b"exact source",
        ArtifactKind::SourceFile,
        "text/plain",
        "https://example.org/p/1.0/source.py",
        enrichment_core::native_time::AcquisitionTime::try_from(
            "2026-09-14T00:00:00.000000Z".to_owned(),
        )
        .unwrap(),
    );
    let gap = Gap {
        kind: EvidenceKind::RuntimeApi,
        reason: GapReason::PolicyDenied,
        detail: "runtime profile not enabled".into(),
        planned_fallback: Some(PlannedFallback {
            producer: "python-runtime".into(),
            profile: ExecutionProfile::Runtime,
            enabled: false,
            next_action: "enable the runtime profile".into(),
        }),
    };
    let first = ProducerRun {
        attempt_id: "attempt_00000000000000000000000000000001"
            .to_owned()
            .try_into()
            .unwrap(),
        producer: "griffe-static".into(),
        producer_version: "1".into(),
        config_digest: enrichment_core::canonical::sha256_hex(b"config"),
        inputs: [("source".into(), artifact.sha256.clone())]
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
        outcome: RunOutcome::Partial,
        gaps: vec![gap.clone()],
        log: Some(String::new()),
    };
    let second = ProducerRun {
        attempt_id: "attempt_00000000000000000000000000000002"
            .to_owned()
            .try_into()
            .unwrap(),
        started_at: enrichment_core::native_time::ObservationTime::try_from(
            "2026-09-14T01:00:00.000000Z".to_owned(),
        )
        .unwrap(),
        finished_at: enrichment_core::native_time::ObservationTime::try_from(
            "2026-09-14T01:00:01.000000Z".to_owned(),
        )
        .unwrap(),
        log: None,
        ..first.clone()
    };
    assert_eq!(first.semantic_binding_id(), second.semantic_binding_id());
    let input =
        InputArtifact::new(first.semantic_binding_id(), "source".into(), &artifact).expect("input");
    let missing = CoverageFact::new(
        first.semantic_binding_id(),
        SubjectRef::Library {
            release_id: format!("rel_{}", "a".repeat(64)).try_into().unwrap(),
        },
        EvidenceKind::RuntimeApi,
        CoverageOutcome::Missing,
        vec![gap],
    )
    .expect("missing");
    let indexed = CoverageFact::new(
        first.semantic_binding_id(),
        SubjectRef::Library {
            release_id: format!("rel_{}", "a".repeat(64)).try_into().unwrap(),
        },
        EvidenceKind::PublicApi,
        CoverageOutcome::Indexed,
        vec![],
    )
    .expect("indexed empty allowed");
    assert_ne!(indexed.coverage_id, missing.coverage_id);
    let attempts = vec![first, second];
    let coverage = vec![indexed, missing];
    let dir = tempfile::tempdir().expect("directory");
    let ctx = SessionContext::new();
    for (name, batch) in [
        (
            "producer_runs",
            projection::producer_runs(&attempts).expect("runs"),
        ),
        (
            "input_artifacts",
            projection::input_artifacts(std::slice::from_ref(&input)).expect("inputs"),
        ),
        (
            "coverage",
            projection::coverage(&coverage).expect("coverage"),
        ),
    ] {
        let path = dir.path().join(format!("{name}.parquet"));
        parquet_fixture::write_parquet(&path, &batch, &[]).expect("write");
        ctx.register_parquet(
            name,
            path.to_str().expect("path"),
            ParquetReadOptions::default().skip_metadata(false),
        )
        .await
        .expect("register");
    }
    let runs = ctx
        .sql("SELECT * FROM datafusion.public.producer_runs ORDER BY attempt_id")
        .await
        .expect("plan")
        .collect()
        .await
        .expect("scan");
    assert_eq!(
        runs.iter()
            .flat_map(|b| projection::producer_runs_from_batch(b).expect("decode"))
            .collect::<Vec<_>>(),
        attempts
    );
    let inputs = ctx
        .sql("SELECT * FROM datafusion.public.input_artifacts")
        .await
        .expect("plan")
        .collect()
        .await
        .expect("scan");
    assert_eq!(
        projection::input_artifacts_from_batch(&inputs[0]).expect("decode"),
        vec![input]
    );
    let back = ctx
        .sql("SELECT * FROM datafusion.public.coverage ORDER BY kind")
        .await
        .expect("plan")
        .collect()
        .await
        .expect("scan");
    assert_eq!(
        back.iter()
            .flat_map(|b| projection::coverage_from_batch(b).expect("decode"))
            .collect::<Vec<_>>(),
        coverage
    );
}

#[tokio::test]
async fn release_metadata_roundtrip() {
    use enrichment_core::evidence::metadata::{ReleaseDetails, ReleaseMetadata};
    use enrichment_core::producer::{
        docsrs::DocsRsMetadata,
        python::{Distribution, ObservationOrigin, WorkerFile, inventory::Entry},
    };
    let mut source = observations()[0].source.clone();
    source.artifact_id = enrichment_core::evidence::artifact_id_for(&"a".repeat(64));
    let distribution = Distribution {
        filename: "pkg-1.0.whl".into(),
        sha256: "a".repeat(64),
        name: Some("pkg".into()),
        version: Some("1.0".into()),
        import_roots: vec!["pkg".into()],
        files: vec![WorkerFile {
            file: "pkg/__init__.pyi".into(),
            module: "pkg".into(),
            origin: ObservationOrigin::Stub,
        }],
        metadata: std::collections::BTreeMap::from([
            ("name".into(), vec!["pkg".into()]),
            ("version".into(), vec!["1.0".into()]),
            (
                "requires-dist".into(),
                vec!["first>=1".into(), "second; extra == 'x'".into()],
            ),
        ]),
        inventory: vec![Entry {
            name: "pkg.f".into(),
            role: "py:function".into(),
            priority: -1,
            uri: "https://example.org/f".into(),
            display: "function β".into(),
        }],
        entry_points: Some(String::new()),
        ..Distribution::default()
    };
    let mut rows = vec![
        ReleaseMetadata::new(
            format!("rel_{}", "a".repeat(64)).try_into().unwrap(),
            ReleaseDetails::RustDocs(DocsRsMetadata::default()),
            source.clone(),
        ),
        ReleaseMetadata::new(
            format!("rel_{}", "a".repeat(64)).try_into().unwrap(),
            ReleaseDetails::RustDocs(DocsRsMetadata {
                targets: Some(vec![]),
                ..DocsRsMetadata::default()
            }),
            source.clone(),
        ),
        ReleaseMetadata::new(
            format!("rel_{}", "a".repeat(64)).try_into().unwrap(),
            ReleaseDetails::PythonDistribution(distribution.clone()),
            source.clone(),
        ),
    ];
    for missing in [vec!["name"], vec!["version"], vec!["name", "version"]] {
        let mut unbuilt = distribution.clone();
        for key in missing {
            unbuilt.metadata.remove(key);
            if key == "name" {
                unbuilt.name = None;
            } else {
                unbuilt.version = None;
            }
        }
        rows.push(ReleaseMetadata::new(
            format!("rel_{}", "a".repeat(64)).try_into().unwrap(),
            ReleaseDetails::PythonDistribution(unbuilt),
            source.clone(),
        ));
    }
    for field in ["name", "version"] {
        for values in [
            vec![],
            vec![""],
            vec!["  "],
            vec!["pkg", "pkg"],
            vec!["pkg", "different"],
        ] {
            let mut invalid = distribution.clone();
            invalid.metadata.insert(
                field.into(),
                values.into_iter().map(str::to_owned).collect(),
            );
            let row = ReleaseMetadata::new(
                format!("rel_{}", "a".repeat(64)).try_into().unwrap(),
                ReleaseDetails::PythonDistribution(invalid),
                source.clone(),
            );
            assert!(projection::metadata::encode(&[row]).is_err());
        }
        for value in [None, Some("forged".into())] {
            let mut invalid = distribution.clone();
            if field == "name" {
                invalid.name = value;
            } else {
                invalid.version = value;
            }
            let row = ReleaseMetadata::new(
                format!("rel_{}", "a".repeat(64)).try_into().unwrap(),
                ReleaseDetails::PythonDistribution(invalid),
                source.clone(),
            );
            assert!(projection::metadata::encode(&[row]).is_err());
        }
    }
    let batch = projection::metadata::encode(&rows).expect("metadata projection");
    assert_eq!(projection::metadata::decode(&batch).expect("decode"), rows);
    let dir = tempfile::tempdir().expect("directory");
    let file = dir.path().join("metadata.parquet");
    parquet_fixture::write_parquet(&file, &batch, &[]).expect("write");
    let session = SessionContext::new();
    session
        .register_parquet(
            "metadata",
            file.to_str().expect("path"),
            ParquetReadOptions::default(),
        )
        .await
        .expect("register");
    let read = session
        .sql("SELECT * FROM metadata ORDER BY metadata_id")
        .await
        .expect("plan")
        .collect()
        .await
        .expect("execute");
    let mut decoded = read
        .iter()
        .flat_map(|b| projection::metadata::decode(b).expect("native read"))
        .collect::<Vec<_>>();
    let mut expected = rows;
    decoded.sort_by(|a, b| a.metadata_id.cmp(&b.metadata_id));
    expected.sort_by(|a, b| a.metadata_id.cmp(&b.metadata_id));
    assert_eq!(decoded, expected);
}
