//! Native expressions for authoritative Arrow records, shared by all producer plans.
use arrow::datatypes::DataType;
use datafusion::{
    common::{ScalarValue, metadata::FieldMetadata},
    error::{DataFusionError, Result},
    logical_expr::{Expr, cast},
    prelude::lit,
};
fn invalid(message: &str) -> DataFusionError {
    DataFusionError::Plan(message.into())
}
/// A bounded ingress literal retains the complete field declaration, including semantics.
pub fn literal<T: crate::native_union::Cell>(value: &T) -> Result<Expr> {
    let parameter = parameter(value)?;
    Ok(Expr::Literal(parameter.value, parameter.metadata))
}
/// SQL parameters and expression literals share the complete declared field, including
/// collection children. Empty collections must not acquire an unrelated SQL element type.
pub fn parameter<T: crate::native_union::Cell>(
    value: &T,
) -> Result<datafusion::common::metadata::ScalarAndMetadata> {
    let field = crate::native_union::field::<T>("value", crate::native_union::Rule::Text);
    let array = T::encode(&[Some(value)])?;
    Ok(datafusion::common::metadata::ScalarAndMetadata::new(
        ScalarValue::try_from_array(array.as_ref(), 0)?,
        Some(FieldMetadata::from(&field)),
    ))
}
/// Native CASE preserves nested Fields at the pinned release, while scalar CASE and
/// coalesce rebuild their top-level Field. Carry the scalar through one declared Struct
/// child and select it afterwards; branch admission still rejects incompatible domains.
pub fn coalesce(values: Vec<Expr>) -> Result<Expr> {
    if values.is_empty() {
        return Err(invalid("coalesce requires a value"));
    }
    Ok(crate::native_selection::coalesce().call(values))
}

pub(crate) fn coalesce_case(values: Vec<Expr>) -> Result<Expr> {
    use datafusion::functions::core::expr_ext::FieldAccessor;
    let mut values = values.into_iter().rev();
    let wrap = |value| crate::native_record::named_struct().call(vec![lit("value"), value]);
    let mut selected = wrap(
        values
            .next()
            .ok_or_else(|| invalid("coalesce requires a value"))?,
    );
    for value in values {
        selected = datafusion::logical_expr::when(value.clone().is_not_null(), wrap(value))
            .otherwise(selected)?;
    }
    Ok(selected.field("value"))
}

pub fn null(kind: &DataType) -> Result<Expr> {
    Ok(lit(ScalarValue::try_from(kind)?))
}
pub fn child(kind: &DataType, name: &str) -> Result<DataType> {
    let DataType::Struct(fields) = kind else {
        return Err(invalid("native record is not a struct"));
    };
    fields
        .find(name)
        .map(|(_, f)| f.data_type().clone())
        .ok_or_else(|| invalid("native record field absent"))
}
/// Build a native named_struct in authoritative field order, with typed nulls for inactive
/// variant members. Preserve declared semantic children through native constructors; physical
/// representation casts belong to the explicit ArrowContract boundary, not a domain-erasing CAST.
pub fn record(kind: &DataType, values: &[(&str, Expr)]) -> Result<Expr> {
    let DataType::Struct(fields) = kind else {
        return Err(invalid("native record is not a struct"));
    };
    for (index, (name, _)) in values.iter().enumerate() {
        if fields.find(name).is_none() || values[..index].iter().any(|(other, _)| other == name) {
            return Err(DataFusionError::Plan(format!(
                "native record has an unknown or repeated field: {name}"
            )));
        }
    }
    let mut args = Vec::with_capacity(fields.len() * 2);
    for field in fields {
        args.push(lit(field.name()));
        let metadata = Some(FieldMetadata::from(field.as_ref()));
        let value = match values.iter().find(|(name, _)| *name == field.name()) {
            Some((_, expr)) => {
                let expr = if crate::native_analysis::has_semantics(field)
                    || matches!(
                        field.data_type(),
                        DataType::Struct(_)
                            | DataType::List(_)
                            | DataType::LargeList(_)
                            | DataType::FixedSizeList(_, _)
                            | DataType::Map(_, _)
                    ) {
                    expr.clone()
                } else {
                    cast(expr.clone(), field.data_type().clone())
                };
                expr.alias_with_metadata(field.name(), metadata)
            }
            None => Expr::Literal(ScalarValue::try_from(field.data_type())?, metadata),
        };
        args.push(value);
    }
    Ok(crate::native_record::record(fields.clone(), args))
}

/// Construct one declared alternative. Inactive payloads are NULL structs, never shared cells.
pub fn variant(kind: &DataType, tag: &str, values: &[(&str, Expr)]) -> Result<Expr> {
    let payload = record(&child(kind, tag)?, values)?;
    record(kind, &[(discriminator(kind)?, lit(tag)), (tag, payload)])
}

fn discriminator(kind: &DataType) -> Result<&str> {
    let DataType::Struct(fields) = kind else {
        return Err(invalid("native union is not a struct"));
    };
    let mut names = fields
        .iter()
        .filter(|field| field.metadata().contains_key("enrichment.union.tags"));
    let name = names
        .next()
        .ok_or_else(|| invalid("native union discriminator absent"))?;
    if names.next().is_some() {
        return Err(invalid("native union has multiple discriminators"));
    }
    Ok(name.name())
}

/// A producer can select a declared variant through a native CASE while retaining each
/// alternative's independent fields. No row-wise dispatch or semantic payload decoding occurs.
pub fn variants(
    kind: &DataType,
    tag: Expr,
    payloads: &[(&str, Vec<(&str, Expr)>)],
) -> Result<Expr> {
    let mut values = vec![(discriminator(kind)?, tag.clone())];
    for (name, fields) in payloads {
        let payload_type = child(kind, name)?;
        let payload = datafusion::logical_expr::when(
            tag.clone().eq(lit(*name)),
            record(&payload_type, fields)?,
        )
        .otherwise(null(&payload_type)?)?;
        values.push((*name, payload));
    }
    record(kind, &values)
}

/// Derive a record by preserving every declared field and replacing named native expressions.
/// Optional parent absence survives the projection; field additions propagate automatically.
/// # Errors
/// Unknown overrides or incompatible native field contracts refuse construction.
pub fn derive_record(value: Expr, kind: &DataType, replacements: &[(&str, Expr)]) -> Result<Expr> {
    use datafusion::functions::core::expr_ext::FieldAccessor;
    let DataType::Struct(fields) = kind else {
        return datafusion::common::plan_err!("derived record requires Struct");
    };
    if replacements
        .iter()
        .any(|(name, _)| fields.find(name).is_none())
    {
        return datafusion::common::plan_err!("derived record overrides unknown field");
    }
    let expressions = fields
        .iter()
        .map(|field| {
            (
                field.name().as_str(),
                replacements
                    .iter()
                    .find(|(name, _)| *name == field.name())
                    .map_or_else(
                        || value.clone().field(field.name()),
                        |(_, expr)| expr.clone(),
                    ),
            )
        })
        .collect::<Vec<_>>();
    datafusion::logical_expr::when(value.is_null(), null(kind)?)
        .otherwise(record(kind, &expressions)?)
}
