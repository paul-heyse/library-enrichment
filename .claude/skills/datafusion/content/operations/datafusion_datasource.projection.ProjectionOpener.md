# `datafusion_datasource::projection::ProjectionOpener`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_datasource.projection.ProjectionOpener.json).

<a id="op-331d2f61348c5625701ec83d"></a>
## ProjectionOpener

`struct` · `datafusion_datasource::projection::ProjectionOpener` · datafusion-datasource 55.1.0

```rust
struct ProjectionOpener
```

Source: `src/projection.rs:48`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

A file opener that handles applying a projection on top of an inner opener.

This includes handling partition columns.

Any projection pushed down will be split up into:
- Simple column indices / column selection
- A remainder projection that this opener applies on top of it

This is meant to simplify projection pushdown for sources like CSV
that can only handle "simple" column selection.

<a id="op-4a6c76585d2aa8967a691130"></a>
## open

`function` · `datafusion_datasource::projection::ProjectionOpener::open` · datafusion-datasource 55.1.0

```rust
fn open(&self, partitioned_file: PartitionedFile) -> Result<FileOpenFuture>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::projection::ProjectionOpener", "path": "ProjectionOpener"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [70, 1], "end": [105, 2], "filename": "src/projection.rs"}, "trait": {"args": null, "id": "datafusion_datasource::file_stream::FileOpener", "path": "FileOpener"}, "trait_path": "datafusion_datasource::file_stream::FileOpener"}`

Source: `src/projection.rs:71`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-916444de749f7912b6a2ab9a"></a>
## try_new

`function` · `datafusion_datasource::projection::ProjectionOpener::try_new` · datafusion-datasource 55.1.0

```rust
fn try_new(projection: SplitProjection, inner: Arc<dyn FileOpener>, file_schema: &Schema) -> Result<Arc<dyn FileOpener>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::projection::ProjectionOpener", "path": "ProjectionOpener"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [55, 1], "end": [68, 2], "filename": "src/projection.rs"}, "trait": null, "trait_path": null}`

Source: `src/projection.rs:56`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
