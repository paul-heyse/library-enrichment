//! Qualified absence and availability are native joins over outcomes, coverage and declarations.
use crate::runtime::QueryRuntime;
use datafusion::{
    common::Result,
    dataframe::DataFrame,
    functions::core::expr_ext::FieldAccessor,
    prelude::{SessionContext, col},
};
use enrichment_core::{
    evidence::EvidenceKind,
    identity::Ecosystem,
    native_union::{NativeStruct, Rule},
    wire::{
        AspectOutcome, Coverage,
        data::DiscoveryFacet,
        research::{AspectSelection, DiscoveryKind, InspectionAspect},
    },
};

enrichment_core::native_struct! { struct Scope {
    ecosystem: Ecosystem => Rule::Text,
    coverage: Coverage => Rule::Text,
    source_available: bool => Rule::Text,
    configuration_available: bool => Rule::Text,
} }
enrichment_core::native_struct! { struct InspectionRow {
    ordinal: usize => Rule::Text, outcome: AspectOutcome => Rule::Text,
} }
enrichment_core::native_struct! { struct DiscoveryRow {
    ordinal: usize => Rule::Text, outcome: DiscoveryFacet => Rule::Text,
} }
enrichment_core::native_struct! { struct Kind { kind: EvidenceKind => Rule::Text } }
enrichment_core::native_struct! { struct Summary { partial: bool => Rule::Text } }
enrichment_core::native_struct! { struct Available { aspect: String => Rule::Text } }
enrichment_core::native_struct! { struct SourceVersion {
    mode: enrichment_core::identity::ResearchMode => Rule::Text,
    declared: Option<String> => Rule::Text,
    requested: String => Rule::NonEmpty,
} }
enrichment_core::native_struct! { struct VersionMatch {
    value: enrichment_core::wire::SourceVersionMatch => Rule::Text,
} }

/// Executed facts retain their own exact producer coverage; subtract their declared kinds
/// before assessing ordinary snapshot coverage, without collapsing one query into another.
pub async fn non_execution_kinds(
    runtime: &QueryRuntime,
    kinds: &[EvidenceKind],
    observations: &[enrichment_core::evidence::execution::ExecutionObservation],
) -> Result<Vec<EvidenceKind>> {
    let session = runtime.session();
    session.register_batch(
        "requested_kinds",
        Kind::batch(
            &kinds
                .iter()
                .map(|kind| Kind { kind: *kind })
                .collect::<Vec<_>>(),
        )?,
    )?;
    session.register_batch(
        "execution_observations",
        enrichment_core::evidence::execution::ExecutionObservation::batch(observations)?,
    )?;
    let frame = session.sql("SELECT DISTINCT r.kind FROM requested_kinds r LEFT ANTI JOIN (SELECT d.evidence_kind FROM execution_observations o JOIN operation.declarations.execution_payloads d ON o.payload.kind=d.kind) e ON r.kind=e.evidence_kind ORDER BY r.kind").await?;
    Ok(runtime
        .records::<Kind>(frame, kinds.len())
        .await?
        .into_iter()
        .map(|row| row.kind)
        .collect())
}

/// Revision identity is admitted against the immutable commit at acquisition; a manifest's
/// release version is a separate claim. Release-mode source version mismatches stay explicit.
pub async fn source_version(
    runtime: &QueryRuntime,
    mode: enrichment_core::identity::ResearchMode,
    declared: Option<&str>,
    requested: &str,
) -> Result<enrichment_core::wire::SourceVersionMatch> {
    let session = runtime.session();
    session.register_batch(
        "source_version",
        SourceVersion::batch(&[SourceVersion {
            mode,
            declared: declared.map(str::to_owned),
            requested: requested.into(),
        }])?,
    )?;
    let frame = session.sql("SELECT CASE WHEN mode='revision' OR declared=requested THEN 'exact' WHEN declared IS NULL THEN 'unknown' ELSE 'mismatched' END AS value FROM source_version").await?;
    Ok(runtime
        .records::<VersionMatch>(frame, 1)
        .await?
        .pop()
        .ok_or_else(|| {
            datafusion::common::DataFusionError::Execution("source version scope missing".into())
        })?
        .value)
}

pub struct InspectionDisposition {
    pub outcomes: Vec<AspectOutcome>,
    pub returned: Vec<String>,
    pub partial: bool,
}
pub struct DiscoveryDisposition {
    pub facets: Vec<DiscoveryFacet>,
    pub partial: bool,
}

fn scope(
    runtime: &QueryRuntime,
    ecosystem: Ecosystem,
    coverage: &Coverage,
    source_available: bool,
    configuration_available: bool,
) -> Result<SessionContext> {
    let session = runtime.session();
    session.register_batch(
        "research_scope",
        Scope::batch(&[Scope {
            ecosystem,
            coverage: coverage.clone(),
            source_available,
            configuration_available,
        }])?,
    )?;
    Ok(session)
}

pub async fn inspection_kinds(
    runtime: &QueryRuntime,
    ecosystem: Ecosystem,
    selection: &[AspectSelection],
) -> Result<Vec<EvidenceKind>> {
    let session = scope(
        runtime,
        ecosystem,
        &Coverage::unassessed(String::new()),
        false,
        false,
    )?;
    session.register_batch("aspect_selection", AspectSelection::batch(selection)?)?;
    let frame = session.sql("SELECT DISTINCT CASE WHEN s.ecosystem='python' THEN d.python_evidence ELSE d.rust_evidence END AS kind FROM aspect_selection a JOIN operation.declarations.inspection_aspects d ON a.aspect=d.aspect CROSS JOIN research_scope s ORDER BY kind").await?;
    Ok(runtime
        .records::<Kind>(frame, InspectionAspect::VALUES.len())
        .await?
        .into_iter()
        .map(|row| row.kind)
        .collect())
}

pub async fn inspection(
    runtime: &QueryRuntime,
    ecosystem: Ecosystem,
    outcomes: Vec<AspectOutcome>,
    coverage: &Coverage,
    source_available: bool,
    configuration_available: bool,
) -> Result<InspectionDisposition> {
    let session = scope(
        runtime,
        ecosystem,
        coverage,
        source_available,
        configuration_available,
    )?;
    let rows = outcomes
        .into_iter()
        .enumerate()
        .map(|(ordinal, outcome)| InspectionRow { ordinal, outcome })
        .collect::<Vec<_>>();
    session.register_batch("research_outcomes", InspectionRow::batch(&rows)?)?;
    let frame = session.sql(r#"
      WITH assessments AS (SELECT unnest(coverage.assessments) AS item FROM research_scope),
      qualified AS (SELECT item.kind AS kind,bool_and(item.state='indexed') AS complete FROM assessments GROUP BY item.kind),
      classified AS (
        SELECT r.*,s.source_available,s.configuration_available,d.observation,
          coalesce(q.complete,false) AS complete
        FROM research_outcomes r JOIN operation.declarations.inspection_aspects d ON r.outcome.aspect=d.aspect CROSS JOIN research_scope s
        LEFT JOIN qualified q ON q.kind=CASE WHEN s.ecosystem='python' THEN d.python_evidence ELSE d.rust_evidence END
      )
      SELECT *, CASE
        WHEN outcome.state='failed' THEN outcome.state
        WHEN (outcome.state='absent' AND NOT complete)
          OR (observation='source' AND NOT source_available)
          OR (observation='configuration' AND NOT configuration_available)
          OR (observation='execution' AND outcome.page.returned=0) THEN 'unavailable'
        ELSE outcome.state END AS selected_state,
        CASE WHEN outcome.state='failed' THEN outcome.reason
          WHEN observation='source' AND NOT source_available THEN 'No trustworthy recorded source window is available'
          WHEN observation='configuration' AND NOT configuration_available THEN 'No observed build configuration is retained; availability has not been established for the requested environment'
          WHEN observation='execution' AND outcome.page.returned=0 THEN 'No retained execution result for this scope; explicit execution intent and an enabled profile are required'
          WHEN outcome.state='absent' AND NOT complete THEN 'No retained match; this aspect lacks complete qualified coverage.'
          ELSE outcome.reason END AS selected_reason FROM classified
    "#).await?;
    crate::native_catalog::work(&session, "selected_outcomes", frame.clone().into_view())?;
    let returned = runtime.records::<Available>(session.sql("SELECT outcome.aspect AS aspect FROM selected_outcomes WHERE selected_state='available' ORDER BY ordinal").await?, InspectionAspect::VALUES.len()).await?.into_iter().map(|row| row.aspect).collect();
    let partial = partial(runtime, &session).await?;
    let outcomes = runtime
        .records(
            project::<AspectOutcome>(frame)?,
            InspectionAspect::VALUES.len(),
        )
        .await?;
    Ok(InspectionDisposition {
        outcomes,
        returned,
        partial,
    })
}

pub async fn discovery(
    runtime: &QueryRuntime,
    facets: Vec<DiscoveryFacet>,
    coverage: &Coverage,
) -> Result<DiscoveryDisposition> {
    let session = scope(runtime, Ecosystem::Rust, coverage, false, false)?;
    let rows = facets
        .into_iter()
        .enumerate()
        .map(|(ordinal, outcome)| DiscoveryRow { ordinal, outcome })
        .collect::<Vec<_>>();
    session.register_batch("research_outcomes", DiscoveryRow::batch(&rows)?)?;
    let frame = session.sql(r#"
      WITH assessments AS (SELECT unnest(coverage.assessments) AS item FROM research_scope),
      qualified AS (SELECT item.kind AS kind,bool_and(item.state='indexed') AS complete FROM assessments GROUP BY item.kind)
      SELECT r.*,CASE WHEN r.outcome.state='absent' AND NOT coalesce(q.complete,false) THEN 'unavailable' ELSE r.outcome.state END AS selected_state,
        CASE WHEN r.outcome.state<>'absent' THEN r.outcome.reason
          WHEN NOT coalesce(q.complete,false) THEN 'No retained match; this facet lacks complete qualified coverage.'
          ELSE 'No retained match in this qualified discovery scope.' END AS selected_reason
      FROM research_outcomes r JOIN operation.declarations.discovery_facets d ON r.outcome.kind=d.kind
      LEFT JOIN qualified q ON q.kind=d.evidence_kind
    "#).await?;
    crate::native_catalog::work(&session, "selected_outcomes", frame.clone().into_view())?;
    let partial = partial(runtime, &session).await?;
    let facets = runtime
        .records(
            project::<DiscoveryFacet>(frame)?,
            DiscoveryKind::VALUES.len(),
        )
        .await?;
    Ok(DiscoveryDisposition { facets, partial })
}

fn project<T: NativeStruct>(frame: DataFrame) -> Result<DataFrame> {
    frame.sort(vec![col("ordinal").sort(true, false)])?.select(
        T::fields()
            .iter()
            .map(|field| {
                match field.name().as_str() {
                    "state" => col("selected_state"),
                    "reason" => col("selected_reason"),
                    name => col("outcome").field(name),
                }
                .alias(field.name())
            })
            .collect::<Vec<_>>(),
    )
}

async fn partial(runtime: &QueryRuntime, session: &SessionContext) -> Result<bool> {
    let frame = session.sql(r#"
      WITH assessments AS (SELECT unnest(coverage.assessments) AS item FROM research_scope),
      assessed AS (SELECT count(*)>0 AND coalesce(bool_and(item.state='indexed'),false) AS complete FROM assessments),
      outcomes AS (SELECT coalesce(bool_or(selected_state IN ('failed','unavailable')),false) AS incomplete FROM selected_outcomes)
      SELECT NOT a.complete OR cardinality(s.coverage.missing)>0 OR o.incomplete AS partial
      FROM research_scope s CROSS JOIN assessed a CROSS JOIN outcomes o
    "#).await?;
    Ok(runtime
        .records::<Summary>(frame, 1)
        .await?
        .pop()
        .ok_or_else(|| {
            datafusion::common::DataFusionError::Execution("research scope missing".into())
        })?
        .partial)
}
