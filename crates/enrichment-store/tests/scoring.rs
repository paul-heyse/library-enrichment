use arrow::{
    array::{Array, BooleanArray, StringViewArray, StructArray, UInt32Array},
    datatypes::{DataType, Field, Schema},
    record_batch::RecordBatch,
};
use datafusion::{
    datasource::MemTable,
    functions::core::expr_ext::FieldAccessor,
    prelude::{col, lit},
};
use enrichment_core::search::spec::SearchSpec;
use enrichment_store::{
    projection::TextColumn,
    runtime::{QueryLimits, QueryRuntime},
    scoring::{self, ScoreKind},
};
use std::sync::Arc;

#[tokio::test]
async fn native_eligibility_includes_summary_short_queries_and_excludes_bonus_only_rows() {
    let dir = tempfile::tempdir().expect("directory");
    let runtime =
        QueryRuntime::new(&dir.path().join("spill"), QueryLimits::default()).expect("runtime");
    let session = runtime.session();
    let schema = Arc::new(Schema::new(vec![
        Field::new("path", DataType::Utf8View, false),
        Field::new("name", DataType::Utf8View, false),
        Field::new("signature", DataType::Utf8View, true),
        Field::new("doc_summary", DataType::Utf8View, true),
        Field::new("docs", DataType::Utf8View, true),
        Field::new("is_reexport", DataType::Boolean, false),
    ]));
    let batch = RecordBatch::try_new(
        schema.clone(),
        vec![
            Arc::new(StringViewArray::from(vec![
                "p::alpha",
                "p::X",
                "p::unmatched",
            ])),
            Arc::new(StringViewArray::from(vec!["alpha", "X", "unmatched"])),
            Arc::new(StringViewArray::from(vec![None::<&str>; 3])),
            Arc::new(StringViewArray::from(vec![
                Some("Needle exists only in the summary"),
                None,
                None,
            ])),
            Arc::new(StringViewArray::from(vec![None::<&str>; 3])),
            Arc::new(BooleanArray::from(vec![false; 3])),
        ],
    )
    .expect("batch");
    session
        .register_table(
            "candidates",
            Arc::new(MemTable::try_new(schema, vec![vec![batch]]).expect("table")),
        )
        .expect("register");
    for (query, path, score) in [("needle", "p::alpha", 85), ("X", "p::X", 805)] {
        let spec = SearchSpec::new(query);
        let frame = session
            .table("candidates")
            .await
            .expect("table")
            .filter(scoring::eligibility(&spec.symbol_clauses))
            .expect("eligibility");
        let native_plan = frame.logical_plan().display_indent().to_string();
        assert!(!native_plan.contains("evidence_symbol_score"));
        let udf = scoring::function(ScoreKind::Symbol, spec);
        let frame = frame
            .select(vec![
                col("path"),
                udf.call(vec![
                    col("path"),
                    col("name"),
                    col("signature"),
                    col("doc_summary"),
                    col("docs"),
                    col("is_reexport"),
                ])
                .alias("ranking"),
            ])
            .expect("projection");
        let output = runtime.execute(frame).await.expect("execute");
        assert_eq!(output.rows, 1);
        let batch = &output.batches[0];
        assert_eq!(
            TextColumn::new(batch.column(0).as_ref())
                .expect("text")
                .get(0),
            Some(path)
        );
        let ranking = batch
            .column(1)
            .as_any()
            .downcast_ref::<StructArray>()
            .expect("structured ranking");
        assert_eq!(
            ranking
                .column_by_name("score")
                .expect("score")
                .as_any()
                .downcast_ref::<UInt32Array>()
                .expect("integer")
                .value(0),
            score
        );
        assert!(!ranking.is_null(0));
    }
    let spec = SearchSpec::new("absent");
    let frame = session
        .table("candidates")
        .await
        .expect("table")
        .filter(scoring::eligibility(&spec.symbol_clauses))
        .expect("filter");
    assert_eq!(runtime.execute(frame).await.expect("empty result").rows, 0);
}

#[tokio::test]
async fn literal_pattern_characters_and_struct_factors_survive_native_fragment_plans() {
    let dir = tempfile::tempdir().expect("directory");
    let runtime =
        QueryRuntime::new(&dir.path().join("spill"), QueryLimits::default()).expect("runtime");
    let session = runtime.session();
    let spec = SearchSpec::new("a_%");
    let frame = session.sql("SELECT 'a_%' AS subject, 'ordinary text' AS text UNION ALL SELECT 'axb', 'ordinary text'").await.expect("fixture").filter(scoring::eligibility(&spec.fragment_clauses)).expect("native filter");
    let ranking =
        scoring::function(ScoreKind::Fragment, spec).call(vec![col("subject"), col("text")]);
    let frame = frame
        .with_column("ranking", ranking)
        .expect("rank")
        .filter(col("ranking").field("score").gt(lit(0u32)))
        .expect("score domain")
        .select(vec![col("subject"), col("ranking").field("factors")])
        .expect("factors");
    let output = runtime.execute(frame).await.expect("execute");
    assert_eq!(output.rows, 1);
    assert_eq!(
        TextColumn::new(output.batches[0].column(0).as_ref())
            .expect("text")
            .get(0),
        Some("a_%")
    );
    assert!(matches!(
        output.batches[0].column(1).data_type(),
        DataType::List(_)
    ));
}
