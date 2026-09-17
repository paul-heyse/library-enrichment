//! One field-aware, bounded final JSON value sink for admitted native results.
//!
//! Arrow's default encoders handle fixed-width primitives. Their `Vec<u8>` API cannot
//! bound a large string or container, so those stream directly to the same counted sink.
//! This layer formats values; native plans own validation, selection and paging.
use arrow::{
    array::{Array, AsArray, StructArray},
    datatypes::{DataType, Field, FieldRef, Fields, Schema},
};
use arrow_schema::extension::{ExtensionType, Json};
use std::{
    io::{self, Write},
    sync::Arc,
};

/// The encoded representation participates in retained-result and replay definitions.
pub const REVISION: &str = "native-json/3";

/// Checks the cap before forwarding bytes, including JSON escaping and delimiters.
pub struct BoundedWriter<W> {
    inner: W,
    written: usize,
    limit: usize,
}
impl<W> BoundedWriter<W> {
    pub fn new(inner: W, limit: usize) -> Self {
        Self {
            inner,
            written: 0,
            limit,
        }
    }
    pub fn written(&self) -> usize {
        self.written
    }
}
impl<W: Write> Write for BoundedWriter<W> {
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        if bytes.len() > self.limit.saturating_sub(self.written) {
            return Err(io::Error::new(
                io::ErrorKind::OutOfMemory,
                "native JSON byte bound",
            ));
        }
        let written = self.inner.write(bytes)?;
        self.written += written;
        Ok(written)
    }
    fn flush(&mut self) -> io::Result<()> {
        self.inner.flush()
    }
}
fn invalid(error: impl std::fmt::Display) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, error.to_string())
}
fn quoted(writer: &mut dyn Write, text: &str) -> io::Result<()> {
    serde_json::to_writer(writer, text).map_err(io::Error::from)
}

/// Write one admitted value without an intermediate owned semantic object.
/// Nulls are explicit; lists retain order; decimals and binary values are exact strings.
/// Full-range UInt64 and Int64 values use decimal strings, including zero and small values.
pub fn write_value(
    writer: impl Write,
    limit: usize,
    field: &FieldRef,
    array: &dyn Array,
    row: usize,
) -> io::Result<usize> {
    crate::native_schema::validate(&Schema::new(vec![field.clone()])).map_err(invalid)?;
    if row >= array.len() || field.data_type() != array.data_type() {
        return Err(invalid("native JSON field/array mismatch"));
    }
    let mut writer = BoundedWriter::new(writer, limit);
    value(&mut writer, field, array, row)?;
    writer.flush()?;
    Ok(writer.written())
}

fn hex(writer: &mut dyn Write, prefix: Option<&str>, bytes: &[u8]) -> io::Result<()> {
    writer.write_all(b"\"")?;
    if let Some(prefix) = prefix {
        writer.write_all(prefix.as_bytes())?;
        writer.write_all(b"_")?;
    }
    const DIGITS: &[u8; 16] = b"0123456789abcdef";
    for byte in bytes {
        writer.write_all(&[
            DIGITS[usize::from(byte >> 4)],
            DIGITS[usize::from(byte & 15)],
        ])?;
    }
    writer.write_all(b"\"")
}

fn member(
    writer: &mut dyn Write,
    first: &mut bool,
    name: &str,
    field: &FieldRef,
    array: &dyn Array,
    row: usize,
) -> io::Result<()> {
    if !*first {
        writer.write_all(b",")?;
    }
    *first = false;
    quoted(writer, name)?;
    writer.write_all(b":")?;
    value(writer, field, array, row)
}

fn object(
    writer: &mut dyn Write,
    fields: &Fields,
    array: &StructArray,
    row: usize,
) -> io::Result<()> {
    let discriminator = fields
        .iter()
        .enumerate()
        .find(|(_, field)| field.metadata().contains_key("enrichment.union.tags"));
    if let Some((index, field)) = discriminator
        && field
            .metadata()
            .get("enrichment.union.content")
            .is_some_and(String::is_empty)
    {
        let kind = array.column(index).as_string::<i32>();
        if kind.is_null(row) {
            return Err(invalid("missing native payload discriminator"));
        }
        let selected = kind.value(row);
        let tags: Vec<String> = serde_json::from_str(&field.metadata()["enrichment.union.tags"])?;
        if !tags.iter().any(|tag| tag == selected) {
            return Err(invalid("unknown native payload discriminator"));
        }
        let Some((active_index, active_field)) = fields.find(selected) else {
            if array
                .columns()
                .iter()
                .enumerate()
                .any(|(child_index, child)| child_index != index && !child.is_null(row))
            {
                return Err(invalid("unit native payload has populated fields"));
            }
            return writer.write_all(b"{}");
        };
        for (child_index, child) in array.columns().iter().enumerate() {
            if child_index != index && child_index != active_index && !child.is_null(row) {
                return Err(invalid("inactive native payload"));
            }
        }
        if array.column(active_index).is_null(row) {
            return Err(invalid("missing native payload"));
        }
        return value(
            writer,
            active_field,
            array.column(active_index).as_ref(),
            row,
        );
    }
    writer.write_all(b"{")?;
    let mut first = true;
    if let Some((index, field)) = discriminator {
        let tags: Vec<String> = serde_json::from_str(&field.metadata()["enrichment.union.tags"])?;
        let kind = array.column(index).as_string::<i32>();
        if kind.is_null(row) {
            return Err(invalid("missing native union discriminator"));
        }
        let selected = kind.value(row);
        if !tags.iter().any(|tag| tag == selected) {
            return Err(invalid("unknown native union variant"));
        }
        member(
            writer,
            &mut first,
            field.name(),
            field,
            array.column(index).as_ref(),
            row,
        )?;
        for (index, child) in fields.iter().enumerate() {
            if child.name() == field.name() {
                continue;
            }
            if tags.contains(child.name()) {
                let payload = array.column(index);
                if child.name() != selected {
                    if !payload.is_null(row) {
                        return Err(invalid("inactive native union payload"));
                    }
                    continue;
                }
                if payload.is_null(row) {
                    return Err(invalid("missing native union payload"));
                }
                if let Some(content) = field.metadata().get("enrichment.union.content") {
                    member(writer, &mut first, content, child, payload.as_ref(), row)?;
                } else {
                    let DataType::Struct(members) = child.data_type() else {
                        return Err(invalid("flattened union requires a record"));
                    };
                    let payload = payload.as_struct();
                    for (index, member_field) in members.iter().enumerate() {
                        member(
                            writer,
                            &mut first,
                            member_field.name(),
                            member_field,
                            payload.column(index).as_ref(),
                            row,
                        )?;
                    }
                }
            } else {
                member(
                    writer,
                    &mut first,
                    child.name(),
                    child,
                    array.column(index).as_ref(),
                    row,
                )?;
            }
        }
    } else {
        object_members(writer, fields, array, row, &mut first)?;
    }

    writer.write_all(b"}")
}

fn object_members(
    writer: &mut dyn Write,
    fields: &Fields,
    array: &StructArray,
    row: usize,
    first: &mut bool,
) -> io::Result<()> {
    for (index, field) in fields.iter().enumerate() {
        let rule = field
            .metadata()
            .get("enrichment.rule")
            .map(|rule| serde_json::from_str::<crate::native_union::Rule>(rule))
            .transpose()
            .map_err(io::Error::other)?;
        if matches!(rule, Some(crate::native_union::Rule::Flatten)) {
            let DataType::Struct(children) = field.data_type() else {
                return Err(invalid("flattened native field must be a record"));
            };
            let nested = array.column(index).as_struct();
            if nested.is_null(row) {
                return Err(invalid("flattened native field must be present"));
            }
            object_members(writer, children, nested, row, first)?;
        } else {
            member(
                writer,
                first,
                field.name(),
                field,
                array.column(index).as_ref(),
                row,
            )?;
        }
    }
    Ok(())
}

fn value(
    writer: &mut dyn Write,
    field: &FieldRef,
    array: &dyn Array,
    row: usize,
) -> io::Result<()> {
    if array.is_null(row) {
        if crate::native_schema::required(field) {
            return Err(invalid("required native JSON value is null"));
        }
        return writer.write_all(b"null");
    }
    if let Some(extension) = field.metadata().get("ARROW:extension:name") {
        use crate::native_types::{ClockType, DigestType, IdentityType};
        match extension.as_str() {
            IdentityType::NAME => {
                let meaning = IdentityType::deserialize_metadata(
                    field
                        .metadata()
                        .get("ARROW:extension:metadata")
                        .map(String::as_str),
                )
                .map_err(invalid)?;
                return hex(
                    writer,
                    Some(meaning.meaning().prefix()),
                    array.as_fixed_size_binary().value(row),
                );
            }
            DigestType::NAME => return hex(writer, None, array.as_fixed_size_binary().value(row)),
            ClockType::NAME => {
                let micros = array
                    .as_primitive::<arrow::datatypes::TimestampMicrosecondType>()
                    .value(row);
                return quoted(
                    writer,
                    &String::from(
                        crate::native_time::EventTime::from_micros(micros).map_err(invalid)?,
                    ),
                );
            }
            Json::NAME => {
                let json = array.as_string::<i32>().value(row);
                serde_json::from_str::<serde::de::IgnoredAny>(json)?;
                return writer.write_all(json.as_bytes());
            }
            _ => return Err(invalid("unknown native JSON extension")),
        }
    }
    match array.data_type() {
        DataType::Null => writer.write_all(b"null"),
        DataType::Utf8 => quoted(writer, array.as_string::<i32>().value(row)),
        DataType::LargeUtf8 => quoted(writer, array.as_string::<i64>().value(row)),
        DataType::Utf8View => quoted(writer, array.as_string_view().value(row)),
        DataType::Binary => hex(writer, None, array.as_binary::<i32>().value(row)),
        DataType::LargeBinary => hex(writer, None, array.as_binary::<i64>().value(row)),
        DataType::BinaryView => hex(writer, None, array.as_binary_view().value(row)),
        DataType::FixedSizeBinary(_) => hex(writer, None, array.as_fixed_size_binary().value(row)),
        DataType::Struct(fields) => object(writer, fields, array.as_struct(), row),
        DataType::List(item) | DataType::LargeList(item) | DataType::FixedSizeList(item, _) => {
            let items = match array.data_type() {
                DataType::List(_) => array.as_list::<i32>().value(row),
                DataType::LargeList(_) => array.as_list::<i64>().value(row),
                _ => array.as_fixed_size_list().value(row),
            };
            writer.write_all(b"[")?;
            for row in 0..items.len() {
                if row != 0 {
                    writer.write_all(b",")?;
                }
                value(writer, item, items.as_ref(), row)?;
            }
            writer.write_all(b"]")
        }
        DataType::Map(entries, _) => {
            let DataType::Struct(fields) = entries.data_type() else {
                return Err(invalid("map entry contract"));
            };
            let entries = array.as_map().value(row);
            let keys = entries.column(0).as_string::<i32>();
            writer.write_all(b"{")?;
            let mut first = true;
            for row in 0..entries.len() {
                if keys.is_null(row) {
                    return Err(invalid("null JSON map key"));
                }
                member(
                    writer,
                    &mut first,
                    keys.value(row),
                    &fields[1],
                    entries.column(1).as_ref(),
                    row,
                )?;
            }
            writer.write_all(b"}")
        }
        DataType::Decimal32(_, _)
        | DataType::Decimal64(_, _)
        | DataType::Decimal128(_, _)
        | DataType::Decimal256(_, _) => {
            let options = arrow::util::display::FormatOptions::default();
            let formatted =
                arrow::util::display::ArrayFormatter::try_new(array, &options).map_err(invalid)?;
            quoted(writer, &formatted.value(row).to_string())
        }
        DataType::UInt64 => quoted(
            writer,
            &array
                .as_primitive::<arrow::datatypes::UInt64Type>()
                .value(row)
                .to_string(),
        ),
        DataType::Int64 => quoted(
            writer,
            &array
                .as_primitive::<arrow::datatypes::Int64Type>()
                .value(row)
                .to_string(),
        ),
        DataType::Boolean
        | DataType::UInt8
        | DataType::UInt16
        | DataType::UInt32
        | DataType::Int8
        | DataType::Int16
        | DataType::Int32 => {
            // These encoders have a fixed upper bound; no value-sized allocation is possible.
            let options = arrow::json::writer::EncoderOptions::default();
            let mut encoder =
                arrow::json::writer::make_encoder(field, array, &options).map_err(invalid)?;
            let mut bytes = Vec::with_capacity(24);
            encoder.encode(row, &mut bytes);
            writer.write_all(&bytes)
        }
        other => Err(invalid(format!(
            "undeclared native JSON representation: {other}"
        ))),
    }
}

/// A top-level record field retains the complete child declarations.
pub fn record_field(fields: Fields) -> FieldRef {
    Arc::new(Field::new("result", DataType::Struct(fields), false))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        evidence::relational::Locator,
        native_union::{Cell, NativeUnion},
    };

    fn encode(field: FieldRef, array: &dyn Array, row: usize) -> Vec<u8> {
        let mut bytes = Vec::new();
        write_value(&mut bytes, 8192, &field, array, row).unwrap();
        bytes
    }

    #[test]
    fn typed_unions_maps_lists_and_clocks_share_the_declared_wire_shape() {
        let locators = [
            Locator::RegistryLine { line: 7 },
            Locator::Artifact,
            Locator::ManifestTable {
                file: "Cargo.toml".into(),
                table: "package".into(),
            },
        ];
        let values =
            <Locator as NativeUnion>::encode(&locators.iter().collect::<Vec<_>>()).unwrap();
        let field = record_field(<Locator as NativeUnion>::fields());
        for (row, locator) in locators.iter().enumerate() {
            let encoded: serde_json::Value =
                serde_json::from_slice(&encode(field.clone(), values.as_ref(), row)).unwrap();
            assert_eq!(encoded, serde_json::to_value(locator).unwrap());
        }
        let time = crate::native_time::EventTime::from_micros(-1).unwrap();
        let values = crate::native_time::EventTime::encode(&[Some(&time)]).unwrap();
        let field = Arc::new(crate::native_union::field::<crate::native_time::EventTime>(
            "time",
            crate::native_union::Rule::Text,
        ));
        assert_eq!(
            encode(field, values.as_ref(), 0),
            br#""1969-12-31T23:59:59.999999Z""#
        );

        let map = std::collections::BTreeMap::from([
            ("empty".to_owned(), Some(Vec::<String>::new())),
            ("unknown".to_owned(), None),
        ]);
        let values =
            <std::collections::BTreeMap<String, Option<Vec<String>>>>::encode(&[Some(&map)])
                .unwrap();
        let field = Arc::new(Field::new("map", values.data_type().clone(), false));
        assert_eq!(
            encode(field, values.as_ref(), 0),
            br#"{"empty":[],"unknown":null}"#
        );
        let request = crate::request::ResearchRequest::ServiceStatus(Default::default());
        let values = <crate::request::ResearchRequest as NativeUnion>::encode(&[&request]).unwrap();
        let field = record_field(<crate::request::ResearchRequest as NativeUnion>::fields());
        assert_eq!(
            serde_json::from_slice::<serde_json::Value>(&encode(field, values.as_ref(), 0))
                .unwrap(),
            serde_json::json!({"method":"service_status","params":{"component":null}})
        );
    }

    #[test]
    fn exact_numeric_binary_domains_and_escaped_byte_bounds() {
        use crate::native_types::{IdentityType, TypeMetadata};
        use arrow::array::{
            BinaryArray, Decimal128Array, FixedSizeBinaryArray, StringArray, UInt64Array,
        };
        let unsigned = UInt64Array::from(vec![u64::MAX]);
        assert_eq!(
            encode(
                Arc::new(Field::new("n", DataType::UInt64, false)),
                &unsigned,
                0
            ),
            br#""18446744073709551615""#
        );
        let decimal = Decimal128Array::from(vec![12345678901234567890123456789012345678_i128])
            .with_precision_and_scale(38, 7)
            .unwrap();
        assert_eq!(
            encode(
                Arc::new(Field::new("decimal", decimal.data_type().clone(), false)),
                &decimal,
                0
            ),
            br#""1234567890123456789012345678901.2345678""#
        );
        let binary = BinaryArray::from_vec(vec![&[0_u8, 255, 195, 40]]);
        assert_eq!(
            encode(
                Arc::new(Field::new("source", DataType::Binary, false)),
                &binary,
                0
            ),
            br#""00ffc328""#
        );
        let ids = FixedSizeBinaryArray::try_from_iter([[255_u8; 32]].into_iter()).unwrap();
        let field = Field::new("id", ids.data_type().clone(), false).with_extension_type(
            IdentityType::try_new(
                ids.data_type(),
                TypeMetadata::new(crate::native_union::Domain::Symbol),
            )
            .unwrap(),
        );
        assert_eq!(
            String::from_utf8(encode(Arc::new(field), &ids, 0)).unwrap(),
            format!("\"symbol_{}\"", "ff".repeat(32))
        );
        let text = format!("é😀\\\"{}", "\u{0001}".repeat(10000));
        let strings = StringArray::from(vec![text.as_str()]);
        let field = Arc::new(Field::new("text", DataType::Utf8, false));
        let exact = serde_json::to_vec(&text).unwrap();
        let mut bytes = Vec::new();
        assert_eq!(
            write_value(&mut bytes, exact.len(), &field, &strings, 0).unwrap(),
            exact.len()
        );
        assert_eq!(bytes, exact);
        bytes.clear();
        let error = write_value(&mut bytes, 31, &field, &strings, 0).unwrap_err();
        assert_eq!(error.kind(), io::ErrorKind::OutOfMemory);
        assert!(bytes.len() <= 31);
    }
}
