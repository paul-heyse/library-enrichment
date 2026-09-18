# `datafusion::datasource::file_format::options::CsvReadOptions`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion.datasource.file_format.options.CsvReadOptions.json).

<a id="op-eff7687d0fbe63eb1941095a"></a>
## CsvReadOptions

`struct` · `datafusion::datasource::file_format::options::CsvReadOptions` · datafusion 55.1.0

```rust
struct CsvReadOptions<'a>
```

Source: `src/datasource/file_format/options.rs:55`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Options that control the reading of CSV files.

Note this structure is supplied when a datasource is created and
can not not vary from statement to statement. For settings that
can vary statement to statement see
[`ConfigOptions`](crate::config::ConfigOptions).

<a id="op-585ba9af12862b404ced551b"></a>
## clone

`function` · `datafusion::datasource::file_format::options::CsvReadOptions::clone` · datafusion 55.1.0

```rust
fn clone(&self) -> CsvReadOptions<'a>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion::datasource::file_format::options::CsvReadOptions", "path": "CsvReadOptions"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [54, 10], "end": [54, 15], "filename": "src/datasource/file_format/options.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/datasource/file_format/options.rs:54`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-05a9d2d4a8abfab161194b78"></a>
## comment

`struct_field` · `datafusion::datasource::file_format::options::CsvReadOptions::comment` · datafusion 55.1.0

```rust
comment: Option<u8>
```

Source: `src/datasource/file_format/options.rs:70`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

If enabled, lines beginning with this byte are ignored.

<a id="op-fac4b309d5c8a8d9ca2e2486"></a>
## comment

`function` · `datafusion::datasource::file_format::options::CsvReadOptions::comment` · datafusion 55.1.0

```rust
fn comment(self, comment: u8) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion::datasource::file_format::options::CsvReadOptions", "path": "CsvReadOptions"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [108, 1], "end": [242, 2], "filename": "src/datasource/file_format/options.rs"}, "trait": null, "trait_path": null}`

Source: `src/datasource/file_format/options.rs:137`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Specify comment char to use for CSV read

<a id="op-ad1f2f7db922d6a17d3993fd"></a>
## default

`function` · `datafusion::datasource::file_format::options::CsvReadOptions::default` · datafusion 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}], "constraints": []}}, "id": "datafusion::datasource::file_format::options::CsvReadOptions", "path": "CsvReadOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [102, 1], "end": [106, 2], "filename": "src/datasource/file_format/options.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/datasource/file_format/options.rs:103`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-082b4d3cf9d118988457ff50"></a>
## delimiter

`struct_field` · `datafusion::datasource::file_format::options::CsvReadOptions::delimiter` · datafusion 55.1.0

```rust
delimiter: u8
```

Source: `src/datasource/file_format/options.rs:62`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

An optional column delimiter. Defaults to `b','`.

<a id="op-dd617334c614d59280f1a66e"></a>
## delimiter

`function` · `datafusion::datasource::file_format::options::CsvReadOptions::delimiter` · datafusion 55.1.0

```rust
fn delimiter(self, delimiter: u8) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion::datasource::file_format::options::CsvReadOptions", "path": "CsvReadOptions"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [108, 1], "end": [242, 2], "filename": "src/datasource/file_format/options.rs"}, "trait": null, "trait_path": null}`

Source: `src/datasource/file_format/options.rs:143`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Specify delimiter to use for CSV read

<a id="op-5268bfcd16422d9b93692346"></a>
## delimiter_option

`function` · `datafusion::datasource::file_format::options::CsvReadOptions::delimiter_option` · datafusion 55.1.0

```rust
fn delimiter_option(self, delimiter: Option<u8>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion::datasource::file_format::options::CsvReadOptions", "path": "CsvReadOptions"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [108, 1], "end": [242, 2], "filename": "src/datasource/file_format/options.rs"}, "trait": null, "trait_path": null}`

Source: `src/datasource/file_format/options.rs:185`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Configure delimiter setting with Option, None value will be ignored

<a id="op-549ecfcb5d3da9dceae6d6aa"></a>
## escape

`function` · `datafusion::datasource::file_format::options::CsvReadOptions::escape` · datafusion 55.1.0

```rust
fn escape(self, escape: u8) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion::datasource::file_format::options::CsvReadOptions", "path": "CsvReadOptions"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [108, 1], "end": [242, 2], "filename": "src/datasource/file_format/options.rs"}, "trait": null, "trait_path": null}`

Source: `src/datasource/file_format/options.rs:161`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Specify delimiter to use for CSV read

<a id="op-aa4e45bfe575a0176760f9a0"></a>
## escape

`struct_field` · `datafusion::datasource::file_format::options::CsvReadOptions::escape` · datafusion 55.1.0

```rust
escape: Option<u8>
```

Source: `src/datasource/file_format/options.rs:68`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

An optional escape character. Defaults to None.

<a id="op-97c5352d28bb10d7046b4a19"></a>
## file_compression_type

`function` · `datafusion::datasource::file_format::options::CsvReadOptions::file_compression_type` · datafusion 55.1.0

```rust
fn file_compression_type(self, file_compression_type: FileCompressionType) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion::datasource::file_format::options::CsvReadOptions", "path": "CsvReadOptions"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [108, 1], "end": [242, 2], "filename": "src/datasource/file_format/options.rs"}, "trait": null, "trait_path": null}`

Source: `src/datasource/file_format/options.rs:214`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Configure file compression type

<a id="op-d6e7cf513554d01766d8b573"></a>
## file_compression_type

`struct_field` · `datafusion::datasource::file_format::options::CsvReadOptions::file_compression_type` · datafusion 55.1.0

```rust
file_compression_type: datasource::file_format::file_compression_type::FileCompressionType
```

Source: `src/datasource/file_format/options.rs:90`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

File compression type

<a id="op-9b325c3b198c22f8da40dee3"></a>
## file_extension

`struct_field` · `datafusion::datasource::file_format::options::CsvReadOptions::file_extension` · datafusion 55.1.0

```rust
file_extension: &'a str
```

Source: `src/datasource/file_format/options.rs:86`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

File extension; only files with this extension are selected for data input.
Defaults to `FileType::CSV.get_ext().as_str()`.

<a id="op-9c93108f596662795509b80f"></a>
## file_extension

`function` · `datafusion::datasource::file_format::options::CsvReadOptions::file_extension` · datafusion 55.1.0

```rust
fn file_extension(self, file_extension: &'a str) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion::datasource::file_format::options::CsvReadOptions", "path": "CsvReadOptions"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [108, 1], "end": [242, 2], "filename": "src/datasource/file_format/options.rs"}, "trait": null, "trait_path": null}`

Source: `src/datasource/file_format/options.rs:179`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Specify the file extension for CSV file selection

<a id="op-ad9cb392ffa50083e431d768"></a>
## file_sort_order

`struct_field` · `datafusion::datasource::file_format::options::CsvReadOptions::file_sort_order` · datafusion 55.1.0

```rust
file_sort_order: Vec<Vec<datafusion_expr::SortExpr>>
```

Source: `src/datasource/file_format/options.rs:92`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Indicates how the file is sorted

<a id="op-d4f9cc1d26ce265ca9c7a8aa"></a>
## file_sort_order

`function` · `datafusion::datasource::file_format::options::CsvReadOptions::file_sort_order` · datafusion 55.1.0

```rust
fn file_sort_order(self, file_sort_order: Vec<Vec<SortExpr>>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion::datasource::file_format::options::CsvReadOptions", "path": "CsvReadOptions"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [108, 1], "end": [242, 2], "filename": "src/datasource/file_format/options.rs"}, "trait": null, "trait_path": null}`

Source: `src/datasource/file_format/options.rs:223`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Configure if file has known sort order

<a id="op-dd80b4be78d2891a3a41026e"></a>
## get_resolved_schema

`function` · `datafusion::datasource::file_format::options::CsvReadOptions::get_resolved_schema` · datafusion 55.1.0

```rust
async fn get_resolved_schema(&self, config: &SessionConfig, state: SessionState, table_path: ListingTableUrl) -> Result<SchemaRef>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}], "constraints": []}}, "id": "datafusion::datasource::file_format::options::CsvReadOptions", "path": "CsvReadOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [626, 1], "end": [665, 2], "filename": "src/datasource/file_format/options.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}], "constraints": []}}, "id": "datafusion::datasource::file_format::options::ReadOptions", "path": "ReadOptions"}, "trait_path": "datafusion::datasource::file_format::options::ReadOptions"}`

Source: `src/datasource/file_format/options.rs:652`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-175da6a50f85066f68491d60"></a>
## has_header

`function` · `datafusion::datasource::file_format::options::CsvReadOptions::has_header` · datafusion 55.1.0

```rust
fn has_header(self, has_header: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion::datasource::file_format::options::CsvReadOptions", "path": "CsvReadOptions"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [108, 1], "end": [242, 2], "filename": "src/datasource/file_format/options.rs"}, "trait": null, "trait_path": null}`

Source: `src/datasource/file_format/options.rs:131`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Configure has_header setting

<a id="op-ce5f3b5bd1bed9be1eab7d40"></a>
## has_header

`struct_field` · `datafusion::datasource::file_format::options::CsvReadOptions::has_header` · datafusion 55.1.0

```rust
has_header: bool
```

Source: `src/datasource/file_format/options.rs:60`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Does the CSV file have a header?

If schema inference is run on a file with no headers, default column names
are created.

<a id="op-321c244888a59c6bde3387a7"></a>
## new

`function` · `datafusion::datasource::file_format::options::CsvReadOptions::new` · datafusion 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion::datasource::file_format::options::CsvReadOptions", "path": "CsvReadOptions"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [108, 1], "end": [242, 2], "filename": "src/datasource/file_format/options.rs"}, "trait": null, "trait_path": null}`

Source: `src/datasource/file_format/options.rs:110`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Create a CSV read option with default presets

<a id="op-ad397844ebba23884d3581d4"></a>
## newlines_in_values

`function` · `datafusion::datasource::file_format::options::CsvReadOptions::newlines_in_values` · datafusion 55.1.0

```rust
fn newlines_in_values(self, newlines_in_values: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion::datasource::file_format::options::CsvReadOptions", "path": "CsvReadOptions"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [108, 1], "end": [242, 2], "filename": "src/datasource/file_format/options.rs"}, "trait": null, "trait_path": null}`

Source: `src/datasource/file_format/options.rs:173`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Specifies whether newlines in (quoted) values are supported.

Parsing newlines in quoted values may be affected by execution behaviour such as
parallel file scanning. Setting this to `true` ensures that newlines in values are
parsed successfully, which may reduce performance.

The default behaviour depends on the `datafusion.catalog.newlines_in_values` setting.

<a id="op-c97ecc3cb7145fcbad7cb88b"></a>
## newlines_in_values

`struct_field` · `datafusion::datasource::file_format::options::CsvReadOptions::newlines_in_values` · datafusion 55.1.0

```rust
newlines_in_values: bool
```

Source: `src/datasource/file_format/options.rs:78`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Specifies whether newlines in (quoted) values are supported.

Parsing newlines in quoted values may be affected by execution behaviour such as
parallel file scanning. Setting this to `true` ensures that newlines in values are
parsed successfully, which may reduce performance.

The default behaviour depends on the `datafusion.catalog.newlines_in_values` setting.

<a id="op-a8bf9e4b62e8e22ebbd5f18e"></a>
## null_regex

`struct_field` · `datafusion::datasource::file_format::options::CsvReadOptions::null_regex` · datafusion 55.1.0

```rust
null_regex: Option<String>
```

Source: `src/datasource/file_format/options.rs:94`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Optional regex to match null values

<a id="op-d655c414944f1096f0dc6881"></a>
## null_regex

`function` · `datafusion::datasource::file_format::options::CsvReadOptions::null_regex` · datafusion 55.1.0

```rust
fn null_regex(self, null_regex: Option<String>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion::datasource::file_format::options::CsvReadOptions", "path": "CsvReadOptions"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [108, 1], "end": [242, 2], "filename": "src/datasource/file_format/options.rs"}, "trait": null, "trait_path": null}`

Source: `src/datasource/file_format/options.rs:229`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Configure the null parsing regex.

<a id="op-104489d7babec0bb95a1b3b4"></a>
## quote

`struct_field` · `datafusion::datasource::file_format::options::CsvReadOptions::quote` · datafusion 55.1.0

```rust
quote: u8
```

Source: `src/datasource/file_format/options.rs:64`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

An optional quote character. Defaults to `b'"'`.

<a id="op-90ed4fdfbaa1eb637fb00fb9"></a>
## quote

`function` · `datafusion::datasource::file_format::options::CsvReadOptions::quote` · datafusion 55.1.0

```rust
fn quote(self, quote: u8) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion::datasource::file_format::options::CsvReadOptions", "path": "CsvReadOptions"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [108, 1], "end": [242, 2], "filename": "src/datasource/file_format/options.rs"}, "trait": null, "trait_path": null}`

Source: `src/datasource/file_format/options.rs:149`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Specify quote to use for CSV read

<a id="op-6adb0c0890c6aae2f519acda"></a>
## schema

`struct_field` · `datafusion::datasource::file_format::options::CsvReadOptions::schema` · datafusion 55.1.0

```rust
schema: Option<&'a arrow::datatypes::Schema>
```

Source: `src/datasource/file_format/options.rs:81`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

An optional schema representing the CSV files. If None, CSV reader will try to infer it
based on data in file.

<a id="op-e58b3d9e174e69db025969a1"></a>
## schema

`function` · `datafusion::datasource::file_format::options::CsvReadOptions::schema` · datafusion 55.1.0

```rust
fn schema(self, schema: &'a Schema) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion::datasource::file_format::options::CsvReadOptions", "path": "CsvReadOptions"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [108, 1], "end": [242, 2], "filename": "src/datasource/file_format/options.rs"}, "trait": null, "trait_path": null}`

Source: `src/datasource/file_format/options.rs:193`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Specify schema to use for CSV read

<a id="op-114d0f53e1fc414a405322e3"></a>
## schema_infer_max_records

`struct_field` · `datafusion::datasource::file_format::options::CsvReadOptions::schema_infer_max_records` · datafusion 55.1.0

```rust
schema_infer_max_records: usize
```

Source: `src/datasource/file_format/options.rs:83`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Max number of rows to read from CSV files for schema inference if needed. Defaults to `DEFAULT_SCHEMA_INFER_MAX_RECORD`.

<a id="op-cf407a9ec65a54ee5b2d7b90"></a>
## schema_infer_max_records

`function` · `datafusion::datasource::file_format::options::CsvReadOptions::schema_infer_max_records` · datafusion 55.1.0

```rust
fn schema_infer_max_records(self, max_records: usize) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion::datasource::file_format::options::CsvReadOptions", "path": "CsvReadOptions"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [108, 1], "end": [242, 2], "filename": "src/datasource/file_format/options.rs"}, "trait": null, "trait_path": null}`

Source: `src/datasource/file_format/options.rs:208`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Configure number of max records to read for schema inference

<a id="op-25f8959af064c0b0d65d6eb2"></a>
## schema_source

`function` · `datafusion::datasource::file_format::options::CsvReadOptions::schema_source` · datafusion 55.1.0

```rust
fn schema_source(&self) -> SchemaSource
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}], "constraints": []}}, "id": "datafusion::datasource::file_format::options::CsvReadOptions", "path": "CsvReadOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [626, 1], "end": [665, 2], "filename": "src/datasource/file_format/options.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}], "constraints": []}}, "id": "datafusion::datasource::file_format::options::ReadOptions", "path": "ReadOptions"}, "trait_path": "datafusion::datasource::file_format::options::ReadOptions"}`

Source: `src/datasource/file_format/options.rs:662`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-058f69259b2e5e2c435126c6"></a>
## table_partition_cols

`function` · `datafusion::datasource::file_format::options::CsvReadOptions::table_partition_cols` · datafusion 55.1.0

```rust
fn table_partition_cols(self, table_partition_cols: Vec<(String, DataType)>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion::datasource::file_format::options::CsvReadOptions", "path": "CsvReadOptions"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [108, 1], "end": [242, 2], "filename": "src/datasource/file_format/options.rs"}, "trait": null, "trait_path": null}`

Source: `src/datasource/file_format/options.rs:199`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Specify table_partition_cols for partition pruning

<a id="op-24ee780da1ef4cb16f7b1dbd"></a>
## table_partition_cols

`struct_field` · `datafusion::datasource::file_format::options::CsvReadOptions::table_partition_cols` · datafusion 55.1.0

```rust
table_partition_cols: Vec<(String, arrow::datatypes::DataType)>
```

Source: `src/datasource/file_format/options.rs:88`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Partition Columns

<a id="op-26153b746953873284ba44a9"></a>
## terminator

`function` · `datafusion::datasource::file_format::options::CsvReadOptions::terminator` · datafusion 55.1.0

```rust
fn terminator(self, terminator: Option<u8>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion::datasource::file_format::options::CsvReadOptions", "path": "CsvReadOptions"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [108, 1], "end": [242, 2], "filename": "src/datasource/file_format/options.rs"}, "trait": null, "trait_path": null}`

Source: `src/datasource/file_format/options.rs:155`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Specify terminator to use for CSV read

<a id="op-a33517f7a9c81e0d658cb269"></a>
## terminator

`struct_field` · `datafusion::datasource::file_format::options::CsvReadOptions::terminator` · datafusion 55.1.0

```rust
terminator: Option<u8>
```

Source: `src/datasource/file_format/options.rs:66`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

An optional terminator character. Defaults to None (CRLF).

<a id="op-6f0e4e1f42f1040480f5667c"></a>
## to_listing_options

`function` · `datafusion::datasource::file_format::options::CsvReadOptions::to_listing_options` · datafusion 55.1.0

```rust
fn to_listing_options(&self, _config: &SessionConfig, table_options: TableOptions) -> ListingOptions
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}], "constraints": []}}, "id": "datafusion::datasource::file_format::options::CsvReadOptions", "path": "CsvReadOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [626, 1], "end": [665, 2], "filename": "src/datasource/file_format/options.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}], "constraints": []}}, "id": "datafusion::datasource::file_format::options::ReadOptions", "path": "ReadOptions"}, "trait_path": "datafusion::datasource::file_format::options::ReadOptions"}`

Source: `src/datasource/file_format/options.rs:627`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-16baa80c1d3d91e0c089b028"></a>
## truncated_rows

`function` · `datafusion::datasource::file_format::options::CsvReadOptions::truncated_rows` · datafusion 55.1.0

```rust
fn truncated_rows(self, truncated_rows: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion::datasource::file_format::options::CsvReadOptions", "path": "CsvReadOptions"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [108, 1], "end": [242, 2], "filename": "src/datasource/file_format/options.rs"}, "trait": null, "trait_path": null}`

Source: `src/datasource/file_format/options.rs:238`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Configure whether to allow truncated rows when parsing.
By default this is set to false and will error if the CSV rows have different lengths
When set to true then it will allow records with less than the expected number of columns and fill the missing columns with nulls.
If the record’s schema is not nullable, then it will still return an error.

<a id="op-260dca851ac96b44516fae68"></a>
## truncated_rows

`struct_field` · `datafusion::datasource::file_format::options::CsvReadOptions::truncated_rows` · datafusion 55.1.0

```rust
truncated_rows: bool
```

Source: `src/datasource/file_format/options.rs:99`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Whether to allow truncated rows when parsing.
By default this is set to false and will error if the CSV rows have different lengths.
When set to true then it will allow records with less than the expected number of columns and fill the missing columns with nulls.
If the record’s schema is not nullable, then it will still return an error.
