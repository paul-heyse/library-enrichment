# `datafusion_datasource_parquet::access_plan::ParquetRowSelection`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_datasource_parquet.access_plan.ParquetRowSelection.json).

<a id="op-e174196529fc82e4a9f028f8"></a>
## ParquetRowSelection

`struct` · `datafusion_datasource_parquet::access_plan::ParquetRowSelection` · datafusion-datasource-parquet 55.1.0

```rust
struct ParquetRowSelection
```

Source: `src/access_plan.rs:115`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

A file-level row selection for a parquet scan.

Attach this type to a [`PartitionedFile`](datafusion_datasource::PartitionedFile)
with [`PartitionedFile::with_extension`](datafusion_datasource::PartitionedFile::with_extension)
when an external index produces a [`RowSelection`](../operations/parquet.arrow.arrow_reader.selection.RowSelection.md#op-42015b29e86ada83b6a84ed5) across the entire parquet
file. DataFusion will use parquet metadata to split it into row-group-level
access when the file is opened.

Unresolved upstream links (retained, not inferred): `datafusion_datasource::PartitionedFile::with_extension`.

<a id="op-675c54318038f2cecb4e931f"></a>
## clone

`function` · `datafusion_datasource_parquet::access_plan::ParquetRowSelection::clone` · datafusion-datasource-parquet 55.1.0

```rust
fn clone(&self) -> ParquetRowSelection
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_parquet::access_plan::ParquetRowSelection", "path": "ParquetRowSelection"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [114, 17], "end": [114, 22], "filename": "src/access_plan.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/access_plan.rs:114`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3fc0969cc62db535d66c7f99"></a>
## eq

`function` · `datafusion_datasource_parquet::access_plan::ParquetRowSelection::eq` · datafusion-datasource-parquet 55.1.0

```rust
fn eq(&self, other: &ParquetRowSelection) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_parquet::access_plan::ParquetRowSelection", "path": "ParquetRowSelection"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [114, 24], "end": [114, 33], "filename": "src/access_plan.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/access_plan.rs:114`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bfc0b95f2bfab9815bbc8fa1"></a>
## fmt

`function` · `datafusion_datasource_parquet::access_plan::ParquetRowSelection::fmt` · datafusion-datasource-parquet 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_parquet::access_plan::ParquetRowSelection", "path": "ParquetRowSelection"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [114, 10], "end": [114, 15], "filename": "src/access_plan.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/access_plan.rs:114`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d13b0bced27451b256e176da"></a>
## from

`function` · `datafusion_datasource_parquet::access_plan::ParquetRowSelection::from` · datafusion-datasource-parquet 55.1.0

```rust
fn from(selection: RowSelection) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_parquet::access_plan::ParquetRowSelection", "path": "ParquetRowSelection"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [136, 1], "end": [140, 2], "filename": "src/access_plan.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "parquet::arrow::arrow_reader::selection::RowSelection", "path": "RowSelection"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/access_plan.rs:137`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cc8b4fdb0fc8ef61441dff50"></a>
## into_inner

`function` · `datafusion_datasource_parquet::access_plan::ParquetRowSelection::into_inner` · datafusion-datasource-parquet 55.1.0

```rust
fn into_inner(self) -> RowSelection
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_parquet::access_plan::ParquetRowSelection", "path": "ParquetRowSelection"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [119, 1], "end": [134, 2], "filename": "src/access_plan.rs"}, "trait": null, "trait_path": null}`

Source: `src/access_plan.rs:131`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

Convert into the underlying [`RowSelection`](../operations/parquet.arrow.arrow_reader.selection.RowSelection.md#op-42015b29e86ada83b6a84ed5).

<a id="op-0b2fe37b65e1f3f0c8b56291"></a>
## new

`function` · `datafusion_datasource_parquet::access_plan::ParquetRowSelection::new` · datafusion-datasource-parquet 55.1.0

```rust
fn new(selection: RowSelection) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_parquet::access_plan::ParquetRowSelection", "path": "ParquetRowSelection"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [119, 1], "end": [134, 2], "filename": "src/access_plan.rs"}, "trait": null, "trait_path": null}`

Source: `src/access_plan.rs:121`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

Create a new file-level parquet row selection.

<a id="op-957ae11ae23a9cc7ef1047bf"></a>
## selection

`function` · `datafusion_datasource_parquet::access_plan::ParquetRowSelection::selection` · datafusion-datasource-parquet 55.1.0

```rust
fn selection(&self) -> &RowSelection
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_parquet::access_plan::ParquetRowSelection", "path": "ParquetRowSelection"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [119, 1], "end": [134, 2], "filename": "src/access_plan.rs"}, "trait": null, "trait_path": null}`

Source: `src/access_plan.rs:126`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

Return a reference to the underlying [`RowSelection`](../operations/parquet.arrow.arrow_reader.selection.RowSelection.md#op-42015b29e86ada83b6a84ed5).
