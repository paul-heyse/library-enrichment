//! Native result contracts use Arrow fields, not a parallel type system.
use arrow::datatypes::{DataType, Field, Schema};
use datafusion::error::{DataFusionError, Result};
use enrichment_core::telemetry::MaterializationFamily;
use enrichment_core::wire::Diagnostic;

/// Check the complete selected physical tree before starting streams. DataFusion owns
/// executable distribution/order/child invariants; native declarations own field meaning.
/// Root-schema equality alone cannot detect a malformed child hidden by a projection.
pub(crate) fn admit_physical(
    root: &std::sync::Arc<dyn datafusion::physical_plan::ExecutionPlan>,
) -> Result<()> {
    use datafusion::physical_plan::execution_plan::InvariantLevel;
    let mut pending = vec![(root.clone(), 0_usize)];
    let mut visited = 0_usize;
    while let Some((plan, depth)) = pending.pop() {
        visited += 1;
        if depth > 512 || visited > 262_144 {
            return datafusion::common::plan_err!("physical plan traversal bound");
        }
        plan.check_invariants(InvariantLevel::Executable)?;
        enrichment_core::native_analysis::validate_derived_fields(plan.schema().fields())?;
        pending.extend(
            plan.children()
                .into_iter()
                .map(|child| (child.clone(), depth + 1)),
        );
    }
    Ok(())
}

pub(crate) fn witnesses(batches: &[arrow::record_batch::RecordBatch]) -> Vec<String> {
    use arrow::array::{LargeStringArray, StringArray, StringViewArray};
    let mut ids = Vec::new();
    for batch in batches {
        for column in batch.columns() {
            let values: Box<dyn Iterator<Item = &str> + '_> =
                if let Some(values) = column.as_any().downcast_ref::<StringArray>() {
                    Box::new(values.iter().flatten())
                } else if let Some(values) = column.as_any().downcast_ref::<LargeStringArray>() {
                    Box::new(values.iter().flatten())
                } else if let Some(values) = column.as_any().downcast_ref::<StringViewArray>() {
                    Box::new(values.iter().flatten())
                } else {
                    continue;
                };
            for value in values.take(8 - ids.len()) {
                // IDs are normally short; damaged values must not inflate a diagnostic.
                let mut end = value.len().min(256);
                while !value.is_char_boundary(end) {
                    end -= 1;
                }
                ids.push(value[..end].to_owned());
            }
            if ids.len() == 8 {
                return ids;
            }
        }
    }
    ids
}

#[derive(Debug, thiserror::Error)]
#[error("{rule} at {stage}: {affected_ids:?}")]
pub struct InvariantFailure {
    pub cause: enrichment_core::wire::DiagnosticCause,
    pub rule: String,
    pub stage: String,
    pub affected_ids: Vec<String>,
    pub operation_id: Option<String>,
}

impl InvariantFailure {
    pub fn error(rule: &str, stage: &str, affected_ids: Vec<String>) -> DataFusionError {
        Self::with_cause(
            rule,
            stage,
            affected_ids,
            enrichment_core::wire::DiagnosticCause::CorruptState,
        )
    }

    pub(crate) fn contract(rule: &str, stage: &str, affected_ids: Vec<String>) -> DataFusionError {
        Self::with_cause(
            rule,
            stage,
            affected_ids,
            enrichment_core::wire::DiagnosticCause::Internal,
        )
    }

    pub(crate) fn with_cause(
        rule: &str,
        stage: &str,
        affected_ids: Vec<String>,
        cause: enrichment_core::wire::DiagnosticCause,
    ) -> DataFusionError {
        DataFusionError::External(Box::new(Self {
            cause,
            rule: rule.into(),
            stage: stage.into(),
            affected_ids: affected_ids.into_iter().take(8).collect(),
            operation_id: crate::runtime::operation_id(),
        }))
    }

    pub fn apply(&self, diagnostic: &mut Diagnostic) {
        diagnostic.stage.clone_from(&self.stage);
        diagnostic.rule = Some(self.rule.clone());
        diagnostic.affected_ids.clone_from(&self.affected_ids);
        diagnostic.correlation_id.clone_from(&self.operation_id);
    }
}

/// Small independent decoder contracts. These use native Arrow fields and are checked before
/// preparation; the runtime then preserves them through analysis and physical execution.
#[derive(Clone, Copy, Debug)]
pub enum QueryFamily {
    Relation(crate::admission::Relation),
    SymbolHeaders,
    Observations { docs: bool },
    FragmentProjection { bounded: bool },
    Count,
    CountUnsigned,
    InvariantWitness,
    PythonClassScope,
    Paths,
    Catalog(crate::control::Table),
    CatalogArtifact,
    StaticInputs,
    Intermediate(MaterializationFamily),
    Search,
    ComparisonAlternatives,
    RevisionDisposition,
}

impl QueryFamily {
    /// Finite operation-local reuse eligibility belongs to the output declaration.
    pub(crate) const fn materialization(self) -> Option<MaterializationFamily> {
        match self {
            Self::Intermediate(family) => Some(family),
            _ => None,
        }
    }

    pub fn require(self, actual: &Schema) -> Result<()> {
        use crate::admission::Relation;
        let relation_fields = |relation: Relation| -> Result<Vec<Field>> {
            Ok(relation
                .schema()?
                .fields()
                .iter()
                .map(|f| f.as_ref().clone())
                .collect())
        };
        let (stage, fields) = match self {
            Self::PythonClassScope => (
                "python_class_scope",
                [
                    "observations",
                    "unsupported_observations",
                    "unsupported_bases",
                ]
                .into_iter()
                .map(|name| Field::new(name, DataType::Int64, false))
                .collect(),
            ),
            Self::InvariantWitness => (
                "invariant_witness",
                vec![Field::new("witness_id", DataType::Utf8, true)],
            ),
            Self::Catalog(table) => (
                table.name(),
                table
                    .schema()?
                    .fields()
                    .iter()
                    // Native control views use Delta's nested read layout. Required values
                    // are enforced by the shared semantic field predicates, not inferred.
                    .map(|f| {
                        enrichment_core::evidence::arrow_model::cells::native_read_field(f, true)
                    })
                    .collect(),
            ),
            Self::CatalogArtifact => (
                "catalog_artifact",
                vec![Field::new(
                    "artifact",
                    enrichment_core::evidence::arrow_model::cells::native_read_field(
                        crate::projection::catalog::job_publications(&[])?
                            .schema()
                            .field_with_name("delivery")?,
                        true,
                    )
                    .data_type()
                    .clone(),
                    true,
                )],
            ),
            Self::StaticInputs => (
                "static_inputs",
                ["sha256", "source_uri"]
                    .into_iter()
                    .map(|name| {
                        Relation::InputArtifacts
                            .schema()?
                            .field_with_name(name)
                            .cloned()
                            .map_err(Into::into)
                    })
                    .collect::<Result<Vec<_>>>()?,
            ),
            Self::Relation(relation) => (relation.name(), relation_fields(relation)?),
            Self::SymbolHeaders => {
                let mut fields = relation_fields(Relation::Symbols)?;
                let definition = Relation::Definitions.schema()?;
                for name in ["kind", "definition_path", "defined_in_package"] {
                    fields.push(definition.field_with_name(name)?.clone());
                }
                fields.push(Field::new("parent_path", DataType::Utf8, true));
                ("symbol_headers", fields)
            }
            Self::Observations { docs } => {
                let schema = Relation::ApiObservations.schema()?;
                let schema = if docs {
                    schema
                } else {
                    crate::projection::inspection_schema(&schema)?
                };
                (
                    "inspection_observations",
                    schema.fields().iter().map(|f| f.as_ref().clone()).collect(),
                )
            }
            Self::FragmentProjection { bounded } => {
                let schema = Relation::Fragments.schema()?;
                let mut fields = ["fragment_id", "kind", "source"]
                    .into_iter()
                    .map(|name| schema.field_with_name(name).cloned())
                    .collect::<std::result::Result<Vec<_>, _>>()?;
                fields.push(
                    schema
                        .field_with_name("subject")?
                        .clone()
                        .with_name("subject_ref"),
                );
                // DataFusion 55.1's substring result is conservatively nullable even with
                // non-null bound arguments. Do not invent a stronger native guarantee.
                fields.push(Field::new("text", DataType::Utf8, bounded));
                // A subject's label is derived through a left join, not copied ID metadata.
                fields.push(Field::new("label", DataType::Utf8, true));
                if bounded {
                    fields.push(Field::new("text_complete", DataType::Boolean, true));
                }
                ("fragment_projection", fields)
            }
            Self::Count => ("count", vec![Field::new("count", DataType::Int64, false)]),
            Self::CountUnsigned => (
                "count_unsigned",
                vec![Field::new("count", DataType::UInt64, false)],
            ),
            Self::Paths => (
                "paths",
                vec![Relation::Symbols.schema()?.field_with_name("path")?.clone()],
            ),
            Self::ComparisonAlternatives => {
                let source = crate::projection::observations(&[])?
                    .schema()
                    .field_with_name("source")?
                    .clone()
                    .with_nullable(true);
                require_fields(actual, &[source], "comparison_alternatives")?;
                let expected = enrichment_core::native_union::field::<
                    enrichment_core::compare::ComparisonValue,
                >("value", enrichment_core::native_union::Rule::Text);
                require_fields(actual, &[expected], "comparison_alternatives")?;
                return Ok(());
            }
            Self::RevisionDisposition => (
                "revision_disposition",
                vec![
                    Field::new("path", DataType::Utf8, true),
                    Field::new("summary", DataType::Boolean, false),
                    Field::new("incomplete", DataType::Boolean, false),
                ],
            ),
            Self::Search | Self::Intermediate(MaterializationFamily::SearchIndex) => {
                let mut fields = vec![
                    Field::new("hit_order", DataType::UInt32, false),
                    Field::new("candidate_id", DataType::Utf8, true),
                    Field::new("fact_id", DataType::Utf8, true),
                    Field::new("label", DataType::Utf8, true),
                    Field::new("path", DataType::Utf8, true),
                    Field::new("symbol_kind", DataType::Utf8, true),
                    Field::new("fragment_kind", DataType::Utf8, true),
                    Field::new("deprecated", DataType::Boolean, true),
                    Field::new(
                        "also_at",
                        DataType::List(std::sync::Arc::new(Field::new(
                            "item",
                            DataType::Utf8,
                            true,
                        ))),
                        true,
                    ),
                    Field::new(
                        "ranking",
                        <enrichment_core::search::Ranking as enrichment_core::native_union::Cell>::data_type(),
                        true,
                    ),
                    Field::new("rank_score", DataType::UInt32, true),
                ];
                if matches!(self, Self::Search) {
                    fields.extend([
                        Field::new("signature", DataType::Utf8, true),
                        Field::new("excerpt", DataType::Utf8, true),
                        Relation::ApiObservations
                            .schema()?
                            .field_with_name("source")?
                            .clone()
                            .with_nullable(true),
                        Relation::ApiObservations
                            .schema()?
                            .field_with_name("subject")?
                            .clone()
                            .with_name("subject_ref")
                            .with_nullable(true),
                    ]);
                }
                ("search_index_or_output", fields)
            }
            Self::Intermediate(MaterializationFamily::ComparisonKeys) => (
                "comparison_keys",
                vec![
                    Field::new("plan", DataType::UInt64, false),
                    Field::new("key", DataType::Utf8, true),
                    Field::new("label", DataType::Utf8, true),
                ],
            ),
            Self::Intermediate(MaterializationFamily::OverviewChildren) => (
                "overview_children",
                vec![
                    Field::new("namespace_path", DataType::Utf8, true),
                    Field::new(
                        "namespace_components",
                        DataType::List(std::sync::Arc::new(Field::new(
                            "item",
                            DataType::Utf8,
                            true,
                        ))),
                        true,
                    ),
                    Field::new("ecosystem", DataType::Utf8, true),
                    Field::new("path", DataType::Utf8, true),
                    Field::new("kind", DataType::Utf8, true),
                    Field::new("definition_id", DataType::Utf8, true),
                    Field::new("doc_summary", DataType::Utf8, true),
                    Field::new("is_reexport", DataType::Boolean, true),
                    Field::new("is_deprecated", DataType::Boolean, true),
                    Field::new("definition_position", DataType::UInt64, false),
                ],
            ),
            Self::Intermediate(MaterializationFamily::OverviewNamespaces) => (
                "overview_namespaces",
                vec![
                    Field::new("path", DataType::Utf8, true),
                    Field::new(
                        "components",
                        DataType::List(std::sync::Arc::new(Field::new(
                            "item",
                            DataType::Utf8,
                            true,
                        ))),
                        true,
                    ),
                    Field::new("ecosystem", DataType::Utf8, true),
                ],
            ),
        };
        for expected in fields {
            let fields = actual
                .fields()
                .iter()
                .filter(|f| f.name() == expected.name())
                .collect::<Vec<_>>();
            if fields.len() != 1 || !compatible(fields[0], &expected, false) {
                return Err(InvariantFailure::contract(
                    "query family field",
                    stage,
                    vec![expected.name().clone()],
                ));
            }
        }
        Ok(())
    }
}

/// Only types produced by the authored comparison axes may cross the JSON value boundary.
/// Nested nulls and native string coercion are supported without accepting arbitrary Arrow
/// extensions whose JSON interpretation would need a separate contract.
/// Explicit family requirements also retain selected field tags. Nullability describes
/// permitted values: a non-null result satisfies a nullable requirement, never the reverse.
pub fn require_fields(actual: &Schema, required: &[Field], stage: &str) -> Result<()> {
    for expected in required {
        let fields = actual
            .fields()
            .iter()
            .filter(|f| f.name() == expected.name())
            .collect::<Vec<_>>();
        if fields.len() != 1 || !compatible(fields[0], expected, true) {
            return Err(InvariantFailure::contract(
                "required result field",
                stage,
                vec![expected.name().clone()],
            ));
        }
    }
    Ok(())
}

/// Preserve native projected names/types and non-null guarantees through preparation.
/// Source provenance may legitimately disappear on synthesized outer-join nulls. Only
/// trusted output-role metadata is required here; it never asserts row membership.
pub(crate) fn result(actual: &Schema, expected: &Schema, stage: &str) -> Result<()> {
    if actual.fields().len() != expected.fields().len() {
        return Err(InvariantFailure::contract(
            "result column count",
            stage,
            Vec::new(),
        ));
    }
    for (actual, expected) in actual.fields().iter().zip(expected.fields()) {
        if !compatible(actual, expected, false) {
            return Err(InvariantFailure::contract(
                "native result field",
                stage,
                vec![expected.name().clone()],
            )
            .context(format!(
                "actual field {}; expected field {}",
                format!("{actual:?}").chars().take(2048).collect::<String>(),
                format!("{expected:?}")
                    .chars()
                    .take(2048)
                    .collect::<String>()
            )));
        }
    }
    Ok(())
}

fn compatible(actual: &Field, expected: &Field, all_metadata: bool) -> bool {
    actual.name() == expected.name()
        && compatible_type(actual.data_type(), expected.data_type(), !all_metadata)
        && (expected.is_nullable() || !actual.is_nullable())
        && expected
            .metadata()
            .iter()
            .filter(|(key, _)| {
                all_metadata
                    || key.starts_with("enrichment.")
                    || key.starts_with("ARROW:extension:")
            })
            .all(|(key, value)| actual.metadata().get(key) == Some(value))
}

fn compatible_type(actual: &DataType, expected: &DataType, coercion: bool) -> bool {
    if actual == expected {
        return true;
    }
    if !coercion {
        return false;
    }
    if actual.is_string() && expected.is_string() {
        return true;
    }
    match (actual, expected) {
        (DataType::Struct(a), DataType::Struct(e)) => {
            a.len() == e.len() && a.iter().zip(e).all(|(a, e)| compatible(a, e, false))
        }
        (DataType::List(a), DataType::List(e))
        | (DataType::LargeList(a), DataType::LargeList(e)) => compatible(a, e, false),
        (DataType::FixedSizeList(a, n), DataType::FixedSizeList(e, m)) => {
            n == m && compatible(a, e, false)
        }
        _ => false,
    }
}

/// After physical planning there is no further logical type coercion. Even empty streams
/// must expose the exact physical fields, including nullability and metadata.
pub(crate) fn physical(actual: &Schema, expected: &Schema, stage: &str) -> Result<()> {
    if actual != expected {
        return Err(InvariantFailure::contract(
            "physical result schema",
            stage,
            Vec::new(),
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use arrow::array::{ArrayRef, LargeStringArray, StringViewArray};
    use arrow::record_batch::RecordBatch;
    use std::sync::Arc;

    #[test]
    fn native_text_witnesses_are_bounded_and_preserve_utf8() {
        let long = "é".repeat(200);
        let batch = RecordBatch::try_from_iter(vec![
            (
                "view",
                Arc::new(StringViewArray::from(vec![None, Some("view-id")])) as ArrayRef,
            ),
            (
                "large",
                Arc::new(LargeStringArray::from(vec![
                    Some("large-id"),
                    Some(long.as_str()),
                ])) as ArrayRef,
            ),
        ])
        .unwrap();
        let ids = witnesses(&[batch]);
        assert_eq!(ids, ["view-id", "large-id", &"é".repeat(128)]);
    }
}
