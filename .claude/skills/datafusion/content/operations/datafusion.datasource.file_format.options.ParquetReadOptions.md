# `datafusion::datasource::file_format::options::ParquetReadOptions`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion.datasource.file_format.options.ParquetReadOptions.json).

<a id="op-f5b35680fd3479d7a2160fdb"></a>
## ParquetReadOptions

`struct` · `datafusion::datasource::file_format::options::ParquetReadOptions` · datafusion 55.1.0

```rust
struct ParquetReadOptions<'a>
```

Source: `src/datasource/file_format/options.rs:251`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Options that control the reading of Parquet files.

Note this structure is supplied when a datasource is created and
can not not vary from statement to statement. For settings that
can vary statement to statement see
[`ConfigOptions`](crate::config::ConfigOptions).

<a id="op-8d74ca629ffe65a923ba9f60"></a>
## clone

`function` · `datafusion::datasource::file_format::options::ParquetReadOptions::clone` · datafusion 55.1.0

```rust
fn clone(&self) -> ParquetReadOptions<'a>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion::datasource::file_format::options::ParquetReadOptions", "path": "ParquetReadOptions"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [250, 10], "end": [250, 15], "filename": "src/datasource/file_format/options.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/datasource/file_format/options.rs:250`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ab364d2ae6510115255812cb"></a>
## default

`function` · `datafusion::datasource::file_format::options::ParquetReadOptions::default` · datafusion 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}], "constraints": []}}, "id": "datafusion::datasource::file_format::options::ParquetReadOptions", "path": "ParquetReadOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [277, 1], "end": [290, 2], "filename": "src/datasource/file_format/options.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/datasource/file_format/options.rs:278`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4b8a10deddac6534abe41764"></a>
## file_decryption_properties

`function` · `datafusion::datasource::file_format::options::ParquetReadOptions::file_decryption_properties` · datafusion 55.1.0

```rust
fn file_decryption_properties(self, file_decryption_properties: ConfigFileDecryptionProperties) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion::datasource::file_format::options::ParquetReadOptions", "path": "ParquetReadOptions"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [292, 1], "end": [353, 2], "filename": "src/datasource/file_format/options.rs"}, "trait": null, "trait_path": null}`

Source: `src/datasource/file_format/options.rs:340`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Configure file decryption properties for reading encrypted Parquet files

<a id="op-a4c65114828e904069ce24c4"></a>
## file_decryption_properties

`struct_field` · `datafusion::datasource::file_format::options::ParquetReadOptions::file_decryption_properties` · datafusion 55.1.0

```rust
file_decryption_properties: Option<datafusion_common::config::ConfigFileDecryptionProperties>
```

Source: `src/datasource/file_format/options.rs:272`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Properties for decryption of Parquet files that use modular encryption

<a id="op-2a4fc400fc6a5d85673cd3dc"></a>
## file_extension

`struct_field` · `datafusion::datasource::file_format::options::ParquetReadOptions::file_extension` · datafusion 55.1.0

```rust
file_extension: &'a str
```

Source: `src/datasource/file_format/options.rs:254`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

File extension; only files with this extension are selected for data input.
Defaults to ".parquet".

<a id="op-f4be5d629ef6755d388ddb93"></a>
## file_extension

`function` · `datafusion::datasource::file_format::options::ParquetReadOptions::file_extension` · datafusion 55.1.0

```rust
fn file_extension(self, file_extension: &'a str) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion::datasource::file_format::options::ParquetReadOptions", "path": "ParquetReadOptions"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [292, 1], "end": [353, 2], "filename": "src/datasource/file_format/options.rs"}, "trait": null, "trait_path": null}`

Source: `src/datasource/file_format/options.rs:299`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Specify file_extension

<a id="op-57c854c1ac908412f6c29709"></a>
## file_sort_order

`function` · `datafusion::datasource::file_format::options::ParquetReadOptions::file_sort_order` · datafusion 55.1.0

```rust
fn file_sort_order(self, file_sort_order: Vec<Vec<SortExpr>>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion::datasource::file_format::options::ParquetReadOptions", "path": "ParquetReadOptions"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [292, 1], "end": [353, 2], "filename": "src/datasource/file_format/options.rs"}, "trait": null, "trait_path": null}`

Source: `src/datasource/file_format/options.rs:334`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Configure if file has known sort order

<a id="op-7e7c524ff0562159985269a4"></a>
## file_sort_order

`struct_field` · `datafusion::datasource::file_format::options::ParquetReadOptions::file_sort_order` · datafusion 55.1.0

```rust
file_sort_order: Vec<Vec<datafusion_expr::SortExpr>>
```

Source: `src/datasource/file_format/options.rs:270`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Indicates how the file is sorted

<a id="op-325202c7dc7eb6687b9f48b2"></a>
## get_resolved_schema

`function` · `datafusion::datasource::file_format::options::ParquetReadOptions::get_resolved_schema` · datafusion 55.1.0

```rust
async fn get_resolved_schema(&self, config: &SessionConfig, state: SessionState, table_path: ListingTableUrl) -> Result<SchemaRef>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}], "constraints": []}}, "id": "datafusion::datasource::file_format::options::ParquetReadOptions", "path": "ParquetReadOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [669, 1], "end": [712, 2], "filename": "src/datasource/file_format/options.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}], "constraints": []}}, "id": "datafusion::datasource::file_format::options::ReadOptions", "path": "ReadOptions"}, "trait_path": "datafusion::datasource::file_format::options::ReadOptions"}`

Source: `src/datasource/file_format/options.rs:699`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3f320e48eb38afbd0cb8426e"></a>
## metadata_size_hint

`struct_field` · `datafusion::datasource::file_format::options::ParquetReadOptions::metadata_size_hint` · datafusion 55.1.0

```rust
metadata_size_hint: Option<usize>
```

Source: `src/datasource/file_format/options.rs:274`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Metadata size hint for Parquet files reading (in bytes)

<a id="op-bbfcb5f777f2ea4f92dc4ebc"></a>
## metadata_size_hint

`function` · `datafusion::datasource::file_format::options::ParquetReadOptions::metadata_size_hint` · datafusion 55.1.0

```rust
fn metadata_size_hint(self, size_hint: Option<usize>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion::datasource::file_format::options::ParquetReadOptions", "path": "ParquetReadOptions"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [292, 1], "end": [353, 2], "filename": "src/datasource/file_format/options.rs"}, "trait": null, "trait_path": null}`

Source: `src/datasource/file_format/options.rs:349`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Configure metadata size hint for Parquet files reading (in bytes)

<a id="op-c7c1a3882b7512f5d6de8298"></a>
## new

`function` · `datafusion::datasource::file_format::options::ParquetReadOptions::new` · datafusion 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion::datasource::file_format::options::ParquetReadOptions", "path": "ParquetReadOptions"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [292, 1], "end": [353, 2], "filename": "src/datasource/file_format/options.rs"}, "trait": null, "trait_path": null}`

Source: `src/datasource/file_format/options.rs:294`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Create a new ParquetReadOptions with default values

<a id="op-0acfee248c406f56bd11b83d"></a>
## parquet_pruning

`function` · `datafusion::datasource::file_format::options::ParquetReadOptions::parquet_pruning` · datafusion 55.1.0

```rust
fn parquet_pruning(self, parquet_pruning: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion::datasource::file_format::options::ParquetReadOptions", "path": "ParquetReadOptions"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [292, 1], "end": [353, 2], "filename": "src/datasource/file_format/options.rs"}, "trait": null, "trait_path": null}`

Source: `src/datasource/file_format/options.rs:305`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Specify parquet_pruning

<a id="op-f107cd9a488910894708b5fc"></a>
## parquet_pruning

`struct_field` · `datafusion::datasource::file_format::options::ParquetReadOptions::parquet_pruning` · datafusion 55.1.0

```rust
parquet_pruning: Option<bool>
```

Source: `src/datasource/file_format/options.rs:259`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Should the parquet reader use the predicate to prune row groups?
If None, uses value in SessionConfig

<a id="op-c275d6e7270fbad3ed8240e2"></a>
## schema

`function` · `datafusion::datasource::file_format::options::ParquetReadOptions::schema` · datafusion 55.1.0

```rust
fn schema(self, schema: &'a Schema) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion::datasource::file_format::options::ParquetReadOptions", "path": "ParquetReadOptions"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [292, 1], "end": [353, 2], "filename": "src/datasource/file_format/options.rs"}, "trait": null, "trait_path": null}`

Source: `src/datasource/file_format/options.rs:319`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Specify schema to use for parquet read

<a id="op-e4092d35358491ee9a721b8e"></a>
## schema

`struct_field` · `datafusion::datasource::file_format::options::ParquetReadOptions::schema` · datafusion 55.1.0

```rust
schema: Option<&'a arrow::datatypes::Schema>
```

Source: `src/datasource/file_format/options.rs:268`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

An optional schema representing the parquet files. If None, parquet reader will try to infer it
based on data in file.

<a id="op-36c1f3bdb645d37b5e452d59"></a>
## schema_source

`function` · `datafusion::datasource::file_format::options::ParquetReadOptions::schema_source` · datafusion 55.1.0

```rust
fn schema_source(&self) -> SchemaSource
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}], "constraints": []}}, "id": "datafusion::datasource::file_format::options::ParquetReadOptions", "path": "ParquetReadOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [669, 1], "end": [712, 2], "filename": "src/datasource/file_format/options.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}], "constraints": []}}, "id": "datafusion::datasource::file_format::options::ReadOptions", "path": "ReadOptions"}, "trait_path": "datafusion::datasource::file_format::options::ReadOptions"}`

Source: `src/datasource/file_format/options.rs:709`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-140f1cb9901e1b6fb7e1e85b"></a>
## skip_metadata

`struct_field` · `datafusion::datasource::file_format::options::ParquetReadOptions::skip_metadata` · datafusion 55.1.0

```rust
skip_metadata: Option<bool>
```

Source: `src/datasource/file_format/options.rs:265`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Should the parquet reader to skip any metadata that may be in
the file Schema? This can help avoid schema conflicts due to
metadata.

If None specified, uses value in SessionConfig

<a id="op-8b088a1d4fed638b083ce74a"></a>
## skip_metadata

`function` · `datafusion::datasource::file_format::options::ParquetReadOptions::skip_metadata` · datafusion 55.1.0

```rust
fn skip_metadata(self, skip_metadata: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion::datasource::file_format::options::ParquetReadOptions", "path": "ParquetReadOptions"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [292, 1], "end": [353, 2], "filename": "src/datasource/file_format/options.rs"}, "trait": null, "trait_path": null}`

Source: `src/datasource/file_format/options.rs:313`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Tell the parquet reader to skip any metadata that may be in
the file Schema. This can help avoid schema conflicts due to
metadata.  Defaults to true.

<a id="op-35870bfc72c09c15c740b328"></a>
## table_partition_cols

`struct_field` · `datafusion::datasource::file_format::options::ParquetReadOptions::table_partition_cols` · datafusion 55.1.0

```rust
table_partition_cols: Vec<(String, arrow::datatypes::DataType)>
```

Source: `src/datasource/file_format/options.rs:256`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Partition Columns

<a id="op-3b252c446e9cedcaf5e45291"></a>
## table_partition_cols

`function` · `datafusion::datasource::file_format::options::ParquetReadOptions::table_partition_cols` · datafusion 55.1.0

```rust
fn table_partition_cols(self, table_partition_cols: Vec<(String, DataType)>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion::datasource::file_format::options::ParquetReadOptions", "path": "ParquetReadOptions"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [292, 1], "end": [353, 2], "filename": "src/datasource/file_format/options.rs"}, "trait": null, "trait_path": null}`

Source: `src/datasource/file_format/options.rs:325`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Specify table_partition_cols for partition pruning

<a id="op-2b18a3d37e0c93c9f909ef86"></a>
## to_listing_options

`function` · `datafusion::datasource::file_format::options::ParquetReadOptions::to_listing_options` · datafusion 55.1.0

```rust
fn to_listing_options(&self, _config: &SessionConfig, table_options: TableOptions) -> ListingOptions
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}], "constraints": []}}, "id": "datafusion::datasource::file_format::options::ParquetReadOptions", "path": "ParquetReadOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [669, 1], "end": [712, 2], "filename": "src/datasource/file_format/options.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}], "constraints": []}}, "id": "datafusion::datasource::file_format::options::ReadOptions", "path": "ReadOptions"}, "trait_path": "datafusion::datasource::file_format::options::ReadOptions"}`

Source: `src/datasource/file_format/options.rs:670`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
