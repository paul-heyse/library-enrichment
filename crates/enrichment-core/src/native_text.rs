//! Format-only UTF-8 coordinate kernel; request eligibility stays in native plans.
use arrow::{
    array::{Array, BooleanArray, UInt32Array},
    datatypes::{DataType, Field, FieldRef},
};
use datafusion::{
    common::{Result, cast::as_string_array},
    logical_expr::{
        ColumnarValue, ReturnFieldArgs, ScalarFunctionArgs, ScalarUDF, ScalarUDFImpl, Signature,
        Volatility,
    },
};
use std::sync::Arc;

pub fn position() -> ScalarUDF {
    ScalarUDF::from(Utf8Position {
        signature: Signature::exact(
            vec![DataType::Utf8, DataType::UInt32, DataType::UInt32],
            Volatility::Immutable,
        ),
    })
}
#[derive(Debug, PartialEq, Eq, Hash)]
struct Utf8Position {
    signature: Signature,
}
impl ScalarUDFImpl for Utf8Position {
    fn name(&self) -> &str {
        "utf8_position_valid"
    }
    fn signature(&self) -> &Signature {
        &self.signature
    }
    fn return_type(&self, _: &[DataType]) -> Result<DataType> {
        Ok(DataType::Boolean)
    }
    fn return_field_from_args(&self, args: ReturnFieldArgs) -> Result<FieldRef> {
        let expected = [DataType::Utf8, DataType::UInt32, DataType::UInt32];
        if args.arg_fields.len() != 3
            || args.arg_fields.iter().zip(expected).any(|(field, kind)| {
                field.data_type() != &kind || crate::native_analysis::has_semantics(field)
            })
        {
            return datafusion::common::plan_err!("UTF-8 coordinate input contract");
        }
        Ok(Arc::new(Field::new(self.name(), DataType::Boolean, false)))
    }
    fn invoke_with_args(&self, args: ScalarFunctionArgs) -> Result<ColumnarValue> {
        crate::native_schema::function_call(self, &args)?;
        let arrays = ColumnarValue::values_to_arrays(&args.args)?;
        let text = as_string_array(&arrays[0])?;
        let line = arrays[1]
            .as_any()
            .downcast_ref::<UInt32Array>()
            .ok_or_else(|| datafusion::common::exec_datafusion_err!("UTF-8 line type"))?;
        let byte = arrays[2]
            .as_any()
            .downcast_ref::<UInt32Array>()
            .ok_or_else(|| datafusion::common::exec_datafusion_err!("UTF-8 byte type"))?;
        let output = BooleanArray::from_iter((0..text.len()).map(|row| {
            Some(
                !text.is_null(row)
                    && !line.is_null(row)
                    && !byte.is_null(row)
                    && crate::evidence::execution::Utf8Position {
                        line: line.value(row),
                        byte: byte.value(row),
                    }
                    .validate(text.value(row))
                    .is_ok(),
            )
        }));
        Ok(ColumnarValue::Array(Arc::new(output)))
    }
}
