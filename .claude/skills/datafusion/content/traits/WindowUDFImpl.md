# WindowUDFImpl

`datafusion_expr::udwf::WindowUDFImpl`

```rust
trait WindowUDFImpl: Debug + DynEq + DynHash + Send + Sync + Any
```

Also reachable as `datafusion::logical_expr::WindowUDFImpl`, `datafusion_expr::WindowUDFImpl`

Prose: [`api/datafusion_expr.udwf.md`](../api/datafusion_expr.udwf.md#windowudfimpl) · records: [`model/datafusion_expr.udwf.json`](../model/datafusion_expr.udwf.json)

## Required

Every implementation must supply these.

```rust
fn field(&self, field_args: WindowUDFFieldArgs<'_>) -> Result<FieldRef>
fn name(&self) -> &str
fn partition_evaluator(&self, partition_evaluator_args: PartitionEvaluatorArgs<'_>) -> Result<Box<dyn PartitionEvaluator>>
fn signature(&self) -> &Signature
```

## Provided

Defaulted, and this is where the capability hides. The default is the conservative answer -- no pushdown, no statistics, no specialization -- so an implementation that overrides none of these works correctly and performs badly.

```rust
fn aliases(&self) -> &[String]
fn coerce_types(&self, _arg_types: &[DataType]) -> Result<Vec<DataType>>
fn documentation(&self) -> Option<&Documentation>
fn expressions(&self, expr_args: ExpressionArgs<'_>) -> Vec<Arc<dyn PhysicalExpr>>
fn limit_effect(&self, _args: &[Arc<dyn PhysicalExpr>]) -> LimitEffect
fn reverse_expr(&self) -> ReversedUDWF
fn simplify(&self) -> Option<WindowFunctionSimplification>
fn sort_options(&self) -> Option<SortOptions>
```

## Implementors (8)

Read one before writing your own.

- `datafusion_expr::expr_fn::SimpleWindowUDF`
- `datafusion_ffi::udwf::ForeignWindowUDF`
- `datafusion_functions_window::cume_dist::CumeDist`
- `datafusion_functions_window::lead_lag::WindowShift`
- `datafusion_functions_window::nth_value::NthValue`
- `datafusion_functions_window::ntile::Ntile`
- `datafusion_functions_window::rank::Rank`
- `datafusion_functions_window::row_number::RowNumber`

## Demonstrated by 1 upstream example(s)

- [`corpus/examples/udf/advanced_udwf.rs`](../corpus/examples/udf/advanced_udwf.rs)

## Documentation

Trait for implementing [`WindowUDF`].

This trait exposes the full API for implementing user defined window functions and
can be used to implement any function.

While the trait depends on [`DynEq`] and [`DynHash`] traits, these should not be
implemented directly. Instead, implement [`Eq`] and [`Hash`] and leverage the
blanket implementations of [`DynEq`] and [`DynHash`].

See [`advanced_udwf.rs`] for a full example with complete implementation and
[`WindowUDF`] for other available options.


[`advanced_udwf.rs`]: https://github.com/apache/datafusion/blob/main/datafusion-examples/examples/udf/advanced_udwf.rs
# Basic Example
```
# use std::sync::LazyLock;
# use arrow::datatypes::{DataType, Field, FieldRef};
# use datafusion_common::{DataFusionError, plan_err, Result};
# use datafusion_expr::{col, Signature, Volatility, PartitionEvaluator, WindowFrame, ExprFunctionExt, Documentation, LimitEffect};
# use datafusion_expr::{WindowUDFImpl, WindowUDF};
# use datafusion_functions_window_common::field::WindowUDFFieldArgs;
# use datafusion_functions_window_common::partition::PartitionEvaluatorArgs;
# use datafusion_expr::window_doc_sections::DOC_SECTION_ANALYTICAL;
# use datafusion_physical_expr_common::physical_expr;
# use std::sync::Arc;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct SmoothIt {
  signature: Signature,
}

impl SmoothIt {
  fn new() -> Self {
    Self {
      signature: Signature::uniform(1, vec![DataType::Int32], Volatility::Immutable),
     }
  }
}

static DOCUMENTATION: LazyLock<Documentation> = LazyLock::new(|| {
    Documentation::builder(DOC_SECTION_ANALYTICAL, "smooths the windows", "smooth_it(2)")
        .with_argument("arg1", "The int32 number to smooth by")
        .build()
});

fn get_doc() -> &'static Documentation {
    &DOCUMENTATION
}

/// Implement the WindowUDFImpl trait for SmoothIt
impl WindowUDFImpl for SmoothIt {
   fn name(&self) -> &str { "smooth_it" }
   fn signature(&self) -> &Signature { &self.signature }
   // The actual implementation would smooth the window
   fn partition_evaluator(
       &self,
       _partition_evaluator_args: PartitionEvaluatorArgs,
   ) -> Result<Box<dyn PartitionEvaluator>> {
       unimplemented!()
   }
   fn field(&self, field_args: WindowUDFFieldArgs) -> Result<FieldRef> {
     if let Some(DataType::Int32) = field_args.get_input_field(0).map(|f| f.data_type().clone()) {
       Ok(Field::new(field_args.name(), DataType::Int32, false).into())
     } else {
       plan_err!("smooth_it only accepts Int32 arguments")
     }
   }
   fn documentation(&self) -> Option<&Documentation> {
     Some(get_doc())
   }
    fn limit_effect(&self, _args: &[Arc<dyn physical_expr::PhysicalExpr>]) -> LimitEffect {
        LimitEffect::Unknown
    }
}

// Create a new WindowUDF from the implementation
let smooth_it = WindowUDF::from(SmoothIt::new());

// Call the function `add_one(col)`
// smooth_it(speed) OVER (PARTITION BY car ORDER BY time ASC)
let expr = smooth_it.call(vec![col("speed")])
    .partition_by(vec![col("car")])
    .order_by(vec![col("time").sort(true, true)])
    .window_frame(WindowFrame::new(None))
    .build()
    .unwrap();
```
