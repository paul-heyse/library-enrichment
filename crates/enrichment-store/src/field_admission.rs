//! Reference relations are generated from declared fields, including optional variants and lists.
//! Segment access uses native field expressions; literal dots in field names are never path syntax.
use arrow::datatypes::{DataType, Field, Schema};
use datafusion::{
    common::Result,
    dataframe::DataFrame,
    functions::core::expr_ext::FieldAccessor,
    logical_expr::{Expr, JoinType},
    prelude::{SessionContext, col, lit},
};
use enrichment_core::native_union::{Domain, Rule, ScopeKey, ScopeNull};

struct ReferenceInput {
    path: Vec<String>,
    domain: Domain,
    rows: DataFrame,
    scope: Vec<ScopeKey>,
}

fn inputs(frame: DataFrame, schema: &Schema, key: &str) -> Result<Vec<ReferenceInput>> {
    let mut output = Vec::new();
    for field in schema.fields() {
        walk(
            frame.clone(),
            field,
            col(field.name()),
            None,
            lit(true),
            key,
            vec![field.name().clone()],
            &mut output,
        )?;
    }
    Ok(output)
}

fn walk(
    frame: DataFrame,
    field: &Field,
    value: Expr,
    parent: Option<Expr>,
    present: Expr,
    key: &str,
    path: Vec<String>,
    output: &mut Vec<ReferenceInput>,
) -> Result<()> {
    let active = present.and(value.clone().is_not_null());
    if let Some(encoded) = field.metadata().get("enrichment.rule") {
        let rule: Rule = serde_json::from_str(encoded)
            .map_err(|error| datafusion::common::DataFusionError::Plan(error.to_string()))?;
        let reference = match rule {
            Rule::Reference(domain) => Some((domain, Vec::new())),
            Rule::ScopedReference { domain, scope } => Some((domain, scope)),
            _ => None,
        };
        if let Some((domain, scope)) = reference {
            let mut columns = vec![col(key).alias("owner"), value.clone().alias("value")];
            for (index, binding) in scope.iter().enumerate() {
                columns.push(
                    access(parent.clone(), &binding.source)?.alias(format!("source_scope_{index}")),
                );
            }
            output.push(ReferenceInput {
                path: path.clone(),
                domain,
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
        DataType::List(item) | DataType::LargeList(item) => {
            let nested = frame
                .filter(active)?
                .select(vec![col(key).alias("owner"), value.alias("member")])?
                .unnest_columns_with_options(
                    &["member"],
                    datafusion::common::UnnestOptions::new().with_preserve_nulls(false),
                )?;
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
        _ => {}
    }
    Ok(())
}

fn access(parent: Option<Expr>, path: &[String]) -> Result<Expr> {
    let (first, rest) = path.split_first().ok_or_else(|| {
        datafusion::common::DataFusionError::Plan("empty reference scope path".into())
    })?;
    let root = parent.map_or_else(|| col(first), |parent| parent.field(first));
    Ok(rest.iter().fold(root, |value, part| value.field(part)))
}

pub(crate) async fn violations(
    session: &SessionContext,
    frame: DataFrame,
    schema: &Schema,
    key: &str,
    release_id: &str,
) -> Result<Vec<(String, DataFrame)>> {
    let mut output = Vec::new();
    for reference in inputs(frame, schema, key)? {
        let rows = if let Some((table, target_key)) = reference.domain.evidence_target() {
            let mut columns = vec![col(target_key).alias("target_value")];
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
                .table(datafusion::common::TableReference::full(
                    "candidate",
                    "evidence",
                    table,
                ))
                .await?
                .select(columns)?;
            reference.rows.join(
                target,
                JoinType::LeftAnti,
                &["value"],
                &["target_value"],
                filter,
            )?
        } else {
            reference
                .rows
                .filter(col("value").not_eq(lit(release_id)))?
        };
        output.push((
            format!("declared reference {:?}", reference.path),
            rows.select(vec![col("owner")])?.limit(0, Some(1))?,
        ));
    }
    Ok(output)
}
