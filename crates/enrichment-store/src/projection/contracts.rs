//! Encoders are the schema authority for the bounded native contract projection.
use super::cells::{batch, column, optional, text};
use crate::native_catalog::{FieldInfo, RelationInfo, RuleInfo};
use arrow::{
    array::{BooleanArray, UInt64Array},
    error::ArrowError,
    record_batch::RecordBatch,
};
use enrichment_core::telemetry::InventorySummary;
use std::sync::Arc;

pub(crate) fn relations(rows: &[RelationInfo]) -> Result<RecordBatch, ArrowError> {
    batch(
        "native_relations",
        vec![
            column(
                "catalog_name",
                text(rows.iter().map(|r| r.catalog.as_str())),
                false,
                "catalog",
            ),
            column(
                "schema_name",
                text(rows.iter().map(|r| r.schema.as_str())),
                false,
                "schema",
            ),
            column(
                "table_name",
                text(rows.iter().map(|r| r.name.as_str())),
                false,
                "relation",
            ),
            column(
                "table_type",
                text(rows.iter().map(|r| r.kind.as_str())),
                false,
                "native_table_type",
            ),
            column(
                "binding_kind",
                text(rows.iter().map(|r| r.trust.as_str())),
                false,
                "binding_kind",
            ),
            column(
                "declared_key",
                optional(rows.iter().map(|r| r.declared_key.as_deref())),
                true,
                "declared_key",
            ),
            column(
                "validated_key",
                optional(rows.iter().map(|r| r.key.as_deref())),
                true,
                "validated_key",
            ),
            column(
                "verified_rows",
                Arc::new(UInt64Array::from(
                    rows.iter().map(|r| r.rows).collect::<Vec<_>>(),
                )),
                true,
                "admitted_rows",
            ),
        ],
    )
}

pub(crate) fn fields(rows: &[FieldInfo]) -> Result<RecordBatch, ArrowError> {
    batch(
        "native_fields",
        vec![
            column(
                "relation",
                text(rows.iter().map(|r| r.relation.as_str())),
                false,
                "relation",
            ),
            column(
                "field_path",
                text(rows.iter().map(|r| r.path.as_str())),
                false,
                "field_path",
            ),
            column(
                "data_type",
                text(rows.iter().map(|r| r.kind.as_str())),
                false,
                "native_type",
            ),
            column(
                "nullable",
                Arc::new(BooleanArray::from(
                    rows.iter().map(|r| r.nullable).collect::<Vec<_>>(),
                )),
                false,
                "nullable",
            ),
            column(
                "required_by_contract",
                Arc::new(BooleanArray::from(
                    rows.iter()
                        .map(|r| r.required_by_contract)
                        .collect::<Vec<_>>(),
                )),
                false,
                "declared-semantic-presence",
            ),
            column(
                "semantic_role",
                optional(rows.iter().map(|r| r.role.as_deref())),
                true,
                "semantic_role",
            ),
        ],
    )
}

pub(crate) fn inventory(summary: &InventorySummary) -> Result<RecordBatch, ArrowError> {
    batch(
        "native_inventory",
        vec![
            column(
                "relations",
                Arc::new(UInt64Array::from(vec![summary.relations.len() as u64])),
                false,
                "count",
            ),
            column(
                "nested_fields",
                Arc::new(UInt64Array::from(vec![summary.nested_fields as u64])),
                false,
                "count",
            ),
            column(
                "truncated",
                Arc::new(BooleanArray::from(vec![summary.truncated])),
                false,
                "truncated",
            ),
        ],
    )
}

pub(crate) fn rules(rows: &[RuleInfo]) -> Result<RecordBatch, ArrowError> {
    batch(
        "native_rules",
        vec![
            column(
                "relation",
                text(rows.iter().map(|r| r.relation.as_str())),
                false,
                "relation",
            ),
            column(
                "rule_id",
                text(rows.iter().map(|r| r.id.as_str())),
                false,
                "rule_id",
            ),
            column(
                "kind",
                text(rows.iter().map(|r| r.kind)),
                false,
                "rule_kind",
            ),
            column(
                "source_field",
                text(rows.iter().map(|r| r.field.as_str())),
                false,
                "field",
            ),
            column(
                "target_relation",
                optional(rows.iter().map(|r| r.target.as_deref())),
                true,
                "relation",
            ),
            column(
                "condition_field",
                optional(rows.iter().map(|r| r.condition.map(|(field, _)| field))),
                true,
                "field",
            ),
            column(
                "condition_value",
                optional(rows.iter().map(|r| r.condition.map(|(_, value)| value))),
                true,
                "literal",
            ),
            column(
                "target_field",
                optional(rows.iter().map(|r| r.target_field.as_deref())),
                true,
                "field",
            ),
        ],
    )
}
