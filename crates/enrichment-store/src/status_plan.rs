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

enrichment_core::native_struct! { pub struct Observations {
    cache_ready: bool => Rule::Text,
    worker_python: Option<std::path::PathBuf> => Rule::Text,
    python_worker_qualified: bool => Rule::Text,
    execution_routes: Vec<enrichment_core::wire::status::ExecutionReadiness> => Rule::SequenceBounds { min: 0, max: 4 },
    execution_detail: String => Rule::Text,
} }
enrichment_core::native_struct! { pub struct Components {
    producers: Vec<enrichment_core::wire::status::ComponentStatus> => Rule::Sequence,
    features: Vec<enrichment_core::wire::status::ComponentStatus> => Rule::Sequence,
} }

/// Status consumes the same per-ecosystem/profile route result as effect admission. A
/// qualified image alone does not make a disabled route usable, and one unavailable image
/// does not hide an independently admitted route from another ecosystem.
pub async fn components(runtime: &QueryRuntime, observed: Observations) -> Result<Components> {
    use enrichment_core::{
        status_components::{self, Definition},
        wire::status::ComponentStatus,
    };
    let session = runtime.session();
    native_catalog::input(
        &session,
        "component_declarations",
        Definition::batch(&status_components::declarations())?,
    )?;
    native_catalog::input(
        &session,
        "component_observations",
        Observations::batch(&[observed])?,
    )?;
    let decisions = session.sql(r#"
        WITH eligible AS (
          SELECT d.*,o.cache_ready,o.worker_python,o.python_worker_qualified,o.execution_detail,
            o.cache_ready AND CASE d.requirement
              WHEN 'store' THEN true
              WHEN 'python_worker' THEN starts_with(o.worker_python,'/') AND o.python_worker_qualified
              WHEN 'rust_execution' THEN array_any_match(o.execution_routes,r -> get_field(r,'ecosystem')='rust' AND get_field(r,'available'))
              WHEN 'python_execution' THEN array_any_match(o.execution_routes,r -> get_field(r,'ecosystem')='python' AND get_field(r,'available'))
              WHEN 'any_execution' THEN array_any_match(o.execution_routes,r -> get_field(r,'available'))
              ELSE false END AS eligible
          FROM component_declarations d CROSS JOIN component_observations o
        ) SELECT ordinal,kind,name,coalesce(eligible,false) AS available,
          CASE WHEN eligible THEN version END AS version,
          CASE WHEN NOT cache_ready THEN 'No writable evidence store is open in this process.'
            WHEN eligible THEN detail
            WHEN requirement='python_worker' AND (worker_python IS NULL OR NOT starts_with(worker_python,'/'))
              THEN 'Configure the absolute interpreter from the service installation.'
            WHEN requirement='python_worker' THEN concat('The static worker ',worker_python,' has not completed a validated job in this daemon process.')
            ELSE concat('No enabled, qualified execution route meets this component requirement: ',execution_detail,
              ' Enable the matching build/runtime profile or repair its qualification prerequisite.')
          END AS detail
        FROM eligible
    "#).await?;
    let row = record(
        &ComponentStatus::data_type(),
        &ComponentStatus::fields()
            .iter()
            .map(|field| (field.name().as_str(), col(field.name())))
            .collect::<Vec<_>>(),
    )?;
    native_catalog::work(
        &session,
        "component_values",
        decisions
            .select(vec![col("ordinal"), col("kind"), row.alias("value")])?
            .into_view(),
    )?;
    let grouped = session.sql("SELECT array_agg(value ORDER BY ordinal) FILTER (WHERE kind='producer') AS producers, array_agg(value ORDER BY ordinal) FILTER (WHERE kind='feature') AS features FROM component_values").await?;
    runtime
        .records(grouped, 1)
        .await?
        .pop()
        .ok_or_else(|| datafusion::common::exec_datafusion_err!("component status missing"))
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
        crate::native_catalog::batch(
            &session,
            "status_plan",
            Input::batch(&[Input {
                status,
                component: component.map(str::to_owned),
            }])?,
        )?
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

#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn component_readiness_uses_enabled_routes_and_independent_ecosystems() -> Result<()> {
        use enrichment_core::{
            identity::Ecosystem, policy::ExecutionProfile, wire::status::ExecutionReadiness,
        };
        let root = tempfile::tempdir()?;
        let runtime = QueryRuntime::new(root.path(), Default::default())?;
        let mut observed = Observations {
            cache_ready: true,
            worker_python: Some("/installed/python".into()),
            python_worker_qualified: false,
            execution_detail: "Captured route decisions".into(),
            execution_routes: vec![
                ExecutionReadiness {
                    ecosystem: Ecosystem::Python,
                    profile: ExecutionProfile::Build,
                    available: true,
                    image_id: Some(format!("sha256:{}", "a".repeat(64))),
                    prerequisites: vec![],
                    actions: vec![],
                },
                ExecutionReadiness {
                    ecosystem: Ecosystem::Rust,
                    profile: ExecutionProfile::Build,
                    available: false,
                    image_id: Some(format!("sha256:{}", "b".repeat(64))),
                    prerequisites: vec![],
                    actions: vec![],
                },
            ],
        };
        let status = components(&runtime, observed.clone()).await?;
        let ready = |values: &Components, name: &str| {
            values
                .producers
                .iter()
                .chain(&values.features)
                .find(|row| row.name == name)
                .expect("declared component")
                .available
        };
        assert_eq!((status.producers.len(), status.features.len()), (6, 4));
        assert!(ready(&status, "ty"));
        assert!(ready(&status, "usage-verification"));
        assert!(!ready(&status, "rust-analyzer"));
        assert!(!ready(&status, "griffe"));
        observed.execution_routes[0].available = false;
        let disabled = components(&runtime, observed.clone()).await?;
        assert!(!ready(&disabled, "usage-verification"));
        assert!(
            !ready(&disabled, "ty"),
            "image presence does not enable its route"
        );
        observed.python_worker_qualified = true;
        assert!(ready(
            &components(&runtime, observed.clone()).await?,
            "griffe"
        ));
        observed.worker_python = None;
        assert!(!ready(
            &components(&runtime, observed.clone()).await?,
            "griffe"
        ));
        observed.cache_ready = false;
        let closed = components(&runtime, observed).await?;
        assert!(
            closed
                .producers
                .iter()
                .chain(&closed.features)
                .all(|row| !row.available && !row.detail.is_empty())
        );
        runtime.close_diagnostics().await?;
        Ok(())
    }
}
