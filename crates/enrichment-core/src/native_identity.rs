//! Shared bounded Arrow value encoding for native semantic identity expressions.
//! Native distinct/order/aggregation and cryptographic functions own identity composition.
use arrow::{
    array::*,
    datatypes::{DataType, Field, FieldRef, Fields, Schema, TimeUnit},
};
use datafusion::{
    error::{DataFusionError, Result},
    logical_expr::{
        ColumnarValue, ReturnFieldArgs, ScalarFunctionArgs, ScalarUDF, ScalarUDFImpl, Signature,
        Volatility,
    },
};
use std::sync::Arc;

/// Native contract identity includes semantic meaning and the selected physical layout.
/// Declaration traversal is mechanical; DataFusion's own hash/encoding functions own the digest.
/// # Errors
/// Unbounded or unsupported declarations refuse before registration or mutation.
pub fn schema_identity(semantic: &Schema, storage: &Schema) -> Result<String> {
    crate::native_contract::Manifest::new(semantic, storage)?.identity()
}

/// Build the one value encoder used by native identity plans. Its declaration supplies types,
/// field names and domain separation; it does not sort sets or define business identity rules.
pub fn canonical_bytes(domain: impl Into<String>, fields: Fields) -> ScalarUDF {
    // Retain source field metadata/nullability and supported offset/view representations.
    // Exact datatype coercion would insert struct casts before full-field admission.
    let signature = Signature::user_defined(Volatility::Immutable);
    ScalarUDF::from(CanonicalBytes {
        fields,
        domain: domain.into(),
        signature,
    })
}

/// Mechanical one-row boundary to the same native identity kernel used by query plans.
pub fn record_bytes(domain: &str, value: ArrayRef) -> Result<Vec<u8>> {
    if value.len() != 1 {
        return Err(invalid("canonical record requires one row"));
    }
    let fields = vec![Arc::new(Field::new(
        "value",
        value.data_type().clone(),
        false,
    ))];
    let function = canonical_bytes(domain, fields.clone().into());
    let output = function.return_field_from_args(ReturnFieldArgs {
        arg_fields: &fields,
        scalar_arguments: &[None],
    })?;
    let array = function
        .invoke_with_args(ScalarFunctionArgs {
            args: vec![ColumnarValue::Array(value)],
            arg_fields: fields,
            number_rows: 1,
            return_field: output,
            config_options: Arc::new(datafusion::common::config::ConfigOptions::default()),
        })?
        .into_array(1)?;
    let bytes = array
        .as_any()
        .downcast_ref::<BinaryArray>()
        .ok_or_else(|| invalid("canonical record output type"))?;
    Ok(bytes.value(0).to_vec())
}

/// Expose Arrow's byte-length kernel for canonical Binary values without text conversion.
pub fn byte_length(value: datafusion::logical_expr::Expr) -> datafusion::logical_expr::Expr {
    ScalarUDF::from(BinaryLength {
        signature: Signature::exact(vec![DataType::Binary], Volatility::Immutable),
    })
    .call(vec![value])
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct BinaryLength {
    signature: Signature,
}
impl ScalarUDFImpl for BinaryLength {
    fn name(&self) -> &str {
        "native_binary_length_v2"
    }
    fn signature(&self) -> &Signature {
        &self.signature
    }
    fn return_type(&self, _: &[DataType]) -> Result<DataType> {
        datafusion::common::plan_err!("binary length requires full fields")
    }
    fn return_field_from_args(&self, args: ReturnFieldArgs) -> Result<FieldRef> {
        let nullable = args
            .arg_fields
            .first()
            .is_none_or(|field| field.is_nullable());
        crate::native_schema::function_output(
            &args,
            &[Arc::new(Field::new("bytes", DataType::Binary, true))],
            self.name(),
            DataType::Int32,
            nullable,
        )
    }
    fn invoke_with_args(&self, args: ScalarFunctionArgs) -> Result<ColumnarValue> {
        crate::native_schema::function_call(self, &args)?;
        let value = args.args[0].to_array(args.number_rows)?;
        Ok(ColumnarValue::Array(
            arrow::compute::kernels::length::length(value.as_ref())?,
        ))
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct CanonicalBytes {
    fields: Fields,
    domain: String,
    signature: Signature,
}

/// The analyzer recognizes the concrete checked encoder, never a caller-supplied SQL name.
pub(crate) fn is_value_encoder(function: &ScalarUDF) -> bool {
    function.inner().downcast_ref::<CanonicalBytes>().is_some()
}
impl ScalarUDFImpl for CanonicalBytes {
    fn name(&self) -> &str {
        "canonical_arrow_bytes_v2"
    }
    fn signature(&self) -> &Signature {
        &self.signature
    }
    fn return_type(&self, _: &[DataType]) -> Result<DataType> {
        Ok(DataType::Binary)
    }
    fn coerce_types(&self, types: &[DataType]) -> Result<Vec<DataType>> {
        if types.len() != self.fields.len()
            || types.iter().zip(&self.fields).any(|(actual, expected)| {
                !crate::native_schema::same_value_type(actual, expected.data_type())
            })
        {
            return Err(invalid(
                "canonical argument value types differ from their declaration",
            ));
        }
        Ok(types.to_vec())
    }
    fn return_field_from_args(&self, args: ReturnFieldArgs) -> Result<FieldRef> {
        crate::native_schema::function_output(
            &args,
            &self.fields,
            self.name(),
            DataType::Binary,
            false,
        )
    }
    fn invoke_with_args(&self, args: ScalarFunctionArgs) -> Result<ColumnarValue> {
        crate::native_schema::function_call(self, &args)?;
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
                field_bytes(field, &mut bytes)?;
                value(array.as_ref(), row, field, &mut bytes)?;
            }
            output.append_value(&bytes);
        }
        Ok(ColumnarValue::Array(Arc::new(output.finish())))
    }
}

fn field_bytes(field: &Field, out: &mut Vec<u8>) -> Result<()> {
    framed(out, field.name().as_bytes())?;
    type_bytes(field.data_type(), out)?;
    let metadata: std::collections::BTreeMap<_, _> = field
        .metadata()
        .iter()
        .filter(|(key, _)| {
            key.starts_with("ARROW:extension:")
                || (key.starts_with("enrichment.") && key.as_str() != "enrichment.role")
        })
        .collect();
    out.extend_from_slice(&(metadata.len() as u64).to_le_bytes());
    for (key, value) in metadata {
        framed(out, key.as_bytes())?;
        framed(out, value.as_bytes())?;
    }
    Ok(())
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
        DataType::Binary | DataType::LargeBinary | DataType::BinaryView => 11,
        DataType::Struct(fields) => {
            out.push(12);
            out.extend_from_slice(&(fields.len() as u64).to_le_bytes());
            for field in fields {
                field_bytes(field, out)?;
            }
            return Ok(());
        }
        DataType::List(field) | DataType::LargeList(field) => {
            out.push(13);
            // Container item names are a physical convention, not record identities.
            field_bytes(&field.as_ref().clone().with_name("item"), out)?;
            return Ok(());
        }
        DataType::FixedSizeBinary(width) if *width > 0 => {
            out.push(14);
            out.extend_from_slice(&width.to_le_bytes());
            return Ok(());
        }
        DataType::Timestamp(unit, timezone) => {
            out.push(15);
            out.push(match unit {
                TimeUnit::Second => 0,
                TimeUnit::Millisecond => 1,
                TimeUnit::Microsecond => 2,
                TimeUnit::Nanosecond => 3,
            });
            out.push(u8::from(timezone.is_some()));
            if let Some(timezone) = timezone {
                framed(out, timezone.as_bytes())?;
            }
            return Ok(());
        }
        DataType::Decimal128(precision, scale) => {
            out.extend_from_slice(&[16, *precision, scale.to_le_bytes()[0]]);
            return Ok(());
        }
        DataType::Decimal256(precision, scale) => {
            out.extend_from_slice(&[17, *precision, scale.to_le_bytes()[0]]);
            return Ok(());
        }
        DataType::Map(entries, sorted) => {
            out.extend_from_slice(&[18, u8::from(*sorted)]);
            field_bytes(entries, out)?;
            return Ok(());
        }
        DataType::Null => 19,
        DataType::Float32 => 20,
        DataType::Float64 => 21,
        DataType::Date32 => 22,
        DataType::Dictionary(index, value) => {
            out.push(23);
            type_bytes(index, out)?;
            type_bytes(value, out)?;
            return Ok(());
        }
        DataType::FixedSizeList(item, length) if *length >= 0 => {
            out.push(24);
            out.extend_from_slice(&length.to_le_bytes());
            field_bytes(item, out)?;
            return Ok(());
        }
        other => return Err(invalid(&format!("undeclared canonical type {other}"))),
    };
    out.push(tag);
    Ok(())
}

fn value(array: &dyn Array, row: usize, declared: &Field, out: &mut Vec<u8>) -> Result<()> {
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
        DataType::LargeBinary => {
            out.push(11);
            framed(
                out,
                array
                    .as_any()
                    .downcast_ref::<LargeBinaryArray>()
                    .ok_or_else(|| invalid("canonical large binary"))?
                    .value(row),
            )?;
        }
        DataType::BinaryView => {
            out.push(11);
            framed(
                out,
                array
                    .as_any()
                    .downcast_ref::<BinaryViewArray>()
                    .ok_or_else(|| invalid("canonical binary view"))?
                    .value(row),
            )?;
        }
        DataType::FixedSizeBinary(_) => {
            out.push(14);
            framed(
                out,
                array
                    .as_any()
                    .downcast_ref::<FixedSizeBinaryArray>()
                    .ok_or_else(|| invalid("canonical fixed binary"))?
                    .value(row),
            )?;
        }
        DataType::Timestamp(unit, _) => match unit {
            TimeUnit::Second => number!(TimestampSecondArray, 15),
            TimeUnit::Millisecond => number!(TimestampMillisecondArray, 15),
            TimeUnit::Microsecond => number!(TimestampMicrosecondArray, 15),
            TimeUnit::Nanosecond => number!(TimestampNanosecondArray, 15),
        },
        DataType::Decimal128(_, _) => number!(Decimal128Array, 16),
        DataType::Decimal256(_, _) => number!(Decimal256Array, 17),
        DataType::Struct(_) => {
            let DataType::Struct(fields) = declared.data_type() else {
                return Err(invalid("canonical struct declaration"));
            };
            out.push(12);
            out.extend_from_slice(&(fields.len() as u64).to_le_bytes());
            let array = array
                .as_any()
                .downcast_ref::<StructArray>()
                .ok_or_else(|| invalid("canonical struct"))?;
            for field in fields {
                let child = array
                    .column_by_name(field.name())
                    .ok_or_else(|| invalid("canonical struct member absent"))?;
                framed(out, field.name().as_bytes())?;
                value(child.as_ref(), row, field, out)?;
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
            let (DataType::List(item) | DataType::LargeList(item)) = declared.data_type() else {
                return Err(invalid("canonical list declaration"));
            };
            let rule = declared
                .metadata()
                .get("enrichment.rule")
                .map(|encoded| serde_json::from_str::<crate::native_union::Rule>(encoded))
                .transpose()
                .map_err(|error| invalid(&error.to_string()))?;
            let values = if matches!(rule, Some(crate::native_union::Rule::Set)) {
                set_values(values)?
            } else {
                values
            };
            out.extend_from_slice(&(values.len() as u64).to_le_bytes());
            for row in 0..values.len() {
                value(values.as_ref(), row, item, out)?;
            }
        }
        DataType::Map(_, _) => {
            let DataType::Map(entry_field, _) = declared.data_type() else {
                return Err(invalid("canonical map declaration"));
            };
            let map = array
                .as_any()
                .downcast_ref::<MapArray>()
                .ok_or_else(|| invalid("canonical map array"))?;
            let entries = map.value(row);
            let indices = arrow::compute::sort_to_indices(entries.column(0).as_ref(), None, None)?;
            let sorted = arrow::compute::take(&entries, &indices, None)?;
            out.push(18);
            out.extend_from_slice(&(sorted.len() as u64).to_le_bytes());
            for row in 0..sorted.len() {
                value(sorted.as_ref(), row, entry_field, out)?;
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

/// The pinned DataFusion list kernels support repeated structs as well as primitives.
/// Arrow's primitive sort_to_indices alone does not support all declared set item types.
fn set_values(values: ArrayRef) -> Result<ArrayRef> {
    let count = i32::try_from(values.len()).map_err(|error| invalid(&error.to_string()))?;
    let mut array: ArrayRef = Arc::new(ListArray::try_new(
        Arc::new(Field::new("item", values.data_type().clone(), true)),
        arrow::buffer::OffsetBuffer::new(vec![0, count].into()),
        values,
        None,
    )?);
    for function in [
        datafusion::functions_nested::set_ops::array_distinct_udf(),
        datafusion::functions_nested::sort::array_sort_udf(),
    ] {
        let fields = vec![Arc::new(Field::new(
            "value",
            array.data_type().clone(),
            false,
        ))];
        let output = function.return_field_from_args(ReturnFieldArgs {
            arg_fields: &fields,
            scalar_arguments: &[None],
        })?;
        array = function
            .invoke_with_args(ScalarFunctionArgs {
                args: vec![ColumnarValue::Array(array)],
                arg_fields: fields,
                number_rows: 1,
                return_field: output,
                config_options: Arc::new(datafusion::common::config::ConfigOptions::default()),
            })?
            .into_array(1)?;
    }
    Ok(array
        .as_any()
        .downcast_ref::<ListArray>()
        .ok_or_else(|| invalid("native set output is not List"))?
        .value(0))
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
                value(
                    &array,
                    row,
                    &Field::new("value", array.data_type().clone(), true),
                    &mut bytes,
                )
                .unwrap();
                bytes
            })
            .collect::<std::collections::BTreeSet<_>>();
        assert_eq!(values.len(), 4);
        let a = StringArray::from(vec!["é"]);
        let b = StringViewArray::from(vec!["é"]);
        let mut first = Vec::new();
        let mut second = Vec::new();
        value(
            &a,
            0,
            &Field::new("value", a.data_type().clone(), true),
            &mut first,
        )
        .unwrap();
        value(
            &b,
            0,
            &Field::new("value", b.data_type().clone(), true),
            &mut second,
        )
        .unwrap();
        assert_eq!(first, second);
    }

    #[test]
    fn schema_identity_binds_domains_layout_and_requiredness_but_not_display_roles() -> Result<()> {
        use std::collections::HashMap;
        let base =
            Field::new("id", DataType::FixedSizeBinary(32), false).with_metadata(HashMap::from([
                ("ARROW:extension:name".into(), "enrichment.identity".into()),
                (
                    "ARROW:extension:metadata".into(),
                    r#"{"version":1,"meaning":"symbol"}"#.into(),
                ),
                ("enrichment.role".into(), "display label".into()),
            ]));
        let storage = Schema::new(vec![Field::new("id", DataType::Binary, false)]);
        let id = schema_identity(&Schema::new(vec![base.clone()]), &storage)?;
        let mut metadata = base.metadata().clone();
        metadata.insert("enrichment.role".into(), "new label".into());
        assert_eq!(
            id,
            schema_identity(
                &Schema::new(vec![base.clone().with_metadata(metadata.clone())]),
                &storage
            )?
        );
        metadata.insert(
            "ARROW:extension:metadata".into(),
            r#"{"version":1,"meaning":"definition"}"#.into(),
        );
        assert_ne!(
            id,
            schema_identity(
                &Schema::new(vec![base.clone().with_metadata(metadata)]),
                &storage
            )?
        );
        assert_ne!(
            id,
            schema_identity(
                &Schema::new(vec![base.clone().with_nullable(true)]),
                &storage
            )?
        );
        assert_ne!(
            id,
            schema_identity(
                &Schema::new(vec![base]),
                &Schema::new(vec![Field::new("id", DataType::LargeBinary, false)])
            )?
        );
        Ok(())
    }

    #[test]
    fn timestamp_and_fixed_binary_frames_preserve_exact_values_and_units() -> Result<()> {
        let values = TimestampMicrosecondArray::from(vec![Some(-1), Some(0), Some(i64::MAX), None])
            .with_timezone("UTC");
        let mut encoded = std::collections::BTreeSet::new();
        for row in 0..values.len() {
            let mut bytes = Vec::new();
            type_bytes(values.data_type(), &mut bytes)?;
            value(
                &values,
                row,
                &Field::new("value", values.data_type().clone(), true),
                &mut bytes,
            )?;
            encoded.insert(bytes);
        }
        assert_eq!(encoded.len(), 4);
        let mut millis = Vec::new();
        let mut micros = Vec::new();
        type_bytes(
            &DataType::Timestamp(TimeUnit::Millisecond, Some("UTC".into())),
            &mut millis,
        )?;
        type_bytes(values.data_type(), &mut micros)?;
        assert_ne!(millis, micros);
        let digest = FixedSizeBinaryArray::try_from_iter([[0u8; 32], [255u8; 32]].iter())?;
        let mut a = Vec::new();
        let mut b = Vec::new();
        value(
            &digest,
            0,
            &Field::new("value", digest.data_type().clone(), true),
            &mut a,
        )?;
        value(
            &digest,
            1,
            &Field::new("value", digest.data_type().clone(), true),
            &mut b,
        )?;
        assert_ne!(a, b);
        Ok(())
    }
}
