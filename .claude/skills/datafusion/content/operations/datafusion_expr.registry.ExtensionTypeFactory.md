# `datafusion_expr::registry::ExtensionTypeFactory`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.registry.ExtensionTypeFactory.json).

<a id="op-b1e90084e8f7f7cc70d7b008"></a>
## ExtensionTypeFactory

`type_alias` · `datafusion_expr::registry::ExtensionTypeFactory` · datafusion-expr 55.1.0

```rust
type ExtensionTypeFactory = dyn Fn(&arrow_schema::DataType, Option<&str>) -> datafusion_common::Result<datafusion_common::types::DFExtensionTypeRef> + Send + Sync
```

Source: `src/registry.rs:353`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

A factory that creates instances of extension types from a storage [`DataType`](../operations/arrow_schema.datatype.DataType.md#op-bf69df5b14436e006d3a531c) and the
metadata.
