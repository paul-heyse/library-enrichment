//! Bounded external metadata syntax. Native relations own identity, tags and admission.
use crate::native_union::Cell;
use arrow::datatypes::{DataType, FieldRef};
use datafusion::{
    common::{DataFusionError, Result},
    logical_expr::{
        ColumnarValue, ReturnFieldArgs, ScalarFunctionArgs, ScalarUDF, ScalarUDFImpl, Signature,
        Volatility,
    },
};
use std::collections::BTreeMap;
type Headers = BTreeMap<String, Vec<String>>;
pub fn headers() -> ScalarUDF {
    ScalarUDF::from(Decode {
        signature: Signature::exact(vec![DataType::Utf8], Volatility::Immutable),
    })
}
#[derive(Debug, PartialEq, Eq, Hash)]
struct Decode {
    signature: Signature,
}
impl ScalarUDFImpl for Decode {
    fn name(&self) -> &str {
        "python_metadata_headers_v1"
    }
    fn signature(&self) -> &Signature {
        &self.signature
    }
    fn return_type(&self, _: &[DataType]) -> Result<DataType> {
        Ok(Headers::data_type())
    }
    fn return_field_from_args(&self, args: ReturnFieldArgs) -> Result<FieldRef> {
        crate::native_schema::text_function_output(
            &args,
            1,
            self.name(),
            Headers::data_type(),
            true,
        )
    }
    fn invoke_with_args(&self, args: ScalarFunctionArgs) -> Result<ColumnarValue> {
        crate::native_schema::function_call(self, &args)?;
        let array = args.args[0].to_array(args.number_rows)?;
        let strings = datafusion::common::cast::as_string_array(&array)?;
        let mut bytes = 0usize;
        let values = strings
            .iter()
            .map(|text| {
                text.map(|text| {
                    bytes = bytes.checked_add(text.len()).ok_or_else(bound)?;
                    if bytes > 3_145_728 || text.len() > 1_048_576 {
                        return Err(bound());
                    }
                    let headers = crate::producer::python::metadata_headers(text);
                    if headers.values().map(Vec::len).sum::<usize>() > 16_384 {
                        return Err(bound());
                    }
                    Ok(headers)
                })
                .transpose()
            })
            .collect::<Result<Vec<_>>>()?;
        Ok(ColumnarValue::Array(Headers::encode(
            &values.iter().map(Option::as_ref).collect::<Vec<_>>(),
        )?))
    }
}
fn bound() -> DataFusionError {
    DataFusionError::ResourcesExhausted("Python metadata byte/header bound".into())
}
