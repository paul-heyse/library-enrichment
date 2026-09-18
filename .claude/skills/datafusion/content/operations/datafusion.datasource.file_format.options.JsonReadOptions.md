# `datafusion::datasource::file_format::options::JsonReadOptions`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion.datasource.file_format.options.JsonReadOptions.json).

<a id="op-1fe01d124d004b1c3f81546e"></a>
## JsonReadOptions

`struct` · `datafusion::datasource::file_format::options::JsonReadOptions` · datafusion 55.1.0

```rust
struct JsonReadOptions<'a>
```

Source: `src/datasource/file_format/options.rs:462`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Options that control the reading of JSON files.

Supports both newline-delimited JSON (NDJSON) and JSON array formats.

Note this structure is supplied when a datasource is created and
can not vary from statement to statement. For settings that
can vary statement to statement see
[`ConfigOptions`](crate::config::ConfigOptions).

<a id="op-0ca0b5c14986682208d82555"></a>
## clone

`function` · `datafusion::datasource::file_format::options::JsonReadOptions::clone` · datafusion 55.1.0

```rust
fn clone(&self) -> JsonReadOptions<'a>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion::datasource::file_format::options::JsonReadOptions", "path": "JsonReadOptions"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [461, 10], "end": [461, 15], "filename": "src/datasource/file_format/options.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/datasource/file_format/options.rs:461`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-40dac0c5ee1da1548a1e43d9"></a>
## default

`function` · `datafusion::datasource::file_format::options::JsonReadOptions::default` · datafusion 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}], "constraints": []}}, "id": "datafusion::datasource::file_format::options::JsonReadOptions", "path": "JsonReadOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [496, 1], "end": [509, 2], "filename": "src/datasource/file_format/options.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/datasource/file_format/options.rs:497`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c5321ae606d6882639f2f207"></a>
## file_compression_type

`function` · `datafusion::datasource::file_format::options::JsonReadOptions::file_compression_type` · datafusion 55.1.0

```rust
fn file_compression_type(self, file_compression_type: FileCompressionType) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion::datasource::file_format::options::JsonReadOptions", "path": "JsonReadOptions"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [511, 1], "end": [579, 2], "filename": "src/datasource/file_format/options.rs"}, "trait": null, "trait_path": null}`

Source: `src/datasource/file_format/options.rs:534`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Specify file_compression_type

<a id="op-c694287d6fb98ce73a7dd425"></a>
## file_compression_type

`struct_field` · `datafusion::datasource::file_format::options::JsonReadOptions::file_compression_type` · datafusion 55.1.0

```rust
file_compression_type: datasource::file_format::file_compression_type::FileCompressionType
```

Source: `src/datasource/file_format/options.rs:473`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

File compression type

<a id="op-b1ddf282adfa4986a484a1ea"></a>
## file_extension

`struct_field` · `datafusion::datasource::file_format::options::JsonReadOptions::file_extension` · datafusion 55.1.0

```rust
file_extension: &'a str
```

Source: `src/datasource/file_format/options.rs:469`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

File extension; only files with this extension are selected for data input.
Defaults to `FileType::JSON.get_ext().as_str()`.

<a id="op-e65c3d6ccfc3fc3c6b085256"></a>
## file_extension

`function` · `datafusion::datasource::file_format::options::JsonReadOptions::file_extension` · datafusion 55.1.0

```rust
fn file_extension(self, file_extension: &'a str) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion::datasource::file_format::options::JsonReadOptions", "path": "JsonReadOptions"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [511, 1], "end": [579, 2], "filename": "src/datasource/file_format/options.rs"}, "trait": null, "trait_path": null}`

Source: `src/datasource/file_format/options.rs:522`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Specify file_extension

<a id="op-0f989aa781a303d0bca2a1b2"></a>
## file_sort_order

`function` · `datafusion::datasource::file_format::options::JsonReadOptions::file_sort_order` · datafusion 55.1.0

```rust
fn file_sort_order(self, file_sort_order: Vec<Vec<SortExpr>>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion::datasource::file_format::options::JsonReadOptions", "path": "JsonReadOptions"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [511, 1], "end": [579, 2], "filename": "src/datasource/file_format/options.rs"}, "trait": null, "trait_path": null}`

Source: `src/datasource/file_format/options.rs:549`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Configure if file has known sort order

<a id="op-1f6e1f07fc6a4e0bc12f7d49"></a>
## file_sort_order

`struct_field` · `datafusion::datasource::file_format::options::JsonReadOptions::file_sort_order` · datafusion 55.1.0

```rust
file_sort_order: Vec<Vec<datafusion_expr::SortExpr>>
```

Source: `src/datasource/file_format/options.rs:477`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Indicates how the file is sorted

<a id="op-11b4db0b67dcebfeb0542fe5"></a>
## get_resolved_schema

`function` · `datafusion::datasource::file_format::options::JsonReadOptions::get_resolved_schema` · datafusion 55.1.0

```rust
async fn get_resolved_schema(&self, config: &SessionConfig, state: SessionState, table_path: ListingTableUrl) -> Result<SchemaRef>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}], "constraints": []}}, "id": "datafusion::datasource::file_format::options::JsonReadOptions", "path": "JsonReadOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [715, 1], "end": [746, 2], "filename": "src/datasource/file_format/options.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}], "constraints": []}}, "id": "datafusion::datasource::file_format::options::ReadOptions", "path": "ReadOptions"}, "trait_path": "datafusion::datasource::file_format::options::ReadOptions"}`

Source: `src/datasource/file_format/options.rs:733`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fbac7ed7f37d0ea7a5941d57"></a>
## infinite

`struct_field` · `datafusion::datasource::file_format::options::JsonReadOptions::infinite` · datafusion 55.1.0

```rust
infinite: bool
```

Source: `src/datasource/file_format/options.rs:475`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Flag indicating whether this file may be unbounded (as in a FIFO file).

<a id="op-06fed3c58a79bfe80d960e97"></a>
## mark_infinite

`function` · `datafusion::datasource::file_format::options::JsonReadOptions::mark_infinite` · datafusion 55.1.0

```rust
fn mark_infinite(self, infinite: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion::datasource::file_format::options::JsonReadOptions", "path": "JsonReadOptions"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [511, 1], "end": [579, 2], "filename": "src/datasource/file_format/options.rs"}, "trait": null, "trait_path": null}`

Source: `src/datasource/file_format/options.rs:528`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Configure mark_infinite setting

<a id="op-190714f9ecdd8dfa7fc7b229"></a>
## newline_delimited

`function` · `datafusion::datasource::file_format::options::JsonReadOptions::newline_delimited` · datafusion 55.1.0

```rust
fn newline_delimited(self, newline_delimited: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion::datasource::file_format::options::JsonReadOptions", "path": "JsonReadOptions"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [511, 1], "end": [579, 2], "filename": "src/datasource/file_format/options.rs"}, "trait": null, "trait_path": null}`

Source: `src/datasource/file_format/options.rs:575`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Set whether to read as newline-delimited JSON.

When `true` (default), expects newline-delimited JSON (NDJSON):
```text
{"key1": 1, "key2": "val"}
{"key1": 2, "key2": "vals"}
```

When `false`, expects JSON array format:
```text
[
  {"key1": 1, "key2": "val"},
  {"key1": 2, "key2": "vals"}
]
```

<a id="op-e5c9080686b08b4b2e5a0562"></a>
## newline_delimited

`struct_field` · `datafusion::datasource::file_format::options::JsonReadOptions::newline_delimited` · datafusion 55.1.0

```rust
newline_delimited: bool
```

Source: `src/datasource/file_format/options.rs:493`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Whether to read as newline-delimited JSON (default: true).

When `true` (default), expects newline-delimited JSON (NDJSON):
```text
{"key1": 1, "key2": "val"}
{"key1": 2, "key2": "vals"}
```

When `false`, expects JSON array format:
```text
[
  {"key1": 1, "key2": "val"},
  {"key1": 2, "key2": "vals"}
]
```

<a id="op-035ebe683f8f322eac71babc"></a>
## schema

`struct_field` · `datafusion::datasource::file_format::options::JsonReadOptions::schema` · datafusion 55.1.0

```rust
schema: Option<&'a arrow::datatypes::Schema>
```

Source: `src/datasource/file_format/options.rs:464`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

The data source schema.

<a id="op-4cf2dcf208909277c1dbf508"></a>
## schema

`function` · `datafusion::datasource::file_format::options::JsonReadOptions::schema` · datafusion 55.1.0

```rust
fn schema(self, schema: &'a Schema) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion::datasource::file_format::options::JsonReadOptions", "path": "JsonReadOptions"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [511, 1], "end": [579, 2], "filename": "src/datasource/file_format/options.rs"}, "trait": null, "trait_path": null}`

Source: `src/datasource/file_format/options.rs:543`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Specify schema to use for NdJson read

<a id="op-1752120e6f195b29dde2242d"></a>
## schema_infer_max_records

`function` · `datafusion::datasource::file_format::options::JsonReadOptions::schema_infer_max_records` · datafusion 55.1.0

```rust
fn schema_infer_max_records(self, schema_infer_max_records: usize) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion::datasource::file_format::options::JsonReadOptions", "path": "JsonReadOptions"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [511, 1], "end": [579, 2], "filename": "src/datasource/file_format/options.rs"}, "trait": null, "trait_path": null}`

Source: `src/datasource/file_format/options.rs:555`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Specify how many rows to read for schema inference

<a id="op-dbfaab19301ed20a23fcc7db"></a>
## schema_infer_max_records

`struct_field` · `datafusion::datasource::file_format::options::JsonReadOptions::schema_infer_max_records` · datafusion 55.1.0

```rust
schema_infer_max_records: usize
```

Source: `src/datasource/file_format/options.rs:466`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Max number of rows to read from JSON files for schema inference if needed. Defaults to `DEFAULT_SCHEMA_INFER_MAX_RECORD`.

<a id="op-d78d588d59fa430b88e2da61"></a>
## schema_source

`function` · `datafusion::datasource::file_format::options::JsonReadOptions::schema_source` · datafusion 55.1.0

```rust
fn schema_source(&self) -> SchemaSource
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}], "constraints": []}}, "id": "datafusion::datasource::file_format::options::JsonReadOptions", "path": "JsonReadOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [715, 1], "end": [746, 2], "filename": "src/datasource/file_format/options.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}], "constraints": []}}, "id": "datafusion::datasource::file_format::options::ReadOptions", "path": "ReadOptions"}, "trait_path": "datafusion::datasource::file_format::options::ReadOptions"}`

Source: `src/datasource/file_format/options.rs:743`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c3b07a025d7ddc858f4943a9"></a>
## table_partition_cols

`function` · `datafusion::datasource::file_format::options::JsonReadOptions::table_partition_cols` · datafusion 55.1.0

```rust
fn table_partition_cols(self, table_partition_cols: Vec<(String, DataType)>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion::datasource::file_format::options::JsonReadOptions", "path": "JsonReadOptions"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [511, 1], "end": [579, 2], "filename": "src/datasource/file_format/options.rs"}, "trait": null, "trait_path": null}`

Source: `src/datasource/file_format/options.rs:513`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Specify table_partition_cols for partition pruning

<a id="op-cf3c3f43ddc86e70d01b37ca"></a>
## table_partition_cols

`struct_field` · `datafusion::datasource::file_format::options::JsonReadOptions::table_partition_cols` · datafusion 55.1.0

```rust
table_partition_cols: Vec<(String, arrow::datatypes::DataType)>
```

Source: `src/datasource/file_format/options.rs:471`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Partition Columns

<a id="op-da966285c2381dfb5d30a33e"></a>
## to_listing_options

`function` · `datafusion::datasource::file_format::options::JsonReadOptions::to_listing_options` · datafusion 55.1.0

```rust
fn to_listing_options(&self, _config: &SessionConfig, table_options: TableOptions) -> ListingOptions
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}], "constraints": []}}, "id": "datafusion::datasource::file_format::options::JsonReadOptions", "path": "JsonReadOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [715, 1], "end": [746, 2], "filename": "src/datasource/file_format/options.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}], "constraints": []}}, "id": "datafusion::datasource::file_format::options::ReadOptions", "path": "ReadOptions"}, "trait_path": "datafusion::datasource::file_format::options::ReadOptions"}`

Source: `src/datasource/file_format/options.rs:716`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
