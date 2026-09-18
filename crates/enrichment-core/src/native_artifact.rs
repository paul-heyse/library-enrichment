//! Linear format facts for artifact prefixes. This kernel reports byte boundaries and
//! escaped lengths; the native relation decides which prefix satisfies the delivery budget.
use crate::native_union::{Cell, Rule};
use arrow::{
    array::Array,
    datatypes::{DataType, Field, FieldRef},
};
use datafusion::{
    common::Result,
    logical_expr::{
        ColumnarValue, ScalarFunctionArgs, ScalarUDF, ScalarUDFImpl, Signature, Volatility,
    },
};
use std::sync::Arc;

crate::native_struct! {
    pub struct Boundary {
        length: u64 => Rule::Text,
        utf8: bool => Rule::Text,
        json_bytes: u64 => Rule::Text,
        resource_bytes: u64 => Rule::Text,
    }
}

pub fn boundaries() -> ScalarUDF {
    ScalarUDF::from(Boundaries {
        signature: Signature::exact(
            vec![
                DataType::Binary,
                DataType::Boolean,
                DataType::UInt64,
                DataType::Boolean,
            ],
            Volatility::Immutable,
        ),
    })
}
#[derive(Debug, PartialEq, Eq, Hash)]
struct Boundaries {
    signature: Signature,
}
impl ScalarUDFImpl for Boundaries {
    fn name(&self) -> &str {
        "artifact_prefix_boundaries"
    }
    fn signature(&self) -> &Signature {
        &self.signature
    }
    fn return_type(&self, _: &[DataType]) -> Result<DataType> {
        Ok(Vec::<Boundary>::data_type())
    }
    fn return_field_from_args(
        &self,
        _: datafusion::logical_expr::ReturnFieldArgs,
    ) -> Result<FieldRef> {
        Ok(Arc::new(Field::new(
            self.name(),
            Vec::<Boundary>::data_type(),
            false,
        )))
    }
    fn invoke_with_args(&self, args: ScalarFunctionArgs) -> Result<ColumnarValue> {
        let arrays = ColumnarValue::values_to_arrays(&args.args)?;
        let bytes = datafusion::common::cast::as_binary_array(&arrays[0])?;
        let text = datafusion::common::cast::as_boolean_array(&arrays[1])?;
        let maximum = datafusion::common::cast::as_uint64_array(&arrays[2])?;
        let eof = datafusion::common::cast::as_boolean_array(&arrays[3])?;
        let rows = (0..bytes.len())
            .map(|row| {
                if arrays.iter().any(|array| array.is_null(row)) {
                    return datafusion::common::exec_err!("artifact prefix inputs must be present");
                }
                let maximum = usize::try_from(maximum.value(row))
                    .map_err(|e| datafusion::error::DataFusionError::External(Box::new(e)))?;
                facts(bytes.value(row), text.value(row), maximum, eof.value(row))
            })
            .collect::<Result<Vec<_>>>()?;
        Ok(ColumnarValue::Array(Vec::<Boundary>::encode(
            &rows.iter().map(Some).collect::<Vec<_>>(),
        )?))
    }
}
fn facts(bytes: &[u8], text: bool, maximum: usize, eof: bool) -> Result<Vec<Boundary>> {
    if maximum > bytes.len() || bytes.len() > crate::operation::results::MAX_BYTES as usize {
        return datafusion::common::exec_err!("artifact prefix byte bound");
    }
    let mut output = Vec::with_capacity(maximum + 1);
    output.push(Boundary {
        length: 0,
        utf8: text,
        json_bytes: 0,
        resource_bytes: 0,
    });
    let mut binary_start = 1;
    if text {
        let (valid, binary_tail) = match std::str::from_utf8(bytes) {
            Ok(value) => (value, false),
            Err(error) => (
                std::str::from_utf8(&bytes[..error.valid_up_to()])
                    .map_err(|e| datafusion::error::DataFusionError::External(Box::new(e)))?,
                error.error_len().is_some() || eof,
            ),
        };
        let (mut json_bytes, mut resource_bytes) = (0, 0);
        for (offset, character) in valid.char_indices() {
            let length = offset + character.len_utf8();
            if length > maximum {
                break;
            }
            let (json, resource) = match character {
                '"' | '\\' => (2, 4),
                '\n' | '\r' | '\t' | '\u{0008}' | '\u{000c}' => (2, 3),
                value if value < '\u{0020}' => (6, 7),
                value => (value.len_utf8() as u64, value.len_utf8() as u64),
            };
            json_bytes += json;
            resource_bytes += resource;
            output.push(Boundary {
                length: length as u64,
                utf8: true,
                json_bytes,
                resource_bytes,
            });
        }
        if !binary_tail {
            return Ok(output);
        }
        binary_start = valid.len() + 1;
    }
    for length in binary_start..=maximum {
        let encoded = (length as u64).div_ceil(3) * 4;
        output.push(Boundary {
            length: length as u64,
            utf8: false,
            json_bytes: encoded,
            resource_bytes: encoded,
        });
    }
    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn artifact_prefix_costs_match_independent_json_encoding() -> Result<()> {
        let text = "a\"\\\n\r\t\u{0000}é🦀";
        for boundary in facts(text.as_bytes(), true, text.len(), true)? {
            let prefix = &text[..boundary.length as usize];
            let encoded = serde_json::to_string(prefix).unwrap();
            assert_eq!(boundary.json_bytes as usize, encoded.len() - 2);
            let nested = serde_json::to_string(&encoded[1..encoded.len() - 1]).unwrap();
            assert_eq!(boundary.resource_bytes as usize, nested.len() - 2);
        }
        let split = facts("é".as_bytes(), true, 1, false)?;
        assert_eq!(split.len(), 1);
        let malformed = facts(b"a\xffb", true, 3, true)?;
        assert_eq!(
            malformed
                .iter()
                .map(|v| (v.length, v.utf8, v.json_bytes))
                .collect::<Vec<_>>(),
            vec![(0, true, 0), (1, true, 1), (2, false, 4), (3, false, 4)]
        );
        Ok(())
    }
}
