# `datafusion_common::error::unqualified_field_not_found`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.error.unqualified_field_not_found.json).

<a id="op-9b88eb4dc796a61202d3dc07"></a>
## unqualified_field_not_found

`function` · `datafusion_common::error::unqualified_field_not_found` · datafusion-common 55.1.0

```rust
fn unqualified_field_not_found(name: &str, schema: &DFSchema) -> DataFusionError
```

Source: `src/error.rs:1168`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Convenience wrapper over [`field_not_found`](../operations/datafusion_common.error.field_not_found.md#op-259a58cc2965d1891151ea1a) for when there is no qualifier
