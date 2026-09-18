# `datafusion_functions_window::create_udwf_expr`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions_window.create_udwf_expr.json).

<a id="op-d903ccbb58f3e1e9e6e08203"></a>
## create_udwf_expr

`macro` · `datafusion_functions_window::create_udwf_expr` · datafusion-functions-window 55.1.0

```rust
macro_rules! create_udwf_expr
```

Source: `src/macros.rs:289`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-window/55.1.0/json).

Create a [`WindowFunction`] expression that exposes a fluent API
which you can use to build more complex expressions.

[`WindowFunction`]: datafusion_expr::Expr::WindowFunction

# Parameters

* `$UDWF`: The struct which defines the [`Signature`] of the
  user-defined window function.
* `$OUT_FN_NAME`: The basename to generate a unique function name like
  `$OUT_FN_NAME_udwf`.
* `$DOC`: Doc comments for UDWF.
* (optional) `[$($PARAM:ident),+]`: An array of 1 or more parameters
  for the generated function. The type of parameters is [`Expr`].
  When omitted this creates a function with zero parameters.

[`Signature`]: datafusion_expr::Signature
[`Expr`]: datafusion_expr::Expr

# Example

1. With Zero Parameters
```
use arrow::datatypes::FieldRef;
# use datafusion_common::arrow::datatypes::{DataType, Field};
# use datafusion_expr::{PartitionEvaluator, Signature, Volatility, WindowUDFImpl};
# use datafusion_functions_window::{create_udwf_expr, get_or_init_udwf};
# use datafusion_functions_window_common::field::WindowUDFFieldArgs;
# use datafusion_functions_window_common::partition::PartitionEvaluatorArgs;

# get_or_init_udwf!(
#     RowNumber,
#     row_number,
#     row_number_udwf,
#     "Returns a unique row number for each row in window partition beginning at 1."
# );
/// Creates `row_number()` API which has zero parameters:
///
///     ```
///     /// Returns a unique row number for each row in window partition
///     /// beginning at 1.
///     pub fn row_number() -> datafusion_expr::Expr {
///        row_number_udwf().call(vec![])
///     }
///     ```
create_udwf_expr!(
    RowNumber,
    row_number,
    row_number_udwf,
    "Returns a unique row number for each row in window partition beginning at 1."
);
#
# assert_eq!(
#     row_number().name_for_alias().unwrap(),
#     "row_number() ROWS BETWEEN UNBOUNDED PRECEDING AND UNBOUNDED FOLLOWING"
# );
#
# #[derive(Debug, PartialEq, Eq, Hash)]
# struct RowNumber {
#     signature: Signature,
# }
# impl Default for RowNumber {
#     fn default() -> Self {
#         Self {
#             signature: Signature::any(0, Volatility::Immutable),
#         }
#     }
# }
# impl WindowUDFImpl for RowNumber {
#     fn name(&self) -> &str {
#         "row_number"
#     }
#     fn signature(&self) -> &Signature {
#         &self.signature
#     }
#     fn partition_evaluator(
#         &self,
#         _partition_evaluator_args: PartitionEvaluatorArgs,
#     ) -> datafusion_common::Result<Box<dyn PartitionEvaluator>> {
#         unimplemented!()
#     }
#     fn field(&self, field_args: WindowUDFFieldArgs) -> datafusion_common::Result<FieldRef> {
#         Ok(Field::new(field_args.name(), DataType::UInt64, false).into())
#     }
# }
```

2. With Multiple Parameters
```
use arrow::datatypes::FieldRef;
#
# use datafusion_expr::{
#     PartitionEvaluator, Signature, TypeSignature, Volatility, WindowUDFImpl,
# };
#
# use datafusion_functions_window::{create_udwf_expr, get_or_init_udwf};
# use datafusion_functions_window_common::field::WindowUDFFieldArgs;
#
# use datafusion_common::arrow::datatypes::Field;
# use datafusion_common::ScalarValue;
# use datafusion_expr::{col, lit};
# use datafusion_functions_window_common::partition::PartitionEvaluatorArgs;
#
# get_or_init_udwf!(Lead, lead,lead_udwf, "user-defined window function");
#
/// Creates `lead(expr, offset, default)` with 3 parameters:
///
///     ```
///     /// Returns a value evaluated at the row that is offset rows
///     /// after the current row within the partition.
///     pub fn lead(
///         expr: datafusion_expr::Expr,
///         offset: datafusion_expr::Expr,
///         default: datafusion_expr::Expr,
///     ) -> datafusion_expr::Expr {
///         lead_udwf().call(vec![expr, offset, default])
///     }
///     ```
create_udwf_expr!(
    Lead,
    lead,
    [expr, offset, default],
    lead_udwf,
    "Returns a value evaluated at the row that is offset rows after the current row within the partition."
);
#
# assert_eq!(
#     lead(col("a"), lit(1i64), lit(ScalarValue::Null))
#         .name_for_alias()
#         .unwrap(),
#     "lead(a,Int64(1),NULL) ROWS BETWEEN UNBOUNDED PRECEDING AND UNBOUNDED FOLLOWING"
# );
#
# #[derive(Debug, PartialEq, Eq, Hash)]
# struct Lead {
#     signature: Signature,
# }
#
# impl Default for Lead {
#     fn default() -> Self {
#         Self {
#             signature: Signature::one_of(
#                 vec![
#                     TypeSignature::Any(1),
#                     TypeSignature::Any(2),
#                     TypeSignature::Any(3),
#                 ],
#                 Volatility::Immutable,
#             ),
#         }
#     }
# }
#
# impl WindowUDFImpl for Lead {
#     fn name(&self) -> &str {
#         "lead"
#     }
#     fn signature(&self) -> &Signature {
#         &self.signature
#     }
#     fn partition_evaluator(
#         &self,
#         partition_evaluator_args: PartitionEvaluatorArgs,
#     ) -> datafusion_common::Result<Box<dyn PartitionEvaluator>> {
#         unimplemented!()
#     }
#     fn field(&self, field_args: WindowUDFFieldArgs) -> datafusion_common::Result<FieldRef> {
#         Ok(Field::new(
#             field_args.name(),
#             field_args.get_input_field(0).unwrap().data_type().clone(),
#             false,
#         ).into())
#     }
# }
```
