//! Exercise the production session's semantic admission rather than a standalone rule fixture.
use crate::runtime::QueryRuntime;
use arrow::{
    array::FixedSizeBinaryArray,
    datatypes::{DataType, Field, Schema},
    record_batch::RecordBatch,
};
use datafusion::{
    common::Result,
    prelude::{SessionContext, col, lit},
};
use std::sync::Arc;

fn domain(name: &str, domain: &str) -> Field {
    use arrow_schema::extension::ExtensionType;
    use enrichment_core::{
        native_types::{IdentityType, TypeMetadata},
        native_union::Domain,
    };
    let meaning = match domain {
        "symbol" => Domain::Symbol,
        "definition" => Domain::Definition,
        _ => panic!("undeclared fixture domain"),
    };
    Field::new(name, DataType::FixedSizeBinary(32), false).with_extension_type(
        IdentityType::try_new(&DataType::FixedSizeBinary(32), TypeMetadata::new(meaning)).unwrap(),
    )
}

fn session(runtime: &QueryRuntime) -> Result<SessionContext> {
    let context = runtime.session();
    let schema = Arc::new(Schema::new(vec![
        domain("a", "symbol"),
        domain("b", "definition"),
    ]));
    crate::native_catalog::input(
        &context,
        "domains",
        RecordBatch::try_new(
            schema,
            vec![
                Arc::new(FixedSizeBinaryArray::try_from_iter(
                    [[1_u8; 32], [2_u8; 32]].into_iter(),
                )?),
                Arc::new(FixedSizeBinaryArray::try_from_iter(
                    [[1_u8; 32], [2_u8; 32]].into_iter(),
                )?),
            ],
        )?,
    )?;
    Ok(context)
}

#[tokio::test]
async fn map_entries_preserves_child_contracts_slices_and_nulls() -> Result<()> {
    use arrow::{
        array::{Array, MapArray, StringArray, StructArray},
        buffer::{NullBuffer, OffsetBuffer},
    };
    let root = tempfile::tempdir()?;
    let runtime = QueryRuntime::new(root.path(), Default::default())?;
    let context = runtime.session();
    let fields = vec![
        Arc::new(
            Field::new("key", DataType::Utf8, false)
                .with_metadata([("enrichment.rule".into(), "nonempty".into())].into()),
        ),
        Arc::new(domain("value", "symbol").with_nullable(true)),
    ]
    .into();
    let entries = StructArray::try_new(
        fields,
        vec![
            Arc::new(StringArray::from(vec!["first", "second"])),
            Arc::new(FixedSizeBinaryArray::try_from_iter(
                [[1_u8; 32], [2_u8; 32]].into_iter(),
            )?),
        ],
        None,
    )?;
    let entry_field = Arc::new(Field::new("entries", entries.data_type().clone(), false));
    let map = MapArray::try_new(
        entry_field.clone(),
        OffsetBuffer::new(vec![0, 1, 2, 2, 2].into()),
        entries,
        Some(NullBuffer::from(vec![true, true, true, false])),
        false,
    )?;
    let batch = RecordBatch::try_new(
        Arc::new(Schema::new(vec![Field::new(
            "inputs",
            map.data_type().clone(),
            true,
        )])),
        vec![Arc::new(map.slice(1, 3))],
    )?;
    crate::native_catalog::input(&context, "maps", batch)?;
    let output = runtime
        .execute(
            context
                .sql("SELECT native_map_entries(inputs) AS entries FROM maps")
                .await?,
        )
        .await?;
    let values = datafusion::common::cast::as_list_array(output.batches[0].column(0).as_ref())?;
    assert_eq!(values.data_type(), &DataType::List(entry_field));
    assert_eq!(values.value_length(0), 1);
    assert_eq!(values.value_length(1), 0);
    assert!(values.is_null(2));
    let unnested = runtime
        .execute(
            context
                .sql("SELECT unnest(native_map_entries(inputs)) AS entry FROM maps")
                .await?,
        )
        .await?;
    assert_eq!(unnested.rows, 1);
    assert_eq!(
        unnested.batches[0].schema().field(0).data_type(),
        values.values().data_type()
    );
    runtime.close_diagnostics().await?;
    Ok(())
}

#[tokio::test]
async fn native_collection_predicates_validate_lambda_domains() -> Result<()> {
    use arrow::{
        array::{Array, BooleanArray, ListArray, StructArray},
        buffer::{NullBuffer, OffsetBuffer},
    };
    let root = tempfile::tempdir()?;
    let runtime = QueryRuntime::new(root.path(), Default::default())?;
    let context = session(&runtime)?;
    let input = runtime.execute(context.table("domains").await?).await?;
    let values = StructArray::from(input.batches[0].clone());
    let item = Arc::new(Field::new("item", values.data_type().clone(), false));
    let lists = ListArray::try_new(
        item,
        OffsetBuffer::new(vec![0, 2, 2, 2].into()),
        Arc::new(values),
        Some(NullBuffer::from(vec![true, true, false])),
    )?;
    let list_field = Field::new("items", lists.data_type().clone(), true);
    crate::native_catalog::input(
        &context,
        "collections",
        RecordBatch::try_new(
            Arc::new(Schema::new(vec![list_field.clone()])),
            vec![Arc::new(lists)],
        )?,
    )?;
    for sql in [
        "SELECT array_any_match(items, x -> x['a'] = x['b']) FROM collections",
        "SELECT array_filter(items, x -> x['a'] = x['b']) FROM collections",
    ] {
        let result = match context.sql(sql).await {
            Ok(frame) => runtime.execute(frame).await.map(|_| ()),
            Err(error) => Err(error),
        };
        let error = result.expect_err(sql);
        assert!(
            error.to_string().contains("semantic field contract"),
            "{sql}: {error}"
        );
    }
    let output = runtime
        .execute(
            context
                .sql(
                    "SELECT array_any_match(items, x -> x['a'] = x['a']) AS any_value, \
         array_filter(items, x -> x['a'] = x['a']) AS retained FROM collections",
                )
                .await?,
        )
        .await?;
    let any = output.batches[0]
        .column(0)
        .as_any()
        .downcast_ref::<BooleanArray>()
        .unwrap();
    assert_eq!(
        any.iter().collect::<Vec<_>>(),
        vec![Some(true), Some(false), None]
    );
    enrichment_core::native_analysis::compatible(
        &list_field,
        output.batches[0].schema().field(1),
        "filtered collection",
    )?;
    runtime.close_diagnostics().await?;
    Ok(())
}

#[tokio::test]
async fn registered_analyzer_refuses_erasure_and_cross_domain_operations() -> Result<()> {
    let root = tempfile::tempdir()?;
    let runtime = QueryRuntime::new(root.path(), Default::default())?;
    let context = session(&runtime)?;
    for sql in [
        "SELECT a = b AS invalid FROM domains",
        "SELECT a IS NOT DISTINCT FROM b AS invalid FROM domains",
        "SELECT a IN (b) AS invalid FROM domains",
        "SELECT a BETWEEN a AND b AS invalid FROM domains",
        "SELECT CAST(a AS VARCHAR) AS invalid FROM domains",
        "SELECT coalesce(a,b) AS invalid FROM domains",
        "SELECT CASE WHEN a = a THEN a ELSE b END AS invalid FROM domains",
        "SELECT a FROM domains UNION ALL SELECT b FROM domains",
        "SELECT min(a) AS invalid FROM domains",
        "SELECT make_array(a) AS invalid FROM domains",
        "SELECT make_array(a,b) AS invalid FROM domains",
        "SELECT array_agg(a) AS invalid FROM domains",
        "SELECT x.a FROM domains x JOIN domains y ON x.a = y.b",
        "SELECT a IN (SELECT b FROM domains) AS invalid FROM domains",
    ] {
        let result = match context.sql(sql).await {
            Ok(frame) => runtime.execute(frame).await.map(|_| ()),
            Err(error) => Err(error),
        };
        let error = result.expect_err(sql);
        assert!(
            error.to_string().contains("semantic field contract"),
            "{sql}: {error}"
        );
    }
    let output = runtime
        .execute(
            context
                .sql("SELECT a, a = a AS equal, a IS NULL AS absent FROM domains")
                .await?,
        )
        .await?;
    assert_eq!(output.rows, 2);
    assert_eq!(
        output.batches[0]
            .schema()
            .field(0)
            .metadata()
            .get("ARROW:extension:metadata"),
        domain("a", "symbol")
            .metadata()
            .get("ARROW:extension:metadata")
    );
    Ok(())
}

#[derive(Debug)]
struct CrossDomainRewrite;
impl CrossDomainRewrite {
    fn inject(
        plan: datafusion::logical_expr::LogicalPlan,
    ) -> Result<datafusion::logical_expr::LogicalPlan> {
        use datafusion::logical_expr::{Filter, LogicalPlan};
        if let LogicalPlan::Projection(mut projection) = plan {
            if !matches!(projection.input.as_ref(), LogicalPlan::Filter(_)) {
                projection.input = Arc::new(LogicalPlan::Filter(Filter::try_new(
                    col("a").eq(col("b")),
                    projection.input,
                )?));
            }
            Ok(LogicalPlan::Projection(projection))
        } else {
            Ok(plan)
        }
    }
}
impl datafusion::optimizer::AnalyzerRule for CrossDomainRewrite {
    fn name(&self) -> &str {
        "fixture_cross_domain_analysis"
    }
    fn analyze(
        &self,
        plan: datafusion::logical_expr::LogicalPlan,
        _: &datafusion::common::config::ConfigOptions,
    ) -> Result<datafusion::logical_expr::LogicalPlan> {
        Self::inject(plan)
    }
}
impl datafusion::optimizer::OptimizerRule for CrossDomainRewrite {
    fn name(&self) -> &str {
        "fixture_cross_domain_optimization"
    }
    fn supports_rewrite(&self) -> bool {
        true
    }
    fn rewrite(
        &self,
        plan: datafusion::logical_expr::LogicalPlan,
        _: &dyn datafusion::optimizer::OptimizerConfig,
    ) -> Result<datafusion::common::tree_node::Transformed<datafusion::logical_expr::LogicalPlan>>
    {
        use datafusion::{common::tree_node::Transformed, logical_expr::LogicalPlan};
        if matches!(&plan,LogicalPlan::Projection(p) if !matches!(p.input.as_ref(),LogicalPlan::Filter(_)))
        {
            Ok(Transformed::yes(Self::inject(plan)?))
        } else {
            Ok(Transformed::no(plan))
        }
    }
}

#[tokio::test]
async fn native_planning_refuses_late_semantic_mutation_with_unchanged_output() -> Result<()> {
    use datafusion::{dataframe::DataFrame, execution::SessionStateBuilder};
    let root = tempfile::tempdir()?;
    let runtime = QueryRuntime::new(root.path(), Default::default())?;
    let context = session(&runtime)?;
    for analysis in [true, false] {
        let (state, plan) = context.sql("SELECT a FROM domains").await?.into_parts();
        let builder = SessionStateBuilder::new_from_existing(state);
        let state = if analysis {
            builder
                .with_analyzer_rule(Arc::new(CrossDomainRewrite))
                .build()
        } else {
            builder
                .with_optimizer_rules(vec![Arc::new(CrossDomainRewrite)])
                .build()
        };
        let frame = DataFrame::new(state, plan);
        let direct = frame
            .clone()
            .create_physical_plan()
            .await
            .expect_err("builder planning must use native admission");
        assert!(
            direct.to_string().contains("semantic field contract"),
            "direct analysis={analysis}: {direct}"
        );
        let error = runtime
            .execute(frame)
            .await
            .expect_err("same output schema cannot authorize cross-domain filter");
        assert!(
            error.to_string().contains("semantic field contract"),
            "analysis={analysis}: {error}"
        );
    }
    runtime.close_diagnostics().await
}

#[test]
fn physical_child_annotations_are_checked_even_when_projection_hides_them() -> Result<()> {
    use datafusion::{
        common::ScalarValue,
        physical_expr::expressions::lit,
        physical_plan::{ExecutionPlan, empty::EmptyExec, projection::ProjectionExec},
    };
    let unknown = Field::new("bad", DataType::FixedSizeBinary(32), true)
        .with_metadata([("ARROW:extension:name".into(), "unknown.identity".into())].into());
    let child = Arc::new(EmptyExec::new(Arc::new(Schema::new(vec![unknown]))));
    let root = Arc::new(ProjectionExec::try_new(
        vec![(lit(ScalarValue::Int64(Some(1))), "safe_output".into())],
        child,
    )?) as Arc<dyn ExecutionPlan>;
    enrichment_core::native_analysis::validate_derived_fields(root.schema().fields())?;
    assert!(crate::preparation::admit_physical(&root).is_err());
    Ok(())
}

#[tokio::test]
async fn native_udfs_prepare_full_fields_and_execute_the_same_contract() -> Result<()> {
    let root = tempfile::tempdir()?;
    let runtime = QueryRuntime::new(root.path(), Default::default())?;
    let context = runtime.session();
    let input = context
        .sql("SELECT '1.2.3' AS version, 'https://example.org/path' AS url")
        .await?;
    let bytes = enrichment_core::native_identity::canonical_bytes(
        "test/field-contract",
        vec![Field::new("version", DataType::Utf8, false)].into(),
    );
    let frame = input.select(vec![
        bytes.call(vec![col("version")]).alias("key"),
        enrichment_core::native_url::parts()
            .call(vec![col("url")])
            .alias("url"),
        enrichment_core::native_version::semver_key()
            .call(vec![col("version")])
            .alias("semver"),
        enrichment_core::native_version::pep440_value()
            .call(vec![col("version")])
            .alias("pep440"),
        enrichment_core::native_version::pep440_matches()
            .call(vec![lit(">=1"), col("version")])
            .alias("matches"),
    ])?;
    assert!(!frame.schema().field(0).is_nullable());
    assert!(frame.schema().field(1).is_nullable());
    assert!(!frame.schema().field(4).is_nullable());
    let output = runtime.execute(frame).await?;
    assert_eq!(output.rows, 1);
    let invalid = session(&runtime)?.table("domains").await?.select(vec![
        enrichment_core::native_url::parts().call(vec![col("a")]),
    ]);
    assert!(
        invalid
            .expect_err("domain passed to URL parser")
            .to_string()
            .contains("ARROW:extension:name")
    );
    Ok(())
}

#[tokio::test]
async fn native_case_allows_absence_without_erasing_nested_domains() -> Result<()> {
    use enrichment_core::{evidence::relational::Locator, native_union::NativeUnion};
    let root = tempfile::tempdir()?;
    let runtime = QueryRuntime::new(root.path(), Default::default())?;
    let session = runtime.session();
    let locators = Locator::encode(&[&Locator::RegistryLine { line: 7 }, &Locator::Artifact])?;
    let field = Field::new("locator", locators.data_type().clone(), false);
    crate::native_catalog::input(
        &session,
        "qualified",
        RecordBatch::try_new(
            Arc::new(Schema::new(vec![
                Field::new("present", DataType::Boolean, false),
                field.clone(),
            ])),
            vec![
                Arc::new(arrow::array::BooleanArray::from(vec![true, false])),
                locators,
            ],
        )?,
    )?;
    let result = runtime
        .execute(
            session
                .sql(
                    "SELECT CASE WHEN present THEN locator ELSE NULL END AS locator FROM qualified",
                )
                .await?,
        )
        .await?;
    assert_eq!(result.rows, 2);
    assert_eq!(
        result
            .batches
            .iter()
            .map(|batch| batch.column(0).null_count())
            .sum::<usize>(),
        1
    );
    enrichment_core::native_analysis::compatible(
        result.batches[0].schema().field(0),
        &field,
        "CASE result",
    )?;
    let aggregate = runtime.execute(session.sql(
        "SELECT count(DISTINCT locator) AS alternatives, first_value(locator) AS first, last_value(locator) AS last,
         CASE WHEN count(DISTINCT locator)=1 THEN first_value(locator) ELSE NULL END AS consensus
         FROM qualified",
    ).await?).await?;
    assert_eq!(aggregate.rows, 1);
    let batch = &aggregate.batches[0];
    assert_eq!(
        datafusion::common::ScalarValue::try_from_array(batch.column(0), 0)?,
        datafusion::common::ScalarValue::Int64(Some(2))
    );
    for index in [1, 2] {
        enrichment_core::native_analysis::compatible(
            batch.schema().field(index),
            &field,
            "native selector",
        )?;
    }
    assert_eq!(batch.column(3).null_count(), 1);
    runtime.close_diagnostics().await?;
    Ok(())
}

#[tokio::test]
async fn native_aggregate_preserves_annotated_empty_lists() -> Result<()> {
    use enrichment_core::native_union::Cell;
    let root = tempfile::tempdir()?;
    let runtime = QueryRuntime::new(root.path(), Default::default())?;
    let context = runtime.session();
    let empty = Vec::<String>::new();
    let hints = vec!["unix".to_owned()];
    let values = Vec::<String>::encode(&[Some(&empty), Some(&hints), None])?;
    let field = Field::new("hints", values.data_type().clone(), true);
    for row in 0..3 {
        let mut scalar = datafusion::common::ScalarValue::try_from_array(&values, row)?;
        assert_eq!(scalar.data_type(), *field.data_type(), "scalar row {row}");
        scalar.compact();
        assert_eq!(
            scalar.data_type(),
            *field.data_type(),
            "compacted row {row}"
        );
        assert_eq!(
            scalar.to_array()?.data_type(),
            field.data_type(),
            "array row {row}"
        );
    }
    crate::native_catalog::input(
        &context,
        "hints",
        RecordBatch::try_new(Arc::new(Schema::new(vec![field.clone()])), vec![values])?,
    )?;
    for selection in [
        "count(DISTINCT hints)",
        "first_value(hints)",
        "last_value(hints)",
    ] {
        let sql = format!("SELECT {selection} FROM hints");
        let output = runtime
            .execute(context.sql(&sql).await?)
            .await
            .unwrap_or_else(|error| panic!("{sql}: {error}"));
        assert_eq!(output.rows, 1);
        if selection != "count(DISTINCT hints)" {
            enrichment_core::native_analysis::compatible(
                output.batches[0].schema().field(0),
                &field,
                selection,
            )?;
        }
    }
    runtime.close_diagnostics().await?;
    Ok(())
}

#[tokio::test]
async fn required_native_record_child_refuses_without_panicking() -> Result<()> {
    let root = tempfile::tempdir()?;
    let runtime = QueryRuntime::new(root.path(), Default::default())?;
    let result = runtime
        .session()
        .sql("SELECT CAST(NULL AS BIGINT) AS missing")
        .await?
        .select(vec![enrichment_core::native_record::record(
            vec![Field::new("required", DataType::Int64, false)].into(),
            vec![lit("required"), col("missing")],
        )])?;
    assert!(runtime.execute(result).await.is_err());
    Ok(())
}

#[test]
fn dictionary_children_participate_in_semantic_admission() -> Result<()> {
    let left = Field::new(
        "value",
        DataType::Dictionary(
            Box::new(DataType::Int32),
            Box::new(DataType::Struct(vec![domain("id", "symbol")].into())),
        ),
        true,
    );
    let right = Field::new(
        "value",
        DataType::Dictionary(
            Box::new(DataType::Int32),
            Box::new(DataType::Struct(vec![domain("id", "definition")].into())),
        ),
        true,
    );
    assert!(enrichment_core::native_analysis::has_semantics(&left));
    assert!(
        enrichment_core::native_analysis::compatible(&left, &right, "dictionary comparison")
            .is_err()
    );
    Ok(())
}

#[tokio::test]
async fn generated_artifact_fields_survive_sql_unnest_and_window_selection() -> Result<()> {
    let root = tempfile::tempdir()?;
    let runtime = QueryRuntime::new(root.path(), Default::default())?;
    let context = runtime.session();
    crate::native_catalog::input(
        &context,
        "attempts",
        crate::projection::catalog::attempts(&[])?,
    )?;
    for sql in [
        "SELECT unnest(acquisitions) AS artifact FROM attempts",
        "WITH raw AS (SELECT unnest(acquisitions) AS artifact, started_at, attempt_id FROM attempts), ranked AS (SELECT artifact, row_number() OVER (PARTITION BY artifact.artifact_id ORDER BY started_at,attempt_id) AS position FROM raw) SELECT artifact FROM ranked WHERE position=1",
    ] {
        let frame = context.sql(sql).await?;
        assert_eq!(
            frame.schema().field(0).data_type(),
            &enrichment_core::evidence::arrow_model::acquisitions::data_type(),
            "{sql}"
        );
        runtime
            .execute_family(
                frame,
                Some(crate::preparation::QueryFamily::CatalogArtifact),
            )
            .await?;
    }
    Ok(())
}

#[tokio::test]
async fn clock_constructors_refuse_precision_loss_and_cross_domain_comparison() -> Result<()> {
    let root = tempfile::tempdir()?;
    let runtime = QueryRuntime::new(root.path(), Default::default())?;
    let context = runtime.session();
    for sql in [
        "SELECT expiry_time(to_timestamp_nanos(1))",
        "SELECT expiry_time(to_timestamp_micros(0)) = update_time(to_timestamp_micros(0))",
    ] {
        let result = match context.sql(sql).await {
            Ok(frame) => runtime.execute(frame).await.map(|_| ()),
            Err(error) => Err(error),
        };
        assert!(result.is_err(), "{sql}");
    }
    let output = runtime.execute(context.sql("SELECT clock_instant(expiry_time(to_timestamp_micros(-1))) AS instant, expiry_time(date_trunc('microsecond',now())) AS captured").await?).await?;
    let rows = enrichment_core::evidence::arrow_model::cells::RowSet::batch(&output.batches[0])?;
    assert_eq!(rows.row(0).timestamp_micros("instant")?, -1);
    assert!(rows.row(0).timestamp_micros("captured")? > 0);
    Ok(())
}
