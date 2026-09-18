# `datafusion_common::assert_ne_or_internal_err`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.assert_ne_or_internal_err.json).

<a id="op-dd781398c5711f92bfeb5af9"></a>
## assert_ne_or_internal_err

`macro` · `datafusion_common::assert_ne_or_internal_err` · datafusion-common 55.1.0

```rust
macro_rules! assert_ne_or_internal_err
```

Source: `src/error.rs:911`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Assert inequality, returning `DataFusionError::Internal` on failure.

# Examples

```text
assert_ne_or_internal_err!(left, right);
assert_ne_or_internal_err!(lhs_expr, rhs_expr, "values must differ");
assert_ne_or_internal_err!(a, b, "context {}", info);
```
