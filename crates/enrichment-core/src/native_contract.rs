//! Generated schema facts and native contract-change selection. Paths are segments, never SQL.
use crate::native_union::{NativeStruct, Rule};
use arrow::datatypes::{DataType, Field, Schema};
use datafusion::{
    common::Result,
    dataframe::DataFrame,
    prelude::{SessionContext, col},
};
use std::collections::BTreeMap;

crate::native_struct! {
    pub struct ContractField {
        projection: String => Rule::Text,
        path: Vec<String> => Rule::Sequence,
        properties: BTreeMap<String, String> => Rule::Map,
    }
}
crate::native_struct! {
    pub struct Manifest {
        fields: Vec<ContractField> => Rule::Set,
    }
}
crate::native_struct! {
    pub struct Change {
        projection: String => Rule::Text,
        path: Vec<String> => Rule::Sequence,
        kind: String => Rule::Text,
        before: Option<ContractField> => Rule::Text,
        after: Option<ContractField> => Rule::Text,
    }
}

fn metadata(values: &std::collections::HashMap<String, String>) -> BTreeMap<String, String> {
    values
        .iter()
        .filter(|(key, _)| key.as_str() != "enrichment.role")
        .map(|(key, value)| (format!("metadata/{key}"), value.clone()))
        .collect()
}

fn shape(kind: &DataType) -> String {
    match kind {
        DataType::Struct(_) => "Struct".into(),
        DataType::List(_) => "List".into(),
        DataType::LargeList(_) => "LargeList".into(),
        DataType::ListView(_) => "ListView".into(),
        DataType::LargeListView(_) => "LargeListView".into(),
        DataType::FixedSizeList(_, size) => format!("FixedSizeList({size})"),
        DataType::Map(_, ordered) => format!("Map({ordered})"),
        DataType::RunEndEncoded(_, _) => "RunEndEncoded".into(),
        DataType::Union(fields, mode) => format!(
            "Union({mode:?},{:?})",
            fields.iter().map(|(tag, _)| tag).collect::<Vec<_>>()
        ),
        DataType::Dictionary(key, value) => format!("Dictionary({key},{})", shape(value)),
        value => value.to_string(),
    }
}

fn capture(
    field: &Field,
    projection: &str,
    path: Vec<String>,
    ordinal: usize,
    rows: &mut Vec<ContractField>,
) {
    let mut properties = metadata(field.metadata());
    properties.extend([
        ("type".into(), shape(field.data_type())),
        ("nullable".into(), field.is_nullable().to_string()),
        ("ordinal".into(), ordinal.to_string()),
    ]);
    rows.push(ContractField {
        projection: projection.into(),
        path: path.clone(),
        properties,
    });
    let mut child = |child: &Field, ordinal| {
        let mut path = path.clone();
        path.push(child.name().clone());
        capture(child, projection, path, ordinal, rows);
    };
    match field.data_type() {
        DataType::Struct(fields) => {
            for (ordinal, field) in fields.iter().enumerate() {
                child(field, ordinal);
            }
        }
        DataType::List(field)
        | DataType::LargeList(field)
        | DataType::ListView(field)
        | DataType::LargeListView(field)
        | DataType::FixedSizeList(field, _)
        | DataType::Map(field, _) => child(field, 0),
        DataType::Union(fields, _) => {
            for (ordinal, (_, field)) in fields.iter().enumerate() {
                child(field, ordinal);
            }
        }
        DataType::RunEndEncoded(ends, values) => {
            child(ends, 0);
            child(values, 1);
        }
        DataType::Dictionary(_, value) => {
            child(
                &Field::new("dictionary_values", value.as_ref().clone(), true),
                0,
            );
        }
        _ => {}
    }
}

impl Manifest {
    /// The existing bounded schema validator runs before recursion. Values remain untrusted;
    /// describing a field never grants admission to a table or execution operation.
    pub fn new(semantic: &Schema, storage: &Schema) -> Result<Self> {
        let mut fields = vec![ContractField {
            projection: "contract".into(),
            path: vec![],
            properties: [
                ("manifest".into(), "2".into()),
                ("canonical".into(), "2".into()),
                ("intrinsic".into(), "7".into()),
                ("extensions".into(), "1".into()),
                ("delta_mapping".into(), "2".into()),
                ("wire".into(), crate::native_json::REVISION.into()),
            ]
            .into(),
        }];
        for (projection, schema) in [("semantic", semantic), ("storage", storage)] {
            crate::native_schema::validate(schema)?;
            fields.push(ContractField {
                projection: projection.into(),
                path: vec![],
                properties: metadata(schema.metadata()),
            });
            for (ordinal, field) in schema.fields().iter().enumerate() {
                capture(
                    field,
                    projection,
                    vec![field.name().clone()],
                    ordinal,
                    &mut fields,
                );
            }
        }
        Ok(Self { fields })
    }

    pub fn identity(&self) -> Result<crate::identity::SchemaContractId> {
        crate::identity::SchemaContractId::try_from_record(self)
    }
}

/// A native full join explains additions, removals and changed rules/layout/codec witnesses.
/// The caller supplies immutable paid inputs and a scoped session. This layer only plans;
/// it cannot create an unaccounted mutable MemTable while verifying a cached contract.
pub async fn changes(
    session: &SessionContext,
    before: DataFrame,
    after: DataFrame,
) -> Result<DataFrame> {
    for (name, input) in [("contract_before", before), ("contract_after", after)] {
        let key = crate::native_key::Key::ContractField;
        let frame = input
            .with_column("field_identity", key.expression())?
            .with_column(
                "descriptor",
                crate::native_record::record(
                    ContractField::fields(),
                    ContractField::fields()
                        .iter()
                        .flat_map(|field| {
                            [datafusion::prelude::lit(field.name()), col(field.name())]
                        })
                        .collect(),
                ),
            )?;
        session.register_table(name, frame.into_view())?;
    }
    session.sql("SELECT coalesce(b.projection,a.projection) AS projection, coalesce(b.path,a.path) AS path, CASE WHEN b.field_identity IS NULL THEN 'added' WHEN a.field_identity IS NULL THEN 'removed' ELSE 'changed' END AS kind, b.descriptor AS before, a.descriptor AS after FROM contract_before b FULL JOIN contract_after a ON b.projection=a.projection AND b.path=a.path WHERE b.field_identity IS DISTINCT FROM a.field_identity ORDER BY projection,path").await
}

/// The one-identity-column admission view of the complete native difference relation.
/// A difference is identified by its typed path and before/after values, not display text.
pub async fn violations(
    session: &SessionContext,
    before: DataFrame,
    after: DataFrame,
) -> Result<DataFrame> {
    changes(session, before, after).await?.select(vec![
        crate::native_key::Key::ContractChange
            .expression()
            .alias("witness_id"),
    ])
}
