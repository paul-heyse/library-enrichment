//! Full-field Arrow collection projections. The pinned `map_entries` kernel recreates
//! key/value fields without metadata; reusing the native map buffers preserves their contract.
use arrow::{
    array::{Array, ListArray},
    datatypes::{DataType, Field, FieldRef},
};
use datafusion::{
    common::{Result, cast::as_map_array, utils::take_function_args},
    logical_expr::{
        ColumnarValue, ReturnFieldArgs, ScalarFunctionArgs, ScalarUDF, ScalarUDFImpl, Signature,
        Volatility,
    },
};
use std::sync::Arc;

mod projection;
pub use projection::{decoded_field, decoded_values, union_member};

pub fn map_entries() -> ScalarUDF {
    ScalarUDF::from(MapEntries {
        signature: Signature::any(1, Volatility::Immutable),
    })
}

pub(crate) fn is_projection(function: &ScalarUDF) -> bool {
    function.inner().downcast_ref::<MapEntries>().is_some()
        || function.inner().downcast_ref::<ListEntries>().is_some()
        || function.inner().downcast_ref::<ListValues>().is_some()
        || projection::is_projection(function)
}

/// The inverse of `list_entries`, used after native aggregation of a one-field Struct.
/// ArrayAgg retains Struct children but reconstructs a scalar item's Field at this pin.
/// The wrapper preserves the scalar domain without a replacement aggregate implementation.
pub fn list_values() -> ScalarUDF {
    ScalarUDF::from(ListValues {
        signature: Signature::user_defined(Volatility::Immutable),
    })
}
#[derive(Debug, PartialEq, Eq, Hash)]
struct ListValues {
    signature: Signature,
}
impl ScalarUDFImpl for ListValues {
    fn name(&self) -> &str {
        "native_list_values"
    }
    fn signature(&self) -> &Signature {
        &self.signature
    }
    fn return_type(&self, _: &[DataType]) -> Result<DataType> {
        datafusion::common::plan_err!("list values requires full fields")
    }
    fn coerce_types(&self, types: &[DataType]) -> Result<Vec<DataType>> {
        if !matches!(types, [DataType::List(_)]) {
            return datafusion::common::plan_err!("list values requires List");
        }
        Ok(types.to_vec())
    }
    fn return_field_from_args(&self, args: ReturnFieldArgs) -> Result<FieldRef> {
        let [field] = take_function_args(self.name(), args.arg_fields)?;
        let DataType::List(entry) = field.data_type() else {
            return datafusion::common::plan_err!("list values requires List");
        };
        let DataType::Struct(fields) = entry.data_type() else {
            return datafusion::common::plan_err!("list values requires Struct entries");
        };
        let [value] = fields.as_ref() else {
            return datafusion::common::plan_err!("list values requires exactly one child");
        };
        if value.name() != "value" {
            return datafusion::common::plan_err!("list values requires the declared value child");
        }
        crate::native_analysis::validate_derived_fields(fields)?;
        Ok(Arc::new(Field::new(
            self.name(),
            DataType::List(Arc::new(value.as_ref().clone().with_name("item"))),
            field.is_nullable(),
        )))
    }
    fn invoke_with_args(&self, args: ScalarFunctionArgs) -> Result<ColumnarValue> {
        crate::native_schema::function_call(self, &args)?;
        let [value] = take_function_args(self.name(), &args.args)?;
        let array = value.clone().into_array(args.number_rows)?;
        let list = datafusion::common::cast::as_list_array(&array)?;
        let entries = datafusion::common::cast::as_struct_array(list.values())?;
        if entries.null_count() != 0 {
            return datafusion::common::exec_err!("list values cannot erase absent entry structs");
        }
        let DataType::List(field) = args.return_field.data_type() else {
            return datafusion::common::internal_err!("list values output shape");
        };
        Ok(ColumnarValue::Array(Arc::new(ListArray::try_new(
            field.clone(),
            list.offsets().clone(),
            entries.column(0).clone(),
            list.nulls().cloned(),
        )?)))
    }
}

/// Wrap list values in a Struct without copying their value buffers. DataFusion's
/// UNNEST preserves Struct child fields; a scalar list item otherwise loses its
/// extension metadata at this pin. Selection and expansion remain native UNNEST.
pub fn list_entries() -> ScalarUDF {
    ScalarUDF::from(ListEntries {
        signature: Signature::user_defined(Volatility::Immutable),
    })
}
#[derive(Debug, PartialEq, Eq, Hash)]
struct ListEntries {
    signature: Signature,
}
impl ScalarUDFImpl for ListEntries {
    fn name(&self) -> &str {
        "native_list_entries"
    }
    fn signature(&self) -> &Signature {
        &self.signature
    }
    fn return_type(&self, _: &[DataType]) -> Result<DataType> {
        datafusion::common::plan_err!("list entries requires full fields")
    }
    fn coerce_types(&self, types: &[DataType]) -> Result<Vec<DataType>> {
        if !matches!(
            types,
            [DataType::List(_) | DataType::LargeList(_) | DataType::FixedSizeList(_, _)]
        ) {
            return datafusion::common::plan_err!("list entries requires a native list");
        }
        Ok(types.to_vec())
    }
    fn return_field_from_args(&self, args: ReturnFieldArgs) -> Result<FieldRef> {
        let [field] = take_function_args(self.name(), args.arg_fields)?;
        let wrap = |item: &FieldRef| {
            Arc::new(Field::new(
                "entries",
                DataType::Struct(vec![Arc::new(item.as_ref().clone().with_name("value"))].into()),
                false,
            ))
        };
        let kind = match field.data_type() {
            DataType::List(item) => DataType::List(wrap(item)),
            DataType::LargeList(item) => DataType::LargeList(wrap(item)),
            DataType::FixedSizeList(item, size) => DataType::FixedSizeList(wrap(item), *size),
            _ => return datafusion::common::plan_err!("list entries requires a native list"),
        };
        crate::native_types::validate_field(field)?;
        Ok(Arc::new(Field::new(self.name(), kind, field.is_nullable())))
    }
    fn invoke_with_args(&self, args: ScalarFunctionArgs) -> Result<ColumnarValue> {
        use arrow::array::{ArrayRef, FixedSizeListArray, LargeListArray, StructArray};
        crate::native_schema::function_call(self, &args)?;
        let [value] = take_function_args(self.name(), &args.args)?;
        let array = value.clone().into_array(args.number_rows)?;
        let wrap = |entry: &FieldRef, values: ArrayRef| -> Result<ArrayRef> {
            let DataType::Struct(fields) = entry.data_type() else {
                return datafusion::common::internal_err!("list entry is not Struct");
            };
            Ok(Arc::new(StructArray::try_new(
                fields.clone(),
                vec![values],
                None,
            )?))
        };
        let output: ArrayRef = match args.return_field.data_type() {
            DataType::List(entry) => {
                let list = datafusion::common::cast::as_list_array(&array)?;
                Arc::new(ListArray::try_new(
                    entry.clone(),
                    list.offsets().clone(),
                    wrap(entry, list.values().clone())?,
                    list.nulls().cloned(),
                )?)
            }
            DataType::LargeList(entry) => {
                let list = datafusion::common::cast::as_large_list_array(&array)?;
                Arc::new(LargeListArray::try_new(
                    entry.clone(),
                    list.offsets().clone(),
                    wrap(entry, list.values().clone())?,
                    list.nulls().cloned(),
                )?)
            }
            DataType::FixedSizeList(entry, size) => {
                let list = datafusion::common::cast::as_fixed_size_list_array(&array)?;
                Arc::new(FixedSizeListArray::try_new(
                    entry.clone(),
                    *size,
                    wrap(entry, list.values().clone())?,
                    list.nulls().cloned(),
                )?)
            }
            _ => return datafusion::common::internal_err!("list entries output shape"),
        };
        Ok(ColumnarValue::Array(output))
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct MapEntries {
    signature: Signature,
}

impl ScalarUDFImpl for MapEntries {
    fn name(&self) -> &str {
        "native_map_entries"
    }
    fn signature(&self) -> &Signature {
        &self.signature
    }
    fn return_type(&self, _: &[DataType]) -> Result<DataType> {
        datafusion::common::internal_err!("native map entries requires full fields")
    }
    fn return_field_from_args(&self, args: ReturnFieldArgs) -> Result<FieldRef> {
        let [field] = take_function_args(self.name(), args.arg_fields)?;
        let DataType::Map(entries, _) = field.data_type() else {
            return datafusion::common::plan_err!("native map entries requires Map");
        };
        Ok(Arc::new(Field::new(
            self.name(),
            DataType::List(entries.clone()),
            field.is_nullable(),
        )))
    }
    fn invoke_with_args(&self, args: ScalarFunctionArgs) -> Result<ColumnarValue> {
        crate::native_schema::function_call(self, &args)?;
        let [value] = take_function_args(self.name(), &args.args)?;
        let array = value.clone().into_array(args.number_rows)?;
        let map = as_map_array(&array)?;
        let DataType::List(field) = args.return_field.data_type() else {
            return datafusion::common::internal_err!("native map entries output shape");
        };
        Ok(ColumnarValue::Array(Arc::new(ListArray::try_new(
            field.clone(),
            map.offsets().clone(),
            Arc::new(map.entries().clone()),
            map.nulls().cloned(),
        )?)))
    }
}
