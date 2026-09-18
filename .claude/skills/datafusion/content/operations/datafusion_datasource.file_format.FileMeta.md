# `datafusion_datasource::file_format::FileMeta`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_datasource.file_format.FileMeta.json).

<a id="op-cd57760f68fb9b912453fbfb"></a>
## FileMeta

`struct` · `datafusion_datasource::file_format::FileMeta` · datafusion-datasource 55.1.0

```rust
struct FileMeta
```

Source: `src/file_format.rs:51`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Metadata fetched from a file, including statistics and ordering.

This struct is returned by [`FileFormat::infer_stats_and_ordering`](../operations/datafusion_datasource.file_format.FileFormat.md#op-a37e5aef3886096f673ca012) to
provide all metadata in a single read, avoiding duplicate I/O operations.

<a id="op-49da4a714efa290b8357c3ae"></a>
## clone

`function` · `datafusion_datasource::file_format::FileMeta::clone` · datafusion-datasource 55.1.0

```rust
fn clone(&self) -> FileMeta
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::file_format::FileMeta", "path": "FileMeta"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [49, 17], "end": [49, 22], "filename": "src/file_format.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/file_format.rs:49`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-63ebd07313d569862c72fb71"></a>
## fmt

`function` · `datafusion_datasource::file_format::FileMeta::fmt` · datafusion-datasource 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::file_format::FileMeta", "path": "FileMeta"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [49, 10], "end": [49, 15], "filename": "src/file_format.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/file_format.rs:49`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-efa5114a8e2a452b2c943cc9"></a>
## new

`function` · `datafusion_datasource::file_format::FileMeta::new` · datafusion-datasource 55.1.0

```rust
fn new(statistics: Statistics) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::file_format::FileMeta", "path": "FileMeta"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [58, 1], "end": [72, 2], "filename": "src/file_format.rs"}, "trait": null, "trait_path": null}`

Source: `src/file_format.rs:60`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Creates a new `FileMeta` with the given statistics and no ordering.

<a id="op-1aec00c07552cced1512278b"></a>
## ordering

`struct_field` · `datafusion_datasource::file_format::FileMeta::ordering` · datafusion-datasource 55.1.0

```rust
ordering: Option<datafusion_physical_expr_common::sort_expr::LexOrdering>
```

Source: `src/file_format.rs:55`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

The ordering (sort order) of the file, if known.

<a id="op-b5696288f4c12d4282480cdf"></a>
## statistics

`struct_field` · `datafusion_datasource::file_format::FileMeta::statistics` · datafusion-datasource 55.1.0

```rust
statistics: datafusion_common::Statistics
```

Source: `src/file_format.rs:53`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Statistics for the file (row counts, byte sizes, column statistics).

<a id="op-f820bfac6666750b44a7981e"></a>
## with_ordering

`function` · `datafusion_datasource::file_format::FileMeta::with_ordering` · datafusion-datasource 55.1.0

```rust
fn with_ordering(self, ordering: Option<LexOrdering>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::file_format::FileMeta", "path": "FileMeta"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [58, 1], "end": [72, 2], "filename": "src/file_format.rs"}, "trait": null, "trait_path": null}`

Source: `src/file_format.rs:68`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Sets the ordering for this file metadata.
