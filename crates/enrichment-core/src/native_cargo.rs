//! Bounded Cargo TOML syntax facts. Source and workspace policy belongs to native plans.
use crate::native_union::{Cell, Rule};
use arrow::datatypes::{DataType, FieldRef};
use datafusion::{
    common::{DataFusionError, Result},
    logical_expr::{
        ColumnarValue, ReturnFieldArgs, ScalarFunctionArgs, ScalarUDF, ScalarUDFImpl, Signature,
        Volatility,
    },
};

pub const MAX_BYTES: usize = 1_048_576;
const MAX_NODES: usize = 16_384;
const MAX_DEPTH: usize = 64;
const MAX_PATH_BYTES: usize = 16_777_216;

crate::native_vocabulary! { pub enum Kind {
    Table = "table", Array = "array", String = "string", Scalar = "scalar",
} }
crate::native_struct! { pub struct Entry {
    path: Vec<String> => Rule::SequenceBounds { min: 1, max: 64 },
    kind: Kind => Rule::Text,
    text: Option<String> => Rule::Text,
} }

pub fn entries() -> ScalarUDF {
    ScalarUDF::from(Entries {
        signature: Signature::exact(vec![DataType::Utf8], Volatility::Immutable),
    })
}
#[derive(Debug, PartialEq, Eq, Hash)]
struct Entries {
    signature: Signature,
}
impl ScalarUDFImpl for Entries {
    fn name(&self) -> &str {
        "cargo_toml_entries_v1"
    }
    fn signature(&self) -> &Signature {
        &self.signature
    }
    fn return_type(&self, _: &[DataType]) -> Result<DataType> {
        Ok(Vec::<Entry>::data_type())
    }
    fn return_field_from_args(&self, args: ReturnFieldArgs) -> Result<FieldRef> {
        crate::native_schema::text_function_output(
            &args,
            1,
            self.name(),
            Vec::<Entry>::data_type(),
            true,
        )
    }
    fn invoke_with_args(&self, args: ScalarFunctionArgs) -> Result<ColumnarValue> {
        crate::native_schema::function_call(self, &args)?;
        let input = args.args[0].to_array(args.number_rows)?;
        let input = datafusion::common::cast::as_string_array(&input)?;
        let mut input_bytes = 0usize;
        let values = input
            .iter()
            .map(|text| {
                text.map(|text| {
                    input_bytes = input_bytes.checked_add(text.len()).ok_or_else(bound)?;
                    if input_bytes > MAX_BYTES {
                        return Err(bound());
                    }
                    decode(text)
                })
                .transpose()
            })
            .collect::<Result<Vec<_>>>()?;
        Ok(ColumnarValue::Array(<Vec<Entry> as Cell>::encode(
            &values.iter().map(Option::as_ref).collect::<Vec<_>>(),
        )?))
    }
}
fn bound() -> DataFusionError {
    DataFusionError::ResourcesExhausted(
        "Cargo syntax exceeds declared byte/depth/node/path bounds".into(),
    )
}
fn decode(text: &str) -> Result<Vec<Entry>> {
    if text.len() > MAX_BYTES {
        return Err(bound());
    }
    let value: toml::Value = toml::from_str(text)
        .map_err(|error| DataFusionError::Execution(format!("invalid Cargo TOML: {error}")))?;
    let mut pending = vec![(Vec::<String>::new(), &value)];
    let mut values = Vec::new();
    let mut path_bytes = 0usize;
    while let Some((path, value)) = pending.pop() {
        if path.len() > MAX_DEPTH || values.len() + pending.len() >= MAX_NODES {
            return Err(bound());
        }
        let prefix_bytes = path.iter().map(String::len).sum::<usize>();
        let mut child_path = |key: &str| -> Result<Vec<String>> {
            path_bytes = path_bytes
                .checked_add(prefix_bytes)
                .and_then(|n| n.checked_add(key.len()))
                .ok_or_else(bound)?;
            if path.len() >= MAX_DEPTH || path_bytes > MAX_PATH_BYTES {
                return Err(bound());
            }
            let mut next = path.clone();
            next.push(key.into());
            Ok(next)
        };
        let kind = match value {
            toml::Value::Table(table) => {
                if table.len() > MAX_NODES - values.len() - pending.len() {
                    return Err(bound());
                }
                for (key, child) in table {
                    pending.push((child_path(key)?, child));
                }
                Kind::Table
            }
            toml::Value::Array(array) => {
                if array.len() > MAX_NODES - values.len() - pending.len() {
                    return Err(bound());
                }
                for (index, child) in array.iter().enumerate() {
                    pending.push((child_path(&index.to_string())?, child));
                }
                Kind::Array
            }
            toml::Value::String(_) => Kind::String,
            _ => Kind::Scalar,
        };
        if !path.is_empty() {
            values.push(Entry {
                path,
                kind,
                text: value.as_str().map(str::to_owned),
            });
        }
    }
    Ok(values)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn syntax_bounds_precede_repeated_path_allocation() {
        let path = "x".repeat(65_536);
        let children = (0..512).map(|i| format!("key{i}=1\n")).collect::<String>();
        let input = format!("['{path}']\n{children}");
        assert!(input.len() < MAX_BYTES);
        assert!(matches!(
            decode(&input),
            Err(DataFusionError::ResourcesExhausted(_))
        ));
        assert!(decode(&format!("{}=1", vec!["a"; 65].join("."))).is_err());
        assert_eq!(
            decode("[package.metadata]\n'a.b'=1")
                .unwrap()
                .iter()
                .filter(|f| f.path == ["package", "metadata", "a.b"])
                .count(),
            1
        );
    }
}
