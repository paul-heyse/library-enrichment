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

pub fn map_entries() -> ScalarUDF {
    ScalarUDF::from(MapEntries {
        signature: Signature::any(1, Volatility::Immutable),
    })
}

pub(crate) fn is_projection(function: &ScalarUDF) -> bool {
    function.inner().downcast_ref::<MapEntries>().is_some()
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
