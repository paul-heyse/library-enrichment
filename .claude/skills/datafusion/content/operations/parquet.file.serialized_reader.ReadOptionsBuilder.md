# `parquet::file::serialized_reader::ReadOptionsBuilder`

Full upstream contracts; raw type trees and source locators in [structured records](parquet.file.serialized_reader.ReadOptionsBuilder.json).

<a id="op-66c780c6fc016ab8c85120dd"></a>
## ReadOptionsBuilder

`struct` · `parquet::file::serialized_reader::ReadOptionsBuilder` · parquet 59.3.0

```rust
struct ReadOptionsBuilder
```

Source: `src/file/serialized_reader.rs:109`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

A builder for [`ReadOptions`](../operations/parquet.file.serialized_reader.ReadOptions.md#op-369feadda83636e9edbaf29f).
For the predicates that are added to the builder,
they will be chained using 'AND' to filter the row groups.

<a id="op-6efdac1f47049dae74be5d77"></a>
## build

`function` · `parquet::file::serialized_reader::ReadOptionsBuilder::build` · parquet 59.3.0

```rust
fn build(self) -> ReadOptions
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::serialized_reader::ReadOptionsBuilder", "path": "ReadOptionsBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [116, 1], "end": [215, 2], "filename": "src/file/serialized_reader.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/serialized_reader.rs:204`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Seal the builder and return the read options

<a id="op-865ee23043d5fd603e560a50"></a>
## default

`function` · `parquet::file::serialized_reader::ReadOptionsBuilder::default` · parquet 59.3.0

```rust
fn default() -> ReadOptionsBuilder
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::serialized_reader::ReadOptionsBuilder", "path": "ReadOptionsBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [108, 10], "end": [108, 17], "filename": "src/file/serialized_reader.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/file/serialized_reader.rs:108`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-866b4b119c8097f7919e1e98"></a>
## new

`function` · `parquet::file::serialized_reader::ReadOptionsBuilder::new` · parquet 59.3.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::serialized_reader::ReadOptionsBuilder", "path": "ReadOptionsBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [116, 1], "end": [215, 2], "filename": "src/file/serialized_reader.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/serialized_reader.rs:118`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

New builder

<a id="op-08003ec54d48c952f04a592c"></a>
## with_column_stats_policy

`function` · `parquet::file::serialized_reader::ReadOptionsBuilder::with_column_stats_policy` · parquet 59.3.0

```rust
fn with_column_stats_policy(self, policy: ParquetStatisticsPolicy) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::serialized_reader::ReadOptionsBuilder", "path": "ReadOptionsBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [116, 1], "end": [215, 2], "filename": "src/file/serialized_reader.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/serialized_reader.rs:189`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Sets the decoding policy for [`statistics`] in the Parquet `ColumnMetaData`.

[`statistics`]:
https://github.com/apache/parquet-format/blob/786142e26740487930ddc3ec5e39d780bd930907/src/main/thrift/parquet.thrift#L912

<a id="op-f173fc39aec882a622e4ab2e"></a>
## with_encoding_stats_as_mask

`function` · `parquet::file::serialized_reader::ReadOptionsBuilder::with_encoding_stats_as_mask` · parquet 59.3.0

```rust
fn with_encoding_stats_as_mask(self, val: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::serialized_reader::ReadOptionsBuilder", "path": "ReadOptionsBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [116, 1], "end": [215, 2], "filename": "src/file/serialized_reader.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/serialized_reader.rs:171`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Set whether to convert the [`encoding_stats`] in the Parquet `ColumnMetaData` to a bitmask
(defaults to `false`).

See [`ColumnChunkMetaData::page_encoding_stats_mask`](../operations/parquet.file.metadata.ColumnChunkMetaData.md#op-afe0cb25a019198512177e3d) for an explanation of why this
might be desirable.

[`encoding_stats`]:
https://github.com/apache/parquet-format/blob/786142e26740487930ddc3ec5e39d780bd930907/src/main/thrift/parquet.thrift#L917

<a id="op-4cf7192ca7743946824223cb"></a>
## with_encoding_stats_policy

`function` · `parquet::file::serialized_reader::ReadOptionsBuilder::with_encoding_stats_policy` · parquet 59.3.0

```rust
fn with_encoding_stats_policy(self, policy: ParquetStatisticsPolicy) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::serialized_reader::ReadOptionsBuilder", "path": "ReadOptionsBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [116, 1], "end": [215, 2], "filename": "src/file/serialized_reader.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/serialized_reader.rs:180`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Sets the decoding policy for [`encoding_stats`] in the Parquet `ColumnMetaData`.

[`encoding_stats`]:
https://github.com/apache/parquet-format/blob/786142e26740487930ddc3ec5e39d780bd930907/src/main/thrift/parquet.thrift#L917

<a id="op-be3de727c379e7c5042138dd"></a>
## with_page_index

`function` · `parquet::file::serialized_reader::ReadOptionsBuilder::with_page_index` · parquet 59.3.0

```rust
fn with_page_index(self) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::serialized_reader::ReadOptionsBuilder", "path": "ReadOptionsBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [116, 1], "end": [215, 2], "filename": "src/file/serialized_reader.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/serialized_reader.rs:145`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Enable reading the page index structures described in
"[Column Index] Layout to Support Page Skipping"

[Column Index]: https://github.com/apache/parquet-format/blob/master/PageIndex.md

<a id="op-b3d6d15b1ab75f8c28276ae7"></a>
## with_parquet_schema

`function` · `parquet::file::serialized_reader::ReadOptionsBuilder::with_parquet_schema` · parquet 59.3.0

```rust
fn with_parquet_schema(self, schema: SchemaDescPtr) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::serialized_reader::ReadOptionsBuilder", "path": "ReadOptionsBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [116, 1], "end": [215, 2], "filename": "src/file/serialized_reader.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/serialized_reader.rs:158`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Provide a Parquet schema to use when decoding the metadata. The schema in the Parquet
footer will be skipped.

<a id="op-e90c9daff463b7ef90b1a8f3"></a>
## with_predicate

`function` · `parquet::file::serialized_reader::ReadOptionsBuilder::with_predicate` · parquet 59.3.0

```rust
fn with_predicate(self, predicate: ReadGroupPredicate) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::serialized_reader::ReadOptionsBuilder", "path": "ReadOptionsBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [116, 1], "end": [215, 2], "filename": "src/file/serialized_reader.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/serialized_reader.rs:124`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Add a predicate on row group metadata to the reading option,
Filter only row groups that match the predicate criteria

<a id="op-89f126e86c2ded2ba0153bf4"></a>
## with_range

`function` · `parquet::file::serialized_reader::ReadOptionsBuilder::with_range` · parquet 59.3.0

```rust
fn with_range(self, start: i64, end: i64) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::serialized_reader::ReadOptionsBuilder", "path": "ReadOptionsBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [116, 1], "end": [215, 2], "filename": "src/file/serialized_reader.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/serialized_reader.rs:131`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Add a range predicate on filtering row groups if their midpoints are within
the Closed-Open range `[start..end) {x | start <= x < end}`

<a id="op-c7daed7a7d696d7c1b4283e6"></a>
## with_reader_properties

`function` · `parquet::file::serialized_reader::ReadOptionsBuilder::with_reader_properties` · parquet 59.3.0

```rust
fn with_reader_properties(self, properties: ReaderProperties) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::serialized_reader::ReadOptionsBuilder", "path": "ReadOptionsBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [116, 1], "end": [215, 2], "filename": "src/file/serialized_reader.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/serialized_reader.rs:151`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Set the [`ReaderProperties`](../operations/parquet.file.properties.ReaderProperties.md#op-10bf0c6bc812b59aa077ae83) configuration.

<a id="op-f02960667816069990a54bc3"></a>
## with_size_stats_policy

`function` · `parquet::file::serialized_reader::ReadOptionsBuilder::with_size_stats_policy` · parquet 59.3.0

```rust
fn with_size_stats_policy(self, policy: ParquetStatisticsPolicy) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::serialized_reader::ReadOptionsBuilder", "path": "ReadOptionsBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [116, 1], "end": [215, 2], "filename": "src/file/serialized_reader.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/serialized_reader.rs:198`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Sets the decoding policy for [`size_statistics`] in the Parquet `ColumnMetaData`.

[`size_statistics`]:
https://github.com/apache/parquet-format/blob/786142e26740487930ddc3ec5e39d780bd930907/src/main/thrift/parquet.thrift#L936
