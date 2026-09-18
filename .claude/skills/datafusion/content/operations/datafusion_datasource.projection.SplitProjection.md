# `datafusion_datasource::projection::SplitProjection`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_datasource.projection.SplitProjection.json).

<a id="op-96e2e4a1d0fd5ad62b84edc5"></a>
## SplitProjection

`struct` · `datafusion_datasource::projection::SplitProjection` · datafusion-datasource 55.1.0

```rust
struct SplitProjection
```

Source: `src/projection.rs:166`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

At a high level the goal of SplitProjection is to take a ProjectionExprs meant to be applied to the table schema
and split that into:
- The projection indices into the file schema (file_indices)
- The projection indices into the partition values (partition_value_indices), which pre-compute both the index into the table schema
  and the index into the partition values array
- A remapped projection that can be applied after the file projection is applied
  This remapped projection has the following properties:
    - Column indices referring to file columns are remapped to [0..file_indices.len())
    - Column indices referring to partition columns are remapped to [file_indices.len()..)

  This allows the ProjectionOpener to easily identify which columns in the remapped projection
  refer to partition columns and substitute them with literals from the partition values.

<a id="op-b005fb36536dd10a0b7d92d4"></a>
## clone

`function` · `datafusion_datasource::projection::SplitProjection::clone` · datafusion-datasource 55.1.0

```rust
fn clone(&self) -> SplitProjection
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::projection::SplitProjection", "path": "SplitProjection"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [165, 17], "end": [165, 22], "filename": "src/projection.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/projection.rs:165`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-db669a6411f08e0ec83d190c"></a>
## file_indices

`struct_field` · `datafusion_datasource::projection::SplitProjection::file_indices` · datafusion-datasource 55.1.0

```rust
file_indices: Vec<usize>
```

Source: `src/projection.rs:170`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Column indices to read from file (public for file sources)

<a id="op-b069d6f3f7baf4363ce88687"></a>
## fmt

`function` · `datafusion_datasource::projection::SplitProjection::fmt` · datafusion-datasource 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::projection::SplitProjection", "path": "SplitProjection"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [165, 10], "end": [165, 15], "filename": "src/projection.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/projection.rs:165`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cc714f2f8fa8455578dd901b"></a>
## new

`function` · `datafusion_datasource::projection::SplitProjection::new` · datafusion-datasource 55.1.0

```rust
fn new(logical_file_schema: &Schema, projection: &ProjectionExprs) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::projection::SplitProjection", "path": "SplitProjection"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [177, 1], "end": [291, 2], "filename": "src/projection.rs"}, "trait": null, "trait_path": null}`

Source: `src/projection.rs:196`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Creates a new [`SplitProjection`](../operations/datafusion_datasource.projection.SplitProjection.md#op-96e2e4a1d0fd5ad62b84edc5) by splitting a projection into
simple file column indices and a remainder projection that is applied after reading the file.

In other words: we get a `Vec<usize>` projection that is meant to be applied on top of `file_schema`
and a remainder projection that is applied to the result of that first projection.

Here `file_schema` is expected to be the *logical* schema of the file, that is the
table schema minus any partition columns.
Partition columns are always expected to be at the end of the table schema.
Note that `file_schema` is *not* the physical schema of the file.

<a id="op-4a80a52d301d96783e368fe8"></a>
## source

`struct_field` · `datafusion_datasource::projection::SplitProjection::source` · datafusion-datasource 55.1.0

```rust
source: datafusion_physical_expr::projection::ProjectionExprs
```

Source: `src/projection.rs:168`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

The original projection this [`SplitProjection`](../operations/datafusion_datasource.projection.SplitProjection.md#op-96e2e4a1d0fd5ad62b84edc5) was derived from

<a id="op-ce210bc243e539502983cee8"></a>
## unprojected

`function` · `datafusion_datasource::projection::SplitProjection::unprojected` · datafusion-datasource 55.1.0

```rust
fn unprojected(table_schema: &TableSchema) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::projection::SplitProjection", "path": "SplitProjection"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [177, 1], "end": [291, 2], "filename": "src/projection.rs"}, "trait": null, "trait_path": null}`

Source: `src/projection.rs:178`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
