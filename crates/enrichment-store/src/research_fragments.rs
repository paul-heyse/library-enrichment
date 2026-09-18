//! Bounded text, recovery attachment and truncation share declared native fragment records.
use crate::runtime::QueryRuntime;
use datafusion::{
    common::{DataFusionError, Result, ScalarValue},
    dataframe::DataFrame,
    logical_expr::when,
    prelude::{col, lit},
};
use enrichment_core::{
    evidence::{TextFragment, arrow_model::expressions::record},
    native_union::{Cell, NativeStruct, Rule},
    wire::{RecoveryAction, data::FragmentProjection},
};

enrichment_core::native_struct! { pub struct SelectedFragment {
    fragment:TextFragment => Rule::Text,
    text_complete:bool => Rule::Text,
} }
enrichment_core::native_struct! { struct Ordered {
    ordinal:usize => Rule::Coordinate(enrichment_core::native_union::Unit::Ordinal),
    selected:SelectedFragment => Rule::Text,
} }
enrichment_core::native_struct! { struct Policy {
    recovery:RecoveryAction => Rule::Text,
    has_more:bool => Rule::Text,
} }
enrichment_core::native_struct! { struct Summary { truncated:bool=>Rule::Text } }

pub(crate) fn from_surface(frame: DataFrame) -> Result<DataFrame> {
    frame.select(vec![
        record(
            &TextFragment::data_type(),
            &[
                ("fragment_id", col("fragment_id")),
                ("kind", col("kind")),
                ("subject", col("subject_ref")),
                ("display_subject", col("label")),
                ("text", col("text")),
                ("source", col("source")),
            ],
        )?
        .alias("fragment"),
        col("text_complete"),
    ])
}

pub struct Delivery {
    pub items: Vec<FragmentProjection>,
    pub truncated: bool,
}
pub async fn deliver(
    runtime: &QueryRuntime,
    items: Vec<SelectedFragment>,
    recovery: RecoveryAction,
    has_more: bool,
) -> Result<Delivery> {
    use datafusion::functions::core::expr_ext::FieldAccessor;
    let maximum = items.len();
    let items = items
        .into_iter()
        .enumerate()
        .map(|(ordinal, selected)| Ordered { ordinal, selected })
        .collect::<Vec<_>>();
    let session = runtime.session();
    crate::native_catalog::input(&session, "fragment_selected", Ordered::batch(&items)?)?;
    crate::native_catalog::input(
        &session,
        "fragment_delivery",
        Policy::batch(&[Policy { recovery, has_more }])?,
    )?;
    let frame=session.sql("SELECT s.*,p.recovery FROM fragment_selected s CROSS JOIN fragment_delivery p ORDER BY s.ordinal").await?;
    let recovery = when(
        col("selected").field("text_complete"),
        lit(ScalarValue::try_from(&RecoveryAction::data_type())?),
    )
    .otherwise(col("recovery"))?;
    let frame = frame.select(vec![
        col("selected").field("fragment").alias("fragment"),
        col("selected")
            .field("text_complete")
            .alias("text_complete"),
        recovery.alias("complete"),
    ])?;
    let items = runtime.records(frame, maximum).await?;
    let summary=runtime.records::<Summary>(session.sql("SELECT p.has_more OR s.incomplete AS truncated FROM fragment_delivery p CROSS JOIN (SELECT count(*) FILTER (WHERE NOT selected.text_complete)>0 AS incomplete FROM fragment_selected) s").await?,1).await?.pop().ok_or_else(||DataFusionError::Internal("fragment delivery summary missing".into()))?;
    Ok(Delivery {
        items,
        truncated: summary.truncated,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use enrichment_core::{
        evidence::{
            FragmentKind,
            relational::{FactSource, Locator, SubjectRef},
        },
        wire::{EvidenceClass, SourceVersionMatch},
    };
    enrichment_core::native_struct! { struct Surface {
        fragment_id:String=>Rule::NonEmpty,
        kind:FragmentKind=>Rule::Text,
        subject_ref:SubjectRef=>Rule::Text,
        label:String=>Rule::Text,
        source:FactSource=>Rule::Text,
        text:String=>Rule::Text,
        text_complete:bool=>Rule::Text,
    } }
    #[tokio::test]
    async fn native_fragment_delivery_keeps_pairing_recovery_presence_and_truncation() -> Result<()>
    {
        let root = tempfile::tempdir()?;
        let runtime = QueryRuntime::new(root.path(), Default::default())?;
        let source = FactSource {
            producer_binding_id: format!("producer_{}", "1".repeat(64)),
            extractor: "docs".into(),
            extractor_version: "1".into(),
            artifact_id: format!("art_{}", "2".repeat(64)),
            source_uri: None,
            source_version_match: SourceVersionMatch::Exact,
            locator: Locator::Artifact,
            evidence_class: EvidenceClass::Declared,
        };
        let first = Surface {
            fragment_id: "fragment_first".into(),
            kind: FragmentKind::DocText,
            subject_ref: SubjectRef::Document {
                artifact_id: source.artifact_id.clone(),
                heading: "λ".into(),
            },
            label: "λ".into(),
            source: source.clone(),
            text: "λ🦀".into(),
            text_complete: false,
        };
        let second = Surface {
            fragment_id: "fragment_second".into(),
            text_complete: true,
            ..first.clone()
        };
        let frame = crate::native_catalog::batch(
            &runtime.session(),
            "research_fragments",
            Surface::batch(&[first, second])?,
        )?;
        let items = runtime
            .records::<SelectedFragment>(from_surface(frame)?, 2)
            .await?;
        assert!(!items[0].text_complete);
        assert!(items[1].text_complete);
        let recovery = RecoveryAction::ReadArtifact {
            artifact_id: source.artifact_id.clone(),
            section: None,
            cursor: None,
        };
        let delivery = deliver(&runtime, items.clone(), recovery.clone(), false).await?;
        assert!(delivery.truncated);
        assert_eq!(delivery.items[0].complete, Some(recovery.clone()));
        assert_eq!(delivery.items[1].complete, None);
        assert_eq!(delivery.items[0].fragment.source, source);
        assert_eq!(delivery.items[0].fragment.fragment_id, "fragment_first");
        let complete = vec![items[1].clone()];
        assert!(
            !deliver(&runtime, complete.clone(), recovery.clone(), false)
                .await?
                .truncated
        );
        assert!(
            deliver(&runtime, complete, recovery.clone(), true)
                .await?
                .truncated
        );
        let empty = deliver(&runtime, vec![], recovery, false).await?;
        assert!(empty.items.is_empty());
        assert!(!empty.truncated);
        runtime.close_diagnostics().await?;
        Ok(())
    }
}
