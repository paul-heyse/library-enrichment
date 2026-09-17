//! Full-field wrapper for the pinned native named_struct kernel.
//! DataFusion 55.1 copies child datatypes but discards their metadata. This adapter preserves
//! declared fields and uses Arrow's fallible StructArray constructor. Field mapping is absent because the
//! declared metadata must survive optimizer rewrites of constant inputs.
use arrow::datatypes::{DataType, Field, FieldRef, Fields};
use datafusion::{
    common::Result,
    logical_expr::{
        ColumnarValue, Expr, ReturnFieldArgs, ScalarFunctionArgs, ScalarUDF, ScalarUDFImpl,
        Signature, Volatility,
    },
};
use std::sync::Arc;

pub fn record(fields: Fields, args: Vec<Expr>) -> Expr {
    ScalarUDF::from(NativeRecord {
        signature: Signature::variadic_any(Volatility::Immutable),
        fields,
    })
    .call(args)
}

pub(crate) fn is_record(function: &ScalarUDF) -> bool {
    function.inner().downcast_ref::<NativeRecord>().is_some()
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct NativeRecord {
    signature: Signature,
    fields: Fields,
}

impl ScalarUDFImpl for NativeRecord {
    fn name(&self) -> &str {
        "native_named_struct_v1"
    }
    fn signature(&self) -> &Signature {
        &self.signature
    }
    fn return_type(&self, _: &[DataType]) -> Result<DataType> {
        datafusion::common::internal_err!("native record requires full fields")
    }
    fn return_field_from_args(&self, args: ReturnFieldArgs) -> Result<FieldRef> {
        let fields = args.arg_fields;
        let native = datafusion::functions::core::named_struct().return_field_from_args(args)?;
        let DataType::Struct(names) = native.data_type() else {
            return datafusion::common::internal_err!("native named_struct output shape");
        };
        if names
            .iter()
            .map(|f| f.name())
            .ne(self.fields.iter().map(|f| f.name()))
        {
            return datafusion::common::plan_err!("native record field names changed");
        }
        let values: Vec<_> = fields.iter().skip(1).step_by(2).cloned().collect();
        crate::native_schema::function_arguments(&values, &self.fields)?;
        Ok(Arc::new(Field::new(
            self.name(),
            DataType::Struct(self.fields.clone()),
            false,
        )))
    }
    fn invoke_with_args(&self, args: ScalarFunctionArgs) -> Result<ColumnarValue> {
        crate::native_schema::function_call(self, &args)?;
        let options = arrow::compute::CastOptions {
            safe: false,
            ..Default::default()
        };
        let mut arrays = Vec::with_capacity(self.fields.len());
        for (arg, field) in args.args.iter().skip(1).step_by(2).zip(&self.fields) {
            let value = arg
                .cast_to(
                    crate::native_schema::nullable_layout(field).data_type(),
                    Some(&options),
                )?
                .into_array(args.number_rows)?;
            arrays.push(crate::native_schema::array_layout(
                value,
                field.data_type(),
            )?);
        }
        Ok(ColumnarValue::Array(Arc::new(
            arrow::array::StructArray::try_new(self.fields.clone(), arrays, None)?,
        )))
    }
}
