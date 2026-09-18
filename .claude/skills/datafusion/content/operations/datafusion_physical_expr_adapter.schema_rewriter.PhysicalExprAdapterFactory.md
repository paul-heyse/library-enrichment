# `datafusion_physical_expr_adapter::schema_rewriter::PhysicalExprAdapterFactory`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_expr_adapter.schema_rewriter.PhysicalExprAdapterFactory.json).

<a id="op-66df2b6b668622dcee375500"></a>
## PhysicalExprAdapterFactory

`trait` · `datafusion_physical_expr_adapter::schema_rewriter::PhysicalExprAdapterFactory` · datafusion-physical-expr-adapter 55.1.0

```rust
trait PhysicalExprAdapterFactory: Send + Sync + std::fmt::Debug
```

Source: `src/schema_rewriter.rs:175`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-adapter/55.1.0/json).

Creates instances of [`PhysicalExprAdapter`](../operations/datafusion_physical_expr_adapter.schema_rewriter.PhysicalExprAdapter.md#op-3d7f05fc0e38d49de65f327a) for given logical and physical schemas.

See [`DefaultPhysicalExprAdapterFactory`](../operations/datafusion_physical_expr_adapter.schema_rewriter.DefaultPhysicalExprAdapterFactory.md#op-a4e5de845da9e33105dd28ae) for the default implementation.

<a id="op-de6cbcbb377e5b988f3c2998"></a>
## create

`function` · `datafusion_physical_expr_adapter::schema_rewriter::PhysicalExprAdapterFactory::create` · datafusion-physical-expr-adapter 55.1.0

```rust
fn create(&self, logical_file_schema: SchemaRef, physical_file_schema: SchemaRef) -> Result<Arc<dyn PhysicalExprAdapter>>
```

Source: `src/schema_rewriter.rs:177`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-adapter/55.1.0/json).

Create a new instance of the physical expression adapter.
