# `datafusion_common::assert_or_internal_err`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.assert_or_internal_err.json).

<a id="op-0a0d8eb0af090d416d5b0978"></a>
## assert_or_internal_err

`macro` · `datafusion_common::assert_or_internal_err` · datafusion-common 55.1.0

```rust
macro_rules! assert_or_internal_err
```

Source: `src/error.rs:841`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Assert a condition, returning `DataFusionError::Internal` on failure.

# Examples

```text
assert_or_internal_err!(predicate);
assert_or_internal_err!(predicate, "human readable message");
assert_or_internal_err!(predicate, format!("details: {}", value));
```
