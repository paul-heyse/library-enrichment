# `parquet::file::metadata::options::ParquetMetaDataOptions`

Full upstream contracts; raw type trees and source locators in [structured records](parquet.file.metadata.options.ParquetMetaDataOptions.json).

<a id="op-4261537800a4fe33ff35b722"></a>
## ParquetMetaDataOptions

`struct` · `parquet::file::metadata::options::ParquetMetaDataOptions` · parquet 59.3.0

```rust
struct ParquetMetaDataOptions
```

Source: `src/file/metadata/options.rs:91`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Options that can be set to control what parts of the Parquet file footer
metadata will be decoded and made present in the [`ParquetMetaData`] returned
by [`ParquetMetaDataReader`] and [`ParquetMetaDataPushDecoder`].

[`ParquetMetaData`]: crate::file::metadata::ParquetMetaData
[`ParquetMetaDataReader`]: crate::file::metadata::ParquetMetaDataReader
[`ParquetMetaDataPushDecoder`]: crate::file::metadata::ParquetMetaDataPushDecoder

<a id="op-6a5c47d1bb865a4c8466a731"></a>
## clone

`function` · `parquet::file::metadata::options::ParquetMetaDataOptions::clone` · parquet 59.3.0

```rust
fn clone(&self) -> ParquetMetaDataOptions
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::options::ParquetMetaDataOptions", "path": "ParquetMetaDataOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [90, 17], "end": [90, 22], "filename": "src/file/metadata/options.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/file/metadata/options.rs:90`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ef73939ff6090351707ee120"></a>
## default

`function` · `parquet::file::metadata::options::ParquetMetaDataOptions::default` · parquet 59.3.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::options::ParquetMetaDataOptions", "path": "ParquetMetaDataOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [99, 1], "end": [109, 2], "filename": "src/file/metadata/options.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/file/metadata/options.rs:100`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f64521799082fc25b1cd282b"></a>
## encoding_stats_as_mask

`function` · `parquet::file::metadata::options::ParquetMetaDataOptions::encoding_stats_as_mask` · parquet 59.3.0

```rust
fn encoding_stats_as_mask(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::options::ParquetMetaDataOptions", "path": "ParquetMetaDataOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [111, 1], "end": [247, 2], "filename": "src/file/metadata/options.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/metadata/options.rs:144`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns whether to present the [`encoding_stats`] field of the Parquet `ColumnMetaData`
as a bitmask (defaults to `true`).

See [`ColumnChunkMetaData::page_encoding_stats_mask`] for an explanation of why this
might be desirable.

[`ColumnChunkMetaData::page_encoding_stats_mask`]:
crate::file::metadata::ColumnChunkMetaData::page_encoding_stats_mask
[`encoding_stats`]:
https://github.com/apache/parquet-format/blob/786142e26740487930ddc3ec5e39d780bd930907/src/main/thrift/parquet.thrift#L917

<a id="op-7345e974d86ed4b055638ccf"></a>
## fmt

`function` · `parquet::file::metadata::options::ParquetMetaDataOptions::fmt` · parquet 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::options::ParquetMetaDataOptions", "path": "ParquetMetaDataOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [90, 10], "end": [90, 15], "filename": "src/file/metadata/options.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/file/metadata/options.rs:90`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-711a1bc544f5d47f2d08cfdc"></a>
## new

`function` · `parquet::file::metadata::options::ParquetMetaDataOptions::new` · parquet 59.3.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::options::ParquetMetaDataOptions", "path": "ParquetMetaDataOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [111, 1], "end": [247, 2], "filename": "src/file/metadata/options.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/metadata/options.rs:113`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Return a new default [`ParquetMetaDataOptions`](../operations/parquet.file.metadata.options.ParquetMetaDataOptions.md#op-4261537800a4fe33ff35b722).

<a id="op-1b3cc07c1e9683b03404c32a"></a>
## schema

`function` · `parquet::file::metadata::options::ParquetMetaDataOptions::schema` · parquet 59.3.0

```rust
fn schema(&self) -> Option<&SchemaDescPtr>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::options::ParquetMetaDataOptions", "path": "ParquetMetaDataOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [111, 1], "end": [247, 2], "filename": "src/file/metadata/options.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/metadata/options.rs:119`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns an optional [`SchemaDescPtr`](../operations/parquet.schema.types.SchemaDescPtr.md#op-eb89590f5ea7f99df17bf043) to use when decoding. If this is not `None` then
the schema in the footer will be skipped.

<a id="op-4a24f0f2b7d4d7dff65204f7"></a>
## set_column_stats_policy

`function` · `parquet::file::metadata::options::ParquetMetaDataOptions::set_column_stats_policy` · parquet 59.3.0

```rust
fn set_column_stats_policy(&mut self, policy: ParquetStatisticsPolicy)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::options::ParquetMetaDataOptions", "path": "ParquetMetaDataOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [111, 1], "end": [247, 2], "filename": "src/file/metadata/options.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/metadata/options.rs:213`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Sets the decoding policy for [`statistics`] in the Parquet `ColumnMetaData`.

The default policy is to decode all `statistics`.

[`statistics`]:
https://github.com/apache/parquet-format/blob/786142e26740487930ddc3ec5e39d780bd930907/src/main/thrift/parquet.thrift#L912

<a id="op-2a5f6446514f2b744ce7034e"></a>
## set_encoding_stats_as_mask

`function` · `parquet::file::metadata::options::ParquetMetaDataOptions::set_encoding_stats_as_mask` · parquet 59.3.0

```rust
fn set_encoding_stats_as_mask(&mut self, val: bool)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::options::ParquetMetaDataOptions", "path": "ParquetMetaDataOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [111, 1], "end": [247, 2], "filename": "src/file/metadata/options.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/metadata/options.rs:161`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Convert [`encoding_stats`] from a vector of [`PageEncodingStats`] to a bitmask. This can
speed up metadata decoding while still enabling some use cases served by the full stats.

Note that if for a given column both this option and `skip_encoding_stats` are `true`, the
stats will be skipped and not be returned as a mask.

See [`ColumnChunkMetaData::page_encoding_stats_mask`] for more information.

[`PageEncodingStats`]: crate::file::metadata::PageEncodingStats
[`ColumnChunkMetaData::page_encoding_stats_mask`]:
crate::file::metadata::ColumnChunkMetaData::page_encoding_stats_mask
[`encoding_stats`]:
https://github.com/apache/parquet-format/blob/786142e26740487930ddc3ec5e39d780bd930907/src/main/thrift/parquet.thrift#L917

<a id="op-4f85a7c3775956b8cf607d22"></a>
## set_encoding_stats_policy

`function` · `parquet::file::metadata::options::ParquetMetaDataOptions::set_encoding_stats_policy` · parquet 59.3.0

```rust
fn set_encoding_stats_policy(&mut self, policy: ParquetStatisticsPolicy)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::options::ParquetMetaDataOptions", "path": "ParquetMetaDataOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [111, 1], "end": [247, 2], "filename": "src/file/metadata/options.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/metadata/options.rs:188`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Sets the decoding policy for [`encoding_stats`] in the Parquet `ColumnMetaData`.

The default policy is to decode all `encoding_stats`.

This option takes precedence over [`Self::encoding_stats_as_mask`](../operations/parquet.file.metadata.options.ParquetMetaDataOptions.md#op-f64521799082fc25b1cd282b).

[`encoding_stats`]:
https://github.com/apache/parquet-format/blob/786142e26740487930ddc3ec5e39d780bd930907/src/main/thrift/parquet.thrift#L917

<a id="op-c9458f834d308e5c3ea8f43a"></a>
## set_schema

`function` · `parquet::file::metadata::options::ParquetMetaDataOptions::set_schema` · parquet 59.3.0

```rust
fn set_schema(&mut self, val: SchemaDescPtr)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::options::ParquetMetaDataOptions", "path": "ParquetMetaDataOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [111, 1], "end": [247, 2], "filename": "src/file/metadata/options.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/metadata/options.rs:124`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Provide a schema to use when decoding the metadata.

<a id="op-c4b3a64bfc53fa8e226e87cc"></a>
## set_size_stats_policy

`function` · `parquet::file::metadata::options::ParquetMetaDataOptions::set_size_stats_policy` · parquet 59.3.0

```rust
fn set_size_stats_policy(&mut self, policy: ParquetStatisticsPolicy)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::options::ParquetMetaDataOptions", "path": "ParquetMetaDataOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [111, 1], "end": [247, 2], "filename": "src/file/metadata/options.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/metadata/options.rs:238`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Sets the decoding policy for [`size_statistics`] in the Parquet `ColumnMetaData`.

The default policy is to decode all `size_statistics`.

[`size_statistics`]:
https://github.com/apache/parquet-format/blob/786142e26740487930ddc3ec5e39d780bd930907/src/main/thrift/parquet.thrift#L936

<a id="op-3984bde4417411a03c65df76"></a>
## skip_column_stats

`function` · `parquet::file::metadata::options::ParquetMetaDataOptions::skip_column_stats` · parquet 59.3.0

```rust
fn skip_column_stats(&self, col_index: usize) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::options::ParquetMetaDataOptions", "path": "ParquetMetaDataOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [111, 1], "end": [247, 2], "filename": "src/file/metadata/options.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/metadata/options.rs:203`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns whether to skip decoding the [`statistics`] in the Parquet `ColumnMetaData`
for the column indexed by `col_index`.

[`statistics`]:
https://github.com/apache/parquet-format/blob/786142e26740487930ddc3ec5e39d780bd930907/src/main/thrift/parquet.thrift#L912

<a id="op-d89ab9ed6959f4a4162a698e"></a>
## skip_encoding_stats

`function` · `parquet::file::metadata::options::ParquetMetaDataOptions::skip_encoding_stats` · parquet 59.3.0

```rust
fn skip_encoding_stats(&self, col_index: usize) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::options::ParquetMetaDataOptions", "path": "ParquetMetaDataOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [111, 1], "end": [247, 2], "filename": "src/file/metadata/options.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/metadata/options.rs:176`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns whether to skip decoding the [`encoding_stats`] in the Parquet `ColumnMetaData`
for the column indexed by `col_index`.

[`encoding_stats`]:
https://github.com/apache/parquet-format/blob/786142e26740487930ddc3ec5e39d780bd930907/src/main/thrift/parquet.thrift#L917

<a id="op-855313c07250cece76b82608"></a>
## skip_size_stats

`function` · `parquet::file::metadata::options::ParquetMetaDataOptions::skip_size_stats` · parquet 59.3.0

```rust
fn skip_size_stats(&self, col_index: usize) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::options::ParquetMetaDataOptions", "path": "ParquetMetaDataOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [111, 1], "end": [247, 2], "filename": "src/file/metadata/options.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/metadata/options.rs:228`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns whether to skip decoding the [`size_statistics`] in the Parquet `ColumnMetaData`
for the column indexed by `col_index`.

[`size_statistics`]:
https://github.com/apache/parquet-format/blob/786142e26740487930ddc3ec5e39d780bd930907/src/main/thrift/parquet.thrift#L936

<a id="op-8f70f5683c8ae909c4a8422d"></a>
## with_column_stats_policy

`function` · `parquet::file::metadata::options::ParquetMetaDataOptions::with_column_stats_policy` · parquet 59.3.0

```rust
fn with_column_stats_policy(self, policy: ParquetStatisticsPolicy) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::options::ParquetMetaDataOptions", "path": "ParquetMetaDataOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [111, 1], "end": [247, 2], "filename": "src/file/metadata/options.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/metadata/options.rs:218`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Call [`Self::set_column_stats_policy`](../operations/parquet.file.metadata.options.ParquetMetaDataOptions.md#op-4a24f0f2b7d4d7dff65204f7) and return `Self` for chaining.

<a id="op-5119f1010aa08c4b35a99c5f"></a>
## with_encoding_stats_as_mask

`function` · `parquet::file::metadata::options::ParquetMetaDataOptions::with_encoding_stats_as_mask` · parquet 59.3.0

```rust
fn with_encoding_stats_as_mask(self, val: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::options::ParquetMetaDataOptions", "path": "ParquetMetaDataOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [111, 1], "end": [247, 2], "filename": "src/file/metadata/options.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/metadata/options.rs:166`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Call [`Self::set_encoding_stats_as_mask`](../operations/parquet.file.metadata.options.ParquetMetaDataOptions.md#op-2a5f6446514f2b744ce7034e) and return `Self` for chaining.

<a id="op-f8552a1f58daafaeeb88cc0c"></a>
## with_encoding_stats_policy

`function` · `parquet::file::metadata::options::ParquetMetaDataOptions::with_encoding_stats_policy` · parquet 59.3.0

```rust
fn with_encoding_stats_policy(self, policy: ParquetStatisticsPolicy) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::options::ParquetMetaDataOptions", "path": "ParquetMetaDataOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [111, 1], "end": [247, 2], "filename": "src/file/metadata/options.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/metadata/options.rs:193`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Call [`Self::set_encoding_stats_policy`](../operations/parquet.file.metadata.options.ParquetMetaDataOptions.md#op-4f85a7c3775956b8cf607d22) and return `Self` for chaining.

<a id="op-232800d28a2ea6ebfedb85f3"></a>
## with_schema

`function` · `parquet::file::metadata::options::ParquetMetaDataOptions::with_schema` · parquet 59.3.0

```rust
fn with_schema(self, val: SchemaDescPtr) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::options::ParquetMetaDataOptions", "path": "ParquetMetaDataOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [111, 1], "end": [247, 2], "filename": "src/file/metadata/options.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/metadata/options.rs:129`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Call [`Self::set_schema`](../operations/parquet.file.metadata.options.ParquetMetaDataOptions.md#op-c9458f834d308e5c3ea8f43a) and return `Self` for chaining.

<a id="op-ac09aac1030dcf0e00a73a9f"></a>
## with_size_stats_policy

`function` · `parquet::file::metadata::options::ParquetMetaDataOptions::with_size_stats_policy` · parquet 59.3.0

```rust
fn with_size_stats_policy(self, policy: ParquetStatisticsPolicy) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::options::ParquetMetaDataOptions", "path": "ParquetMetaDataOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [111, 1], "end": [247, 2], "filename": "src/file/metadata/options.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/metadata/options.rs:243`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Call [`Self::set_size_stats_policy`](../operations/parquet.file.metadata.options.ParquetMetaDataOptions.md#op-c4b3a64bfc53fa8e226e87cc) and return `Self` for chaining.
