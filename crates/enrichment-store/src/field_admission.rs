//! Reference relations are generated from declared fields, including optional variants and lists.
//! Segment access uses native field expressions; literal dots in field names are never path syntax.
use arrow::datatypes::{DataType, Field, Schema};
use datafusion::{
    common::{Column, Result, TableReference},
    dataframe::DataFrame,
    functions::core::expr_ext::FieldAccessor,
    logical_expr::{Expr, JoinType},
    prelude::{SessionContext, col, lit},
};
use enrichment_core::native_union::{Domain, ReferenceTarget, Rule, ScopeKey, ScopeNull};

struct ReferenceInput {
    path: Vec<String>,
    target: ReferenceTarget,
    rows: DataFrame,
    scope: Vec<ScopeKey>,
}
struct ReferenceInputs {
    identity: Option<Domain>,
    references: Vec<ReferenceInput>,
}

fn contains_identity(field: &Field, domain: Domain) -> Result<bool> {
    use arrow_schema::extension::ExtensionType;
    use enrichment_core::native_types::IdentityType;
    if field
        .metadata()
        .get("ARROW:extension:name")
        .map(String::as_str)
        == Some(IdentityType::NAME)
        && *IdentityType::deserialize_metadata(
            field
                .metadata()
                .get("ARROW:extension:metadata")
                .map(String::as_str),
        )?
        .meaning()
            == domain
    {
        return Ok(true);
    }
    match field.data_type() {
        DataType::Struct(fields) => {
            for child in fields {
                if contains_identity(child, domain)? {
                    return Ok(true);
                }
            }
            Ok(false)
        }
        DataType::List(item)
        | DataType::LargeList(item)
        | DataType::ListView(item)
        | DataType::LargeListView(item)
        | DataType::FixedSizeList(item, _)
        | DataType::Map(item, _) => contains_identity(item, domain),
        DataType::Union(fields, _) => {
            for (_, field) in fields.iter() {
                if contains_identity(field, domain)? {
                    return Ok(true);
                }
            }
            Ok(false)
        }
        DataType::RunEndEncoded(_, value) => contains_identity(value, domain),
        DataType::Dictionary(_, value) => {
            contains_identity(&Field::new("value", value.as_ref().clone(), true), domain)
        }
        _ => Ok(false),
    }
}

fn inputs(
    frame: DataFrame,
    schema: &Schema,
    key: &str,
    identity: Option<Domain>,
) -> Result<Vec<ReferenceInput>> {
    enrichment_core::native_schema::validate(schema)?;
    let mut output = ReferenceInputs {
        identity,
        references: Vec::new(),
    };
    for field in schema.fields() {
        walk(
            frame.clone(),
            field,
            Expr::Column(Column::from_name(field.name())),
            None,
            lit(true),
            key,
            vec![field.name().clone()],
            &mut output,
        )?;
    }
    Ok(output.references)
}

fn walk(
    frame: DataFrame,
    field: &Field,
    value: Expr,
    parent: Option<Expr>,
    present: Expr,
    key: &str,
    path: Vec<String>,
    output: &mut ReferenceInputs,
) -> Result<()> {
    if let Some(decoded) = enrichment_core::native_collections::decoded_field(field) {
        return walk(
            frame,
            &decoded,
            enrichment_core::native_collections::decoded_values().call(vec![value]),
            parent,
            present,
            key,
            path,
            output,
        );
    }
    let identity = output.identity;
    if let Some(domain) = identity
        && !contains_identity(field, domain)?
    {
        return Ok(());
    }
    let active = if matches!(field.data_type(), DataType::Union(_, _)) {
        present
    } else {
        present.and(value.clone().is_not_null())
    };
    if let Some(domain) = identity {
        use arrow_schema::extension::ExtensionType;
        use enrichment_core::native_types::IdentityType;
        if field
            .metadata()
            .get("ARROW:extension:name")
            .map(String::as_str)
            == Some(IdentityType::NAME)
            && *IdentityType::deserialize_metadata(
                field
                    .metadata()
                    .get("ARROW:extension:metadata")
                    .map(String::as_str),
            )?
            .meaning()
                == domain
        {
            output.references.push(ReferenceInput {
                path: path.clone(),
                target: ReferenceTarget::Evidence(domain),
                rows: frame.clone().filter(active.clone())?.select(vec![
                    Expr::Column(Column::from_name(key)).alias("owner"),
                    value.clone().alias("value"),
                ])?,
                scope: Vec::new(),
            });
        }
    } else if let Some(encoded) = field.metadata().get("enrichment.rule") {
        let rule: Rule = serde_json::from_str(encoded)
            .map_err(|error| datafusion::common::DataFusionError::Plan(error.to_string()))?;
        if let Some((target, scope)) = rule.reference() {
            let mut columns = vec![
                Expr::Column(Column::from_name(key)).alias("owner"),
                value.clone().alias("value"),
            ];
            for (index, binding) in scope.iter().enumerate() {
                columns.push(
                    access(parent.clone(), &binding.source)?.alias(format!("source_scope_{index}")),
                );
            }
            output.references.push(ReferenceInput {
                path: path.clone(),
                target,
                rows: frame.clone().filter(active.clone())?.select(columns)?,
                scope,
            });
        }
    }
    match field.data_type() {
        DataType::Struct(fields) => {
            for child in fields {
                let mut path = path.clone();
                path.push(child.name().clone());
                walk(
                    frame.clone(),
                    child,
                    value.clone().field(child.name()),
                    Some(value.clone()),
                    active.clone(),
                    key,
                    path,
                    output,
                )?;
            }
        }
        DataType::List(item)
        | DataType::LargeList(item)
        | DataType::FixedSizeList(item, _)
        | DataType::Map(item, _) => {
            let value = if matches!(field.data_type(), DataType::Map(_, _)) {
                enrichment_core::native_collections::map_entries().call(vec![value])
            } else {
                enrichment_core::native_collections::list_entries().call(vec![value])
            };
            let nested = frame
                .filter(active)?
                .select(vec![
                    Expr::Column(Column::from_name(key)).alias("owner"),
                    value.alias("member"),
                ])?
                .unnest_columns_with_options(
                    &["member"],
                    datafusion::common::UnnestOptions::new().with_preserve_nulls(false),
                )?;
            let nested = if matches!(field.data_type(), DataType::Map(_, _)) {
                nested
            } else {
                nested.with_column("member", col("member").field("value"))?
            };
            let mut path = path;
            path.push("[]".into());
            walk(
                nested,
                item,
                col("member"),
                None,
                lit(true),
                "owner",
                path,
                output,
            )?;
        }
        DataType::Union(children, _) => {
            let tag = datafusion::functions::core::expr_fn::union_tag(value.clone());
            for (type_id, child) in children.iter() {
                let mut path = path.clone();
                path.push(child.name().clone());
                walk(
                    frame.clone(),
                    child,
                    enrichment_core::native_collections::union_member(type_id)
                        .call(vec![value.clone()]),
                    None,
                    active.clone().and(tag.clone().eq(lit(child.name()))),
                    key,
                    path,
                    output,
                )?;
            }
        }
        _ => {}
    }
    Ok(())
}

fn access(parent: Option<Expr>, path: &[String]) -> Result<Expr> {
    let (first, rest) = path.split_first().ok_or_else(|| {
        datafusion::common::DataFusionError::Plan("empty reference scope path".into())
    })?;
    let root = parent.map_or_else(
        || Expr::Column(Column::from_name(first)),
        |parent| parent.field(first),
    );
    Ok(rest.iter().fold(root, |value, part| value.field(part)))
}

/// Schema-declared identity references, across optional structs and list branches.
/// The field's extension selects the domain; names and JSON payloads carry no policy.
pub(crate) fn identity_values(
    frame: DataFrame,
    schema: &Schema,
    key: &str,
    domain: Domain,
) -> Result<Option<DataFrame>> {
    let mut result: Option<DataFrame> = None;
    for reference in inputs(frame, schema, key, Some(domain))? {
        result = Some(match result {
            Some(prior) => prior.union(reference.rows)?,
            None => reference.rows,
        });
    }
    result.map(DataFrame::distinct).transpose()
}

/// The immutable namespace determines where relation references resolve. Evidence domain
/// references require an exact release; control rows do not borrow evidence authority from
/// their nested request/result values.
pub(crate) enum ReferenceNamespace<'a> {
    Evidence(&'a enrichment_core::identity::ReleaseId),
    Records,
}

pub(crate) async fn violations(
    session: &SessionContext,
    source: TableReference,
    schema: &Schema,
    key: &str,
    namespace: ReferenceNamespace<'_>,
) -> Result<Vec<(String, DataFrame)>> {
    let catalog = source
        .catalog()
        .ok_or_else(|| {
            datafusion::common::plan_datafusion_err!("reference requires a bound catalog")
        })?
        .to_owned();
    let schema_name = source
        .schema()
        .ok_or_else(|| {
            datafusion::common::plan_datafusion_err!("reference requires a bound schema")
        })?
        .to_owned();
    let frame = session.table(source).await?;
    let mut output = Vec::new();
    for reference in inputs(frame, schema, key, None)? {
        let (table, field) = match reference.target {
            ReferenceTarget::Relation { table, field } => (table, field),
            ReferenceTarget::Evidence(domain) => {
                let ReferenceNamespace::Evidence(release_id) = namespace else {
                    continue;
                };
                if let Some((table, field)) = domain.evidence_target() {
                    (table.into(), vec![field.into()])
                } else if domain == Domain::Release && reference.scope.is_empty() {
                    output.push((
                        format!("declared reference {:?}", reference.path),
                        reference
                            .rows
                            .filter(col("value").not_eq(lit(release_id)))?
                            .select(vec![col("owner")])?
                            .limit(0, Some(1))?,
                    ));
                    continue;
                } else {
                    return datafusion::common::plan_err!("unbound evidence reference domain");
                }
            }
        };
        let mut columns = vec![access(None, &field)?.alias("target_value")];
        let mut filter = None;
        for (index, scope) in reference.scope.iter().enumerate() {
            columns.push(access(None, &scope.target)?.alias(format!("target_scope_{index}")));
            let source = col(format!("source_scope_{index}"));
            let equal = source.clone().eq(col(format!("target_scope_{index}")));
            let predicate = match scope.null {
                ScopeNull::Exact => equal,
                ScopeNull::Unspecified => source.is_null().or(equal),
            };
            filter = Some(filter.map_or_else(
                || predicate.clone(),
                |prior: Expr| prior.and(predicate.clone()),
            ));
        }
        let target = session
            .table(TableReference::full(
                catalog.clone(),
                schema_name.clone(),
                table,
            ))
            .await?
            .select(columns)?;
        let rows = reference.rows.join(
            target,
            JoinType::LeftAnti,
            &["value"],
            &["target_value"],
            filter,
        )?;
        output.push((
            format!("declared reference {:?}", reference.path),
            rows.select(vec![col("owner")])?.limit(0, Some(1))?,
        ));
    }
    Ok(output)
}

#[cfg(test)]
#[path = "field_admission/tests.rs"]
mod reference_tests;

#[cfg(test)]
mod tests {
    use super::*;
    use enrichment_core::{identity::SnapshotId, native_union::NativeStruct};
    enrichment_core::native_struct! { struct Child { selected: Option<SnapshotId> => Rule::Text } }
    enrichment_core::native_struct! { struct Input {
        owner: String => Rule::Text,
        child: Option<Child> => Rule::Text,
        members: Vec<SnapshotId> => Rule::Sequence,
        decoy: String => Rule::Text,
    } }
    enrichment_core::native_struct! { struct Reference { owner: String => Rule::Text, value: SnapshotId => Rule::Text } }

    #[tokio::test]
    async fn plan19_schema_identity_closure_handles_optional_lists_and_decoys() -> Result<()> {
        let directory = tempfile::tempdir()?;
        let runtime = crate::runtime::QueryRuntime::new(directory.path(), Default::default())?;
        let a = SnapshotId::try_from(format!("snap_{}", "a".repeat(64)))
            .map_err(|e| datafusion::common::DataFusionError::External(Box::new(e)))?;
        let b = SnapshotId::try_from(format!("snap_{}", "b".repeat(64)))
            .map_err(|e| datafusion::common::DataFusionError::External(Box::new(e)))?;
        let rows = [
            Input {
                owner: "one".into(),
                child: Some(Child {
                    selected: Some(a.clone()),
                }),
                members: vec![a.clone(), b.clone()],
                decoy: b.to_string(),
            },
            Input {
                owner: "two".into(),
                child: None,
                members: vec![],
                decoy: a.to_string(),
            },
        ];
        let batch = Input::batch(&rows)?;
        let schema = batch.schema();
        let plan = identity_values(
            crate::native_catalog::batch(&runtime.session(), "field_admission", batch)?,
            schema.as_ref(),
            "owner",
            Domain::Snapshot,
        )?
        .expect("declared snapshot paths");
        let mut result = runtime.records::<Reference>(plan, 8).await?;
        result.sort_by(|a, b| a.value.cmp(&b.value));
        assert_eq!(
            result,
            vec![
                Reference {
                    owner: "one".into(),
                    value: a
                },
                Reference {
                    owner: "one".into(),
                    value: b
                }
            ]
        );
        runtime.close_diagnostics().await?;
        Ok(())
    }
}
