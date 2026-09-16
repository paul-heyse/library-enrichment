//! Native expressions for authoritative Arrow records, shared by all producer plans.
use arrow::datatypes::DataType;
use datafusion::{
    common::ScalarValue,
    error::{DataFusionError, Result},
    functions::core::expr_fn::named_struct,
    logical_expr::{Expr, cast},
    prelude::lit,
};
fn invalid(message: &str) -> DataFusionError {
    DataFusionError::Plan(message.into())
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
/// variant members. Arrow's cast and the Delta admission rules own values and constraints.
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
        args.push(cast(
            match values.iter().find(|(name, _)| *name == field.name()) {
                Some((_, expr)) => expr.clone(),
                None => null(field.data_type())?,
            },
            field.data_type().clone(),
        ));
    }
    Ok(cast(named_struct(args), kind.clone()))
}
