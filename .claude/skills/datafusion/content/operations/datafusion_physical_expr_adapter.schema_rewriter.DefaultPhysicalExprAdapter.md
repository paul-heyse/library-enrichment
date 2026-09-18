# `datafusion_physical_expr_adapter::schema_rewriter::DefaultPhysicalExprAdapter`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_expr_adapter.schema_rewriter.DefaultPhysicalExprAdapter.json).

<a id="op-0786dd5cec45ace7db749372"></a>
## DefaultPhysicalExprAdapter

`struct` · `datafusion_physical_expr_adapter::schema_rewriter::DefaultPhysicalExprAdapter` · datafusion-physical-expr-adapter 55.1.0

```rust
struct DefaultPhysicalExprAdapter
```

Source: `src/schema_rewriter.rs:242`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-adapter/55.1.0/json).

Default implementation of [`PhysicalExprAdapter`](../operations/datafusion_physical_expr_adapter.schema_rewriter.PhysicalExprAdapter.md#op-3d7f05fc0e38d49de65f327a) for rewriting physical
expressions to match different schemas.

## Overview

 [`DefaultPhysicalExprAdapter`](../operations/datafusion_physical_expr_adapter.schema_rewriter.DefaultPhysicalExprAdapter.md#op-0786dd5cec45ace7db749372) rewrites physical expressions to match
 different schemas, including:

- **Type casting**: When logical and physical schemas have different types, expressions are
  automatically wrapped with cast operations. For example, `lit(ScalarValue::Int32(123)) = int64_column`
  gets rewritten to `lit(ScalarValue::Int32(123)) = cast(int64_column, 'Int32')`.
  Note that this does not attempt to simplify such expressions - that is done by shared simplifiers.

- **Missing columns**: When a column exists in the logical schema but not in the physical schema,
  references to it are replaced with null literals.

- **Struct field access**: Expressions like `struct_column.field_that_is_missing_in_schema` are
  rewritten to `null` when the field doesn't exist in the physical schema.

- **Default column values**: Partition column references can be replaced with their literal values
  when scanning specific partitions. See [`replace_columns_with_literals`](../operations/datafusion_physical_expr_adapter.schema_rewriter.replace_columns_with_literals.md#op-c25b062649f7c614e6bac900) for more details.

# Example

```rust
# use datafusion_physical_expr_adapter::{DefaultPhysicalExprAdapterFactory, PhysicalExprAdapterFactory};
# use arrow::datatypes::Schema;
# use std::sync::Arc;
#
# fn example(
#     predicate: std::sync::Arc<dyn datafusion_physical_expr_common::physical_expr::PhysicalExpr>,
#     physical_file_schema: &Schema,
#     logical_file_schema: &Schema,
# ) -> datafusion_common::Result<()> {
let factory = DefaultPhysicalExprAdapterFactory;
let adapter =
    factory.create(Arc::new(logical_file_schema.clone()), Arc::new(physical_file_schema.clone()))?;
let adapted_predicate = adapter.rewrite(predicate)?;
# Ok(())
# }
```

<a id="op-7585ef8459f8bbb38293462d"></a>
## clone

`function` · `datafusion_physical_expr_adapter::schema_rewriter::DefaultPhysicalExprAdapter::clone` · datafusion-physical-expr-adapter 55.1.0

```rust
fn clone(&self) -> DefaultPhysicalExprAdapter
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_adapter::schema_rewriter::DefaultPhysicalExprAdapter", "path": "DefaultPhysicalExprAdapter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [241, 17], "end": [241, 22], "filename": "src/schema_rewriter.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/schema_rewriter.rs:241`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-adapter/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-12ffe1f132931984f7c4c13e"></a>
## fmt

`function` · `datafusion_physical_expr_adapter::schema_rewriter::DefaultPhysicalExprAdapter::fmt` · datafusion-physical-expr-adapter 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_adapter::schema_rewriter::DefaultPhysicalExprAdapter", "path": "DefaultPhysicalExprAdapter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [241, 10], "end": [241, 15], "filename": "src/schema_rewriter.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/schema_rewriter.rs:241`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-adapter/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2d3afd0a6acfe54ff0dbfe89"></a>
## new

`function` · `datafusion_physical_expr_adapter::schema_rewriter::DefaultPhysicalExprAdapter::new` · datafusion-physical-expr-adapter 55.1.0

```rust
fn new(logical_file_schema: SchemaRef, physical_file_schema: SchemaRef) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_adapter::schema_rewriter::DefaultPhysicalExprAdapter", "path": "DefaultPhysicalExprAdapter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [247, 1], "end": [258, 2], "filename": "src/schema_rewriter.rs"}, "trait": null, "trait_path": null}`

Source: `src/schema_rewriter.rs:252`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-adapter/55.1.0/json).

Create a new instance of the default physical expression adapter.

This adapter rewrites expressions to match the physical schema of the file being scanned,
handling type mismatches and missing columns by filling them with default values.

<a id="op-8eb7b63de62f59f85f73d2f5"></a>
## rewrite

`function` · `datafusion_physical_expr_adapter::schema_rewriter::DefaultPhysicalExprAdapter::rewrite` · datafusion-physical-expr-adapter 55.1.0

```rust
fn rewrite(&self, expr: Arc<dyn PhysicalExpr>) -> Result<Arc<dyn PhysicalExpr>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_adapter::schema_rewriter::DefaultPhysicalExprAdapter", "path": "DefaultPhysicalExprAdapter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [260, 1], "end": [269, 2], "filename": "src/schema_rewriter.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr_adapter::schema_rewriter::PhysicalExprAdapter", "path": "PhysicalExprAdapter"}, "trait_path": "datafusion_physical_expr_adapter::schema_rewriter::PhysicalExprAdapter"}`

Source: `src/schema_rewriter.rs:261`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-adapter/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
