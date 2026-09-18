# `datafusion::datasource::file_format::options::ArrowReadOptions`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion.datasource.file_format.options.ArrowReadOptions.json).

<a id="op-ddba1b304c1dd1c6baded2eb"></a>
## ArrowReadOptions

`struct` · `datafusion::datasource::file_format::options::ArrowReadOptions` · datafusion 55.1.0

```rust
struct ArrowReadOptions<'a>
```

Source: `src/datasource/file_format/options.rs:362`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Options that control the reading of ARROW files.

Note this structure is supplied when a datasource is created and
can not not vary from statement to statement. For settings that
can vary statement to statement see
[`ConfigOptions`](crate::config::ConfigOptions).

<a id="op-016a5ea980815641f1071004"></a>
## clone

`function` · `datafusion::datasource::file_format::options::ArrowReadOptions::clone` · datafusion 55.1.0

```rust
fn clone(&self) -> ArrowReadOptions<'a>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion::datasource::file_format::options::ArrowReadOptions", "path": "ArrowReadOptions"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [361, 10], "end": [361, 15], "filename": "src/datasource/file_format/options.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/datasource/file_format/options.rs:361`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b75ae3d89f0ed37a6a120e41"></a>
## default

`function` · `datafusion::datasource::file_format::options::ArrowReadOptions::default` · datafusion 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}], "constraints": []}}, "id": "datafusion::datasource::file_format::options::ArrowReadOptions", "path": "ArrowReadOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [374, 1], "end": [382, 2], "filename": "src/datasource/file_format/options.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/datasource/file_format/options.rs:375`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7b96103d22b4ee32ea7dd707"></a>
## file_extension

`struct_field` · `datafusion::datasource::file_format::options::ArrowReadOptions::file_extension` · datafusion 55.1.0

```rust
file_extension: &'a str
```

Source: `src/datasource/file_format/options.rs:368`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

File extension; only files with this extension are selected for data input.
Defaults to `FileType::ARROW.get_ext().as_str()`.

<a id="op-819e91b6fac111e4a5dc53a6"></a>
## get_resolved_schema

`function` · `datafusion::datasource::file_format::options::ArrowReadOptions::get_resolved_schema` · datafusion 55.1.0

```rust
async fn get_resolved_schema(&self, config: &SessionConfig, state: SessionState, table_path: ListingTableUrl) -> Result<SchemaRef>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}], "constraints": []}}, "id": "datafusion::datasource::file_format::options::ArrowReadOptions", "path": "ArrowReadOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [779, 1], "end": [805, 2], "filename": "src/datasource/file_format/options.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}], "constraints": []}}, "id": "datafusion::datasource::file_format::options::ReadOptions", "path": "ReadOptions"}, "trait_path": "datafusion::datasource::file_format::options::ReadOptions"}`

Source: `src/datasource/file_format/options.rs:792`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-20046ffc21f4e0a36d2a61ff"></a>
## schema

`function` · `datafusion::datasource::file_format::options::ArrowReadOptions::schema` · datafusion 55.1.0

```rust
fn schema(self, schema: &'a Schema) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion::datasource::file_format::options::ArrowReadOptions", "path": "ArrowReadOptions"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [384, 1], "end": [399, 2], "filename": "src/datasource/file_format/options.rs"}, "trait": null, "trait_path": null}`

Source: `src/datasource/file_format/options.rs:395`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Specify schema to use for AVRO read

<a id="op-adaf08bae901214357a88ff0"></a>
## schema

`struct_field` · `datafusion::datasource::file_format::options::ArrowReadOptions::schema` · datafusion 55.1.0

```rust
schema: Option<&'a arrow::datatypes::Schema>
```

Source: `src/datasource/file_format/options.rs:364`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

The data source schema.

<a id="op-fffea1d23d56dd602bd944ec"></a>
## schema_source

`function` · `datafusion::datasource::file_format::options::ArrowReadOptions::schema_source` · datafusion 55.1.0

```rust
fn schema_source(&self) -> SchemaSource
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}], "constraints": []}}, "id": "datafusion::datasource::file_format::options::ArrowReadOptions", "path": "ArrowReadOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [779, 1], "end": [805, 2], "filename": "src/datasource/file_format/options.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}], "constraints": []}}, "id": "datafusion::datasource::file_format::options::ReadOptions", "path": "ReadOptions"}, "trait_path": "datafusion::datasource::file_format::options::ReadOptions"}`

Source: `src/datasource/file_format/options.rs:802`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-468b7f108d83f5c80105a3b6"></a>
## table_partition_cols

`function` · `datafusion::datasource::file_format::options::ArrowReadOptions::table_partition_cols` · datafusion 55.1.0

```rust
fn table_partition_cols(self, table_partition_cols: Vec<(String, DataType)>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion::datasource::file_format::options::ArrowReadOptions", "path": "ArrowReadOptions"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [384, 1], "end": [399, 2], "filename": "src/datasource/file_format/options.rs"}, "trait": null, "trait_path": null}`

Source: `src/datasource/file_format/options.rs:386`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Specify table_partition_cols for partition pruning

<a id="op-f8a250bc11e88bbb2268f7a0"></a>
## table_partition_cols

`struct_field` · `datafusion::datasource::file_format::options::ArrowReadOptions::table_partition_cols` · datafusion 55.1.0

```rust
table_partition_cols: Vec<(String, arrow::datatypes::DataType)>
```

Source: `src/datasource/file_format/options.rs:371`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Partition Columns

<a id="op-433f0dc6b4103d9caa867619"></a>
## to_listing_options

`function` · `datafusion::datasource::file_format::options::ArrowReadOptions::to_listing_options` · datafusion 55.1.0

```rust
fn to_listing_options(&self, _config: &SessionConfig, _table_options: TableOptions) -> ListingOptions
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}], "constraints": []}}, "id": "datafusion::datasource::file_format::options::ArrowReadOptions", "path": "ArrowReadOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [779, 1], "end": [805, 2], "filename": "src/datasource/file_format/options.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}], "constraints": []}}, "id": "datafusion::datasource::file_format::options::ReadOptions", "path": "ReadOptions"}, "trait_path": "datafusion::datasource::file_format::options::ReadOptions"}`

Source: `src/datasource/file_format/options.rs:780`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
