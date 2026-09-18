//! Binary SHA-256 values with one declared algorithm and a strict textual boundary.
use crate::native_union::Cell;
use arrow::{
    array::ArrayRef,
    datatypes::{DataType, Field},
    error::ArrowError,
};
use arrow_schema::extension::ExtensionType;
use datafusion::{
    common::{DataFusionError, Result, ScalarValue},
    logical_expr::Expr,
};
use std::{collections::HashMap, fmt};

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
)]
#[serde(into = "String", try_from = "String")]
#[schemars(inline, extend("pattern" = "^[0-9a-f]{64}$"))]
pub struct Sha256Digest([u8; 32]);

impl Sha256Digest {
    pub fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }

    /// DataFusion owns hashing; the checked projection attaches the declared fixed-width type.
    pub fn expression(bytes: Expr) -> Expr {
        crate::native_id::digest_from_hash(datafusion::functions::crypto::expr_fn::sha256(bytes))
    }

    /// The same built-in native hash at byte ingress. The scalar kernel borrows its single
    /// owned input; a large payload never passes through expression-simplifier cloning.
    /// Callers own admission and physical execution of this one input copy.
    pub fn hash(bytes: &[u8]) -> Result<Self> {
        use datafusion::logical_expr::{ColumnarValue, ReturnFieldArgs, ScalarFunctionArgs};
        use std::sync::Arc;
        if bytes.len() > crate::native_bytes::MAX_BYTES {
            return datafusion::common::resources_err!("native digest input exceeds byte bound");
        }
        let function = datafusion::functions::crypto::sha256();
        let fields = vec![Arc::new(Field::new("bytes", DataType::Binary, false))];
        let output = function.return_field_from_args(ReturnFieldArgs {
            arg_fields: &fields,
            scalar_arguments: &[None],
        })?;
        let value = function.invoke_with_args(ScalarFunctionArgs {
            args: vec![ColumnarValue::Scalar(ScalarValue::Binary(Some(
                bytes.to_vec(),
            )))],
            arg_fields: fields,
            number_rows: 1,
            return_field: output,
            config_options: Default::default(),
        })?;
        match value {
            ColumnarValue::Scalar(ScalarValue::Binary(Some(value))) => value
                .try_into()
                .map(Self)
                .map_err(|_| DataFusionError::Internal("SHA-256 width changed".into())),
            _ => Err(DataFusionError::Internal(
                "native SHA-256 did not return one binary digest".into(),
            )),
        }
    }
}
impl From<[u8; 32]> for Sha256Digest {
    fn from(value: [u8; 32]) -> Self {
        Self(value)
    }
}
impl From<Sha256Digest> for String {
    fn from(value: Sha256Digest) -> Self {
        value.to_string()
    }
}
impl fmt::Display for Sha256Digest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for byte in self.0 {
            write!(f, "{byte:02x}")?;
        }
        Ok(())
    }
}
impl TryFrom<String> for Sha256Digest {
    type Error = &'static str;
    fn try_from(value: String) -> std::result::Result<Self, Self::Error> {
        if value.len() != 64
            || !value
                .bytes()
                .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
        {
            return Err("SHA-256 requires exactly 64 lowercase hexadecimal digits");
        }
        let mut bytes = [0; 32];
        for (byte, pair) in bytes.iter_mut().zip(value.as_bytes().as_chunks::<2>().0) {
            let digit = |b| if b <= b'9' { b - b'0' } else { b - b'a' + 10 };
            *byte = digit(pair[0]) * 16 + digit(pair[1]);
        }
        Ok(Self(bytes))
    }
}
impl Cell for Sha256Digest {
    fn data_type() -> DataType {
        DataType::FixedSizeBinary(32)
    }
    fn metadata() -> HashMap<String, String> {
        let extension = crate::native_types::DigestType::try_new(
            &Self::data_type(),
            crate::native_types::TypeMetadata::new(crate::native_types::DigestAlgorithm::Sha256),
        )
        .expect("declared SHA-256 digest");
        Field::new("digest", Self::data_type(), false)
            .with_extension_type(extension)
            .metadata()
            .clone()
    }
    fn encode(values: &[Option<&Self>]) -> std::result::Result<ArrayRef, ArrowError> {
        <[u8; 32] as Cell>::encode(
            &values
                .iter()
                .map(|value| value.map(Self::as_bytes))
                .collect::<Vec<_>>(),
        )
    }
    fn decode(
        row: crate::evidence::arrow_model::cells::Row<'_>,
        name: &str,
    ) -> std::result::Result<Self, ArrowError> {
        row.fixed_binary(name).map(Self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn sha256_binary_native_and_wire_vectors_are_independent_and_strict() -> Result<()> {
        for (bytes, hex) in [
            (
                b"".as_slice(),
                "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
            ),
            (
                b"abc".as_slice(),
                "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad",
            ),
        ] {
            let value = Sha256Digest::hash(bytes)?;
            assert_eq!(value.to_string(), hex);
            assert_eq!(Sha256Digest::try_from(hex.to_owned()).unwrap(), value);
            let field = crate::native_union::field::<Sha256Digest>(
                "digest",
                crate::native_union::Rule::Text,
            );
            let array = Sha256Digest::encode(&[Some(&value)])?;
            let mut native = Vec::new();
            crate::native_json::write_value(
                &mut native,
                128,
                &std::sync::Arc::new(field),
                array.as_ref(),
                0,
            )
            .map_err(|e| DataFusionError::External(Box::new(e)))?;
            assert_eq!(native, serde_json::to_vec(&value).unwrap());
        }
        for bad in [
            "0".repeat(63),
            "0".repeat(65),
            "A".repeat(64),
            "g".repeat(64),
        ] {
            assert!(Sha256Digest::try_from(bad).is_err());
        }
        let digest =
            crate::native_union::field::<Sha256Digest>("value", crate::native_union::Rule::Text);
        let identity = crate::native_union::field::<crate::identity::SnapshotId>(
            "value",
            crate::native_union::Rule::Text,
        );
        assert!(
            crate::native_schema::function_arguments(
                &[std::sync::Arc::new(identity)],
                &[std::sync::Arc::new(digest)]
            )
            .is_err(),
            "same physical width is not the same meaning"
        );
        Ok(())
    }
}
