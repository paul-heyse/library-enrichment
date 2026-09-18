# `datafusion_physical_expr_common::utils::scatter`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_expr_common.utils.scatter.json).

<a id="op-c13739f754b049645ae832fb"></a>
## scatter

`function` · `datafusion_physical_expr_common::utils::scatter` · datafusion-physical-expr-common 55.1.0

```rust
fn scatter(mask: &arrow::array::BooleanArray, truthy: &dyn Array) -> datafusion_common::Result<arrow::array::ArrayRef>
```

Source: `src/utils.rs:76`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Scatter `truthy` array by boolean mask. When the mask evaluates `true`, next values of `truthy`
are taken, when the mask evaluates `false` values null values are filled.

# Arguments
* `mask` - Boolean values used to determine where to put the `truthy` values
* `truthy` - All values of this array are to scatter according to `mask` into final result.
