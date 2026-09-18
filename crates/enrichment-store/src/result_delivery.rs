//! Native retained-view selection. The byte sink reports measurements; DataFusion chooses a view.
use crate::{result::Index, runtime::QueryRuntime};
use datafusion::{common::Result, functions::core::expr_ext::FieldAccessor, prelude::*};
use enrichment_core::{
    native_union::{Cell, NativeStruct, Rule},
    operation::results::ResultRecord,
    wire::{
        ArtifactHandle, Coverage, DeliveryDescriptor, DeliveryLimits, RecoveryAction,
        data::ToolData,
        research::{ArtifactSection, ResultSectionName},
    },
};

enrichment_core::native_struct! {
    struct ViewInput {
        header: enrichment_core::operation::results::ResultHeader => Rule::Text,
        delivery: DeliveryDescriptor => Rule::Text,
        handles: Vec<ArtifactHandle> => Rule::Sequence,
        coverage_details: RecoveryAction => Rule::Text,
    }
}
enrichment_core::native_struct! {
    struct ViewPolicy {
        rank: u32 => Rule::Text,
        include_handles: bool => Rule::Text,
        full_coverage: bool => Rule::Text,
    }
}
enrichment_core::native_struct! {
    struct Measured {
        rank: u32 => Rule::Text,
        bytes: u64 => Rule::Text,
        result: ResultRecord => Rule::Text,
    }
}
enrichment_core::native_struct! {
    struct Minimum { bytes: u64 => Rule::Text }
}
enrichment_core::native_struct! {
    struct InlineFit { fits: bool => Rule::Text }
}
enrichment_core::native_struct! {
    struct DeliveryInput { result: ResultRecord => Rule::Text }
}

#[derive(Debug, thiserror::Error)]
#[error("delivery requires at least {minimum} encoded bytes")]
pub struct MinimumBudget {
    pub minimum: usize,
}

pub struct DeliveryOptions {
    pub inline: usize,
    pub requested: Option<usize>,
    pub request_id: enrichment_core::wire::RequestId,
    pub profile: enrichment_core::mcp_delivery::DeliveryProfile,
}

/// One native cap policy supplies both selection witnesses and the final delivery owner.
pub async fn byte_budget(
    runtime: &QueryRuntime,
    configured: usize,
    requested: Option<usize>,
) -> Result<usize> {
    enrichment_core::native_struct! { struct Input {
        configured:usize => Rule::Text,
        requested:Option<usize> => Rule::Text,
    } }
    enrichment_core::native_struct! { struct Budget {bytes:usize=>Rule::Text} }
    let session = runtime.session();
    crate::native_catalog::input(
        &session,
        "delivery_limits",
        Input::batch(&[Input {
            configured,
            requested,
        }])?,
    )?;
    runtime.records::<Budget>(session.sql("SELECT least(greatest(coalesce(requested,configured),CAST(1024 AS BIGINT UNSIGNED)),greatest(configured,CAST(1024 AS BIGINT UNSIGNED))) AS bytes FROM delivery_limits").await?,1).await?
        .pop().map(|value|value.bytes).ok_or_else(||datafusion::common::internal_datafusion_err!("delivery budget missing"))
}

/// Inline eligibility is the same native encoded-size predicate as retained-view selection.
/// The transport codec measures; this plan alone decides whether the full answer fits.
pub async fn inline(
    runtime: &QueryRuntime,
    value: &enrichment_core::wire::Envelope,
    limit: usize,
    profile: enrichment_core::mcp_delivery::DeliveryProfile,
) -> Result<bool> {
    let input = ResultRecord::from_envelope(value)
        .map_err(datafusion::common::DataFusionError::Execution)?;
    // Capture the typed struct directly instead of rebuilding the entire payload expression.
    let frame = crate::native_catalog::batch(
        &runtime.session(),
        "result_delivery",
        DeliveryInput::batch(&[DeliveryInput { result: input }])?,
    )?;
    let measured = frame.with_column(
        "bytes",
        enrichment_core::native_transport::delivery(
            runtime.session().runtime_env().memory_pool.clone(),
            col("result"),
            &value.request_id,
            profile,
        ),
    )?;
    let selected = measured
        .filter(col("bytes").lt_eq(lit(limit as u64)))?
        .select(vec![lit(true).alias("fits")])?;
    Ok(!runtime.records::<InlineFit>(selected, 1).await?.is_empty())
}

/// Project bounded alternatives from the admitted header, then select the richest fitting view.
/// A caller can retrieve every omitted field from the same immutable result and coverage section.
pub async fn retained(
    runtime: &QueryRuntime,
    header: enrichment_core::operation::results::ResultHeader,
    artifact: &enrichment_core::evidence::Artifact,
    index: &Index,
    options: DeliveryOptions,
) -> std::io::Result<enrichment_core::wire::Envelope> {
    let DeliveryOptions {
        inline,
        requested,
        request_id,
        profile,
    } = options;
    use enrichment_core::evidence::arrow_model::expressions::{derive_record, record};
    let plan = async {
        let sections = index
            .sections
            .keys()
            .filter_map(|name| ResultSectionName::parse(name))
            .collect();
        let delivery = DeliveryDescriptor::retained(
            artifact.artifact_id.clone(),
            sections,
            DeliveryLimits {
                requested_max_bytes: requested,
                effective_max_bytes: Some(inline),
            },
        );
        let handle = ArtifactHandle {
            receipt: artifact.clone(),
            uri: enrichment_core::wire::ArtifactUri::try_from(format!(
                "{}artifacts/{}",
                enrichment_core::wire::ids::ARTIFACT_URI_SCHEME,
                artifact.artifact_id
            ))
            .map_err(|error| datafusion::common::DataFusionError::Execution(error.to_string()))?,
            description: "Complete native result and independently readable sections".into(),
        };
        let input = ViewInput {
            header,
            delivery,
            handles: vec![handle],
            coverage_details: RecoveryAction::ReadArtifact {
                artifact_id: artifact.artifact_id.clone(),
                section: Some(ArtifactSection::Result {
                    name: ResultSectionName::Coverage,
                }),
                cursor: None,
            },
        };
        let session = runtime.session();
        crate::native_catalog::work(
            &session,
            "delivery_input",
            crate::native_catalog::batch(&session, "result_delivery", ViewInput::batch(&[input])?)?
                .into_view(),
        )?;
        crate::native_catalog::work(
            &session,
            "delivery_policy",
            crate::native_catalog::batch(
                &session,
                "result_delivery",
                ViewPolicy::batch(&[
                    ViewPolicy {
                        rank: 0,
                        include_handles: true,
                        full_coverage: true,
                    },
                    ViewPolicy {
                        rank: 1,
                        include_handles: false,
                        full_coverage: true,
                    },
                    ViewPolicy {
                        rank: 2,
                        include_handles: false,
                        full_coverage: false,
                    },
                ])?,
            )?
            .into_view(),
        )?;
        let frame = session
            .sql("SELECT * FROM delivery_input CROSS JOIN delivery_policy")
            .await?;
        let coverage = col("header").field("coverage");
        let short = derive_record(
            coverage.clone(),
            &Coverage::data_type(),
            &[
                (
                    "limitations",
                    enrichment_core::evidence::arrow_model::expressions::literal(
                        &Vec::<String>::new(),
                    )?,
                ),
                (
                    "assessments",
                    enrichment_core::evidence::arrow_model::expressions::literal(&Vec::<
                        enrichment_core::wire::evidence::ScopeAssessment,
                    >::new(
                    ))?,
                ),
                ("details", col("coverage_details")),
            ],
        )?;
        let selected_coverage =
            datafusion::logical_expr::when(col("full_coverage"), coverage).otherwise(short)?;
        let header = derive_record(
            col("header"),
            &enrichment_core::operation::results::ResultHeader::data_type(),
            &[("coverage", selected_coverage)],
        )?;
        let handles = datafusion::logical_expr::when(col("include_handles"), col("handles"))
            .otherwise(
                enrichment_core::evidence::arrow_model::expressions::literal(
                    &Vec::<ArtifactHandle>::new(),
                )?,
            )?;
        let selected = record(
            &ResultRecord::data_type(),
            &[
                ("header", header),
                ("delivery", col("delivery")),
                ("artifacts", handles),
                (
                    "data",
                    enrichment_core::evidence::arrow_model::expressions::literal(&match &index
                        .record
                        .data
                    {
                        ToolData::JobControl(job) => ToolData::JobControl(job.clone()),
                        _ => ToolData::default(),
                    })?,
                ),
                (
                    "evidence",
                    enrichment_core::evidence::arrow_model::expressions::literal(&Vec::<
                        enrichment_core::wire::Evidence,
                    >::new(
                    ))?,
                ),
            ],
        )?;
        let frame = frame
            .select(vec![col("rank"), selected.alias("result")])?
            .with_column(
                "bytes",
                enrichment_core::native_transport::delivery(
                    runtime.session().runtime_env().memory_pool.clone(),
                    col("result"),
                    &request_id,
                    profile,
                ),
            )?
            .select(vec![col("rank"), col("bytes"), col("result")])?;
        crate::native_catalog::work(&session, "measured_delivery", frame.into_view())?;
        let selected = session
            .sql(&format!(
                "SELECT * FROM measured_delivery WHERE bytes<={inline} ORDER BY rank LIMIT 1"
            ))
            .await?;
        let rows = runtime.records::<Measured>(selected, 1).await?;
        if let Some(row) = rows.into_iter().next() {
            return Ok(Ok(row.result.into_envelope(request_id)));
        }
        let minimum = runtime
            .records::<Minimum>(
                session
                    .sql("SELECT min(bytes) AS bytes FROM measured_delivery")
                    .await?,
                1,
            )
            .await?
            .into_iter()
            .next()
            .ok_or_else(|| {
                datafusion::common::DataFusionError::Execution(
                    "delivery policy has no candidate".into(),
                )
            })?;
        Ok::<_, datafusion::common::DataFusionError>(Err(MinimumBudget {
            minimum: minimum.bytes as usize,
        }))
    }
    .await
    .map_err(std::io::Error::other)?;
    plan.map_err(|error| std::io::Error::new(std::io::ErrorKind::OutOfMemory, error))
}
