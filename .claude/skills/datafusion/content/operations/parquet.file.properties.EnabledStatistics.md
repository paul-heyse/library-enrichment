# `parquet::file::properties::EnabledStatistics`

Full upstream contracts; raw type trees and source locators in [structured records](parquet.file.properties.EnabledStatistics.json).

<a id="op-0bdf156eacebacc657f23a92"></a>
## EnabledStatistics

`enum` · `parquet::file::properties::EnabledStatistics` · parquet 59.3.0

```rust
enum EnabledStatistics
```

Source: `src/file/properties.rs:1367`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Controls the level of statistics to be computed by the writer and stored in
the parquet file.

Enabling statistics makes the resulting Parquet file larger and requires
more time to read the parquet footer.

Statistics can be used to improve query performance by pruning row groups
and pages during query execution if the query engine supports evaluating the
predicate using the statistics.

<a id="op-de078fa8f540ac9d7d3949a2"></a>
## Chunk

`variant` · `parquet::file::properties::EnabledStatistics::Chunk` · parquet 59.3.0

```rust
Chunk
```

Source: `src/file/properties.rs:1375`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Compute column chunk-level statistics but not page-level.

Setting this option will store one set of statistics for each relevant
column for each row group. The more row groups written, the more
statistics will be stored.

<a id="op-ade089ff9507137cca75d3b1"></a>
## Err

`assoc_type` · `parquet::file::properties::EnabledStatistics::Err` · parquet 59.3.0

```rust
Err
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::properties::EnabledStatistics", "path": "EnabledStatistics"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1388, 1], "end": [1399, 2], "filename": "src/file/properties.rs"}, "trait": {"args": null, "id": "core::str::traits::FromStr", "path": "FromStr"}, "trait_path": "core::str::traits::FromStr"}`

Source: `src/file/properties.rs:1389`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a54aec7b98f8822fdcc64f83"></a>
## None

`variant` · `parquet::file::properties::EnabledStatistics::None` · parquet 59.3.0

```rust
None
```

Source: `src/file/properties.rs:1369`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Compute no statistics.

<a id="op-918e760bc6c733e101ef05af"></a>
## Page

`variant` · `parquet::file::properties::EnabledStatistics::Page` · parquet 59.3.0

```rust
Page
```

Source: `src/file/properties.rs:1385`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Compute page-level and column chunk-level statistics.

Setting this option will store one set of statistics for each relevant
column for each row group. In addition, this will enable the writing
of the column index (the offset index is always written regardless of
this setting). See [`ParquetColumnIndex`] for
more information.

[`ParquetColumnIndex`]: crate::file::metadata::ParquetColumnIndex

<a id="op-d7c2c325e87112a851e58475"></a>
## clone

`function` · `parquet::file::properties::EnabledStatistics::clone` · parquet 59.3.0

```rust
fn clone(&self) -> EnabledStatistics
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::properties::EnabledStatistics", "path": "EnabledStatistics"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1366, 17], "end": [1366, 22], "filename": "src/file/properties.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/file/properties.rs:1366`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e02acb68a9524eade1531cdc"></a>
## default

`function` · `parquet::file::properties::EnabledStatistics::default` · parquet 59.3.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::properties::EnabledStatistics", "path": "EnabledStatistics"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1401, 1], "end": [1405, 2], "filename": "src/file/properties.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/file/properties.rs:1402`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d5ad6a67a7d801ef91ef96fa"></a>
## eq

`function` · `parquet::file::properties::EnabledStatistics::eq` · parquet 59.3.0

```rust
fn eq(&self, other: &EnabledStatistics) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::properties::EnabledStatistics", "path": "EnabledStatistics"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1366, 34], "end": [1366, 43], "filename": "src/file/properties.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/file/properties.rs:1366`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-65a0e4056cf7ba342c165ecf"></a>
## fmt

`function` · `parquet::file::properties::EnabledStatistics::fmt` · parquet 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::properties::EnabledStatistics", "path": "EnabledStatistics"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1366, 10], "end": [1366, 15], "filename": "src/file/properties.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/file/properties.rs:1366`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-358d1dc5cfcbb7d57ece146c"></a>
## from_str

`function` · `parquet::file::properties::EnabledStatistics::from_str` · parquet 59.3.0

```rust
fn from_str(s: &str) -> Result<Self, Self::Err>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::properties::EnabledStatistics", "path": "EnabledStatistics"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1388, 1], "end": [1399, 2], "filename": "src/file/properties.rs"}, "trait": {"args": null, "id": "core::str::traits::FromStr", "path": "FromStr"}, "trait_path": "core::str::traits::FromStr"}`

Source: `src/file/properties.rs:1391`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.
