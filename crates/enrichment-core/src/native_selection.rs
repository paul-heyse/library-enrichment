//! Preserve full Fields while delegating selection to DataFusion's CASE execution.
use arrow::datatypes::{DataType, FieldRef};
use datafusion::{
    common::{Result, ScalarValue, metadata::FieldMetadata},
    logical_expr::{
        ColumnarValue, Expr, ExprSchemable, ReturnFieldArgs, ScalarFunctionArgs, ScalarUDF,
        ScalarUDFImpl, Signature, Volatility,
        simplify::{ExprSimplifyResult, SimplifyContext},
    },
};
use std::sync::Arc;

pub fn coalesce() -> ScalarUDF {
    NativeCoalesce {
        signature: Signature::user_defined(Volatility::Immutable),
    }
    .into()
}
pub(crate) fn is_coalesce(function: &ScalarUDF) -> bool {
    function.inner().downcast_ref::<NativeCoalesce>().is_some()
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct NativeCoalesce {
    signature: Signature,
}

impl ScalarUDFImpl for NativeCoalesce {
    fn name(&self) -> &str {
        "native_coalesce"
    }
    fn signature(&self) -> &Signature {
        &self.signature
    }
    fn return_type(&self, _: &[DataType]) -> Result<DataType> {
        datafusion::common::internal_err!("native coalesce requires complete fields")
    }
    fn coerce_types(&self, types: &[DataType]) -> Result<Vec<DataType>> {
        // The pinned union coercer rejects NULL + FixedSizeBinary. Keep literal NULL
        // until simplify can give it the selected full Field; a cast here discards its
        // literal witness before return-field admission sees the argument again.
        if let Some(selected) = types.iter().find(|kind| !kind.is_null())
            && types.iter().all(|kind| kind.is_null() || kind == selected)
        {
            return Ok(types.to_vec());
        }
        datafusion::functions::core::coalesce()
            .inner()
            .coerce_types(types)
    }
    fn return_field_from_args(&self, args: ReturnFieldArgs) -> Result<FieldRef> {
        let selected = args
            .arg_fields
            .iter()
            .find(|field| crate::native_analysis::has_semantics(field))
            .or_else(|| {
                args.arg_fields
                    .iter()
                    .find(|field| !field.data_type().is_null())
            })
            .or_else(|| args.arg_fields.first())
            .ok_or_else(|| {
                datafusion::common::DataFusionError::Plan("coalesce requires a value".into())
            })?;
        for (field, scalar) in args.arg_fields.iter().zip(args.scalar_arguments) {
            if !crate::native_analysis::has_semantics(field)
                && (field.data_type().is_null() || scalar.is_some_and(|value| value.is_null()))
            {
                continue;
            }
            crate::native_analysis::compatible(selected, field, self.name())?;
        }
        let native = datafusion::functions::core::coalesce().return_field_from_args(args)?;
        Ok(Arc::new(
            selected
                .as_ref()
                .clone()
                .with_name(self.name())
                .with_data_type(native.data_type().clone())
                .with_nullable(native.is_nullable()),
        ))
    }
    fn simplify(&self, args: Vec<Expr>, info: &SimplifyContext) -> Result<ExprSimplifyResult> {
        let fields = args
            .iter()
            .map(|arg| arg.to_field(info.schema()).map(|(_, field)| field))
            .collect::<Result<Vec<_>>>()?;
        let scalars = args
            .iter()
            .map(|arg| match arg {
                Expr::Literal(value, _) => Some(value),
                _ => None,
            })
            .collect::<Vec<_>>();
        let output = self.return_field_from_args(ReturnFieldArgs {
            arg_fields: &fields,
            scalar_arguments: &scalars,
        })?;
        let args = args
            .into_iter()
            .zip(fields)
            .map(|(arg, field)| {
                if matches!(&arg, Expr::Literal(value,_) if value.is_null())
                    && !crate::native_analysis::has_semantics(&field)
                {
                    Ok(Expr::Literal(
                        ScalarValue::try_from(output.data_type())?,
                        Some(FieldMetadata::from(output.as_ref())),
                    ))
                } else {
                    Ok(arg)
                }
            })
            .collect::<Result<Vec<_>>>()?;
        Ok(ExprSimplifyResult::Simplified(
            crate::evidence::arrow_model::expressions::coalesce_case(args)?,
        ))
    }
    fn invoke_with_args(&self, _: ScalarFunctionArgs) -> Result<ColumnarValue> {
        datafusion::common::internal_err!("native coalesce must lower to CASE")
    }
    fn short_circuits(&self) -> bool {
        true
    }
    fn conditional_arguments<'a>(
        &self,
        args: &'a [Expr],
    ) -> Option<(Vec<&'a Expr>, Vec<&'a Expr>)> {
        args.split_first()
            .map(|(first, rest)| (vec![first], rest.iter().collect()))
    }
}
