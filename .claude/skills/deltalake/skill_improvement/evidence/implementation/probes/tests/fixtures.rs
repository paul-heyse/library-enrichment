use arrow_array::RecordBatch;
use deltalake::{DeltaTable, DeltaTableBuilder};
use futures::TryStreamExt;
use std::path::{Path, PathBuf};

fn fixture(name: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("fixtures")
        .join(name)
}

#[tokio::test]
async fn deletion_vectors_filter_rows_and_optimize_keeps_logical_rows() {
    fn copy_tree(from: &Path, to: &Path) {
        std::fs::create_dir_all(to).unwrap();
        for entry in std::fs::read_dir(from).unwrap() {
            let entry = entry.unwrap();
            let target = to.join(entry.file_name());
            if entry.path().is_dir() {
                copy_tree(&entry.path(), &target);
            } else {
                std::fs::copy(entry.path(), target).unwrap();
            }
        }
    }
    let temp = tempfile::tempdir().unwrap();
    copy_tree(&fixture("table-with-dv-small"), temp.path());
    let table = DeltaTableBuilder::from_url(url::Url::from_directory_path(temp.path()).unwrap())
        .unwrap()
        .load()
        .await
        .unwrap();
    async fn logical(table: &DeltaTable) -> Vec<i64> {
        let (_, stream) = table.scan_table().await.unwrap();
        let batches: Vec<RecordBatch> = stream.try_collect().await.unwrap();
        let mut values = Vec::new();
        for b in batches {
            let a = arrow_cast::cast(b.column(0), &arrow_schema::DataType::Int64).unwrap();
            values.extend(
                a.as_any()
                    .downcast_ref::<arrow_array::Int64Array>()
                    .unwrap()
                    .values()
                    .iter()
                    .copied(),
            );
        }
        values.sort();
        values
    }
    assert_eq!(logical(&table).await, (1..9).collect::<Vec<_>>());
    let ctx = datafusion::prelude::SessionContext::new();
    let raw = ctx
        .read_parquet(temp.path().to_str().unwrap(), Default::default())
        .await
        .unwrap()
        .collect()
        .await
        .unwrap();
    assert_eq!(raw.iter().map(RecordBatch::num_rows).sum::<usize>(), 10);
    let (table, _) = table.optimize().await.unwrap();
    assert_eq!(logical(&table).await, (1..9).collect::<Vec<_>>());
    let (table, _) = table
        .delete()
        .with_predicate(datafusion::prelude::col("value").eq(datafusion::prelude::lit(1_i64)))
        .await
        .unwrap();
    assert_eq!(logical(&table).await, (2..9).collect::<Vec<_>>());
}

#[tokio::test]
async fn column_mapping_reads_logical_names_but_cdf_rejects_mapping() {
    let table = DeltaTableBuilder::from_url(
        url::Url::from_directory_path(fixture("table_with_column_mapping")).unwrap(),
    )
    .unwrap()
    .load()
    .await
    .unwrap();
    let (_, stream) = table.scan_table().await.unwrap();
    let batches: Vec<RecordBatch> = stream.try_collect().await.unwrap();
    assert_eq!(batches.iter().map(RecordBatch::num_rows).sum::<usize>(), 5);
    assert!(
        batches[0]
            .schema()
            .field_with_name("Company Very Short")
            .is_ok()
    );
    assert!(batches[0].schema().field_with_name("Super Name").is_ok());
    let ctx = datafusion::prelude::SessionContext::new();
    let error = table
        .scan_cdf()
        .with_starting_version(0)
        .build(&ctx.state(), None)
        .await
        .unwrap_err();
    assert!(error.to_string().to_lowercase().contains("column mapping"));
}

#[test]
fn recognized_feature_and_excluded_profile_are_not_reader_support() {
    use deltalake::kernel::Protocol;
    use deltalake::kernel::transaction::PROTOCOL;
    for feature in ["identityColumns", "timestampNanos"] {
        let protocol: Protocol = serde_json::from_value(
            serde_json::json!({"minReaderVersion":3,"minWriterVersion":7,
            "readerFeatures":[feature],"writerFeatures":[feature]}),
        )
        .unwrap();
        assert!(PROTOCOL.can_read_from_protocol(&protocol).is_err());
    }
    let protocol: Protocol = serde_json::from_value(
        serde_json::json!({"minReaderVersion":3,"minWriterVersion":7,
        "readerFeatures":["deletionVectors"],"writerFeatures":["deletionVectors"]}),
    )
    .unwrap();
    assert!(PROTOCOL.can_read_from_protocol(&protocol).is_ok());
}
