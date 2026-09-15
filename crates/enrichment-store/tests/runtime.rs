#[path = "support/write_parquet.rs"]
mod parquet_fixture;

use arrow::{
    array::{ArrayRef, Int64Array, StringArray},
    datatypes::{DataType, Field, Schema},
    record_batch::RecordBatch,
};
use datafusion::prelude::ParquetReadOptions;
use enrichment_store::runtime::{QueryLimits, QueryRuntime};
use std::sync::Arc;

fn descriptor() -> enrichment_store::runtime::OperationDescriptor {
    enrichment_store::runtime::OperationDescriptor {
        method: "native-runtime-fixture".into(),
        request_digest: "0".repeat(64),
        policy_digest: "1".repeat(64),
    }
}

#[tokio::test]
async fn native_coercion_and_product_udfs_survive_empty_scalar_and_partitioned_execution() {
    use datafusion::datasource::MemTable;
    use enrichment_core::search::spec::SearchSpec;
    use enrichment_store::scoring::{ScoreKind, function};
    let dir = tempfile::tempdir().unwrap();
    for partitions in [1, 4] {
        let runtime = QueryRuntime::new(
            &dir.path().join(format!("spill-{partitions}")),
            QueryLimits {
                partitions,
                ..Default::default()
            },
        )
        .unwrap();
        let session = runtime.session();
        let schema = Arc::new(Schema::new(vec![Field::new("path", DataType::Utf8, false)]));
        let batch = RecordBatch::try_new(
            schema.clone(),
            vec![Arc::new(StringArray::from(vec!["Widget", "Shape"]))],
        )
        .unwrap();
        session
            .register_table(
                "input",
                Arc::new(MemTable::try_new(schema, vec![vec![batch]; partitions]).unwrap()),
            )
            .unwrap();
        for expression in ["path", "named_struct('path', path)", "make_array(path)"] {
            let sql = format!(
                "SELECT {expression} AS value FROM (SELECT path FROM input UNION ALL SELECT CAST(NULL AS VARCHAR) AS path)"
            );
            let output = runtime
                .execute(session.sql(&sql).await.unwrap())
                .await
                .unwrap();
            assert_eq!(output.rows, partitions * 2 + 1);
        }
        let udf = function(ScoreKind::Fragment, SearchSpec::new("Widget"));
        assert_ne!(udf, function(ScoreKind::Fragment, SearchSpec::new("Shape")));
        session.register_udf(udf);
        for (sql, rows) in [
            (
                "SELECT evidence_fragment_score_v2('Widget', 'Widget docs') AS score",
                1,
            ),
            (
                "SELECT evidence_fragment_score_v2(path, path) AS score FROM input",
                partitions * 2,
            ),
            (
                "SELECT evidence_fragment_score_v2(path, path) AS score FROM input WHERE false",
                0,
            ),
        ] {
            let frame = session.sql(sql).await.unwrap();
            let field = frame.schema().as_arrow().field(0).clone();
            assert!(field.is_nullable(), "unmatched score is null");
            assert!(
                field
                    .metadata()
                    .get("enrichment.function")
                    .unwrap()
                    .starts_with("evidence_fragment_score_v2:")
            );
            enrichment_store::preparation::require_fields(
                frame.schema().as_arrow(),
                &[field],
                "product_udf",
            )
            .unwrap();
            let output = runtime.execute(frame).await.unwrap();
            assert_eq!(output.rows, rows);
            for batch in output.batches {
                assert!(
                    batch
                        .schema()
                        .field(0)
                        .metadata()
                        .contains_key("enrichment.function")
                );
            }
        }
        assert!(runtime.diagnostics().last().unwrap().completed);
    }
}

#[tokio::test]
async fn blocking_projection_keeps_the_operation_identity_and_result_budget() {
    let dir = tempfile::tempdir().unwrap();
    let runtime = QueryRuntime::new(
        &dir.path().join("spill"),
        QueryLimits {
            result_bytes: 100,
            ..Default::default()
        },
    )
    .unwrap();
    runtime
        .operation("blocking-result".into(), descriptor(), async {
            enrichment_store::runtime::charge_result(60).unwrap();
            let context = enrichment_store::runtime::capture_operation();
            let failure = tokio::task::spawn_blocking(move || {
                context.run(|| {
                    assert_eq!(
                        enrichment_store::runtime::operation_id().as_deref(),
                        Some("blocking-result")
                    );
                    enrichment_store::runtime::charge_result(60).unwrap_err()
                })
            })
            .await
            .unwrap();
            let diagnostic = enrichment_store::QueryError::from(failure).diagnostic();
            assert_eq!(diagnostic.observed, Some(120));
            assert_eq!(diagnostic.allowed, Some(100));
            assert_eq!(
                diagnostic.correlation_id.as_deref(),
                Some("blocking-result")
            );
        })
        .await
        .unwrap();
}

#[tokio::test]
async fn a_timed_out_blocking_read_keeps_admission_until_its_worker_exits() {
    let dir = tempfile::tempdir().unwrap();
    let runtime = QueryRuntime::new(
        &dir.path().join("spill"),
        QueryLimits {
            concurrency: 1,
            deadline: std::time::Duration::from_millis(40),
            ..Default::default()
        },
    )
    .unwrap();
    let (started, running) = tokio::sync::oneshot::channel();
    let (release, wait) = std::sync::mpsc::sync_channel(1);
    let owned = runtime.clone();
    let request = tokio::spawn(async move {
        owned
            .operation("timed-out-reader".into(), descriptor(), async {
                owned
                    .blocking(move || {
                        started.send(()).unwrap();
                        wait.recv_timeout(std::time::Duration::from_secs(2))
                            .unwrap();
                    })
                    .await
            })
            .await
    });
    running.await.unwrap();
    assert!(request.await.unwrap().is_err());
    assert_eq!(runtime.operational_counters().admitted, 1);
    let query = runtime.session().sql("SELECT 1").await.unwrap();
    assert!(
        tokio::time::timeout(std::time::Duration::from_millis(10), runtime.execute(query))
            .await
            .is_err()
    );
    release.send(()).unwrap();
    let query = runtime.session().sql("SELECT 1").await.unwrap();
    assert_eq!(runtime.execute(query).await.unwrap().rows, 1);
    assert_eq!(runtime.operational_counters().admitted, 0);
}

#[tokio::test]
async fn a_timed_out_arrow_sink_keeps_admission_until_its_worker_exits() {
    let dir = tempfile::tempdir().unwrap();
    let runtime = QueryRuntime::new(
        &dir.path().join("spill"),
        QueryLimits {
            concurrency: 1,
            deadline: std::time::Duration::from_millis(40),
            ..Default::default()
        },
    )
    .unwrap();
    let (started, running) = tokio::sync::oneshot::channel();
    let (release, wait) = std::sync::mpsc::sync_channel(1);
    let owned = runtime.clone();
    let request = tokio::spawn(async move {
        owned
            .operation("timed-out-reader".into(), descriptor(), async {
                owned
                    .fold_blocking(
                        owned
                            .session()
                            .sql("SELECT count(*) AS count FROM (VALUES (1))")
                            .await
                            .unwrap(),
                        enrichment_store::preparation::QueryFamily::Count,
                        1,
                        (Some(started), wait),
                        |(mut started, wait), batch| {
                            assert_eq!(batch.num_rows(), 1);
                            started.take().unwrap().send(()).unwrap();
                            wait.recv_timeout(std::time::Duration::from_secs(2))
                                .unwrap();
                            Ok((started, wait))
                        },
                    )
                    .await
            })
            .await
    });
    running.await.unwrap();
    assert!(matches!(request.await.unwrap(), Err(_) | Ok(Err(_))));
    assert_eq!(runtime.operational_counters().admitted, 1);
    let query = runtime.session().sql("SELECT 1").await.unwrap();
    assert!(
        tokio::time::timeout(std::time::Duration::from_millis(10), runtime.execute(query))
            .await
            .is_err()
    );
    release.send(()).unwrap();
    let query = runtime.session().sql("SELECT 1").await.unwrap();
    assert_eq!(runtime.execute(query).await.unwrap().rows, 1);
    assert_eq!(runtime.operational_counters().admitted, 0);
}

#[tokio::test]
async fn operation_shares_materialization_budget_and_does_not_reacquire_its_permit() {
    let dir = tempfile::tempdir().unwrap();
    let probe = QueryRuntime::new(&dir.path().join("probe"), QueryLimits::default()).unwrap();
    let query = "SELECT repeat('a', 256) AS value";
    let bytes = probe
        .execute(probe.session().sql(query).await.unwrap())
        .await
        .unwrap()
        .arrow_bytes;
    let runtime = QueryRuntime::new(
        &dir.path().join("spill"),
        QueryLimits {
            concurrency: 1,
            result_bytes: bytes * 2 - 1,
            ..Default::default()
        },
    )
    .unwrap();
    let session = runtime.session();
    let result = runtime
        .operation("outer-request".into(), descriptor(), async {
            let first = runtime
                .execute(session.sql(query).await.unwrap())
                .await
                .unwrap();
            // A nested operation is the same owner, even when a caller supplies another label.
            runtime
                .operation("nested".into(), descriptor(), async {
                    let failure = runtime
                        .execute(session.sql(query).await.unwrap())
                        .await
                        .unwrap_err();
                    let diagnostic = enrichment_store::QueryError::from(failure).diagnostic();
                    assert_eq!(
                        diagnostic.cause,
                        enrichment_core::wire::DiagnosticCause::Capacity
                    );
                    assert_eq!(diagnostic.correlation_id.as_deref(), Some("outer-request"));
                    assert!(diagnostic.observed > diagnostic.allowed);
                })
                .await
                .unwrap();
            drop(first);
        })
        .await;
    assert!(result.is_ok());
    assert!(
        runtime
            .diagnostics()
            .iter()
            .all(|q| q.operation_id.as_deref() == Some("outer-request"))
    );
    assert!(runtime.diagnostics().iter().all(|q| {
        q.binding.as_ref().is_some_and(|binding| {
            binding.request.method == "native-runtime-fixture"
                && binding.request.policy_digest == "1".repeat(64)
                && binding.retained_byte_limit == bytes * 2 - 1
        })
    }));
    assert!(
        runtime
            .execute(session.sql(query).await.unwrap())
            .await
            .is_ok(),
        "failure releases admission"
    );
}

#[tokio::test]
async fn operation_deadline_includes_non_query_result_work_and_preserves_origin() {
    let dir = tempfile::tempdir().unwrap();
    let runtime = QueryRuntime::new(
        &dir.path().join("spill"),
        QueryLimits {
            concurrency: 1,
            deadline: std::time::Duration::from_millis(20),
            ..Default::default()
        },
    )
    .unwrap();
    let error = runtime
        .operation("slow-result".into(), descriptor(), async {
            tokio::time::sleep(std::time::Duration::from_millis(100)).await;
        })
        .await
        .unwrap_err();
    let diagnostic = enrichment_store::QueryError::from(error).diagnostic();
    assert_eq!(
        diagnostic.cause,
        enrichment_core::wire::DiagnosticCause::Deadline
    );
    assert_eq!(diagnostic.correlation_id.as_deref(), Some("slow-result"));
    runtime
        .operation("next".into(), descriptor(), async {})
        .await
        .unwrap();
}

#[tokio::test]
async fn native_memory_pressure_preserves_owner_and_releases_admission() {
    let dir = tempfile::tempdir().unwrap();
    let runtime = QueryRuntime::new(
        dir.path(),
        QueryLimits {
            memory_bytes: 128,
            concurrency: 1,
            ..Default::default()
        },
    )
    .unwrap();
    let query = "SELECT CAST(value AS VARCHAR) || repeat('x', 128) AS key
        FROM unnest(range(0, 10000)) AS t(value) ORDER BY key DESC LIMIT 100";
    runtime
        .operation("memory-pressure".into(), descriptor(), async {
            let frame = runtime.session().sql(query).await.unwrap();
            let error = runtime.execute(frame).await.unwrap_err();
            let diagnostic = enrichment_store::QueryError::from(error).diagnostic();
            assert_eq!(
                diagnostic.cause,
                enrichment_core::wire::DiagnosticCause::Capacity
            );
            assert_eq!(
                diagnostic.correlation_id.as_deref(),
                Some("memory-pressure")
            );
        })
        .await
        .unwrap();
    assert_eq!(runtime.session().runtime_env().memory_pool.reserved(), 0);
    assert_eq!(runtime.operational_counters().admitted, 0);
    assert_eq!(
        runtime
            .execute(runtime.session().sql("SELECT 1").await.unwrap())
            .await
            .unwrap()
            .rows,
        1
    );
}

#[tokio::test]
async fn native_session_templates_isolate_catalogs_and_execution_metrics() {
    let dir = tempfile::tempdir().unwrap();
    let runtime = QueryRuntime::new(&dir.path().join("spill"), QueryLimits::default()).unwrap();
    let left = runtime.session();
    let right = runtime.session();
    left.register_table(
        "private_values",
        left.sql("SELECT upper('left') AS value")
            .await
            .unwrap()
            .into_view(),
    )
    .unwrap();
    assert!(right.table("private_values").await.is_err());
    right
        .register_table(
            "private_values",
            right
                .sql("SELECT upper(value) AS value FROM (VALUES ('right'), ('second')) AS t(value)")
                .await
                .unwrap()
                .into_view(),
        )
        .unwrap();
    let (one, two) = tokio::join!(
        runtime.execute(left.table("private_values").await.unwrap()),
        runtime.execute(right.table("private_values").await.unwrap()),
    );
    assert_eq!(one.unwrap().rows, 1);
    assert_eq!(two.unwrap().rows, 2);
    let diagnostics = runtime.diagnostics();
    let mut rows = diagnostics
        .iter()
        .map(|d| d.output_rows)
        .collect::<Vec<_>>();
    rows.sort_unstable();
    assert_eq!(rows, [1, 2]);
    assert_ne!(diagnostics[0].query_id, diagnostics[1].query_id);
    assert!(runtime.session().table("private_values").await.is_err());
    left.deregister_table("private_values").unwrap();
    assert_eq!(
        runtime
            .execute(right.table("private_values").await.unwrap())
            .await
            .unwrap()
            .rows,
        2
    );
}

#[tokio::test]
async fn diagnostics_observe_the_executed_scan_and_bound_failed_query_history() {
    let dir = tempfile::tempdir().unwrap();
    let file = dir.path().join("values.parquet");
    let schema = Arc::new(Schema::new(vec![
        Field::new("id", DataType::Int64, false),
        Field::new("value", DataType::Utf8, false),
    ]));
    let arrays: Vec<ArrayRef> = vec![
        Arc::new(Int64Array::from_iter_values(0..4096)),
        Arc::new(StringArray::from_iter_values(
            (0..4096).map(|i| format!("retained-{i}")),
        )),
    ];
    let batch = RecordBatch::try_new(Arc::clone(&schema), arrays).unwrap();
    parquet_fixture::write_parquet(&file, &batch, &[]).unwrap();
    let runtime = QueryRuntime::new(
        &dir.path().join("spill"),
        QueryLimits {
            result_rows: 2,
            concurrency: 1,
            ..Default::default()
        },
    )
    .unwrap();
    let session = runtime.session();
    session
        .register_parquet("t", file.to_str().unwrap(), ParquetReadOptions::default())
        .await
        .unwrap();
    let output = runtime
        .execute(
            session
                .sql("SELECT id FROM t WHERE id > 4092 ORDER BY id LIMIT 2")
                .await
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(output.rows, 2);
    let records = runtime.diagnostics();
    let record = records.last().unwrap();
    assert!(record.completed && !record.truncated);
    assert_eq!(record.output_rows, 2);
    assert!(record.logical.contains("Filter") && record.physical.contains("DataSourceExec"));
    assert!(
        record
            .metrics
            .iter()
            .any(|m| m.name == "bytes_scanned" && m.value.is_some_and(|v| v > 0))
    );
    assert!(
        record
            .metrics
            .iter()
            .filter(|m| m.unit == "pruning")
            .all(|m| m.value.is_none() && m.pruned.is_some() && m.matched.is_some())
    );
    assert!(
        runtime
            .execute(session.sql("SELECT id FROM t LIMIT 3").await.unwrap())
            .await
            .is_err()
    );
    assert!(!runtime.diagnostics().last().unwrap().completed);
    for _ in 0..10 {
        assert_eq!(
            runtime
                .execute(session.sql("SELECT id FROM t LIMIT 1").await.unwrap())
                .await
                .unwrap()
                .rows,
            1
        );
    }
    assert_eq!(runtime.diagnostics().len(), 8);
    assert!(serde_json::to_vec(&runtime.diagnostics()).unwrap().len() < 8 * 512 * 1024);
    let retained = dir.path().join("spill/query-failures.json");
    let failure: serde_json::Value =
        serde_json::from_slice(&std::fs::read(&retained).unwrap()).unwrap();
    assert_eq!(failure.as_array().unwrap().len(), 1);
    assert_eq!(failure[0]["diagnostic"]["cause"], "capacity");
    drop(runtime);
    let reopened = QueryRuntime::new(&dir.path().join("spill"), QueryLimits::default()).unwrap();
    assert!(reopened.diagnostics().is_empty());
    assert_eq!(
        serde_json::from_slice::<serde_json::Value>(&std::fs::read(retained).unwrap()).unwrap(),
        failure
    );
}

#[tokio::test]
async fn native_field_contracts_accept_outer_join_nulls_and_reject_missing_roles() {
    use datafusion::{common::metadata::FieldMetadata, prelude::col};
    use enrichment_store::preparation::require_fields;
    let dir = tempfile::tempdir().unwrap();
    let runtime = QueryRuntime::new(&dir.path().join("spill"), QueryLimits::default()).unwrap();
    let session = runtime.session();
    let role =
        std::collections::HashMap::from([("enrichment.role".into(), "optional-reference".into())]);
    let required = Field::new("reference", DataType::Int64, true).with_metadata(role.clone());
    for empty in [false, true] {
        let mut frame = session.sql("SELECT r.id AS reference FROM (VALUES (1::BIGINT)) AS l(id) LEFT JOIN (VALUES (2::BIGINT)) AS r(id) ON false").await.unwrap()
            .select(vec![col("reference").alias_with_metadata("reference", Some(FieldMetadata::from(role.clone()))) ]).unwrap();
        if empty {
            frame = frame.filter(datafusion::prelude::lit(false)).unwrap();
        }
        require_fields(
            frame.schema().as_arrow(),
            std::slice::from_ref(&required),
            "logical",
        )
        .unwrap();
        let output = runtime.execute(frame).await.unwrap();
        assert_eq!(output.rows, usize::from(!empty));
        for batch in output.batches {
            require_fields(&batch.schema(), std::slice::from_ref(&required), "actual").unwrap();
        }
    }
    let invalid = Schema::new(vec![Field::new("reference", DataType::Int64, true)]);
    let error = enrichment_store::QueryError::from(
        require_fields(&invalid, &[required], "fixture").unwrap_err(),
    );
    assert_eq!(error.diagnostic().affected_ids, ["reference"]);
    assert_eq!(error.diagnostic().stage, "fixture");
    assert_eq!(
        error.diagnostic().cause,
        enrichment_core::wire::DiagnosticCause::Internal
    );
    assert!(matches!(
        error.diagnostic().actions[0],
        enrichment_core::wire::RecoveryAction::ReportDefect { .. }
    ));
}

#[tokio::test]
async fn durable_jobs_have_independent_correlation_and_release_output_ownership() {
    let dir = tempfile::tempdir().unwrap();
    let runtime = QueryRuntime::new(
        &dir.path().join("spill"),
        QueryLimits {
            concurrency: 1,
            ..Default::default()
        },
    )
    .unwrap();
    runtime
        .operation("caller".into(), descriptor(), async {
            let child = runtime.clone();
            tokio::spawn(async move {
                child
                    .job_operation(
                        "job-owned".into(),
                        descriptor(),
                        std::time::Duration::from_secs(2),
                        async {
                            child
                                .execute(child.session().sql("SELECT 1").await.unwrap())
                                .await
                                .unwrap();
                        },
                    )
                    .await;
            })
            .await
            .unwrap();
            runtime
                .execute(runtime.session().sql("SELECT 2").await.unwrap())
                .await
                .unwrap();
        })
        .await
        .unwrap();
    assert_eq!(
        runtime
            .diagnostics()
            .iter()
            .map(|q| q.operation_id.as_deref())
            .collect::<Vec<_>>(),
        [Some("job-owned"), Some("caller")]
    );
}

#[tokio::test]
async fn preparation_failure_retains_its_own_query_identity() {
    use datafusion::{
        dataframe::DataFrame,
        logical_expr::{Expr, LogicalPlanBuilder},
        prelude::lit,
    };
    let dir = tempfile::tempdir().unwrap();
    let runtime = QueryRuntime::new(&dir.path().join("spill"), QueryLimits::default()).unwrap();
    let session = runtime.session();
    runtime
        .execute(session.sql("SELECT 1").await.unwrap())
        .await
        .unwrap();
    let mut plan = LogicalPlanBuilder::empty(true)
        .project(vec![lit(1)])
        .unwrap()
        .build()
        .unwrap();
    let datafusion::logical_expr::LogicalPlan::Projection(projection) = &mut plan else {
        panic!("projection")
    };
    projection.expr = vec![Expr::BinaryExpr(datafusion::logical_expr::BinaryExpr::new(
        Box::new(lit(true)),
        datafusion::logical_expr::Operator::Plus,
        Box::new(lit(false)),
    ))];
    let frame = DataFrame::new(session.state(), plan);
    assert!(runtime.execute(frame).await.is_err());
    let records = runtime.diagnostics();
    assert_eq!(records.len(), 2);
    assert_ne!(records[0].query_id, records[1].query_id);
    assert!(!records[1].completed);
    assert_eq!(records[1].stage, "analysis");
    assert!(records[1].physical.is_empty());
}

#[tokio::test]
async fn independent_family_requirement_fails_before_execution_with_its_own_trace() {
    let dir = tempfile::tempdir().unwrap();
    let runtime = QueryRuntime::new(&dir.path().join("spill"), QueryLimits::default()).unwrap();
    let error = runtime
        .execute_family(
            runtime.session().sql("SELECT 1 AS kind").await.unwrap(),
            Some(enrichment_store::preparation::QueryFamily::Coverage),
        )
        .await
        .unwrap_err();
    let diagnostic = enrichment_store::QueryError::from(error).diagnostic();
    assert_eq!(
        diagnostic.cause,
        enrichment_core::wire::DiagnosticCause::Internal
    );
    assert_eq!(diagnostic.affected_ids, ["kind"]);
    let traces = runtime.diagnostics();
    assert_eq!(traces.len(), 1);
    assert_eq!(traces[0].family.as_deref(), Some("Coverage"));
    assert!(!traces[0].completed);
    assert!(traces[0].physical.is_empty());
}

#[tokio::test]
async fn invariant_witnesses_are_bounded_and_retained_with_the_owning_operation() {
    let dir = tempfile::tempdir().unwrap();
    let runtime = QueryRuntime::new(&dir.path().join("spill"), QueryLimits::default()).unwrap();
    let session = runtime.session();
    runtime
        .require_empty(
            session
                .sql("SELECT 'none' AS artifact_id WHERE false")
                .await
                .unwrap(),
            "fixture closure",
            "fixture_admission",
        )
        .await
        .unwrap();
    let error = runtime.operation("witness-operation".into(), descriptor(), async {
        runtime.require_empty(session.sql("SELECT CAST(value AS VARCHAR) AS artifact_id FROM generate_series(1, 100) ORDER BY value").await.unwrap(),
            "fixture closure", "fixture_admission").await
    }).await.unwrap().unwrap_err();
    let diagnostic = enrichment_store::QueryError::from(error).diagnostic();
    assert_eq!(
        diagnostic.cause,
        enrichment_core::wire::DiagnosticCause::CorruptState
    );
    assert_eq!(
        diagnostic.affected_ids,
        (1..=8).map(|i| i.to_string()).collect::<Vec<_>>()
    );
    assert_eq!(
        diagnostic.correlation_id.as_deref(),
        Some("witness-operation")
    );
    let saved: serde_json::Value = serde_json::from_slice(
        &std::fs::read(dir.path().join("spill/query-failures.json")).unwrap(),
    )
    .unwrap();
    assert_eq!(
        saved[0]["last_operation_query"]["family"],
        "InvariantWitness"
    );
    assert_eq!(saved[0]["last_operation_query"]["output_rows"], 8);
    assert_eq!(saved[0]["diagnostic"]["affected_ids"][7], "8");
}
