# `datafusion::datasource::file_format::options::AvroReadOptions`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion.datasource.file_format.options.AvroReadOptions.json).

<a id="op-09dfca33e95fa045b2e3db85"></a>
## AvroReadOptions

`struct` · `datafusion::datasource::file_format::options::AvroReadOptions` · datafusion 55.1.0

```rust
struct AvroReadOptions<'a>
```

Source: `src/datasource/file_format/options.rs:408`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Options that control the reading of AVRO files.

Note this structure is supplied when a datasource is created and
can not not vary from statement to statement. For settings that
can vary statement to statement see
[`ConfigOptions`](crate::config::ConfigOptions).

<a id="op-6c65945fb32d2bd14998bc15"></a>
## clone

`function` · `datafusion::datasource::file_format::options::AvroReadOptions::clone` · datafusion 55.1.0

```rust
fn clone(&self) -> AvroReadOptions<'a>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion::datasource::file_format::options::AvroReadOptions", "path": "AvroReadOptions"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [407, 10], "end": [407, 15], "filename": "src/datasource/file_format/options.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/datasource/file_format/options.rs:407`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e4bd4e47e4d515d077b8b60b"></a>
## default

`function` · `datafusion::datasource::file_format::options::AvroReadOptions::default` · datafusion 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}], "constraints": []}}, "id": "datafusion::datasource::file_format::options::AvroReadOptions", "path": "AvroReadOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [419, 1], "end": [427, 2], "filename": "src/datasource/file_format/options.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/datasource/file_format/options.rs:420`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3ab14c61ee6fce01b9fbf6c0"></a>
## file_extension

`struct_field` · `datafusion::datasource::file_format::options::AvroReadOptions::file_extension` · datafusion 55.1.0

```rust
file_extension: &'a str
```

Source: `src/datasource/file_format/options.rs:414`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

File extension; only files with this extension are selected for data input.
Defaults to `FileType::AVRO.get_ext().as_str()`.

<a id="op-3f85d4a38056f926584c7218"></a>
## get_resolved_schema

`function` · `datafusion::datasource::file_format::options::AvroReadOptions::get_resolved_schema` · datafusion 55.1.0

```rust
async fn get_resolved_schema(&self, config: &SessionConfig, state: SessionState, table_path: ListingTableUrl) -> Result<SchemaRef>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}], "constraints": []}}, "id": "datafusion::datasource::file_format::options::AvroReadOptions", "path": "AvroReadOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [750, 1], "end": [776, 2], "filename": "src/datasource/file_format/options.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}], "constraints": []}}, "id": "datafusion::datasource::file_format::options::ReadOptions", "path": "ReadOptions"}, "trait_path": "datafusion::datasource::file_format::options::ReadOptions"}`

Source: `src/datasource/file_format/options.rs:763`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7fed4554a14e1d5115d08165"></a>
## schema

`function` · `datafusion::datasource::file_format::options::AvroReadOptions::schema` · datafusion 55.1.0

```rust
fn schema(self, schema: &'a Schema) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion::datasource::file_format::options::AvroReadOptions", "path": "AvroReadOptions"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [429, 1], "end": [444, 2], "filename": "src/datasource/file_format/options.rs"}, "trait": null, "trait_path": null}`

Source: `src/datasource/file_format/options.rs:440`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Specify schema to use for AVRO read

<a id="op-f935e62fc70855f891e9d2cf"></a>
## schema

`struct_field` · `datafusion::datasource::file_format::options::AvroReadOptions::schema` · datafusion 55.1.0

```rust
schema: Option<&'a arrow::datatypes::Schema>
```

Source: `src/datasource/file_format/options.rs:410`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

The data source schema.

<a id="op-e4360afb9f1791ee8f87239d"></a>
## schema_source

`function` · `datafusion::datasource::file_format::options::AvroReadOptions::schema_source` · datafusion 55.1.0

```rust
fn schema_source(&self) -> SchemaSource
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}], "constraints": []}}, "id": "datafusion::datasource::file_format::options::AvroReadOptions", "path": "AvroReadOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [750, 1], "end": [776, 2], "filename": "src/datasource/file_format/options.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}], "constraints": []}}, "id": "datafusion::datasource::file_format::options::ReadOptions", "path": "ReadOptions"}, "trait_path": "datafusion::datasource::file_format::options::ReadOptions"}`

Source: `src/datasource/file_format/options.rs:773`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1b464a1c184822ff739a88ed"></a>
## table_partition_cols

`struct_field` · `datafusion::datasource::file_format::options::AvroReadOptions::table_partition_cols` · datafusion 55.1.0

```rust
table_partition_cols: Vec<(String, arrow::datatypes::DataType)>
```

Source: `src/datasource/file_format/options.rs:416`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Partition Columns

<a id="op-8585ffa2b36f55c31c54582a"></a>
## table_partition_cols

`function` · `datafusion::datasource::file_format::options::AvroReadOptions::table_partition_cols` · datafusion 55.1.0

```rust
fn table_partition_cols(self, table_partition_cols: Vec<(String, DataType)>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion::datasource::file_format::options::AvroReadOptions", "path": "AvroReadOptions"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [429, 1], "end": [444, 2], "filename": "src/datasource/file_format/options.rs"}, "trait": null, "trait_path": null}`

Source: `src/datasource/file_format/options.rs:431`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Specify table_partition_cols for partition pruning

<a id="op-1650ca696d6cae92600e846c"></a>
## to_listing_options

`function` · `datafusion::datasource::file_format::options::AvroReadOptions::to_listing_options` · datafusion 55.1.0

```rust
fn to_listing_options(&self, _config: &SessionConfig, _table_options: TableOptions) -> ListingOptions
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}], "constraints": []}}, "id": "datafusion::datasource::file_format::options::AvroReadOptions", "path": "AvroReadOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [750, 1], "end": [776, 2], "filename": "src/datasource/file_format/options.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}], "constraints": []}}, "id": "datafusion::datasource::file_format::options::ReadOptions", "path": "ReadOptions"}, "trait_path": "datafusion::datasource::file_format::options::ReadOptions"}`

Source: `src/datasource/file_format/options.rs:751`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
