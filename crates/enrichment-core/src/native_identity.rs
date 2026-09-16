//! Shared bounded Arrow value encoding for native semantic identity expressions.
//! Native distinct/order/aggregation and cryptographic functions own identity composition.
use arrow::{
    array::*,
    datatypes::{DataType, Fields},
};
use datafusion::{
    error::{DataFusionError, Result},
    logical_expr::{
        ColumnarValue, ScalarFunctionArgs, ScalarUDF, ScalarUDFImpl, Signature, Volatility,
    },
};
use std::sync::Arc;

/// Build the one value encoder used by native identity plans. Its declaration supplies types,
/// field names and domain separation; it does not sort sets or define business identity rules.
pub fn canonical_bytes(domain: impl Into<String>, fields: Fields) -> ScalarUDF {
    let signature = Signature::exact(
        fields
            .iter()
            .map(|field| field.data_type().clone())
            .collect(),
        Volatility::Immutable,
    );
    ScalarUDF::from(CanonicalBytes {
        fields,
        domain: domain.into(),
        signature,
    })
}

/// Expose Arrow's byte-length kernel for canonical Binary values without text conversion.
pub fn byte_length(value: datafusion::logical_expr::Expr) -> datafusion::logical_expr::Expr {
    datafusion::logical_expr::create_udf(
        "arrow_binary_length_v1",
        vec![DataType::Binary],
        DataType::Int32,
        Volatility::Immutable,
        Arc::new(|values| {
            let [value] = values else {
                return Err(invalid("binary length argument count"));
            };
            match value {
                ColumnarValue::Array(array) => Ok(ColumnarValue::Array(
                    arrow::compute::kernels::length::length(array.as_ref())?,
                )),
                ColumnarValue::Scalar(value) => {
                    let array = value.to_array()?;
                    let lengths = arrow::compute::kernels::length::length(array.as_ref())?;
                    Ok(ColumnarValue::Scalar(
                        datafusion::common::ScalarValue::try_from_array(lengths.as_ref(), 0)?,
                    ))
                }
            }
        }),
    )
    .call(vec![value])
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct CanonicalBytes {
    fields: Fields,
    domain: String,
    signature: Signature,
}
impl ScalarUDFImpl for CanonicalBytes {
    fn name(&self) -> &str {
        "canonical_arrow_bytes_v1"
    }
    fn signature(&self) -> &Signature {
        &self.signature
    }
    fn return_type(&self, _: &[DataType]) -> Result<DataType> {
        Ok(DataType::Binary)
    }
    fn invoke_with_args(&self, args: ScalarFunctionArgs) -> Result<ColumnarValue> {
        if args.args.len() != self.fields.len() {
            return Err(invalid(
                "canonical argument count differs from its declaration",
            ));
        }
        let arrays = args
            .args
            .into_iter()
            .map(|v| v.into_array(args.number_rows))
            .collect::<Result<Vec<_>>>()?;
        if arrays.iter().any(|array| array.len() != args.number_rows) {
            return Err(invalid("canonical argument row count differs"));
        }
        let mut output = BinaryBuilder::new();
        let mut bytes = Vec::new();
        for row in 0..args.number_rows {
            bytes.clear();
            framed(&mut bytes, self.domain.as_bytes())?;
            for (field, array) in self.fields.iter().zip(&arrays) {
                framed(&mut bytes, field.name().as_bytes())?;
                type_bytes(field.data_type(), &mut bytes)?;
                value(array.as_ref(), row, &mut bytes)?;
            }
            output.append_value(&bytes);
        }
        Ok(ColumnarValue::Array(Arc::new(output.finish())))
    }
}
fn framed(out: &mut Vec<u8>, bytes: &[u8]) -> Result<()> {
    if out
        .len()
        .checked_add(8)
        .and_then(|size| size.checked_add(bytes.len()))
        .is_none_or(|size| size > 16 * 1024 * 1024)
    {
        return Err(DataFusionError::ResourcesExhausted(
            "canonical row exceeds 16 MiB".into(),
        ));
    }
    out.extend_from_slice(&(bytes.len() as u64).to_le_bytes());
    out.extend_from_slice(bytes);
    Ok(())
}
fn type_bytes(kind: &DataType, out: &mut Vec<u8>) -> Result<()> {
    let tag = match kind {
        DataType::Utf8 | DataType::LargeUtf8 | DataType::Utf8View => 1,
        DataType::Boolean => 2,
        DataType::Int8 => 3,
        DataType::Int16 => 4,
        DataType::Int32 => 5,
        DataType::Int64 => 6,
        DataType::UInt8 => 7,
        DataType::UInt16 => 8,
        DataType::UInt32 => 9,
        DataType::UInt64 => 10,
        DataType::Binary => 11,
        DataType::Struct(fields) => {
            out.push(12);
            out.extend_from_slice(&(fields.len() as u64).to_le_bytes());
            for field in fields {
                framed(out, field.name().as_bytes())?;
                type_bytes(field.data_type(), out)?;
            }
            return Ok(());
        }
        DataType::List(field) | DataType::LargeList(field) => {
            out.push(13);
            type_bytes(field.data_type(), out)?;
            return Ok(());
        }
        other => return Err(invalid(&format!("undeclared canonical type {other}"))),
    };
    out.push(tag);
    Ok(())
}

fn value(array: &dyn Array, row: usize, out: &mut Vec<u8>) -> Result<()> {
    if array.is_null(row) {
        out.push(0);
        return Ok(());
    }
    out.push(1);
    macro_rules! number {
        ($array:ty,$tag:expr) => {{
            out.push($tag);
            out.extend_from_slice(
                &array
                    .as_any()
                    .downcast_ref::<$array>()
                    .ok_or_else(|| invalid("canonical numeric representation"))?
                    .value(row)
                    .to_le_bytes(),
            );
        }};
    }
    match array.data_type() {
        DataType::Utf8 | DataType::LargeUtf8 | DataType::Utf8View => {
            out.push(1);
            framed(out, text_value(array, row)?.as_bytes())?;
        }
        DataType::Boolean => {
            out.push(2);
            out.push(u8::from(
                array
                    .as_any()
                    .downcast_ref::<BooleanArray>()
                    .ok_or_else(|| invalid("canonical bool"))?
                    .value(row),
            ));
        }
        DataType::Int8 => number!(Int8Array, 3),
        DataType::Int16 => number!(Int16Array, 4),
        DataType::Int32 => number!(Int32Array, 5),
        DataType::Int64 => number!(Int64Array, 6),
        DataType::UInt8 => number!(UInt8Array, 7),
        DataType::UInt16 => number!(UInt16Array, 8),
        DataType::UInt32 => number!(UInt32Array, 9),
        DataType::UInt64 => number!(UInt64Array, 10),
        DataType::Binary => {
            out.push(11);
            framed(
                out,
                array
                    .as_any()
                    .downcast_ref::<BinaryArray>()
                    .ok_or_else(|| invalid("canonical binary"))?
                    .value(row),
            )?;
        }
        DataType::Struct(fields) => {
            out.push(12);
            out.extend_from_slice(&(fields.len() as u64).to_le_bytes());
            let array = array
                .as_any()
                .downcast_ref::<StructArray>()
                .ok_or_else(|| invalid("canonical struct"))?;
            for (field, child) in fields.iter().zip(array.columns()) {
                framed(out, field.name().as_bytes())?;
                value(child.as_ref(), row, out)?;
            }
        }
        DataType::List(_) | DataType::LargeList(_) => {
            out.push(13);
            let values = if let Some(array) = array.as_any().downcast_ref::<ListArray>() {
                array.value(row)
            } else {
                array
                    .as_any()
                    .downcast_ref::<LargeListArray>()
                    .ok_or_else(|| invalid("canonical list"))?
                    .value(row)
            };
            out.extend_from_slice(&(values.len() as u64).to_le_bytes());
            for row in 0..values.len() {
                value(values.as_ref(), row, out)?;
            }
        }
        other => return Err(invalid(&format!("undeclared canonical type {other}"))),
    }
    if out.len() > 16 * 1024 * 1024 {
        return Err(DataFusionError::ResourcesExhausted(
            "canonical row exceeds 16 MiB".into(),
        ));
    }
    Ok(())
}
fn text_value(array: &dyn Array, row: usize) -> Result<&str> {
    use arrow::array::AsArray;
    match array.data_type() {
        DataType::Utf8 => array.as_string_opt::<i32>().map(|values| values.value(row)),
        DataType::LargeUtf8 => array.as_string_opt::<i64>().map(|values| values.value(row)),
        DataType::Utf8View => array.as_string_view_opt().map(|values| values.value(row)),
        _ => None,
    }
    .ok_or_else(|| invalid("canonical string representation"))
}

fn invalid(message: &str) -> DataFusionError {
    DataFusionError::Execution(message.into())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn canonical_values_distinguish_null_empty_and_component_boundaries() {
        let array = StringArray::from(vec![None, Some(""), Some("a\0b"), Some("ab")]);
        let values = (0..4)
            .map(|row| {
                let mut bytes = Vec::new();
                value(&array, row, &mut bytes).unwrap();
                bytes
            })
            .collect::<std::collections::BTreeSet<_>>();
        assert_eq!(values.len(), 4);
        let a = StringArray::from(vec!["é"]);
        let b = StringViewArray::from(vec!["é"]);
        let mut first = Vec::new();
        let mut second = Vec::new();
        value(&a, 0, &mut first).unwrap();
        value(&b, 0, &mut second).unwrap();
        assert_eq!(first, second);
    }
}
