# `datafusion_functions_window::define_udwf_and_expr`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions_window.define_udwf_and_expr.json).

<a id="op-30dcccf0fd0ba5dc6bd869e4"></a>
## define_udwf_and_expr

`macro` · `datafusion_functions_window::define_udwf_and_expr` · datafusion-functions-window 55.1.0

```rust
macro_rules! define_udwf_and_expr
```

Source: `src/macros.rs:637`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-window/55.1.0/json).

Defines a user-defined window function.

Combines [`get_or_init_udwf!`](../operations/datafusion_functions_window.get_or_init_udwf.md#op-8b8b9651b600ac64790c6256) and [`create_udwf_expr!`](../operations/datafusion_functions_window.create_udwf_expr.md#op-d903ccbb58f3e1e9e6e08203) into a
single macro for convenience.

# Arguments

* `$UDWF`: The struct which defines the [`Signature`] of the
  user-defined window function.
* `$OUT_FN_NAME`: The basename to generate a unique function name like
  `$OUT_FN_NAME_udwf`.
* (optional) `[$($PARAM:ident),+]`: An array of 1 or more parameters
  for the generated function. The type of parameters is [`Expr`].
  When omitted this creates a function with zero parameters.
* `$DOC`: Doc comments for UDWF.
* (optional) `$CTOR`: Pass a custom constructor. When omitted it
  automatically resolves to `$UDWF::default()`.

[`Signature`]: datafusion_expr::Signature
[`Expr`]: datafusion_expr::Expr

# Usage

## Expression API With Zero parameters
1. Uses default constructor for UDWF.

```
use arrow::datatypes::FieldRef;
# use datafusion_common::arrow::datatypes::{DataType, Field};
# use datafusion_expr::{PartitionEvaluator, Signature, Volatility, WindowUDFImpl};
#
# use datafusion_functions_window_common::field::WindowUDFFieldArgs;
# use datafusion_functions_window::{define_udwf_and_expr, get_or_init_udwf, create_udwf_expr};
# use datafusion_functions_window_common::partition::PartitionEvaluatorArgs;
#
/// 1. Defines the `simple_udwf()` user-defined window function.
///
/// 2. Defines the expression API:
///     ```
///     pub fn simple() -> datafusion_expr::Expr {
///         simple_udwf().call(vec![])
///     }
///     ```
define_udwf_and_expr!(
    SimpleUDWF,
    simple,
    simple_udwf,
    "a simple user-defined window function"
);
#
# assert_eq!(simple_udwf().name(), "simple_user_defined_window_function");
#
#  #[derive(Debug, PartialEq, Eq, Hash)]
#  struct SimpleUDWF {
#      signature: Signature,
#  }
#
#  impl Default for SimpleUDWF {
#      fn default() -> Self {
#          Self {
#             signature: Signature::any(0, Volatility::Immutable),
#          }
#      }
#  }
#
#  impl WindowUDFImpl for SimpleUDWF {
#      fn name(&self) -> &str {
#          "simple_user_defined_window_function"
#      }
#      fn signature(&self) -> &Signature {
#          &self.signature
#      }
#      fn partition_evaluator(
#          &self,
#          partition_evaluator_args: PartitionEvaluatorArgs,
#      ) -> datafusion_common::Result<Box<dyn PartitionEvaluator>> {
#          unimplemented!()
#      }
#      fn field(&self, field_args: WindowUDFFieldArgs) -> datafusion_common::Result<FieldRef> {
#          Ok(Field::new(field_args.name(), DataType::Int64, false).into())
#      }
#  }
#
```

2. Uses a custom constructor for UDWF.

```
use arrow::datatypes::FieldRef;
# use datafusion_common::arrow::datatypes::{DataType, Field};
# use datafusion_expr::{PartitionEvaluator, Signature, Volatility, WindowUDFImpl};
# use datafusion_functions_window::{create_udwf_expr, define_udwf_and_expr, get_or_init_udwf};
# use datafusion_functions_window_common::field::WindowUDFFieldArgs;
# use datafusion_functions_window_common::partition::PartitionEvaluatorArgs;
#
/// 1. Defines the `row_number_udwf()` user-defined window function.
///
/// 2. Defines the expression API:
///     ```
///     pub fn row_number() -> datafusion_expr::Expr {
///         row_number_udwf().call(vec![])
///     }
///     ```
define_udwf_and_expr!(
    RowNumber,
    row_number,
    row_number_udwf,
    "Returns a unique row number for each row in window partition beginning at 1.",
    RowNumber::new // <-- custom constructor
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
# impl RowNumber {
#     fn new() -> Self {
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

## Expression API With Multiple Parameters
3. Uses default constructor for UDWF

```
use arrow::datatypes::FieldRef;
#
# use datafusion_expr::{
#     PartitionEvaluator, Signature, TypeSignature, Volatility, WindowUDFImpl,
# };
#
# use datafusion_functions_window::{create_udwf_expr, define_udwf_and_expr, get_or_init_udwf};
# use datafusion_functions_window_common::field::WindowUDFFieldArgs;
#
# use datafusion_common::arrow::datatypes::Field;
# use datafusion_common::ScalarValue;
# use datafusion_expr::{col, lit};
# use datafusion_functions_window_common::partition::PartitionEvaluatorArgs;
#
/// 1. Defines the `lead_udwf()` user-defined window function.
///
/// 2. Defines the expression API:
///     ```
///     pub fn lead(
///         expr: datafusion_expr::Expr,
///         offset: datafusion_expr::Expr,
///         default: datafusion_expr::Expr,
///     ) -> datafusion_expr::Expr {
///         lead_udwf().call(vec![expr, offset, default])
///     }
///     ```
define_udwf_and_expr!(
    Lead,
    lead,
    [expr, offset, default],        // <- 3 parameters
    lead_udwf,
    "user-defined window function"
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
#         _partition_evaluator_args: PartitionEvaluatorArgs,
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
4. Uses custom constructor for UDWF

```
use arrow::datatypes::FieldRef;
#
# use datafusion_expr::{
#     PartitionEvaluator, Signature, TypeSignature, Volatility, WindowUDFImpl,
# };
#
# use datafusion_functions_window::{create_udwf_expr, define_udwf_and_expr, get_or_init_udwf};
# use datafusion_functions_window_common::field::WindowUDFFieldArgs;
#
# use datafusion_common::arrow::datatypes::Field;
# use datafusion_common::ScalarValue;
# use datafusion_expr::{col, lit};
# use datafusion_functions_window_common::partition::PartitionEvaluatorArgs;
#
/// 1. Defines the `lead_udwf()` user-defined window function.
///
/// 2. Defines the expression API:
///     ```
///     pub fn lead(
///         expr: datafusion_expr::Expr,
///         offset: datafusion_expr::Expr,
///         default: datafusion_expr::Expr,
///     ) -> datafusion_expr::Expr {
///         lead_udwf().call(vec![expr, offset, default])
///     }
///     ```
define_udwf_and_expr!(
    Lead,
    lead,
    [expr, offset, default],        // <- 3 parameters
    lead_udwf,
    "user-defined window function",
    Lead::new                       // <- Custom constructor
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
# impl Lead {
#     fn new() -> Self {
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
#         _partition_evaluator_args: PartitionEvaluatorArgs,
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
