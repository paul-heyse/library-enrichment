//! Bounded variable-width bytes: Arrow Binary in native records, lower-case hex on wire.
use crate::{evidence::arrow_model::cells, native_union::Cell};
use arrow::{
    array::{ArrayRef, BinaryArray},
    datatypes::DataType,
    error::ArrowError,
};
use std::sync::Arc;

pub const MAX_BYTES: usize = 64 * 1024 * 1024;

#[derive(Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
#[serde(try_from = "String", into = "String")]
#[schemars(with = "String")]
pub struct NativeBytes(Arc<[u8]>);
impl std::fmt::Debug for NativeBytes {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("NativeBytes")
            .field("bytes", &self.0.len())
            .finish()
    }
}
impl NativeBytes {
    pub fn new(bytes: Vec<u8>) -> Result<Self, String> {
        if bytes.len() > MAX_BYTES {
            return Err("native binary byte bound".into());
        }
        Ok(Self(bytes.into()))
    }
    pub fn as_slice(&self) -> &[u8] {
        &self.0
    }
}
impl TryFrom<String> for NativeBytes {
    type Error = String;
    fn try_from(value: String) -> Result<Self, String> {
        if value.len() > MAX_BYTES * 2 || !value.len().is_multiple_of(2) {
            return Err("native binary hex width".into());
        }
        fn nibble(value: u8) -> Result<u8, String> {
            match value {
                b'0'..=b'9' => Ok(value - b'0'),
                b'a'..=b'f' => Ok(value - b'a' + 10),
                _ => Err("native binary requires lower-case hex".into()),
            }
        }
        Self::new(
            value
                .as_bytes()
                .as_chunks::<2>()
                .0
                .iter()
                .map(|pair| Ok(nibble(pair[0])? * 16 + nibble(pair[1])?))
                .collect::<Result<Vec<_>, String>>()?,
        )
    }
}
impl From<NativeBytes> for String {
    fn from(value: NativeBytes) -> Self {
        const HEX: &[u8] = b"0123456789abcdef";
        let mut text = String::with_capacity(value.0.len() * 2);
        for value in value.0.iter() {
            text.push(HEX[(value >> 4) as usize] as char);
            text.push(HEX[(value & 15) as usize] as char);
        }
        text
    }
}
impl Cell for NativeBytes {
    fn data_type() -> DataType {
        DataType::Binary
    }
    fn encode(values: &[Option<&Self>]) -> Result<ArrayRef, ArrowError> {
        Ok(Arc::new(
            values
                .iter()
                .map(|value| value.map(Self::as_slice))
                .collect::<BinaryArray>(),
        ))
    }
    fn decode(row: cells::Row<'_>, name: &str) -> Result<Self, ArrowError> {
        let bytes = row.binary(name)?;
        if bytes.len() > MAX_BYTES {
            return Err(cells::invalid("native binary byte bound"));
        }
        Ok(Self(Arc::from(bytes)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::native_union::{NativeStruct, Rule};
    crate::native_struct! {
        struct Fixture { payload: NativeBytes => Rule::BinaryBytes { max: 4 } }
    }
    #[test]
    fn plan19_binary_native_and_wire_agree() -> Result<(), Box<dyn std::error::Error>> {
        let value = Fixture {
            payload: NativeBytes::new(vec![0, 127, 128, 255])?,
        };
        let batch = Fixture::batch(std::slice::from_ref(&value))?;
        assert_eq!(batch.schema().field(0).data_type(), &DataType::Binary);
        let rows = cells::RowSet::batch(&batch)?;
        assert_eq!(<Fixture as NativeStruct>::decode(rows.row(0))?, value);
        let field = batch.schema().field(0).clone();
        let mut bytes = Vec::new();
        crate::native_json::write_value(
            &mut bytes,
            128,
            &Arc::new(field),
            batch.column(0).as_ref(),
            0,
        )?;
        assert_eq!(
            serde_json::from_slice::<NativeBytes>(&bytes)?,
            value.payload
        );
        assert_eq!(serde_json::to_vec(&value.payload)?, bytes);
        let schema = schemars::schema_for!(Fixture);
        assert_eq!(schema.as_value()["properties"]["payload"]["maxLength"], 8);
        for invalid in ["0", "00FF", "xx"] {
            assert!(NativeBytes::try_from(invalid.to_owned()).is_err());
        }
        Ok(())
    }
}
