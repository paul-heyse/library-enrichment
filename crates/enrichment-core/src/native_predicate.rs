//! A field-local DataFusion predicate with a fixed invocation schema.
//!
//! The predicate remains a native expression, compiled once by DataFusion. Its lambda
//! variables are local to the supplied field, independent of outer physical filter and
//! projection rewrites. No application row evaluator or duplicated policy lives here.
use arrow::{
    datatypes::{DataType, FieldRef, Schema},
    record_batch::RecordBatch,
};
use datafusion::{
    common::{DFSchema, Result},
    logical_expr::{
        ColumnarValue, Expr, ReturnFieldArgs, ScalarFunctionArgs, ScalarUDF, ScalarUDFImpl,
        Signature, Volatility, execution_props::ExecutionProps,
        physical_planning_context::PhysicalPlanningContext,
    },
    physical_expr::{PhysicalExpr, create_physical_expr},
};
use std::{
    hash::{Hash, Hasher},
    sync::Arc,
};

/// Compile a generated field predicate; bind every physical invocation to that full field.
/// # Errors
/// Invalid fields or a non-Boolean native predicate refuse construction.
pub fn bind(field: FieldRef, expression: Expr) -> Result<Expr> {
    bind_fields(vec![field], expression)
}
/// A bounded native predicate keeps sibling fields in scope without materializing a row object.
pub fn bind_fields(fields: Vec<FieldRef>, expression: Expr) -> Result<Expr> {
    let fields = fields
        .into_iter()
        .map(|field| Arc::new(field.as_ref().clone().with_nullable(true)))
        .collect::<Vec<_>>();
    let schema = Schema::new(fields.clone());
    crate::native_schema::validate(&schema)?;
    let logical = DFSchema::try_from(schema.clone())?;
    let expression = expression.resolve_lambda_variables(&logical)?.data;
    let physical = create_physical_expr(
        &expression,
        &logical,
        &ExecutionProps::new(),
        &PhysicalPlanningContext::default(),
    )?;
    if physical.data_type(&schema)? != DataType::Boolean {
        return datafusion::common::plan_err!("native field predicate requires Boolean output");
    }
    let signature = Signature::exact(
        fields
            .iter()
            .map(|field| field.data_type().clone())
            .collect(),
        Volatility::Immutable,
    );
    let inputs = fields
        .iter()
        .map(|field| Expr::Column(datafusion::common::Column::from_name(field.name())))
        .collect();
    Ok(ScalarUDF::from(NativePredicate {
        fields,
        expression,
        physical,
        signature,
    })
    .call(inputs))
}

pub(crate) fn is_predicate(function: &ScalarUDF) -> bool {
    function.inner().downcast_ref::<NativePredicate>().is_some()
}

#[derive(Debug)]
struct NativePredicate {
    fields: Vec<FieldRef>,
    expression: Expr,
    physical: Arc<dyn PhysicalExpr>,
    signature: Signature,
}
impl PartialEq for NativePredicate {
    fn eq(&self, other: &Self) -> bool {
        self.fields == other.fields && self.expression == other.expression
    }
}
impl Eq for NativePredicate {}
impl Hash for NativePredicate {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.fields.hash(state);
        self.expression.hash(state);
    }
}
impl ScalarUDFImpl for NativePredicate {
    fn name(&self) -> &str {
        "native_field_predicate_v1"
    }
    fn signature(&self) -> &Signature {
        &self.signature
    }
    fn return_type(&self, _: &[DataType]) -> Result<DataType> {
        datafusion::common::plan_err!("native predicate requires full fields")
    }
    fn return_field_from_args(&self, args: ReturnFieldArgs) -> Result<FieldRef> {
        crate::native_schema::function_output(
            &args,
            &self.fields,
            self.name(),
            DataType::Boolean,
            self.physical.nullable(&Schema::new(self.fields.clone()))?,
        )
    }
    fn invoke_with_args(&self, args: ScalarFunctionArgs) -> Result<ColumnarValue> {
        crate::native_schema::function_call(self, &args)?;
        let arrays = args
            .args
            .iter()
            .map(|arg| arg.to_array(args.number_rows))
            .collect::<Result<Vec<_>>>()?;
        let batch = RecordBatch::try_new(Arc::new(Schema::new(self.fields.clone())), arrays)?;
        self.physical.evaluate(&batch)
    }
}
