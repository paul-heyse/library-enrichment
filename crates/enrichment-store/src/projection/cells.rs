use std::collections::{BTreeMap, HashMap};
use std::sync::Arc;

use arrow::array::{
    Array, ArrayRef, LargeStringArray, ListBuilder, StringArray, StringBuilder, StringViewArray,
    StructArray,
};
use arrow::buffer::NullBuffer;
use arrow::error::ArrowError;
use arrow::record_batch::RecordBatch;
use arrow_schema::{DataType, Field, Schema};

pub(super) fn invalid(message: impl Into<String>) -> ArrowError {
    ArrowError::InvalidArgumentError(message.into())
}

enum CellColumn {
    Text(TextColumn),
    Bool(arrow::array::BooleanArray),
    U32(arrow::array::UInt32Array),
    U64(arrow::array::UInt64Array),
    I64(arrow::array::Int64Array),
    List(arrow::array::ListArray, TextColumn),
    Records(arrow::array::ListArray, RowSet),
    Struct(StructArray, RowSet),
}

impl CellColumn {
    fn new(array: &dyn Array) -> Result<Self, ArrowError> {
        use arrow::array::{BooleanArray, ListArray, UInt32Array, UInt64Array};
        if let Some(a) = array.as_any().downcast_ref::<BooleanArray>() {
            Ok(Self::Bool(a.clone()))
        } else if let Some(a) = array.as_any().downcast_ref::<UInt32Array>() {
            Ok(Self::U32(a.clone()))
        } else if let Some(a) = array.as_any().downcast_ref::<UInt64Array>() {
            Ok(Self::U64(a.clone()))
        } else if let Some(a) = array.as_any().downcast_ref::<arrow::array::Int64Array>() {
            Ok(Self::I64(a.clone()))
        } else if let Some(a) = array.as_any().downcast_ref::<ListArray>() {
            if let Some(values) = a.values().as_any().downcast_ref::<StructArray>() {
                Ok(Self::Records(
                    a.clone(),
                    RowSet::new(values.fields(), values.columns())?,
                ))
            } else {
                Ok(Self::List(a.clone(), TextColumn::new(a.values().as_ref())?))
            }
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
            Self::U32(a) => a.is_null(row),
            Self::U64(a) => a.is_null(row),
            Self::I64(a) => a.is_null(row),
            Self::List(a, _) => a.is_null(row),
            Self::Records(a, _) => a.is_null(row),
            Self::Struct(a, _) => a.is_null(row),
        }
    }
}

/// Physical accessors are resolved once, not repeatedly downcast for every decoded row.
pub(super) struct RowSet(BTreeMap<String, CellColumn>);

impl RowSet {
    fn new(fields: &arrow_schema::Fields, columns: &[ArrayRef]) -> Result<Self, ArrowError> {
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

    pub(super) fn batch(batch: &RecordBatch) -> Result<Self, ArrowError> {
        Self::new(batch.schema().fields(), batch.columns())
    }

    pub(super) fn row(&self, index: usize) -> Row<'_> {
        Row { set: self, index }
    }
}

#[derive(Clone, Copy)]
pub(super) struct Row<'a> {
    set: &'a RowSet,
    index: usize,
}

impl<'a> Row<'a> {
    fn column(self, name: &str) -> Result<&'a CellColumn, ArrowError> {
        self.set
            .0
            .get(name)
            .ok_or_else(|| invalid(format!("missing column {name}")))
    }

    pub(super) fn optional_text(self, name: &str) -> Result<Option<&'a str>, ArrowError> {
        match self.column(name)? {
            CellColumn::Text(c) => Ok(c.get(self.index)),
            _ => Err(invalid(format!("non-text column {name}"))),
        }
    }

    pub(super) fn text(self, name: &str) -> Result<&'a str, ArrowError> {
        self.optional_text(name)?
            .ok_or_else(|| invalid(format!("null required {name}")))
    }

    pub(super) fn owned(self, name: &str) -> Result<Option<String>, ArrowError> {
        Ok(self.optional_text(name)?.map(str::to_owned))
    }

    pub(super) fn optional_bool(self, name: &str) -> Result<Option<bool>, ArrowError> {
        match self.column(name)? {
            CellColumn::Bool(c) => Ok((!c.is_null(self.index)).then(|| c.value(self.index))),
            _ => Err(invalid(format!("non-boolean column {name}"))),
        }
    }

    pub(super) fn boolean(self, name: &str) -> Result<bool, ArrowError> {
        self.optional_bool(name)?
            .ok_or_else(|| invalid(format!("null required {name}")))
    }

    pub(super) fn number(self, name: &str) -> Result<u64, ArrowError> {
        match self.column(name)? {
            CellColumn::U32(c) if !c.is_null(self.index) => Ok(u64::from(c.value(self.index))),
            CellColumn::U64(c) if !c.is_null(self.index) => Ok(c.value(self.index)),
            CellColumn::I64(c) if !c.is_null(self.index) => u64::try_from(c.value(self.index))
                .map_err(|_| invalid(format!("negative count in {name}"))),
            _ => Err(invalid(format!("null or non-integer column {name}"))),
        }
    }

    pub(super) fn optional_number(self, name: &str) -> Result<Option<u64>, ArrowError> {
        if self.column(name)?.is_null(self.index) {
            Ok(None)
        } else {
            self.number(name).map(Some)
        }
    }

    pub(super) fn optional_signed(self, name: &str) -> Result<Option<i64>, ArrowError> {
        if self.column(name)?.is_null(self.index) {
            Ok(None)
        } else {
            self.signed(name).map(Some)
        }
    }
    pub(super) fn signed(self, name: &str) -> Result<i64, ArrowError> {
        match self.column(name)? {
            CellColumn::I64(c) if !c.is_null(self.index) => Ok(c.value(self.index)),
            _ => Err(invalid(format!("null or non-signed-integer column {name}"))),
        }
    }

    pub(super) fn list(self, name: &str) -> Result<Vec<String>, ArrowError> {
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

    pub(super) fn optional_struct(self, name: &str) -> Result<Option<Self>, ArrowError> {
        match self.column(name)? {
            CellColumn::Struct(c, set) => Ok((!c.is_null(self.index)).then(|| set.row(self.index))),
            _ => Err(invalid(format!("non-struct column {name}"))),
        }
    }

    pub(super) fn records(self, name: &str) -> Result<Vec<Self>, ArrowError> {
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

    pub(super) fn structure(self, name: &str) -> Result<Self, ArrowError> {
        self.optional_struct(name)?
            .ok_or_else(|| invalid(format!("null required {name}")))
    }

    /// A tagged variant may populate only its declared fields, even if the extra field is
    /// nullable in the union schema. This catches malformed alternatives before indexing.
    pub(super) fn variant(self, allowed: &[&str]) -> Result<(), ArrowError> {
        for (name, column) in &self.set.0 {
            if name != "kind" && !allowed.contains(&name.as_str()) && !column.is_null(self.index) {
                return Err(invalid(format!(
                    "unexpected {name} for {}",
                    self.text("kind")?
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

pub(super) fn text<'a>(values: impl IntoIterator<Item = &'a str>) -> ArrayRef {
    let values: Vec<_> = values.into_iter().collect();
    let mut builder =
        StringBuilder::with_capacity(values.len(), values.iter().map(|v| v.len()).sum());
    for value in values {
        builder.append_value(value);
    }
    Arc::new(builder.finish())
}

pub(super) fn optional<'a>(values: impl IntoIterator<Item = Option<&'a str>>) -> ArrayRef {
    let values: Vec<_> = values.into_iter().collect();
    let mut builder =
        StringBuilder::with_capacity(values.len(), values.iter().flatten().map(|v| v.len()).sum());
    for value in values {
        builder.append_option(value);
    }
    Arc::new(builder.finish())
}

pub(super) fn list<'a>(values: impl IntoIterator<Item = &'a [String]>) -> ArrayRef {
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

pub(super) fn record_list(
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

pub(super) fn field(name: &str, array: &ArrayRef, nullable: bool, role: &str) -> Field {
    Field::new(name, array.data_type().clone(), nullable).with_metadata(HashMap::from([
        ("enrichment.role".into(), role.into()),
        ("enrichment.contract".into(), super::VERSION.into()),
        (
            "enrichment.null".into(),
            if nullable { "unobserved" } else { "forbidden" }.into(),
        ),
    ]))
}

pub(super) fn column(name: &str, array: ArrayRef, nullable: bool, role: &str) -> (Field, ArrayRef) {
    (field(name, &array, nullable, role), array)
}

pub(super) fn structure(
    columns: Vec<(Field, ArrayRef)>,
    valid: Option<Vec<bool>>,
) -> Result<ArrayRef, ArrowError> {
    let (fields, arrays): (Vec<_>, Vec<_>) = columns.into_iter().unzip();
    Ok(Arc::new(StructArray::try_new(
        fields.into(),
        arrays,
        valid.map(NullBuffer::from),
    )?))
}

pub(super) fn batch(
    table: &str,
    columns: Vec<(Field, ArrayRef)>,
) -> Result<RecordBatch, ArrowError> {
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
