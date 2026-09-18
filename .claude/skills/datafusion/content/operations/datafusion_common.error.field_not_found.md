# `datafusion_common::error::field_not_found`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.error.field_not_found.json).

<a id="op-259a58cc2965d1891151ea1a"></a>
## field_not_found

`function` · `datafusion_common::error::field_not_found` · datafusion-common 55.1.0

```rust
fn field_not_found<R: Into<TableReference>>(qualifier: Option<R>, name: &str, schema: &DFSchema) -> DataFusionError
```

Source: `src/error.rs:1156`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Create a "field not found" DataFusion::SchemaError
