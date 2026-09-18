# `datafusion_expr::expr::intersect_metadata_for_union`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.expr.intersect_metadata_for_union.json).

<a id="op-022c8de0da3ba871d91b1850"></a>
## intersect_metadata_for_union

`function` · `datafusion_expr::expr::intersect_metadata_for_union` · datafusion-expr 55.1.0

```rust
fn intersect_metadata_for_union<'a>(metadatas: impl IntoIterator<Item = &'a SchemaFieldMetadata>) -> SchemaFieldMetadata
```

Source: `src/expr.rs:648`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Intersects multiple metadata instances for UNION operations.

This function implements the intersection strategy used by UNION operations,
where only metadata keys that exist in ALL inputs with identical values
are preserved in the result.

# Union Metadata Behavior

Union operations require consistent metadata across all branches:
- Only metadata keys present in ALL union branches are kept
- For each kept key, the value must be identical across all branches
- If a key has different values across branches, it is excluded from the result
- If any input has no metadata, the result will be empty

# Arguments

* `metadatas` - An iterator of `SchemaFieldMetadata` instances to intersect

# Returns

A new `SchemaFieldMetadata` containing only the intersected metadata
