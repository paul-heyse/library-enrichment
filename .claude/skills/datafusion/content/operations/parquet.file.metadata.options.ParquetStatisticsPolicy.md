# `parquet::file::metadata::options::ParquetStatisticsPolicy`

Full upstream contracts; raw type trees and source locators in [structured records](parquet.file.metadata.options.ParquetStatisticsPolicy.json).

<a id="op-e15152f3d6d699122954ea6d"></a>
## ParquetStatisticsPolicy

`enum` · `parquet::file::metadata::options::ParquetStatisticsPolicy` · parquet 59.3.0

```rust
enum ParquetStatisticsPolicy
```

Source: `src/file/metadata/options.rs:48`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Enum to control decoding of some Parquet statistics fields.

# Example
```rust
use parquet::file::metadata::ParquetStatisticsPolicy;
use parquet::file::serialized_reader::ReadOptionsBuilder;
use parquet::arrow::arrow_reader::ArrowReaderOptions;

// Set arrow options to skip encoding statistics for all columns.
let options =
    ArrowReaderOptions::new().with_encoding_stats_policy(ParquetStatisticsPolicy::SkipAll);

// Set serialized reader options to decode encoding statistics for all columns.
let options =
    ReadOptionsBuilder::new().with_encoding_stats_policy(ParquetStatisticsPolicy::KeepAll)
    .build();

// Set arrow options to skip encoding statistics for all columns, but to decode statistics
// for columns 0 and 1.
let options = ArrowReaderOptions::new()
    .with_encoding_stats_policy(ParquetStatisticsPolicy::skip_except(&[0, 1]));
```

<a id="op-83ebc5ea36be1fd08428d206"></a>
## KeepAll

`variant` · `parquet::file::metadata::options::ParquetStatisticsPolicy::KeepAll` · parquet 59.3.0

```rust
KeepAll
```

Source: `src/file/metadata/options.rs:51`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Decode the relevant statistics for all columns.

<a id="op-b5e6b3c2a05134ff9a19845a"></a>
## SkipAll

`variant` · `parquet::file::metadata::options::ParquetStatisticsPolicy::SkipAll` · parquet 59.3.0

```rust
SkipAll
```

Source: `src/file/metadata/options.rs:53`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Skip decoding the relevant statistics for all columns.

<a id="op-ac5f293e32f9a22cff69e8d3"></a>
## SkipExcept

`variant` · `parquet::file::metadata::options::ParquetStatisticsPolicy::SkipExcept` · parquet 59.3.0

```rust
SkipExcept
```

Source: `src/file/metadata/options.rs:56`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Skip decoding the relevant statistics for all columns not in the provided set
of column indices.

<a id="op-75304c1433c2f635f3f35d19"></a>
## clone

`function` · `parquet::file::metadata::options::ParquetStatisticsPolicy::clone` · parquet 59.3.0

```rust
fn clone(&self) -> ParquetStatisticsPolicy
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::options::ParquetStatisticsPolicy", "path": "ParquetStatisticsPolicy"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [47, 26], "end": [47, 31], "filename": "src/file/metadata/options.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/file/metadata/options.rs:47`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-02cd792b4fb0b817da5a0ab0"></a>
## default

`function` · `parquet::file::metadata::options::ParquetStatisticsPolicy::default` · parquet 59.3.0

```rust
fn default() -> ParquetStatisticsPolicy
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::options::ParquetStatisticsPolicy", "path": "ParquetStatisticsPolicy"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [47, 10], "end": [47, 17], "filename": "src/file/metadata/options.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/file/metadata/options.rs:47`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ef79e185267aae46527ff3a2"></a>
## fmt

`function` · `parquet::file::metadata::options::ParquetStatisticsPolicy::fmt` · parquet 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::options::ParquetStatisticsPolicy", "path": "ParquetStatisticsPolicy"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [47, 19], "end": [47, 24], "filename": "src/file/metadata/options.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/file/metadata/options.rs:47`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2b1b85c25f2b2be5f99f7855"></a>
## skip_except

`function` · `parquet::file::metadata::options::ParquetStatisticsPolicy::skip_except` · parquet 59.3.0

```rust
fn skip_except(keep: &[usize]) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::options::ParquetStatisticsPolicy", "path": "ParquetStatisticsPolicy"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [59, 1], "end": [81, 2], "filename": "src/file/metadata/options.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/metadata/options.rs:63`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Create a `ParquetStatisticsPolicy` to skip all columns except those in `keep`.

If `keep` is empty, then this returns [`Self::SkipAll`](../operations/parquet.file.metadata.options.ParquetStatisticsPolicy.md#op-b5e6b3c2a05134ff9a19845a)
