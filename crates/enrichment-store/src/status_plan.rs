//! Status component, coverage and outcome selection use one bound native request.
use crate::{native_catalog, runtime::QueryRuntime};
use datafusion::{common::Result, functions::core::expr_ext::FieldAccessor, prelude::*};
use enrichment_core::{
    evidence::arrow_model::expressions::record,
    native_union::{Cell, NativeStruct, Rule},
    wire::{Coverage, Outcome, status::StatusData},
};

enrichment_core::native_struct! {
    struct Input { status: StatusData => Rule::Text, component: Option<String> => Rule::Text }
}
enrichment_core::native_struct! {
    pub struct SelectedStatus {
        summary: String => Rule::Text,
        status: StatusData => Rule::Text,
        coverage: Coverage => Rule::Text,
        outcome: Outcome => Rule::Text,
    }
}

pub async fn select(
    runtime: &QueryRuntime,
    status: StatusData,
    component: Option<&str>,
) -> Result<SelectedStatus> {
    let session = runtime.session();
    native_catalog::work(
        &session,
        "status_input",
        session
            .read_batch(Input::batch(&[Input {
                status,
                component: component.map(str::to_owned),
            }])?)?
            .into_view(),
    )?;
    // Flatten once before selecting components. Repeated nested extraction through
    // successive projections can give the optimizer two identically named leaves.
    // These declared aliases also make the status reconstruction independent of
    // optimizer-generated names, with every original field carried exactly once.
    let mut columns = StatusData::fields()
        .iter()
        .map(|field| col("status").field(field.name()).alias(field.name()))
        .collect::<Vec<_>>();
    columns.push(col("component"));
    let mut frame = session.table("status_input").await?.select(columns)?;
    for field in ["producers", "features"] {
        let member = Expr::LambdaVariable(datafusion::logical_expr::expr::LambdaVariable::new(
            "component_status".into(),
            Some(std::sync::Arc::new(enrichment_core::native_union::field::<
                enrichment_core::wire::status::ComponentStatus,
            >(
                "component_status", Rule::Text
            ))),
        ));
        let predicate = col("component")
            .is_null()
            .or(member.field("name").eq(col("component")));
        frame = frame.with_column(
            field,
            datafusion::functions_nested::expr_fn::array_filter(
                col(field),
                datafusion::logical_expr::expr_fn::lambda(vec!["component_status"], predicate),
            ),
        )?;
    }
    native_catalog::work(&session, "status_selected", frame.into_view())?;
    let frame = session.sql("WITH matched AS (SELECT *, component IS NULL OR cardinality(producers)+cardinality(features)>0 AS found FROM status_selected) SELECT *, CASE WHEN component IS NULL THEN 'Service status as reported by the daemon.' WHEN found THEN concat('Status for `',component,'`.') ELSE concat('No component named `',component,'` is known to this build.') END AS summary, CASE WHEN component IS NULL THEN 'installed components and their availability' ELSE concat('components matching `',component,'`') END AS scope, CASE WHEN component IS NULL THEN make_array('daemon','producers','features','sandbox') WHEN found THEN make_array('daemon','producers','features') ELSE make_array('daemon') END AS indexed, CASE WHEN found THEN CAST(make_array() AS VARCHAR[]) ELSE make_array(concat('producers matching `',component,'`'),concat('features matching `',component,'`')) END AS missing, CASE WHEN found THEN make_array('Reports what is installed, not whether library evidence has been indexed.') ELSE make_array(concat('`',component,'` did not match any component this build reports. That is not evidence that no such component exists; call service.status with no filter to see the full list.')) END AS limitations FROM matched").await?;
    let status = record(
        &StatusData::data_type(),
        &StatusData::fields()
            .iter()
            .map(|field| (field.name().as_str(), col(field.name())))
            .collect::<Vec<_>>(),
    )?;
    let coverage = record(
        &Coverage::data_type(),
        &[
            ("scope", col("scope")),
            ("indexed", col("indexed")),
            ("missing", col("missing")),
            ("limitations", col("limitations")),
            (
                "assessments",
                enrichment_core::evidence::arrow_model::expressions::literal(&Vec::<
                    enrichment_core::wire::evidence::ScopeAssessment,
                >::new(
                ))?,
            ),
        ],
    )?;
    let outcome = datafusion::logical_expr::when(
        col("found"),
        enrichment_core::evidence::arrow_model::expressions::literal(&Outcome::Ok { job: None })?,
    )
    .otherwise(
        enrichment_core::evidence::arrow_model::expressions::literal(&Outcome::Partial {
            job: None,
        })?,
    )?;
    let selected = record(
        &SelectedStatus::data_type(),
        &[
            ("summary", col("summary")),
            ("status", status),
            ("coverage", coverage),
            ("outcome", outcome),
        ],
    )?;
    let frame = frame.select(vec![selected.alias("selected")])?.select(
        SelectedStatus::fields()
            .iter()
            .map(|field| col("selected").field(field.name()).alias(field.name()))
            .collect::<Vec<_>>(),
    )?;
    runtime
        .records::<SelectedStatus>(frame, 1)
        .await?
        .pop()
        .ok_or_else(|| {
            datafusion::common::DataFusionError::Execution(
                "status selection returned no row".into(),
            )
        })
}
