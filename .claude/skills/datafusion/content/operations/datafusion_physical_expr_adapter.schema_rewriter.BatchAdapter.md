# `datafusion_physical_expr_adapter::schema_rewriter::BatchAdapter`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_expr_adapter.schema_rewriter.BatchAdapter.json).

<a id="op-edcc6a5fbe6af1e296777540"></a>
## BatchAdapter

`struct` · `datafusion_physical_expr_adapter::schema_rewriter::BatchAdapter` · datafusion-physical-expr-adapter 55.1.0

```rust
struct BatchAdapter
```

Source: `src/schema_rewriter.rs:780`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-adapter/55.1.0/json).

Adapter for transforming record batches to match a target schema.

Create instances via [`BatchAdapterFactory`](../operations/datafusion_physical_expr_adapter.schema_rewriter.BatchAdapterFactory.md#op-739e1733de121a3801ee33b4).

## Performance

The adapter pre-computes the projection expressions during creation,
so the [`adapt_batch`](BatchAdapter::adapt_batch) call is efficient and suitable
for use in hot paths like streaming file scans.

<a id="op-2260c99b7086f11d9eccad2e"></a>
## adapt_batch

`function` · `datafusion_physical_expr_adapter::schema_rewriter::BatchAdapter::adapt_batch` · datafusion-physical-expr-adapter 55.1.0

```rust
fn adapt_batch(&self, batch: &RecordBatch) -> Result<RecordBatch>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_adapter::schema_rewriter::BatchAdapter", "path": "BatchAdapter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [784, 1], "end": [792, 2], "filename": "src/schema_rewriter.rs"}, "trait": null, "trait_path": null}`

Source: `src/schema_rewriter.rs:789`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-adapter/55.1.0/json).

Adapt the given record batch to match the target schema.

The input batch *must* conform to the source schema used when
creating this adapter.

<a id="op-be3931efa38ce26f63616890"></a>
## fmt

`function` · `datafusion_physical_expr_adapter::schema_rewriter::BatchAdapter::fmt` · datafusion-physical-expr-adapter 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_adapter::schema_rewriter::BatchAdapter", "path": "BatchAdapter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [779, 10], "end": [779, 15], "filename": "src/schema_rewriter.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/schema_rewriter.rs:779`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-adapter/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
