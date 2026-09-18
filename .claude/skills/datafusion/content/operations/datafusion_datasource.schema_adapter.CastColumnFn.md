# `datafusion_datasource::schema_adapter::CastColumnFn`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_datasource.schema_adapter.CastColumnFn.json).

<a id="op-0b3d3f95ce172431739f3c78"></a>
## CastColumnFn

`type_alias` · `datafusion_datasource::schema_adapter::CastColumnFn` · datafusion-datasource 55.1.0

```rust
type CastColumnFn = dyn Fn(&arrow::array::ArrayRef, &arrow::datatypes::Field, &arrow::compute::CastOptions<'_>) -> datafusion_common::Result<arrow::array::ArrayRef> + Send + Sync
```

Source: `src/schema_adapter.rs:43`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Deprecated: Function type for casting columns.

This type has been removed. Use [`PhysicalExprAdapterFactory`] instead.
See `upgrading.md` for more details.

[`PhysicalExprAdapterFactory`]: datafusion_physical_expr_adapter::PhysicalExprAdapterFactory
