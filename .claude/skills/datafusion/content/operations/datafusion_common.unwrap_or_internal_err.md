# `datafusion_common::unwrap_or_internal_err`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.unwrap_or_internal_err.json).

<a id="op-64515f0b53ad89e1c069d8f1"></a>
## unwrap_or_internal_err

`macro` · `datafusion_common::unwrap_or_internal_err` · datafusion-common 55.1.0

```rust
macro_rules! unwrap_or_internal_err
```

Source: `src/error.rs:820`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Unwrap an `Option` if possible. Otherwise return an `DataFusionError::Internal`.
In normal usage of DataFusion the unwrap should always succeed.

Example: `let values = unwrap_or_internal_err!(values)`
