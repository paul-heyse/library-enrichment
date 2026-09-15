# `datafusion_expr::udwf`

Crate `datafusion-expr` · 4 public items · structured records in [`model/datafusion_expr.udwf.json`](../model/datafusion_expr.udwf.json)

## LimitEffect

`enum` · `datafusion_expr::udwf::LimitEffect`

Also reachable as `datafusion::logical_expr::LimitEffect`, `datafusion_expr::LimitEffect`

```rust
enum LimitEffect
```

**Variants**: `None`, `Unknown`, `Relative`, `Absolute`

the effect this function will have on the limit pushdown

---

## ReversedUDWF

`enum` · `datafusion_expr::udwf::ReversedUDWF`

Also reachable as `datafusion::logical_expr::ReversedUDWF`, `datafusion_expr::ReversedUDWF`

```rust
enum ReversedUDWF
```

**Variants**: `Identical`, `NotSupported`, `Reversed`

---

## WindowUDF

`struct` · `datafusion_expr::udwf::WindowUDF`

Also reachable as `datafusion::logical_expr::WindowUDF`, `datafusion_expr::WindowUDF`

```rust
struct WindowUDF
```

**Implements**: `core::convert::From`, `core::fmt::Display`, `datafusion_expr::type_coercion::functions::UDFCoercionExt`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, PartialOrd

**Methods** (16)

```rust
fn aliases(&self) -> &[String]
fn call(&self, args: Vec<Expr>) -> Expr
fn coerce_types(&self, arg_types: &[DataType]) -> Result<Vec<DataType>>
fn documentation(&self) -> Option<&Documentation>
fn expressions(&self, expr_args: ExpressionArgs<'_>) -> Vec<Arc<dyn PhysicalExpr>>
fn field(&self, field_args: WindowUDFFieldArgs<'_>) -> Result<FieldRef>
fn inner(&self) -> &Arc<dyn WindowUDFImpl>
fn name(&self) -> &str
fn new_from_impl<F>(fun: F) -> WindowUDF where F: WindowUDFImpl + 'static
fn new_from_shared_impl(fun: Arc<dyn WindowUDFImpl>) -> WindowUDF
fn partition_evaluator_factory(&self, partition_evaluator_args: PartitionEvaluatorArgs<'_>) -> Result<Box<dyn PartitionEvaluator>>
fn reverse_expr(&self) -> ReversedUDWF
fn signature(&self) -> &Signature
fn simplify(&self) -> Option<WindowFunctionSimplification>
fn sort_options(&self) -> Option<SortOptions>
fn with_aliases(self, aliases: impl IntoIterator<Item = &'static str>) -> Self
```

**via `core::convert::From`**

```rust
fn from(fun: F) -> Self
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result
```

**via `datafusion_expr::type_coercion::functions::UDFCoercionExt`**

```rust
fn coerce_types(&self, arg_types: &[DataType]) -> Result<Vec<DataType>>
fn name(&self) -> &str
fn signature(&self) -> &Signature
```

Logical representation of a user-defined window function (UDWF).

A Window Function is called via the SQL `OVER` clause:

```sql
SELECT first_value(col) OVER (PARTITION BY a, b ORDER BY c) FROM foo;
```

A UDWF is different from a user defined function (UDF) in that it is
stateful across batches.

See the documentation on [`PartitionEvaluator`] for more details

1. For simple use cases, use [`create_udwf`] (examples in
   [`simple_udwf.rs`]).

2. For advanced use cases, use [`WindowUDFImpl`] which provides full API
   access (examples in [`advanced_udwf.rs`]).

# API Note
This is a separate struct from `WindowUDFImpl` to maintain backwards
compatibility with the older API.

[`PartitionEvaluator`]: crate::PartitionEvaluator
[`create_udwf`]: crate::expr_fn::create_udwf
[`simple_udwf.rs`]: https://github.com/apache/datafusion/blob/main/datafusion-examples/examples/udf/simple_udwf.rs
[`advanced_udwf.rs`]: https://github.com/apache/datafusion/blob/main/datafusion-examples/examples/udf/advanced_udwf.rs

---

## WindowUDFImpl

`trait` · `datafusion_expr::udwf::WindowUDFImpl`

Also reachable as `datafusion::logical_expr::WindowUDFImpl`, `datafusion_expr::WindowUDFImpl`

```rust
trait WindowUDFImpl: Debug + DynEq + DynHash + Send + Sync + Any
```

**Implementors** (8)

- `datafusion_expr::expr_fn::SimpleWindowUDF`
- `datafusion_ffi::udwf::ForeignWindowUDF`
- `datafusion_functions_window::cume_dist::CumeDist`
- `datafusion_functions_window::lead_lag::WindowShift`
- `datafusion_functions_window::nth_value::NthValue`
- `datafusion_functions_window::ntile::Ntile`
- `datafusion_functions_window::rank::Rank`
- `datafusion_functions_window::row_number::RowNumber`

**Methods** (12)

```rust
fn aliases(&self) -> &[String]
fn coerce_types(&self, _arg_types: &[DataType]) -> Result<Vec<DataType>>
fn documentation(&self) -> Option<&Documentation>
fn expressions(&self, expr_args: ExpressionArgs<'_>) -> Vec<Arc<dyn PhysicalExpr>>
fn field(&self, field_args: WindowUDFFieldArgs<'_>) -> Result<FieldRef>
fn limit_effect(&self, _args: &[Arc<dyn PhysicalExpr>]) -> LimitEffect
fn name(&self) -> &str
fn partition_evaluator(&self, partition_evaluator_args: PartitionEvaluatorArgs<'_>) -> Result<Box<dyn PartitionEvaluator>>
fn reverse_expr(&self) -> ReversedUDWF
fn signature(&self) -> &Signature
fn simplify(&self) -> Option<WindowFunctionSimplification>
fn sort_options(&self) -> Option<SortOptions>
```

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

---
