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

#[tokio::test]
async fn native_string_encodings_keep_scores_metadata_and_arguments_without_utf8_casts() {
    use arrow::array::{ArrayRef, LargeStringArray, StringArray};
    use datafusion::{
        common::tree_node::{TreeNode, TreeNodeRecursion},
        logical_expr::{Expr, ExprSchemable},
    };
    let dir = tempfile::tempdir().unwrap();
    let runtime = QueryRuntime::new(dir.path(), QueryLimits::default()).unwrap();
    let mut reference = None;
    for kind in [DataType::Utf8, DataType::LargeUtf8, DataType::Utf8View] {
        let text = |values: Vec<Option<&str>>| -> ArrayRef {
            let array: ArrayRef = match kind {
                DataType::Utf8 => Arc::new(StringArray::from(values)),
                DataType::LargeUtf8 => Arc::new(LargeStringArray::from(values)),
                DataType::Utf8View => Arc::new(StringViewArray::from(values)),
                _ => unreachable!(),
            };
            array.slice(1, 3)
        };
        let batch = RecordBatch::try_from_iter(vec![
            (
                "path",
                text(vec![
                    Some("unused"),
                    Some("p::éclair"),
                    Some("p::a_%"),
                    Some("p::alias"),
                ]),
            ),
            (
                "name",
                text(vec![
                    Some("unused"),
                    Some("éclair"),
                    Some("a_%"),
                    Some("alias"),
                ]),
            ),
            (
                "docs",
                text(vec![
                    None,
                    Some("Unicode éclair"),
                    None,
                    Some("éclair docs"),
                ]),
            ),
        ])
        .unwrap();
        let session = runtime.session();
        let source = session.read_batch(batch).unwrap();
        let score = scoring::function(ScoreKind::Symbol, SearchSpec::new("éclair"));
        let frame = source
            .select(vec![
                score
                    .call(vec![
                        col("path"),
                        col("name"),
                        col("docs"),
                        col("docs"),
                        col("docs"),
                        lit(true),
                    ])
                    .alias("ranking"),
            ])
            .unwrap();
        let (state, logical) = frame.clone().into_parts();
        let analyzed = state
            .analyzer()
            .execute_and_check(logical, state.config_options(), |_, _| {})
            .unwrap();
        let mut found = 0;
        analyzed
            .apply_with_subqueries(|plan| {
                for expression in plan.expressions() {
                    expression.apply(|expr| {
                        if let Expr::ScalarFunction(function) = expr
                            && function.name() == "evidence_symbol_score_v2"
                        {
                            found += 1;
                            for arg in &function.args[..5] {
                                assert!(!matches!(arg, Expr::Cast(_)));
                                assert_eq!(
                                    arg.get_type(plan.inputs()[0].schema().as_ref()).unwrap(),
                                    kind
                                );
                            }
                        }
                        Ok(TreeNodeRecursion::Continue)
                    })?;
                }
                Ok(TreeNodeRecursion::Continue)
            })
            .unwrap();
        assert_eq!(found, 1);
        let output = runtime.execute(frame.clone()).await.unwrap();
        let batch = &output.batches[0];
        let actual = (batch.schema(), batch.column(0).to_data());
        if let Some(expected) = &reference {
            assert_eq!(expected, &actual);
        } else {
            reference = Some(actual);
        }
        assert_eq!(
            runtime
                .execute(frame.limit(0, Some(0)).unwrap())
                .await
                .unwrap()
                .rows,
            0
        );
    }
}
