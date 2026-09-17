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

#[derive(Debug, thiserror::Error)]
#[error("delivery requires at least {minimum} encoded bytes")]
pub struct MinimumBudget {
    pub minimum: usize,
}

/// Project bounded alternatives from the admitted header, then select the richest fitting view.
/// A caller can retrieve every omitted field from the same immutable result and coverage section.
pub async fn retained(
    runtime: &QueryRuntime,
    header: enrichment_core::operation::results::ResultHeader,
    artifact: &enrichment_core::evidence::Artifact,
    index: &Index,
    inline: usize,
    requested: Option<usize>,
    request_id: enrichment_core::wire::RequestId,
) -> std::io::Result<enrichment_core::wire::Envelope> {
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
            session.read_batch(ViewInput::batch(&[input])?)?.into_view(),
        )?;
        crate::native_catalog::work(
            &session,
            "delivery_policy",
            session
                .read_batch(ViewPolicy::batch(&[
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
                ])?)?
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
                    enrichment_core::evidence::arrow_model::expressions::literal(
                        &ToolData::default(),
                    )?,
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
        let frame = frame.select(vec![
            col("rank"),
            lit(0u64).alias("bytes"),
            selected.alias("result"),
        ])?;
        let mut candidates = runtime.records::<Measured>(frame, 3).await?;
        for candidate in &mut candidates {
            candidate.bytes = enrichment_core::canonical::serialized_size(
                &candidate.result.clone().into_envelope(request_id.clone()),
                crate::result::MAX_BYTES as usize,
            )? as u64;
        }
        crate::native_catalog::work(
            &session,
            "measured_delivery",
            session
                .read_batch(Measured::batch(&candidates)?)?
                .into_view(),
        )?;
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
