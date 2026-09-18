# `datafusion_functions_window::get_or_init_udwf`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions_window.get_or_init_udwf.json).

<a id="op-8b8b9651b600ac64790c6256"></a>
## get_or_init_udwf

`macro` · `datafusion_functions_window::get_or_init_udwf` · datafusion-functions-window 55.1.0

```rust
macro_rules! get_or_init_udwf
```

Source: `src/macros.rs:93`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-window/55.1.0/json).

Lazily initializes a user-defined window function exactly once
when called concurrently. Repeated calls return a reference to the
same instance.

# Parameters

* `$UDWF`: The struct which defines the [`Signature`](datafusion_expr::Signature)
  of the user-defined window function.
* `$OUT_FN_NAME`: The expression function name
  `UDWF_FN` : The unique function name
* `$DOC`: Doc comments for UDWF.
* (optional) `$CTOR`: Pass a custom constructor. When omitted it
  automatically resolves to `$UDWF::default()`.

# Example

```
use arrow::datatypes::FieldRef;
# use datafusion_common::arrow::datatypes::{DataType, Field};
# use datafusion_expr::{PartitionEvaluator, Signature, Volatility, WindowUDFImpl};
#
# use datafusion_functions_window_common::field::WindowUDFFieldArgs;
# use datafusion_functions_window::get_or_init_udwf;
# use datafusion_functions_window_common::partition::PartitionEvaluatorArgs;
#
/// Defines the `simple_udwf()` user-defined window function.
get_or_init_udwf!(
    SimpleUDWF,
    simple,
    simple_udwf,
    "Simple user-defined window function doc comment."
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
#         _partition_evaluator_args: PartitionEvaluatorArgs,
#      ) -> datafusion_common::Result<Box<dyn PartitionEvaluator>> {
#          unimplemented!()
#      }
#      fn field(&self, field_args: WindowUDFFieldArgs) -> datafusion_common::Result<FieldRef> {
#          Ok(Field::new(field_args.name(), DataType::Int64, false).into())
#      }
#  }
#
```
