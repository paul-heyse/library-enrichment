use std::collections::{BTreeMap, HashMap};
use std::sync::Arc;

use arrow::array::{
    Array, ArrayRef, LargeStringArray, ListBuilder, StringArray, StringBuilder, StringViewArray,
    StructArray,
};
use arrow::buffer::NullBuffer;
use arrow::datatypes::{DataType, Field, Schema};
use arrow::error::ArrowError;
use arrow::record_batch::RecordBatch;

pub fn invalid(message: impl Into<String>) -> ArrowError {
    ArrowError::InvalidArgumentError(message.into())
}

enum CellColumn {
    Text(TextColumn),
    Bool(arrow::array::BooleanArray),
    U16(arrow::array::UInt16Array),
    U32(arrow::array::UInt32Array),
    U64(arrow::array::UInt64Array),
    I64(arrow::array::Int64Array),
    Timestamp(arrow::array::TimestampMicrosecondArray),
    I32(arrow::array::Int32Array),
    List(arrow::array::ListArray, TextColumn),
    Values(arrow::array::ListArray),
    Map(arrow::array::MapArray),
    Records(arrow::array::ListArray, RowSet),
    Struct(StructArray, RowSet),
}

impl CellColumn {
    fn new(array: &dyn Array) -> Result<Self, ArrowError> {
        use arrow::array::{BooleanArray, ListArray, UInt32Array, UInt64Array};
        if let Some(a) = array.as_any().downcast_ref::<BooleanArray>() {
            Ok(Self::Bool(a.clone()))
        } else if let Some(a) = array.as_any().downcast_ref::<arrow::array::UInt16Array>() {
            Ok(Self::U16(a.clone()))
        } else if let Some(a) = array.as_any().downcast_ref::<UInt32Array>() {
            Ok(Self::U32(a.clone()))
        } else if let Some(a) = array.as_any().downcast_ref::<UInt64Array>() {
            Ok(Self::U64(a.clone()))
        } else if let Some(a) = array
            .as_any()
            .downcast_ref::<arrow::array::TimestampMicrosecondArray>()
        {
            Ok(Self::Timestamp(a.clone()))
        } else if let Some(a) = array.as_any().downcast_ref::<arrow::array::Int64Array>() {
            Ok(Self::I64(a.clone()))
        } else if let Some(a) = array.as_any().downcast_ref::<arrow::array::Int32Array>() {
            Ok(Self::I32(a.clone()))
        } else if let Some(a) = array.as_any().downcast_ref::<ListArray>() {
            if let Some(values) = a.values().as_any().downcast_ref::<StructArray>() {
                Ok(Self::Records(
                    a.clone(),
                    RowSet::new(values.fields(), values.columns())?,
                ))
            } else if let Ok(text) = TextColumn::new(a.values().as_ref()) {
                Ok(Self::List(a.clone(), text))
            } else {
                Ok(Self::Values(a.clone()))
            }
        } else if let Some(a) = array.as_any().downcast_ref::<arrow::array::MapArray>() {
            Ok(Self::Map(a.clone()))
        } else if let Some(a) = array.as_any().downcast_ref::<StructArray>() {
            Ok(Self::Struct(
                a.clone(),
                RowSet::new(a.fields(), a.columns())?,
            ))
        } else {
            Ok(Self::Text(TextColumn::new(array)?))
        }
    }

    fn is_null(&self, row: usize) -> bool {
        match self {
            Self::Text(a) => a.get(row).is_none(),
            Self::Bool(a) => a.is_null(row),
            Self::U16(a) => a.is_null(row),
            Self::U32(a) => a.is_null(row),
            Self::U64(a) => a.is_null(row),
            Self::I64(a) => a.is_null(row),
            Self::Timestamp(a) => a.is_null(row),
            Self::I32(a) => a.is_null(row),
            Self::List(a, _) => a.is_null(row),
            Self::Values(a) => a.is_null(row),
            Self::Map(a) => a.is_null(row),
            Self::Records(a, _) => a.is_null(row),
            Self::Struct(a, _) => a.is_null(row),
        }
    }
}

/// Physical accessors are resolved once, not repeatedly downcast for every decoded row.
pub struct RowSet(BTreeMap<String, CellColumn>);

impl RowSet {
    fn new(fields: &arrow::datatypes::Fields, columns: &[ArrayRef]) -> Result<Self, ArrowError> {
        let mut out = BTreeMap::new();
        for (field, array) in fields.iter().zip(columns) {
            if out
                .insert(field.name().clone(), CellColumn::new(array.as_ref())?)
                .is_some()
            {
                return Err(invalid("duplicate column name"));
            }
        }
        Ok(Self(out))
    }

    pub fn batch(batch: &RecordBatch) -> Result<Self, ArrowError> {
        Self::new(batch.schema().fields(), batch.columns())
    }

    pub fn row(&self, index: usize) -> Row<'_> {
        Row { set: self, index }
    }
}

#[derive(Clone, Copy)]
pub struct Row<'a> {
    set: &'a RowSet,
    index: usize,
}

impl<'a> Row<'a> {
    pub fn is_null(self, name: &str) -> Result<bool, ArrowError> {
        Ok(self.column(name)?.is_null(self.index))
    }

    pub fn exact_fields(self, fields: &arrow::datatypes::Fields) -> Result<(), ArrowError> {
        if self.set.0.len() != fields.len()
            || fields
                .iter()
                .any(|field| !self.set.0.contains_key(field.name()))
        {
            return Err(invalid("record fields differ from declared contract"));
        }
        Ok(())
    }
    fn column(self, name: &str) -> Result<&'a CellColumn, ArrowError> {
        self.set
            .0
            .get(name)
            .ok_or_else(|| invalid(format!("missing column {name}")))
    }

    pub fn optional_text(self, name: &str) -> Result<Option<&'a str>, ArrowError> {
        match self.column(name)? {
            CellColumn::Text(c) => Ok(c.get(self.index)),
            _ => Err(invalid(format!("non-text column {name}"))),
        }
    }

    pub fn text(self, name: &str) -> Result<&'a str, ArrowError> {
        self.optional_text(name)?
            .ok_or_else(|| invalid(format!("null required {name}")))
    }

    pub fn owned(self, name: &str) -> Result<Option<String>, ArrowError> {
        Ok(self.optional_text(name)?.map(str::to_owned))
    }

    pub fn optional_bool(self, name: &str) -> Result<Option<bool>, ArrowError> {
        match self.column(name)? {
            CellColumn::Bool(c) => Ok((!c.is_null(self.index)).then(|| c.value(self.index))),
            _ => Err(invalid(format!("non-boolean column {name}"))),
        }
    }

    pub fn boolean(self, name: &str) -> Result<bool, ArrowError> {
        self.optional_bool(name)?
            .ok_or_else(|| invalid(format!("null required {name}")))
    }

    pub fn number(self, name: &str) -> Result<u64, ArrowError> {
        match self.column(name)? {
            CellColumn::U16(c) if !c.is_null(self.index) => Ok(u64::from(c.value(self.index))),
            CellColumn::U32(c) if !c.is_null(self.index) => Ok(u64::from(c.value(self.index))),
            CellColumn::U64(c) if !c.is_null(self.index) => Ok(c.value(self.index)),
            CellColumn::I64(c) if !c.is_null(self.index) => u64::try_from(c.value(self.index))
                .map_err(|_| invalid(format!("negative count in {name}"))),
            _ => Err(invalid(format!("null or non-integer column {name}"))),
        }
    }

    pub fn optional_number(self, name: &str) -> Result<Option<u64>, ArrowError> {
        if self.column(name)?.is_null(self.index) {
            Ok(None)
        } else {
            self.number(name).map(Some)
        }
    }

    pub fn optional_signed(self, name: &str) -> Result<Option<i64>, ArrowError> {
        if self.column(name)?.is_null(self.index) {
            Ok(None)
        } else {
            self.signed(name).map(Some)
        }
    }
    pub fn timestamp_micros(self, name: &str) -> Result<i64, ArrowError> {
        match self.column(name)? {
            CellColumn::Timestamp(array) if !array.is_null(self.index) => {
                Ok(array.value(self.index))
            }
            _ => Err(invalid("required UTC microsecond value")),
        }
    }

    pub fn signed(self, name: &str) -> Result<i64, ArrowError> {
        match self.column(name)? {
            CellColumn::I64(c) if !c.is_null(self.index) => Ok(c.value(self.index)),
            CellColumn::I32(c) if !c.is_null(self.index) => Ok(i64::from(c.value(self.index))),
            _ => Err(invalid(format!("null or non-signed-integer column {name}"))),
        }
    }

    pub fn list_values(self, name: &str) -> Result<ArrayRef, ArrowError> {
        match self.column(name)? {
            CellColumn::List(array, _)
            | CellColumn::Records(array, _)
            | CellColumn::Values(array)
                if !array.is_null(self.index) =>
            {
                Ok(array.value(self.index))
            }
            _ => Err(invalid(format!("null or non-list column {name}"))),
        }
    }

    pub fn map_entries(self, name: &str) -> Result<StructArray, ArrowError> {
        match self.column(name)? {
            CellColumn::Map(array) if !array.is_null(self.index) => Ok(array.value(self.index)),
            _ => Err(invalid(format!("null or non-map column {name}"))),
        }
    }

    pub fn list(self, name: &str) -> Result<Vec<String>, ArrowError> {
        match self.column(name)? {
            CellColumn::List(c, values) if !c.is_null(self.index) => {
                let offsets = c.value_offsets();
                let start =
                    usize::try_from(offsets[self.index]).map_err(|e| invalid(e.to_string()))?;
                let end =
                    usize::try_from(offsets[self.index + 1]).map_err(|e| invalid(e.to_string()))?;
                (start..end)
                    .map(|i| values.required(i).map(str::to_owned))
                    .collect()
            }
            _ => Err(invalid(format!("null or non-list column {name}"))),
        }
    }

    pub fn optional_struct(self, name: &str) -> Result<Option<Self>, ArrowError> {
        match self.column(name)? {
            CellColumn::Struct(c, set) => Ok((!c.is_null(self.index)).then(|| set.row(self.index))),
            _ => Err(invalid(format!("non-struct column {name}"))),
        }
    }

    pub fn records(self, name: &str) -> Result<Vec<Self>, ArrowError> {
        match self.column(name)? {
            CellColumn::Records(c, rows) if !c.is_null(self.index) => {
                let offsets = c.value_offsets();
                let start =
                    usize::try_from(offsets[self.index]).map_err(|e| invalid(e.to_string()))?;
                let end =
                    usize::try_from(offsets[self.index + 1]).map_err(|e| invalid(e.to_string()))?;
                if (start..end).any(|i| c.values().is_null(i)) {
                    return Err(invalid("null required record in list"));
                }
                Ok((start..end).map(|i| rows.row(i)).collect())
            }
            _ => Err(invalid(format!("null or non-record-list column {name}"))),
        }
    }

    pub fn structure(self, name: &str) -> Result<Self, ArrowError> {
        self.optional_struct(name)?
            .ok_or_else(|| invalid(format!("null required {name}")))
    }

    /// A tagged variant may populate only its declared fields, even if the extra field is
    /// nullable in the union schema. This catches malformed alternatives before indexing.
    pub fn variant(self, allowed: &[&str]) -> Result<(), ArrowError> {
        self.variant_named("kind", allowed)
    }
    pub fn variant_named(self, discriminator: &str, allowed: &[&str]) -> Result<(), ArrowError> {
        for (name, column) in &self.set.0 {
            if name != discriminator
                && !allowed.contains(&name.as_str())
                && !column.is_null(self.index)
            {
                return Err(invalid(format!(
                    "unexpected {name} for {}",
                    self.text(discriminator)?
                )));
            }
        }
        Ok(())
    }
}

/// Resolve a supported text representation once per column, including dictionary values.
/// Clones share Arrow buffers; dictionary decoding happens once at the batch boundary.
pub enum TextColumn {
    Utf8(StringArray),
    Large(LargeStringArray),
    View(StringViewArray),
}

impl TextColumn {
    /// Resolve text without assuming DataFusion's physical string representation.
    ///
    /// # Errors
    /// Rejects non-text values and malformed dictionary casts.
    pub fn new(array: &dyn Array) -> Result<Self, ArrowError> {
        if let Some(a) = array.as_any().downcast_ref::<StringArray>() {
            Ok(Self::Utf8(a.clone()))
        } else if let Some(a) = array.as_any().downcast_ref::<LargeStringArray>() {
            Ok(Self::Large(a.clone()))
        } else if let Some(a) = array.as_any().downcast_ref::<StringViewArray>() {
            Ok(Self::View(a.clone()))
        } else if let DataType::Dictionary(_, value) = array.data_type()
            && matches!(
                value.as_ref(),
                DataType::Utf8 | DataType::LargeUtf8 | DataType::Utf8View
            )
        {
            Self::new(arrow::compute::cast(array, &DataType::Utf8)?.as_ref())
        } else {
            Err(ArrowError::SchemaError(format!(
                "expected text, found {}",
                array.data_type()
            )))
        }
    }

    /// Borrow a cell; a missing value is not an empty string.
    #[must_use]
    pub fn get(&self, row: usize) -> Option<&str> {
        match self {
            Self::Utf8(a) => (!a.is_null(row)).then(|| a.value(row)),
            Self::Large(a) => (!a.is_null(row)).then(|| a.value(row)),
            Self::View(a) => (!a.is_null(row)).then(|| a.value(row)),
        }
    }

    /// Read a required cell without inventing a default for corrupt evidence.
    ///
    /// # Errors
    /// Null required values are rejected.
    pub fn required(&self, row: usize) -> Result<&str, ArrowError> {
        self.get(row)
            .ok_or_else(|| ArrowError::InvalidArgumentError("null required text".into()))
    }
}

pub fn text<'a>(values: impl IntoIterator<Item = &'a str>) -> ArrayRef {
    let values: Vec<_> = values.into_iter().collect();
    let mut builder =
        StringBuilder::with_capacity(values.len(), values.iter().map(|v| v.len()).sum());
    for value in values {
        builder.append_value(value);
    }
    Arc::new(builder.finish())
}

pub fn optional<'a>(values: impl IntoIterator<Item = Option<&'a str>>) -> ArrayRef {
    let values: Vec<_> = values.into_iter().collect();
    let mut builder =
        StringBuilder::with_capacity(values.len(), values.iter().flatten().map(|v| v.len()).sum());
    for value in values {
        builder.append_option(value);
    }
    Arc::new(builder.finish())
}

pub fn list<'a>(values: impl IntoIterator<Item = &'a [String]>) -> ArrayRef {
    let values: Vec<_> = values.into_iter().collect();
    let items = values.iter().map(|v| v.len()).sum();
    let bytes = values.iter().flat_map(|v| v.iter()).map(String::len).sum();
    let mut builder =
        ListBuilder::with_capacity(StringBuilder::with_capacity(items, bytes), values.len())
            .with_field(Field::new("item", DataType::Utf8, false));
    for values in values {
        for value in values {
            builder.values().append_value(value);
        }
        builder.append(true);
    }
    Arc::new(builder.finish())
}

pub fn record_list(
    lengths: impl IntoIterator<Item = usize>,
    values: ArrayRef,
) -> Result<ArrayRef, ArrowError> {
    let mut offsets = vec![0i32];
    let mut total = 0i32;
    for length in lengths {
        total = total
            .checked_add(i32::try_from(length).map_err(|e| invalid(e.to_string()))?)
            .ok_or_else(|| invalid("list offset overflow"))?;
        offsets.push(total);
    }
    Ok(Arc::new(arrow::array::ListArray::try_new(
        Arc::new(Field::new("item", values.data_type().clone(), false)),
        arrow::buffer::OffsetBuffer::new(offsets.into()),
        values,
        None,
    )?))
}

pub fn field(name: &str, array: &ArrayRef, nullable: bool, role: &str) -> Field {
    Field::new(name, array.data_type().clone(), nullable).with_metadata(HashMap::from([
        ("enrichment.role".into(), role.into()),
        ("enrichment.contract".into(), super::VERSION.into()),
        (
            "enrichment.null".into(),
            if nullable { "unobserved" } else { "forbidden" }.into(),
        ),
    ]))
}

pub fn column(name: &str, array: ArrayRef, nullable: bool, role: &str) -> (Field, ArrayRef) {
    (field(name, &array, nullable, role), array)
}

pub fn ruled_column(
    name: &str,
    array: ArrayRef,
    nullable: bool,
    role: &str,
    rule: crate::native_union::Rule,
) -> (Field, ArrayRef) {
    let (mut field, array) = column(name, array, nullable, role);
    field.metadata_mut().insert(
        "enrichment.rule".into(),
        serde_json::to_string(&rule).expect("finite field rule"),
    );
    (field, array)
}

pub fn structure(
    columns: Vec<(Field, ArrayRef)>,
    valid: Option<Vec<bool>>,
) -> Result<ArrayRef, ArrowError> {
    let (fields, arrays): (Vec<_>, Vec<_>) = columns.into_iter().unzip();
    let length = arrays
        .first()
        .map(|array| array.len())
        .or_else(|| valid.as_ref().map(Vec::len))
        .unwrap_or(0);
    Ok(Arc::new(StructArray::try_new_with_length(
        fields.into(),
        arrays,
        valid.map(NullBuffer::from),
        length,
    )?))
}

pub fn batch(table: &str, columns: Vec<(Field, ArrayRef)>) -> Result<RecordBatch, ArrowError> {
    // Delta's native Parquet reader exposes nullable nested fields and list items. Keep
    // that read layout in Arrow, and retain required semantic membership as an enforced
    // field contract. Native expressions can then cross the scan boundary without a
    // nullable-to-required collection cast inside Parquet predicate evaluation.
    let columns = columns
        .into_iter()
        .map(|(field, array)| {
            let field = native_read_field(&field, false);
            Ok((
                field.clone(),
                arrow::compute::cast(array.as_ref(), field.data_type())?,
            ))
        })
        .collect::<Result<Vec<_>, ArrowError>>()?;
    let (fields, arrays): (Vec<_>, Vec<_>) = columns.into_iter().unzip();
    let schema = Schema::new_with_metadata(
        fields,
        HashMap::from([
            ("enrichment.contract".into(), super::VERSION.into()),
            ("enrichment.relation".into(), table.into()),
        ]),
    );
    RecordBatch::try_new(Arc::new(schema), arrays)
}

/// Delta's nullable nested read layout, retaining required values as semantic field metadata.
pub fn native_read_field(field: &Field, nested: bool) -> Field {
    let datatype = match field.data_type() {
        DataType::Struct(fields) => DataType::Struct(
            fields
                .iter()
                .map(|field| native_read_field(field, true))
                .collect(),
        ),
        DataType::List(item) => DataType::List(Arc::new(native_read_field(item, true))),
        DataType::LargeList(item) => DataType::LargeList(Arc::new(native_read_field(item, true))),
        other => other.clone(),
    };
    let mut metadata = field.metadata().clone();
    metadata.entry("enrichment.null".into()).or_insert_with(|| {
        if field.is_nullable() {
            "unobserved"
        } else {
            "forbidden"
        }
        .into()
    });
    field
        .clone()
        .with_data_type(datatype)
        .with_nullable(nested || field.is_nullable())
        .with_metadata(metadata)
}
