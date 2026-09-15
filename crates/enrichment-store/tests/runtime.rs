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
}
