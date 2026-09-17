//! Library-parsed version values expressed in a form native Arrow sorting can compare.
use arrow::{
    array::{Array, BinaryBuilder, StringArray},
    datatypes::{DataType, FieldRef},
};
use datafusion::{
    error::{DataFusionError, Result},
    logical_expr::{
        ColumnarValue, ReturnFieldArgs, ScalarFunctionArgs, ScalarUDF, ScalarUDFImpl, Signature,
        Volatility,
    },
};
use std::sync::Arc;

/// SemVer precedence key. Build metadata is excluded exactly as `cmp_precedence` specifies.
/// Selection, prerelease/yanked policy, ranking and neighbour windows stay in native plans.
pub fn semver_key() -> ScalarUDF {
    ScalarUDF::from(SemverKey {
        signature: Signature::exact(vec![DataType::Utf8], Volatility::Immutable),
    })
}
#[derive(Debug, PartialEq, Eq, Hash)]
struct SemverKey {
    signature: Signature,
}
impl ScalarUDFImpl for SemverKey {
    fn name(&self) -> &str {
        "semver_precedence_key_v1"
    }
    fn signature(&self) -> &Signature {
        &self.signature
    }
    fn return_type(&self, _: &[DataType]) -> Result<DataType> {
        Ok(DataType::Binary)
    }
    fn return_field_from_args(&self, args: ReturnFieldArgs) -> Result<FieldRef> {
        crate::native_schema::text_function_output(&args, 1, self.name(), DataType::Binary, true)
    }
    fn invoke_with_args(&self, args: ScalarFunctionArgs) -> Result<ColumnarValue> {
        crate::native_schema::function_call(self, &args)?;
        if args.args.len() != 1 {
            return Err(DataFusionError::Plan(
                "version key requires one input".into(),
            ));
        }
        let array = args.args[0].clone().into_array(args.number_rows)?;
        if array.len() != args.number_rows {
            return Err(DataFusionError::Execution(
                "version input row count mismatch".into(),
            ));
        }
        let values = array
            .as_any()
            .downcast_ref::<StringArray>()
            .ok_or_else(|| {
                DataFusionError::Plan("version key requires native Utf8 coercion".into())
            })?;
        let mut output = BinaryBuilder::new();
        for row in 0..values.len() {
            if values.is_null(row) {
                output.append_null();
                continue;
            }
            if values.value(row).len() > 65_536 {
                return Err(DataFusionError::ResourcesExhausted(
                    "version value exceeds 64 KiB".into(),
                ));
            }
            match semver::Version::parse(values.value(row)) {
                Ok(version) => output.append_value(precedence_key(&version)),
                Err(_) => output.append_null(),
            }
        }
        Ok(ColumnarValue::Array(Arc::new(output.finish())))
    }
}

fn precedence_key(version: &semver::Version) -> Vec<u8> {
    let mut bytes = Vec::with_capacity(25 + version.pre.len() * 10);
    for number in [version.major, version.minor, version.patch] {
        bytes.extend(number.to_be_bytes());
    }
    bytes.push(u8::from(version.pre.is_empty()));
    if !version.pre.is_empty() {
        for part in version.pre.as_str().split('.') {
            // Zero terminates the identifier sequence, so a prefix precedes its extension.
            bytes.push(1);
            let numeric = part.bytes().all(|byte| byte.is_ascii_digit());
            bytes.push(u8::from(!numeric));
            if numeric {
                bytes.extend((part.len() as u64).to_be_bytes());
            }
            bytes.extend(part.as_bytes());
            bytes.push(0); // SemVer identifiers contain no NUL.
        }
    }
    bytes.push(0);
    bytes
}

/// Parsed PEP 440 values for native sorting, exact equality and environment fields.
pub fn pep440_value() -> ScalarUDF {
    ScalarUDF::from(Pep440Value {
        signature: Signature::exact(vec![DataType::Utf8], Volatility::Immutable),
    })
}
fn pep440_fields() -> arrow::datatypes::Fields {
    use arrow::datatypes::Field;
    vec![
        Field::new("precedence", DataType::Binary, false),
        Field::new(
            "release",
            DataType::List(Arc::new(Field::new("item", DataType::UInt64, true))),
            false,
        ),
        Field::new("prerelease", DataType::Boolean, false),
    ]
    .into()
}
#[derive(Debug, PartialEq, Eq, Hash)]
struct Pep440Value {
    signature: Signature,
}
impl ScalarUDFImpl for Pep440Value {
    fn name(&self) -> &str {
        "pep440_value_v1"
    }
    fn signature(&self) -> &Signature {
        &self.signature
    }
    fn return_type(&self, _: &[DataType]) -> Result<DataType> {
        Ok(DataType::Struct(pep440_fields()))
    }
    fn return_field_from_args(&self, args: ReturnFieldArgs) -> Result<FieldRef> {
        crate::native_schema::text_function_output(
            &args,
            1,
            self.name(),
            DataType::Struct(pep440_fields()),
            true,
        )
    }
    fn invoke_with_args(&self, args: ScalarFunctionArgs) -> Result<ColumnarValue> {
        crate::native_schema::function_call(self, &args)?;
        use arrow::array::{BooleanBuilder, ListBuilder, StructArray, UInt64Builder};
        let arrays = string_inputs(&args, 1)?;
        let values = arrays[0]
            .as_any()
            .downcast_ref::<StringArray>()
            .ok_or_else(|| DataFusionError::Internal("PEP 440 native coercion".into()))?;
        let mut keys = BinaryBuilder::new();
        let mut releases = ListBuilder::new(UInt64Builder::new());
        let mut prereleases = BooleanBuilder::new();
        let mut validity = Vec::with_capacity(args.number_rows);
        for value in values {
            let parsed = value.and_then(|value| value.parse::<pep440_rs::Version>().ok());
            if let Some(version) = parsed {
                keys.append_value(pep440_key(&version));
                for value in version.release() {
                    releases.values().append_value(*value);
                }
                releases.append(true);
                prereleases.append_value(version.any_prerelease());
                validity.push(true);
            } else {
                keys.append_null();
                releases.append(false);
                prereleases.append_null();
                validity.push(false);
            }
        }
        Ok(ColumnarValue::Array(Arc::new(StructArray::try_new(
            pep440_fields(),
            vec![
                Arc::new(keys.finish()),
                Arc::new(releases.finish()),
                Arc::new(prereleases.finish()),
            ],
            Some(arrow::buffer::NullBuffer::from(validity)),
        )?)))
    }
}

fn string_inputs(args: &ScalarFunctionArgs, arity: usize) -> Result<Vec<arrow::array::ArrayRef>> {
    if args.args.len() != arity {
        return Err(DataFusionError::Plan("version kernel arity".into()));
    }
    args.args
        .iter()
        .map(|arg| {
            let array = arg.clone().into_array(args.number_rows)?;
            if array.len() != args.number_rows {
                return Err(DataFusionError::Execution(
                    "version kernel row count".into(),
                ));
            }
            let strings = array
                .as_any()
                .downcast_ref::<StringArray>()
                .ok_or_else(|| {
                    DataFusionError::Plan("version kernel requires Utf8 coercion".into())
                })?;
            if strings.iter().flatten().any(|s| s.len() > 65_536) {
                return Err(DataFusionError::ResourcesExhausted(
                    "version value exceeds 64 KiB".into(),
                ));
            }
            Ok(array)
        })
        .collect()
}

/// PEP 440's library-defined specifier predicate; native plans own all eligibility composition.
pub fn pep440_matches() -> ScalarUDF {
    ScalarUDF::from(Pep440Matches {
        signature: Signature::exact(vec![DataType::Utf8, DataType::Utf8], Volatility::Immutable),
    })
}
#[derive(Debug, PartialEq, Eq, Hash)]
struct Pep440Matches {
    signature: Signature,
}
impl ScalarUDFImpl for Pep440Matches {
    fn name(&self) -> &str {
        "pep440_matches_v1"
    }
    fn signature(&self) -> &Signature {
        &self.signature
    }
    fn return_type(&self, _: &[DataType]) -> Result<DataType> {
        Ok(DataType::Boolean)
    }
    fn return_field_from_args(&self, args: ReturnFieldArgs) -> Result<FieldRef> {
        let nullable = args.arg_fields.iter().any(|field| field.is_nullable());
        crate::native_schema::text_function_output(
            &args,
            2,
            self.name(),
            DataType::Boolean,
            nullable,
        )
    }
    fn invoke_with_args(&self, args: ScalarFunctionArgs) -> Result<ColumnarValue> {
        crate::native_schema::function_call(self, &args)?;
        let arrays = string_inputs(&args, 2)?;
        let arrays = arrays
            .iter()
            .map(|a| {
                a.as_any()
                    .downcast_ref::<StringArray>()
                    .ok_or_else(|| DataFusionError::Internal("PEP 440 native coercion".into()))
            })
            .collect::<Result<Vec<_>>>()?;
        let mut output = arrow::array::BooleanBuilder::new();
        for (specifier, value) in arrays[0].iter().zip(arrays[1].iter()) {
            match (specifier, value) {
                (Some(specifier), Some(value)) => {
                    let specifier =
                        specifier
                            .parse::<pep440_rs::VersionSpecifiers>()
                            .map_err(|e| {
                                DataFusionError::Execution(format!(
                                    "invalid PEP 440 specifier: {e}"
                                ))
                            })?;
                    let value = value.parse::<pep440_rs::Version>().map_err(|e| {
                        DataFusionError::Execution(format!("invalid PEP 440 version: {e}"))
                    })?;
                    output.append_value(specifier.contains(&value));
                }
                _ => output.append_null(),
            }
        }
        Ok(ColumnarValue::Array(Arc::new(output.finish())))
    }
}

fn pep440_key(version: &pep440_rs::Version) -> Vec<u8> {
    use pep440_rs::{LocalSegment, PrereleaseKind};
    let mut bytes = version.epoch().to_be_bytes().to_vec();
    let release = version.release();
    let last = release
        .iter()
        .rposition(|value| *value != 0)
        .map_or(0, |index| index + 1);
    for value in &release[..last] {
        bytes.push(1);
        bytes.extend(value.to_be_bytes());
    }
    bytes.push(0);
    // Parsed public values have no internal solver-only min/max sentinel components.
    let (stage, pre_number, dev) = if let Some(pre) = version.pre() {
        (
            match pre.kind {
                PrereleaseKind::Alpha => 2,
                PrereleaseKind::Beta => 3,
                PrereleaseKind::Rc => 4,
            },
            pre.number,
            version.dev().unwrap_or(u64::MAX),
        )
    } else if version.post().is_some() {
        (6, 0, version.dev().unwrap_or(u64::MAX))
    } else if let Some(dev) = version.dev() {
        (1, 0, dev)
    } else {
        (5, 0, 0)
    };
    bytes.push(stage);
    bytes.extend(pre_number.to_be_bytes());
    bytes.push(u8::from(version.post().is_some()));
    if let Some(post) = version.post() {
        bytes.extend(post.to_be_bytes());
    }
    bytes.extend(dev.to_be_bytes());
    for segment in version.local() {
        bytes.push(1);
        match segment {
            LocalSegment::String(value) => {
                bytes.push(0);
                bytes.extend(value.as_bytes());
                bytes.push(0);
            }
            LocalSegment::Number(value) => {
                bytes.push(1);
                bytes.extend(value.to_be_bytes());
            }
        }
    }
    bytes.push(0);
    bytes
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn pep440_native_order_matches_library_for_epochs_padding_pre_post_dev_and_local() {
        let versions = [
            "0",
            "0.0",
            "1",
            "1.0",
            "1.0.0",
            "1.0.0.1",
            "1.1",
            "1.0.dev456",
            "1.0a1",
            "1.0a2.dev456",
            "1.0a12.dev456",
            "1.0a12",
            "1.0b1.dev456",
            "1.0b2",
            "1.0b2.post345.dev456",
            "1.0b2.post345",
            "1.0b2-346",
            "1.0c1.dev456",
            "1.0c1",
            "1.0rc2",
            "1.0c3",
            "1.0",
            "1.0.post456.dev34",
            "1.0.post456",
            "1.0+abc",
            "1.0+abc.1",
            "1.0+1",
            "1.0+2",
            "1.0+10",
            "1!0",
            "2!0",
            "18446744073709551615!1.0",
        ]
        .map(|s| s.parse::<pep440_rs::Version>().unwrap());
        for a in &versions {
            for b in &versions {
                assert_eq!(pep440_key(a).cmp(&pep440_key(b)), a.cmp(b), "{a} / {b}");
            }
        }
    }

    #[test]
    fn native_byte_order_matches_library_precedence() {
        let versions = [
            "0.0.0",
            "1.0.0-alpha",
            "1.0.0-alpha.1",
            "1.0.0-alpha.beta",
            "1.0.0-beta",
            "1.0.0-beta.2",
            "1.0.0-beta.11",
            "1.0.0-rc.1",
            "1.0.0",
            "1.0.0+first",
            "1.0.0+second",
            "1.0.0-1",
            "1.0.0-10",
            "1.0.0-2",
            "1.0.0-999999999999999999999999999999",
            "1.0.0-a-1",
            "1.0.0-a1",
            "1.0.1",
            "1.1.0",
            "2.0.0",
            "18446744073709551615.0.0",
        ]
        .map(|text| semver::Version::parse(text).unwrap());
        for a in &versions {
            for b in &versions {
                assert_eq!(
                    precedence_key(a).cmp(&precedence_key(b)),
                    a.cmp_precedence(b),
                    "{a} / {b}"
                );
            }
        }
    }
}
