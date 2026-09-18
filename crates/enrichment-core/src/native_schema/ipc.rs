//! Validate the native schema-only IPC boundary before Arrow's allocating conversion.
//! Arrow owns FlatBuffer verification and schema construction; this preflight applies
//! our declared bounds and checks the shapes on which Arrow 59's converter panics.
use super::{MAX_DEPTH, MAX_FIELDS, MAX_METADATA_BYTES};
use arrow::{datatypes::Schema, ipc};
use datafusion::common::{DataFusionError, Result};
use std::collections::HashSet;

fn invalid() -> DataFusionError {
    DataFusionError::Plan("invalid or unbounded native schema-only IPC".into())
}
#[derive(Default)]
struct Budget {
    fields: usize,
    bytes: usize,
}
impl Budget {
    fn bytes(&mut self, bytes: usize) -> Result<()> {
        self.bytes = self.bytes.checked_add(bytes).ok_or_else(invalid)?;
        if self.bytes > MAX_METADATA_BYTES {
            return Err(invalid());
        }
        Ok(())
    }
}
fn metadata<'a>(pairs: impl Iterator<Item = ipc::KeyValue<'a>>, budget: &mut Budget) -> Result<()> {
    let mut keys = HashSet::new();
    for pair in pairs {
        let (Some(key), Some(value)) = (pair.key(), pair.value()) else {
            return Err(invalid());
        };
        budget.bytes(key.len().checked_add(value.len()).ok_or_else(invalid)?)?;
        if !keys.insert(key) {
            return Err(invalid());
        }
    }
    Ok(())
}
fn unit(unit: ipc::TimeUnit) -> bool {
    matches!(
        unit,
        ipc::TimeUnit::SECOND
            | ipc::TimeUnit::MILLISECOND
            | ipc::TimeUnit::MICROSECOND
            | ipc::TimeUnit::NANOSECOND
    )
}
fn field(value: ipc::Field<'_>, depth: usize, budget: &mut Budget) -> Result<()> {
    budget.fields += 1;
    if budget.fields > MAX_FIELDS || depth > MAX_DEPTH {
        return Err(invalid());
    }
    let name = value
        .name()
        .filter(|name| !name.is_empty())
        .ok_or_else(invalid)?;
    budget.bytes(name.len())?;
    metadata(
        value
            .custom_metadata()
            .into_iter()
            .flat_map(|pairs| pairs.iter()),
        budget,
    )?;
    if let Some(dictionary) = value.dictionary()
        && !dictionary
            .indexType()
            .is_some_and(|index| [8, 16, 32, 64].contains(&index.bitWidth()))
    {
        return Err(invalid());
    }
    let children = value.children();
    let arity = children.map_or(0, |fields| fields.len());
    let valid = match value.type_type() {
        ipc::Type::Null
        | ipc::Type::Bool
        | ipc::Type::Binary
        | ipc::Type::LargeBinary
        | ipc::Type::BinaryView
        | ipc::Type::Utf8
        | ipc::Type::LargeUtf8
        | ipc::Type::Utf8View => arity == 0,
        ipc::Type::Int => {
            arity == 0
                && value
                    .type_as_int()
                    .is_some_and(|v| [8, 16, 32, 64].contains(&v.bitWidth()))
        }
        ipc::Type::FixedSizeBinary => {
            arity == 0
                && value
                    .type_as_fixed_size_binary()
                    .is_some_and(|v| v.byteWidth() >= 0)
        }
        ipc::Type::FloatingPoint => {
            arity == 0
                && value.type_as_floating_point().is_some_and(|v| {
                    matches!(
                        v.precision(),
                        ipc::Precision::HALF | ipc::Precision::SINGLE | ipc::Precision::DOUBLE
                    )
                })
        }
        ipc::Type::Date => {
            arity == 0
                && value.type_as_date().is_some_and(|v| {
                    matches!(v.unit(), ipc::DateUnit::DAY | ipc::DateUnit::MILLISECOND)
                })
        }
        ipc::Type::Time => {
            arity == 0
                && value.type_as_time().is_some_and(|v| {
                    matches!(
                        (v.bitWidth(), v.unit()),
                        (32, ipc::TimeUnit::SECOND | ipc::TimeUnit::MILLISECOND)
                            | (64, ipc::TimeUnit::MICROSECOND | ipc::TimeUnit::NANOSECOND)
                    )
                })
        }
        ipc::Type::Timestamp => {
            if let Some(v) = value.type_as_timestamp() {
                budget.bytes(v.timezone().map_or(0, str::len))?;
                arity == 0 && unit(v.unit())
            } else {
                false
            }
        }
        ipc::Type::Interval => {
            arity == 0
                && value.type_as_interval().is_some_and(|v| {
                    matches!(
                        v.unit(),
                        ipc::IntervalUnit::YEAR_MONTH
                            | ipc::IntervalUnit::DAY_TIME
                            | ipc::IntervalUnit::MONTH_DAY_NANO
                    )
                })
        }
        ipc::Type::Duration => {
            arity == 0 && value.type_as_duration().is_some_and(|v| unit(v.unit()))
        }
        ipc::Type::Struct_ => true,
        ipc::Type::List | ipc::Type::LargeList | ipc::Type::ListView | ipc::Type::LargeListView => {
            arity == 1
        }
        ipc::Type::FixedSizeList => {
            arity == 1
                && value
                    .type_as_fixed_size_list()
                    .is_some_and(|v| v.listSize() >= 0)
        }
        ipc::Type::RunEndEncoded => arity == 2,
        ipc::Type::Map => {
            arity == 1
                && value.type_as_map().is_some()
                && children.is_some_and(|fields| {
                    fields.get(0).type_type() == ipc::Type::Struct_
                        && fields
                            .get(0)
                            .children()
                            .is_some_and(|fields| fields.len() == 2)
                })
        }
        ipc::Type::Decimal => {
            arity == 0
                && value.type_as_decimal().is_some_and(|v| {
                    let precision = match v.bitWidth() {
                        32 => 9,
                        64 => 18,
                        128 => 38,
                        256 => 76,
                        _ => return false,
                    };
                    (1..=precision).contains(&v.precision()) && i8::try_from(v.scale()).is_ok()
                })
        }
        ipc::Type::Union => value.type_as_union().is_some_and(|v| {
            if arity > 128 || !matches!(v.mode(), ipc::UnionMode::Dense | ipc::UnionMode::Sparse) {
                return false;
            }
            let mut seen = HashSet::new();
            v.typeIds().is_none_or(|ids| {
                ids.len() == arity
                    && ids
                        .iter()
                        .all(|id| (0..128).contains(&id) && seen.insert(id))
            })
        }),
        _ => false,
    };
    if !valid {
        return Err(invalid());
    }
    let mut names = HashSet::new();
    for child in children.into_iter().flat_map(|fields| fields.iter()) {
        if !names.insert(child.name()) {
            return Err(invalid());
        }
        field(child, depth + 1, budget)?;
    }
    Ok(())
}

/// Only the current schema message plus end marker is admitted. No record stream,
/// alternate framing or trailing payload is a semantic-contract registry value.
pub fn decode(bytes: &[u8]) -> Result<Schema> {
    if bytes.len() < 16 || bytes.len() > crate::native_bytes::MAX_BYTES || bytes[..4] != [255; 4] {
        return Err(invalid());
    }
    let length = usize::try_from(i32::from_le_bytes(
        bytes[4..8].try_into().map_err(|_| invalid())?,
    ))
    .map_err(|_| invalid())?;
    let end = 8usize.checked_add(length).ok_or_else(invalid)?;
    if end.checked_add(8) != Some(bytes.len()) || bytes[end..] != [255, 255, 255, 255, 0, 0, 0, 0] {
        return Err(invalid());
    }
    let message = ipc::root_as_message(&bytes[8..end]).map_err(|_| invalid())?;
    if message.version() != ipc::MetadataVersion::V5 || message.bodyLength() != 0 {
        return Err(invalid());
    }
    let schema = message.header_as_schema().ok_or_else(invalid)?;
    if !schema.endianness().equals_to_target_endianness() {
        return Err(invalid());
    }
    let mut budget = Budget::default();
    metadata(
        schema
            .custom_metadata()
            .into_iter()
            .flat_map(|pairs| pairs.iter()),
        &mut budget,
    )?;
    let fields = schema.fields().ok_or_else(invalid)?;
    let mut names = HashSet::new();
    for value in fields {
        if !names.insert(value.name()) {
            return Err(invalid());
        }
        field(value, 0, &mut budget)?;
    }
    let decoded = ipc::convert::fb_to_schema(schema);
    super::validate(&decoded)?;
    Ok(decoded)
}

#[cfg(test)]
mod tests {
    use super::*;
    use arrow::{
        datatypes::{DataType, Field},
        ipc::writer::StreamWriter,
        record_batch::RecordBatch,
    };
    use std::{collections::HashMap, sync::Arc};

    fn encoded(schema: &Schema, record: bool) -> Result<Vec<u8>> {
        let mut bytes = Vec::new();
        let mut writer = StreamWriter::try_new(&mut bytes, schema)?;
        if record {
            writer.write(&RecordBatch::new_empty(Arc::new(schema.clone())))?;
        }
        writer.finish()?;
        Ok(bytes)
    }

    #[test]
    fn plan19_schema_ipc_preserves_native_fields_and_refuses_extra_payload() -> Result<()> {
        let item = Arc::new(Field::new("value", DataType::Utf8, true).with_metadata(
            HashMap::from([("source-contract".into(), "nested-value".into())]),
        ));
        let schema = Schema::new_with_metadata(
            vec![
                Field::new("values", DataType::List(item), true),
                Field::new(
                    "labels",
                    DataType::Dictionary(Box::new(DataType::Int16), Box::new(DataType::Utf8)),
                    false,
                ),
                Field::new(
                    "row",
                    DataType::Struct(vec![Field::new("id", DataType::UInt64, false)].into()),
                    true,
                ),
            ],
            HashMap::from([("contract".into(), "current".into())]),
        );
        let bytes = encoded(&schema, false)?;
        assert_eq!(decode(&bytes)?, schema);
        for end in [0, 4, 8, bytes.len() - 1, bytes.len() - 8] {
            assert!(decode(&bytes[..end]).is_err());
        }
        let mut oversized = bytes.clone();
        oversized[4..8].copy_from_slice(&i32::MAX.to_le_bytes());
        assert!(decode(&oversized).is_err());
        let mut extra = bytes;
        extra.extend_from_slice(&[0; 8]);
        assert!(decode(&extra).is_err());
        assert!(
            decode(&encoded(
                &Schema::new(vec![Field::new("id", DataType::UInt64, false)]),
                true
            )?)
            .is_err()
        );
        Ok(())
    }

    #[test]
    fn plan19_schema_ipc_refuses_declaration_expansion_before_conversion() -> Result<()> {
        let duplicate = Schema::new(vec![Field::new("id", DataType::UInt64, false); 2]);
        assert!(decode(&encoded(&duplicate, false)?).is_err());
        let excessive = Schema::new_with_metadata(
            Vec::<Field>::new(),
            HashMap::from([("oversized".into(), "x".repeat(MAX_METADATA_BYTES))]),
        );
        assert!(decode(&encoded(&excessive, false)?).is_err());
        Ok(())
    }
}
