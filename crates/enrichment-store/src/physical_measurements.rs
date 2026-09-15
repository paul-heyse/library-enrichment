//! Manual paired experiments over the production codec, exact provider and runtime.
//! This is a bounded measurement harness, never an alternate production strategy.
use super::*;
use crate::{
    admission::Relation,
    runtime::{QueryLimits, QueryRuntime},
};
use arrow::record_batch::RecordBatch;
use datafusion::catalog::CatalogProvider;
use enrichment_core::{
    evidence::{
        SymbolKind,
        relational::{ApiObservation, ApiOrigin, ApiPayload, FactSource, Locator, SubjectRef},
    },
    wire::{EvidenceClass, SourceVersionMatch},
};
use parquet::{
    arrow::ArrowWriter,
    file::properties::{EnabledStatistics, WriterProperties},
};
use std::time::Instant;

fn observations(ecosystem: &str, rows: usize) -> Vec<ApiObservation> {
    (0..rows)
        .map(|i| {
            ApiObservation::new(
                SubjectRef::Symbol {
                    symbol_id: format!("sym_{i}"),
                },
                ApiOrigin::Source,
                "env_measure".into(),
                ApiPayload {
                    declared_kind: SymbolKind::Function,
                    signature: Some(if ecosystem == "rust" {
                        format!("fn value_{i}() -> usize")
                    } else {
                        format!("def value_{i}() -> int")
                    }),
                    doc_summary: Some(format!("Value {i}")),
                    docs: (i % 7 != 0)
                        .then(|| format!("{i}: {}", "typed documentation é ".repeat(128))),
                    deprecated: None,
                    cfg_hints: vec!["feature:default".into()],
                    python: None,
                },
                FactSource {
                    producer_binding_id: "producer_measure".into(),
                    extractor: ecosystem.into(),
                    extractor_version: "fixture".into(),
                    artifact_id: "art_measure".into(),
                    source_uri: None,
                    source_version_match: SourceVersionMatch::Exact,
                    locator: Locator::Lines {
                        file: Some("source.txt".into()),
                        start: 1,
                        end: 3,
                    },
                    evidence_class: EvidenceClass::StaticallyExtracted,
                },
            )
            .unwrap()
        })
        .collect()
}

#[derive(Clone, Copy)]
struct Layout {
    name: &'static str,
    group_rows: usize,
    file_rows: usize,
    pages: bool,
    fine_pages: bool,
    bloom: bool,
    groups: usize,
}

async fn files(
    root: &Path,
    batch: &RecordBatch,
    layout: Layout,
) -> (ExactParquet, serde_json::Value) {
    std::fs::create_dir(root).unwrap();
    let mut files = vec![];
    let start = Instant::now();
    let mut bytes = 0u64;
    let mut row_groups = 0;
    for offset in (0..batch.num_rows()).step_by(layout.file_rows) {
        let path = root.join(format!("{offset}.parquet"));
        let mut properties = WriterProperties::builder()
            .set_max_row_group_row_count(Some(layout.group_rows))
            .set_compression(parquet::basic::Compression::ZSTD(
                parquet::basic::ZstdLevel::try_new(3).unwrap(),
            ))
            .set_statistics_enabled(if layout.pages {
                EnabledStatistics::Page
            } else {
                EnabledStatistics::Chunk
            })
            .set_offset_index_disabled(!layout.pages);
        if layout.fine_pages {
            properties = properties
                .set_data_page_row_count_limit(128)
                .set_write_batch_size(128);
        }
        if layout.bloom {
            properties = properties
                .set_column_bloom_filter_enabled("observation_id".into(), true)
                .set_column_bloom_filter_max_ndv("observation_id".into(), layout.group_rows as u64);
        }
        let mut writer = ArrowWriter::try_new(
            std::fs::File::create(&path).unwrap(),
            batch.schema(),
            Some(properties.build()),
        )
        .unwrap();
        let part = batch.slice(offset, layout.file_rows.min(batch.num_rows() - offset));
        writer.write(&part).unwrap();
        writer.finish().unwrap();

        drop(writer);
        bytes += path.metadata().unwrap().len();
        files.push((path.clone(), FileWitness::read(&path).unwrap()));
    }
    let write_us = start.elapsed().as_micros();
    let start = Instant::now();
    for (path, _) in &files {
        let (digest, bytes) = enrichment_core::canonical::sha256_reader(
            std::fs::File::open(path).unwrap(),
            64 * 1024 * 1024,
        )
        .unwrap();
        let reader = parquet::arrow::arrow_reader::ParquetRecordBatchReaderBuilder::try_new(
            std::fs::File::open(path).unwrap(),
        )
        .unwrap();
        row_groups += reader.metadata().row_groups().len();
        let rows = reader.metadata().file_metadata().num_rows() as u64;
        crate::parquet_admission::validate_cancellable(
            crate::parquet_admission::Request {
                relation: crate::parquet_admission::Domain::Evidence(Relation::ApiObservations),
                path: path.clone(),
                digest,
                bytes,
                rows,
                limits: crate::admission::AdmissionLimits::default(),
            },
            &std::sync::atomic::AtomicBool::new(false),
        )
        .unwrap();
    }
    let admission_us = start.elapsed().as_micros();
    let start = Instant::now();
    let provider = ExactParquet::new_many(files, batch.schema(), Constraints::default())
        .await
        .unwrap()
        .with_verified_rows(batch.num_rows() as u64)
        .unwrap();
    let bind_us = start.elapsed().as_micros();
    (
        provider,
        serde_json::json!({"file_bytes":bytes,"write_us":write_us,"admission_us":admission_us,"provider_us":bind_us,"row_groups":row_groups}),
    )
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
#[ignore = "manual paired P8 physical strategy measurement"]
async fn paired_parquet_strategies() {
    let dir = tempfile::tempdir().unwrap();
    let base = Layout {
        name: "base",
        group_rows: 1024,
        file_rows: 4096,
        pages: true,
        fine_pages: false,
        bloom: false,
        groups: 1,
    };
    let layouts = [
        base,
        Layout {
            name: "group256",
            group_rows: 256,
            ..base
        },
        Layout {
            name: "group4096",
            group_rows: 4096,
            ..base
        },
        Layout {
            name: "small_files",
            file_rows: 256,
            ..base
        },
        Layout {
            name: "parallel_files",
            file_rows: 256,
            groups: 4,
            ..base
        },
        Layout {
            name: "fine_pages",
            fine_pages: true,
            ..base
        },
        Layout {
            name: "no_pages",
            pages: false,
            ..base
        },
        Layout {
            name: "key_bloom",
            bloom: true,
            ..base
        },
    ];
    for ecosystem in ["rust", "python"] {
        for rows in [32, 4096] {
            let observations = observations(ecosystem, rows);
            let batch = crate::projection::observations(&observations).unwrap();
            for round in 0..3 {
                let variants: Vec<_> = if round % 2 == 0 {
                    layouts.into_iter().collect()
                } else {
                    layouts.into_iter().rev().collect()
                };
                let mut expected = std::collections::BTreeMap::new();
                for layout in variants {
                    let root = dir
                        .path()
                        .join(format!("{ecosystem}-{rows}-{round}-{}", layout.name));
                    let (provider, construction) = files(&root, &batch, layout).await;
                    for decoder in [false, true] {
                        let runtime = QueryRuntime::new(
                            &root.join(format!("spill-{decoder}")),
                            QueryLimits {
                                partitions: layout.groups,
                                native: enrichment_core::config::NativeQueryConfig {
                                    decoder_filter: decoder,
                                    reorder_filters: decoder,
                                    ..Default::default()
                                },
                                ..Default::default()
                            },
                        )
                        .unwrap();
                        let catalog = crate::native_catalog::BoundCatalog::default().with_schema(
                            crate::native_catalog::BindingKind::AdmittedEvidence,
                            crate::native_catalog::Tables::from([(
                                "api_observations".into(),
                                Arc::new(provider.clone()) as Arc<dyn TableProvider>,
                            )]),
                        );
                        let session = runtime
                            .bound_session(std::collections::BTreeMap::from([(
                                "snapshot".into(),
                                Arc::new(catalog) as Arc<dyn CatalogProvider>,
                            )]))
                            .unwrap();
                        for clients in [1, 4] {
                            for query in ["id", "absent", "all"] {
                                let predicate = match query {
                                    "id" => format!(
                                        "observation_id = '{}'",
                                        observations[rows / 2].observation_id
                                    ),
                                    "absent" => "observation_id = 'obs_absent'".into(),
                                    _ => "true".into(),
                                };
                                let sql = format!(
                                    "SELECT count(*) AS rows, sum(length(payload.docs)) AS bytes FROM snapshot.evidence.api_observations WHERE {predicate}"
                                );
                                for warm in [false, true] {
                                    let start = Instant::now();
                                    let mut futures = vec![];
                                    for _ in 0..clients {
                                        futures.push(
                                            runtime.execute(session.sql(&sql).await.unwrap()),
                                        );
                                    }
                                    for output in futures::future::join_all(futures).await {
                                        let output = output.unwrap();
                                        let value = arrow::util::pretty::pretty_format_batches(
                                            &output.batches,
                                        )
                                        .unwrap()
                                        .to_string();
                                        assert_eq!(
                                            expected.entry(query).or_insert_with(|| value.clone()),
                                            &value
                                        );
                                    }
                                    let wall_us = start.elapsed().as_micros();
                                    eprintln!(
                                        "P14_PARQUET {}",
                                        serde_json::json!({"ecosystem":ecosystem,"rows":rows,"round":round,"layout":layout.name,"decoder":decoder,"clients":clients,"query":query,"warm":warm,"wall_us":wall_us,"construction":construction,"counters":runtime.operational_counters(),"last_plan":runtime.diagnostics().last()})
                                    );
                                }
                            }
                        }
                        assert_eq!(
                            runtime.operational_counters().managed_memory_reserved_bytes,
                            0
                        );
                    }
                }
            }
        }
    }
}

#[tokio::test]
async fn exact_multi_file_counts_keep_nullable_filters_and_limits_correct() {
    let dir = tempfile::tempdir().unwrap();
    let values = observations("rust", 3);
    let batch = crate::projection::observations(&values).unwrap();
    let layout = Layout {
        name: "fixture",
        group_rows: 2,
        file_rows: 2,
        pages: true,
        fine_pages: false,
        bloom: true,
        groups: 2,
    };
    let (provider, _) = files(&dir.path().join("files"), &batch, layout).await;
    let runtime = QueryRuntime::new(
        &dir.path().join("spill"),
        QueryLimits {
            partitions: 4,
            ..Default::default()
        },
    )
    .unwrap();
    let session = runtime.session();
    crate::native_catalog::work(&session, "observations", Arc::new(provider)).unwrap();
    for (sql, expected) in [
        ("SELECT count(*) AS count FROM observations", 3),
        ("SELECT count(payload.docs) AS count FROM observations", 2),
        (
            "SELECT count(*) AS count FROM observations WHERE payload.docs IS NULL",
            1,
        ),
        (
            "SELECT count(*) AS count FROM (SELECT * FROM observations LIMIT 1)",
            1,
        ),
    ] {
        let output = runtime
            .execute(session.sql(sql).await.unwrap())
            .await
            .unwrap();
        assert_eq!(
            output.batches[0]
                .column(0)
                .as_any()
                .downcast_ref::<arrow::array::Int64Array>()
                .unwrap()
                .value(0),
            expected
        );
    }
}
