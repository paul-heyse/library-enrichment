//! One native citation/excerpt projection for search, discovery, inspection and acquisition.
use crate::runtime::QueryRuntime;
use datafusion::{
    common::{DataFusionError, Result},
    dataframe::DataFrame,
    functions::{
        core::expr_ext::FieldAccessor,
        string::expr_fn::concat,
        unicode::expr_fn::{character_length, left},
    },
    logical_expr::{Expr, when},
    prelude::{col, lit},
};
use enrichment_core::{
    evidence::{
        TextFragment,
        arrow_model::expressions::record,
        relational::{FactSource, SubjectRef},
    },
    native_union::{Cell, NativeStruct, Rule},
    wire::Evidence,
};

enrichment_core::native_struct! {
    /// A qualified fact before presentation. The excerpt is excluded from its identity.
    pub struct CitationInput {
        fact_id: String => Rule::NonEmpty,
        subject: SubjectRef => Rule::Text,
        display_subject: String => Rule::Text,
        source: FactSource => Rule::Text,
        text: String => Rule::Text,
    }
}
enrichment_core::native_struct! { struct OrderedInput {
    ordinal:usize => Rule::Coordinate(enrichment_core::native_union::Unit::Ordinal),
    input:CitationInput => Rule::Text,
} }
enrichment_core::native_struct! { struct Selected { evidence:Evidence => Rule::Text } }

/// Unicode scalar length, one ellipsis when truncated, and no text for a zero-width request.
/// DataFusion evaluates this policy wherever a research citation is presented.
pub(crate) fn excerpt(text: Expr, characters: usize) -> Result<Expr> {
    let maximum =
        i64::try_from(characters).map_err(|error| DataFusionError::Plan(error.to_string()))?;
    when(lit(maximum).eq(lit(0i64)), lit(""))
        .when(
            character_length(text.clone()).gt(lit(maximum)),
            concat(vec![
                left(text.clone(), lit(maximum.saturating_sub(1).max(0))),
                lit("…"),
            ]),
        )
        .otherwise(text)
}

pub(crate) fn citation(input: Expr, characters: usize) -> Result<Expr> {
    record(
        &Evidence::data_type(),
        &[
            (
                "evidence_id",
                enrichment_core::native_key::Key::Citation.bind(vec![
                    input.clone().field("fact_id"),
                    input.clone().field("subject"),
                    input.clone().field("source"),
                ])?,
            ),
            ("subject", input.clone().field("subject")),
            ("display_subject", input.clone().field("display_subject")),
            ("source", input.clone().field("source")),
            ("excerpt", excerpt(input.field("text"), characters)?),
        ],
    )
}

fn selected(frame: DataFrame, maximum: usize, characters: usize) -> Result<DataFrame> {
    frame
        .sort(vec![col("ordinal").sort(true, false)])?
        .limit(0, Some(maximum))?
        .select(vec![citation(col("input"), characters)?.alias("evidence")])
}

pub async fn inputs(
    runtime: &QueryRuntime,
    inputs: Vec<CitationInput>,
    maximum: usize,
    characters: usize,
) -> Result<Vec<Evidence>> {
    let values = inputs
        .into_iter()
        .enumerate()
        .map(|(ordinal, input)| OrderedInput { ordinal, input })
        .collect::<Vec<_>>();
    let frame = crate::native_catalog::batch(
        &runtime.session(),
        "research_citations",
        OrderedInput::batch(&values)?,
    )?;
    Ok(runtime
        .records::<Selected>(selected(frame, maximum, characters)?, maximum)
        .await?
        .into_iter()
        .map(|row| row.evidence)
        .collect())
}

pub async fn fragments(
    runtime: &QueryRuntime,
    fragments: Vec<&TextFragment>,
    maximum: usize,
    characters: usize,
) -> Result<Vec<Evidence>> {
    inputs(
        runtime,
        fragments
            .into_iter()
            .map(|fragment| CitationInput {
                fact_id: fragment.fragment_id.clone(),
                subject: fragment.subject.clone(),
                display_subject: fragment.display_subject.clone(),
                source: fragment.source.clone(),
                text: fragment.text.clone(),
            })
            .collect(),
        maximum,
        characters,
    )
    .await
}

#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn native_citations_preserve_qualified_identity_order_and_exact_excerpt_bounds()
    -> Result<()> {
        use enrichment_core::{
            evidence::relational::Locator,
            wire::{EvidenceClass, SourceVersionMatch},
        };
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
        let input = CitationInput {
            fact_id: "z_first".into(),
            subject: SubjectRef::Document {
                artifact_id: source.artifact_id.clone(),
                heading: "first".into(),
            },
            display_subject: "first".into(),
            source,
            text: "λ🦀BC".into(),
        };
        let mut next = input.clone();
        next.fact_id = "a_second".into();
        let all = inputs(
            &runtime,
            vec![input.clone(), next.clone(), input.clone()],
            3,
            20,
        )
        .await?;
        assert_eq!(all.len(), 3);
        assert_eq!(all[0], all[2], "duplicates are preserved");
        assert_ne!(all[0].evidence_id, all[1].evidence_id);
        for (width, expected) in [(0, ""), (1, "…"), (3, "λ🦀…"), (4, "λ🦀BC")] {
            let selected = inputs(&runtime, vec![input.clone(), next.clone()], 1, width).await?;
            assert_eq!(selected.len(), 1);
            assert_eq!(
                selected[0].evidence_id, all[0].evidence_id,
                "sampling follows input ordinal"
            );
            assert_eq!(selected[0].excerpt, expected);
        }
        let mut other = input.clone();
        other.source.extractor_version = "2".into();
        let changed = inputs(&runtime, vec![other], 1, 20).await?;
        assert_ne!(
            changed[0].evidence_id, all[0].evidence_id,
            "provenance is identity"
        );
        assert!(inputs(&runtime, vec![input], 0, 2).await?.is_empty());
        assert!(inputs(&runtime, vec![], 100, 2).await?.is_empty());
        runtime.close_diagnostics().await?;
        Ok(())
    }
}
