# `datafusion_physical_expr_adapter::schema_rewriter::DefaultPhysicalExprAdapterFactory`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_expr_adapter.schema_rewriter.DefaultPhysicalExprAdapterFactory.json).

<a id="op-a4e5de845da9e33105dd28ae"></a>
## DefaultPhysicalExprAdapterFactory

`struct` · `datafusion_physical_expr_adapter::schema_rewriter::DefaultPhysicalExprAdapterFactory` · datafusion-physical-expr-adapter 55.1.0

```rust
struct DefaultPhysicalExprAdapterFactory
```

Source: `src/schema_rewriter.rs:185`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-adapter/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-67fc1f7828ce6e01ee4c7685"></a>
## clone

`function` · `datafusion_physical_expr_adapter::schema_rewriter::DefaultPhysicalExprAdapterFactory::clone` · datafusion-physical-expr-adapter 55.1.0

```rust
fn clone(&self) -> DefaultPhysicalExprAdapterFactory
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_adapter::schema_rewriter::DefaultPhysicalExprAdapterFactory", "path": "DefaultPhysicalExprAdapterFactory"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [184, 17], "end": [184, 22], "filename": "src/schema_rewriter.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/schema_rewriter.rs:184`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-adapter/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-90593403c72e05ed5b55962f"></a>
## create

`function` · `datafusion_physical_expr_adapter::schema_rewriter::DefaultPhysicalExprAdapterFactory::create` · datafusion-physical-expr-adapter 55.1.0

```rust
fn create(&self, logical_file_schema: SchemaRef, physical_file_schema: SchemaRef) -> Result<Arc<dyn PhysicalExprAdapter>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_adapter::schema_rewriter::DefaultPhysicalExprAdapterFactory", "path": "DefaultPhysicalExprAdapterFactory"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [187, 1], "end": [198, 2], "filename": "src/schema_rewriter.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr_adapter::schema_rewriter::PhysicalExprAdapterFactory", "path": "PhysicalExprAdapterFactory"}, "trait_path": "datafusion_physical_expr_adapter::schema_rewriter::PhysicalExprAdapterFactory"}`

Source: `src/schema_rewriter.rs:188`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-adapter/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4f55a952097c3a8865ee7a55"></a>
## fmt

`function` · `datafusion_physical_expr_adapter::schema_rewriter::DefaultPhysicalExprAdapterFactory::fmt` · datafusion-physical-expr-adapter 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_adapter::schema_rewriter::DefaultPhysicalExprAdapterFactory", "path": "DefaultPhysicalExprAdapterFactory"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [184, 10], "end": [184, 15], "filename": "src/schema_rewriter.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/schema_rewriter.rs:184`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-adapter/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
