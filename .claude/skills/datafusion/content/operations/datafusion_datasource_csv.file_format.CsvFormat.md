# `datafusion_datasource_csv::file_format::CsvFormat`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_datasource_csv.file_format.CsvFormat.json).

<a id="op-be96ebde8567146a8ce2670f"></a>
## CsvFormat

`struct` · `datafusion_datasource_csv::file_format::CsvFormat` · datafusion-datasource-csv 55.1.0

```rust
struct CsvFormat
```

Source: `src/file_format.rs:135`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-csv/55.1.0/json).

Character Separated Value [`FileFormat`](../operations/datafusion_datasource.file_format.FileFormat.md#op-81da8109cfb5e6919dc4a7ee) implementation.

<a id="op-660a5c778288d0d3682dc80b"></a>
## compression_type

`function` · `datafusion_datasource_csv::file_format::CsvFormat::compression_type` · datafusion-datasource-csv 55.1.0

```rust
fn compression_type(&self) -> Option<FileCompressionType>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_csv::file_format::CsvFormat", "path": "CsvFormat"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [358, 1], "end": [499, 2], "filename": "src/file_format.rs"}, "trait": {"args": null, "id": "datafusion_datasource::file_format::FileFormat", "path": "FileFormat"}, "trait_path": "datafusion_datasource::file_format::FileFormat"}`

Source: `src/file_format.rs:371`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-csv/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1ec3b4d55438885ee7c30097"></a>
## create_physical_plan

`function` · `datafusion_datasource_csv::file_format::CsvFormat::create_physical_plan` · datafusion-datasource-csv 55.1.0

```rust
async fn create_physical_plan(&self, state: &dyn Session, conf: FileScanConfig) -> Result<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_csv::file_format::CsvFormat", "path": "CsvFormat"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [358, 1], "end": [499, 2], "filename": "src/file_format.rs"}, "trait": {"args": null, "id": "datafusion_datasource::file_format::FileFormat", "path": "FileFormat"}, "trait_path": "datafusion_datasource::file_format::FileFormat"}`

Source: `src/file_format.rs:420`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-csv/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8f87ba07ad6fe0bb5cc56beb"></a>
## create_writer_physical_plan

`function` · `datafusion_datasource_csv::file_format::CsvFormat::create_writer_physical_plan` · datafusion-datasource-csv 55.1.0

```rust
async fn create_writer_physical_plan(&self, input: Arc<dyn ExecutionPlan>, state: &dyn Session, conf: FileSinkConfig, order_requirements: Option<LexRequirement>) -> Result<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_csv::file_format::CsvFormat", "path": "CsvFormat"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [358, 1], "end": [499, 2], "filename": "src/file_format.rs"}, "trait": {"args": null, "id": "datafusion_datasource::file_format::FileFormat", "path": "FileFormat"}, "trait_path": "datafusion_datasource::file_format::FileFormat"}`

Source: `src/file_format.rs:455`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-csv/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3d3c80dc67913ebb554c79a4"></a>
## default

`function` · `datafusion_datasource_csv::file_format::CsvFormat::default` · datafusion-datasource-csv 55.1.0

```rust
fn default() -> CsvFormat
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_csv::file_format::CsvFormat", "path": "CsvFormat"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [134, 17], "end": [134, 24], "filename": "src/file_format.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/file_format.rs:134`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-csv/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-05ede3ceb087e44d50181709"></a>
## delimiter

`function` · `datafusion_datasource_csv::file_format::CsvFormat::delimiter` · datafusion-datasource-csv 55.1.0

```rust
fn delimiter(&self) -> u8
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_csv::file_format::CsvFormat", "path": "CsvFormat"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [139, 1], "end": [322, 2], "filename": "src/file_format.rs"}, "trait": null, "trait_path": null}`

Source: `src/file_format.rs:309`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-csv/55.1.0/json).

The delimiter character.

<a id="op-11270c68fcd3269f74854faf"></a>
## escape

`function` · `datafusion_datasource_csv::file_format::CsvFormat::escape` · datafusion-datasource-csv 55.1.0

```rust
fn escape(&self) -> Option<u8>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_csv::file_format::CsvFormat", "path": "CsvFormat"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [139, 1], "end": [322, 2], "filename": "src/file_format.rs"}, "trait": null, "trait_path": null}`

Source: `src/file_format.rs:319`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-csv/55.1.0/json).

The escape character.

<a id="op-b58b0e1c2e028d88976267c8"></a>
## file_source

`function` · `datafusion_datasource_csv::file_format::CsvFormat::file_source` · datafusion-datasource-csv 55.1.0

```rust
fn file_source(&self, table_schema: TableSchema) -> Arc<dyn FileSource>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_csv::file_format::CsvFormat", "path": "CsvFormat"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [358, 1], "end": [499, 2], "filename": "src/file_format.rs"}, "trait": {"args": null, "id": "datafusion_datasource::file_format::FileFormat", "path": "FileFormat"}, "trait_path": "datafusion_datasource::file_format::FileFormat"}`

Source: `src/file_format.rs:492`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-csv/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-28b5bc08b1b7233998454d87"></a>
## fmt

`function` · `datafusion_datasource_csv::file_format::CsvFormat::fmt` · datafusion-datasource-csv 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_csv::file_format::CsvFormat", "path": "CsvFormat"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [134, 10], "end": [134, 15], "filename": "src/file_format.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/file_format.rs:134`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-csv/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-43ab0a762b5bd0e62caeb784"></a>
## get_ext

`function` · `datafusion_datasource_csv::file_format::CsvFormat::get_ext` · datafusion-datasource-csv 55.1.0

```rust
fn get_ext(&self) -> String
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_csv::file_format::CsvFormat", "path": "CsvFormat"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [358, 1], "end": [499, 2], "filename": "src/file_format.rs"}, "trait": {"args": null, "id": "datafusion_datasource::file_format::FileFormat", "path": "FileFormat"}, "trait_path": "datafusion_datasource::file_format::FileFormat"}`

Source: `src/file_format.rs:359`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-csv/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9169c5143eb58bc553e9d6d6"></a>
## get_ext_with_compression

`function` · `datafusion_datasource_csv::file_format::CsvFormat::get_ext_with_compression` · datafusion-datasource-csv 55.1.0

```rust
fn get_ext_with_compression(&self, file_compression_type: &FileCompressionType) -> Result<String>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_csv::file_format::CsvFormat", "path": "CsvFormat"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [358, 1], "end": [499, 2], "filename": "src/file_format.rs"}, "trait": {"args": null, "id": "datafusion_datasource::file_format::FileFormat", "path": "FileFormat"}, "trait_path": "datafusion_datasource::file_format::FileFormat"}`

Source: `src/file_format.rs:363`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-csv/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a0895805c0fa6d6e25761dea"></a>
## has_header

`function` · `datafusion_datasource_csv::file_format::CsvFormat::has_header` · datafusion-datasource-csv 55.1.0

```rust
fn has_header(&self) -> Option<bool>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_csv::file_format::CsvFormat", "path": "CsvFormat"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [139, 1], "end": [322, 2], "filename": "src/file_format.rs"}, "trait": null, "trait_path": null}`

Source: `src/file_format.rs:241`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-csv/55.1.0/json).

Returns `Some(true)` if the first line is a header, `Some(false)` if
it is not, and `None` if it is not specified.

<a id="op-1bc24378ea1213645a9072a3"></a>
## infer_schema

`function` · `datafusion_datasource_csv::file_format::CsvFormat::infer_schema` · datafusion-datasource-csv 55.1.0

```rust
async fn infer_schema(&self, state: &dyn Session, store: &Arc<dyn ObjectStore>, objects: &[ObjectMeta]) -> Result<SchemaRef>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_csv::file_format::CsvFormat", "path": "CsvFormat"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [358, 1], "end": [499, 2], "filename": "src/file_format.rs"}, "trait": {"args": null, "id": "datafusion_datasource::file_format::FileFormat", "path": "FileFormat"}, "trait_path": "datafusion_datasource::file_format::FileFormat"}`

Source: `src/file_format.rs:375`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-csv/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d2c530616edee8b0e05baaaa"></a>
## infer_schema_from_stream

`function` · `datafusion_datasource_csv::file_format::CsvFormat::infer_schema_from_stream` · datafusion-datasource-csv 55.1.0

```rust
async fn infer_schema_from_stream(&self, state: &dyn Session, records_to_read: usize, stream: impl Stream<Item = Result<Bytes>>) -> Result<(Schema, usize)>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_csv::file_format::CsvFormat", "path": "CsvFormat"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [501, 1], "end": [626, 2], "filename": "src/file_format.rs"}, "trait": null, "trait_path": null}`

Source: `src/file_format.rs:518`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-csv/55.1.0/json).

Return the inferred schema reading up to records_to_read from a
stream of delimited chunks returning the inferred schema and the
number of lines that were read.

This method can handle CSV files with different numbers of columns.
The inferred schema will be the union of all columns found across all files.
Files with fewer columns will have missing columns filled with null values.

# Example

If you have two CSV files:
- `file1.csv`: `col1,col2,col3`
- `file2.csv`: `col1,col2,col3,col4,col5`

The inferred schema will contain all 5 columns, with files that don't
have columns 4 and 5 having null values for those columns.

<a id="op-4864f83ed5d4631406296b16"></a>
## infer_stats

`function` · `datafusion_datasource_csv::file_format::CsvFormat::infer_stats` · datafusion-datasource-csv 55.1.0

```rust
async fn infer_stats(&self, _state: &dyn Session, _store: &Arc<dyn ObjectStore>, table_schema: SchemaRef, _object: &ObjectMeta) -> Result<Statistics>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_csv::file_format::CsvFormat", "path": "CsvFormat"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [358, 1], "end": [499, 2], "filename": "src/file_format.rs"}, "trait": {"args": null, "id": "datafusion_datasource::file_format::FileFormat", "path": "FileFormat"}, "trait_path": "datafusion_datasource::file_format::FileFormat"}`

Source: `src/file_format.rs:410`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-csv/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-180993a02071940f4109d418"></a>
## options

`function` · `datafusion_datasource_csv::file_format::CsvFormat::options` · datafusion-datasource-csv 55.1.0

```rust
fn options(&self) -> &CsvOptions
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_csv::file_format::CsvFormat", "path": "CsvFormat"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [139, 1], "end": [322, 2], "filename": "src/file_format.rs"}, "trait": null, "trait_path": null}`

Source: `src/file_format.rs:204`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-csv/55.1.0/json).

Retrieve the csv options

<a id="op-14f893a2d23ef58144117311"></a>
## quote

`function` · `datafusion_datasource_csv::file_format::CsvFormat::quote` · datafusion-datasource-csv 55.1.0

```rust
fn quote(&self) -> u8
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_csv::file_format::CsvFormat", "path": "CsvFormat"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [139, 1], "end": [322, 2], "filename": "src/file_format.rs"}, "trait": null, "trait_path": null}`

Source: `src/file_format.rs:314`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-csv/55.1.0/json).

The quote character.

<a id="op-aabcd2bc9e2d193b311a1a52"></a>
## read_to_delimited_chunks_from_stream

`function` · `datafusion_datasource_csv::file_format::CsvFormat::read_to_delimited_chunks_from_stream` · datafusion-datasource-csv 55.1.0

```rust
fn read_to_delimited_chunks_from_stream<'a>(&self, stream: BoxStream<'a, Result<Bytes>>) -> BoxStream<'a, Result<Bytes>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_csv::file_format::CsvFormat", "path": "CsvFormat"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [139, 1], "end": [322, 2], "filename": "src/file_format.rs"}, "trait": null, "trait_path": null}`

Source: `src/file_format.rs:172`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-csv/55.1.0/json).

Convert a stream of bytes into a stream of [`Bytes`] containing newline
delimited CSV records, while accounting for `\` and `"`.

Unresolved upstream links (retained, not inferred): ``Bytes``.

<a id="op-4959af2c28059263df9ac1da"></a>
## with_comment

`function` · `datafusion_datasource_csv::file_format::CsvFormat::with_comment` · datafusion-datasource-csv 55.1.0

```rust
fn with_comment(self, comment: Option<u8>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_csv::file_format::CsvFormat", "path": "CsvFormat"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [139, 1], "end": [322, 2], "filename": "src/file_format.rs"}, "trait": null, "trait_path": null}`

Source: `src/file_format.rs:246`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-csv/55.1.0/json).

Lines beginning with this byte are ignored.

<a id="op-e4899961732292950ae7386d"></a>
## with_delimiter

`function` · `datafusion_datasource_csv::file_format::CsvFormat::with_delimiter` · datafusion-datasource-csv 55.1.0

```rust
fn with_delimiter(self, delimiter: u8) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_csv::file_format::CsvFormat", "path": "CsvFormat"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [139, 1], "end": [322, 2], "filename": "src/file_format.rs"}, "trait": null, "trait_path": null}`

Source: `src/file_format.rs:253`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-csv/55.1.0/json).

The character separating values within a row.
- default to ','

<a id="op-310719f89daf3c61b9bd450b"></a>
## with_escape

`function` · `datafusion_datasource_csv::file_format::CsvFormat::with_escape` · datafusion-datasource-csv 55.1.0

```rust
fn with_escape(self, escape: Option<u8>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_csv::file_format::CsvFormat", "path": "CsvFormat"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [139, 1], "end": [322, 2], "filename": "src/file_format.rs"}, "trait": null, "trait_path": null}`

Source: `src/file_format.rs:267`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-csv/55.1.0/json).

The escape character in a row.
- default is None

<a id="op-9a2c6a37f3b77e968827d2b0"></a>
## with_file_compression_type

`function` · `datafusion_datasource_csv::file_format::CsvFormat::with_file_compression_type` · datafusion-datasource-csv 55.1.0

```rust
fn with_file_compression_type(self, file_compression_type: FileCompressionType) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_csv::file_format::CsvFormat", "path": "CsvFormat"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [139, 1], "end": [322, 2], "filename": "src/file_format.rs"}, "trait": null, "trait_path": null}`

Source: `src/file_format.rs:293`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-csv/55.1.0/json).

Set a `FileCompressionType` of CSV
- defaults to `FileCompressionType::UNCOMPRESSED`

<a id="op-f2e76ddbc8df483fb0cafb8e"></a>
## with_has_header

`function` · `datafusion_datasource_csv::file_format::CsvFormat::with_has_header` · datafusion-datasource-csv 55.1.0

```rust
fn with_has_header(self, has_header: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_csv::file_format::CsvFormat", "path": "CsvFormat"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [139, 1], "end": [322, 2], "filename": "src/file_format.rs"}, "trait": null, "trait_path": null}`

Source: `src/file_format.rs:222`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-csv/55.1.0/json).

Set true to indicate that the first line is a header.
- default to true

<a id="op-b4e64fb1cf5b754ba48a768f"></a>
## with_newlines_in_values

`function` · `datafusion_datasource_csv::file_format::CsvFormat::with_newlines_in_values` · datafusion-datasource-csv 55.1.0

```rust
fn with_newlines_in_values(self, newlines_in_values: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_csv::file_format::CsvFormat", "path": "CsvFormat"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [139, 1], "end": [322, 2], "filename": "src/file_format.rs"}, "trait": null, "trait_path": null}`

Source: `src/file_format.rs:286`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-csv/55.1.0/json).

Specifies whether newlines in (quoted) values are supported.

Parsing newlines in quoted values may be affected by execution behaviour such as
parallel file scanning. Setting this to `true` ensures that newlines in values are
parsed successfully, which may reduce performance.

The default behaviour depends on the `datafusion.catalog.newlines_in_values` setting.

<a id="op-27f294810b62fe55a62779bc"></a>
## with_null_regex

`function` · `datafusion_datasource_csv::file_format::CsvFormat::with_null_regex` · datafusion-datasource-csv 55.1.0

```rust
fn with_null_regex(self, null_regex: Option<String>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_csv::file_format::CsvFormat", "path": "CsvFormat"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [139, 1], "end": [322, 2], "filename": "src/file_format.rs"}, "trait": null, "trait_path": null}`

Source: `src/file_format.rs:234`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-csv/55.1.0/json).

Set the regex to use for null values in the CSV reader.
- default to treat empty values as null.

<a id="op-8611c5e7d6709d2afb51eb4c"></a>
## with_options

`function` · `datafusion_datasource_csv::file_format::CsvFormat::with_options` · datafusion-datasource-csv 55.1.0

```rust
fn with_options(self, options: CsvOptions) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_csv::file_format::CsvFormat", "path": "CsvFormat"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [139, 1], "end": [322, 2], "filename": "src/file_format.rs"}, "trait": null, "trait_path": null}`

Source: `src/file_format.rs:198`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-csv/55.1.0/json).

Set the csv options

<a id="op-c2ad5865b2a8e5a9a93cd594"></a>
## with_quote

`function` · `datafusion_datasource_csv::file_format::CsvFormat::with_quote` · datafusion-datasource-csv 55.1.0

```rust
fn with_quote(self, quote: u8) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_csv::file_format::CsvFormat", "path": "CsvFormat"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [139, 1], "end": [322, 2], "filename": "src/file_format.rs"}, "trait": null, "trait_path": null}`

Source: `src/file_format.rs:260`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-csv/55.1.0/json).

The quote character in a row.
- default to '"'

<a id="op-dea318d264199b2757813fa7"></a>
## with_schema_infer_max_rec

`function` · `datafusion_datasource_csv::file_format::CsvFormat::with_schema_infer_max_rec` · datafusion-datasource-csv 55.1.0

```rust
fn with_schema_infer_max_rec(self, max_rec: usize) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_csv::file_format::CsvFormat", "path": "CsvFormat"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [139, 1], "end": [322, 2], "filename": "src/file_format.rs"}, "trait": null, "trait_path": null}`

Source: `src/file_format.rs:215`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-csv/55.1.0/json).

Set a limit in terms of records to scan to infer the schema
- default to `DEFAULT_SCHEMA_INFER_MAX_RECORD`

# Behavior when set to 0

When `max_rec` is set to 0, schema inference is disabled and all fields
will be inferred as `Utf8` (string) type, regardless of their actual content.

<a id="op-904c18f648fb01240b701e21"></a>
## with_terminator

`function` · `datafusion_datasource_csv::file_format::CsvFormat::with_terminator` · datafusion-datasource-csv 55.1.0

```rust
fn with_terminator(self, terminator: Option<u8>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_csv::file_format::CsvFormat", "path": "CsvFormat"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [139, 1], "end": [322, 2], "filename": "src/file_format.rs"}, "trait": null, "trait_path": null}`

Source: `src/file_format.rs:274`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-csv/55.1.0/json).

The character used to indicate the end of a row.
- default to None (CRLF)

<a id="op-f31da286fd98bab62d650f2d"></a>
## with_truncate_rows

`function` · `datafusion_datasource_csv::file_format::CsvFormat::with_truncate_rows` · datafusion-datasource-csv 55.1.0

```rust
fn with_truncate_rows(self, truncate_rows: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_csv::file_format::CsvFormat", "path": "CsvFormat"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [139, 1], "end": [322, 2], "filename": "src/file_format.rs"}, "trait": null, "trait_path": null}`

Source: `src/file_format.rs:303`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-csv/55.1.0/json).

Set whether rows should be truncated to the column width
- defaults to false

<a id="op-a72b377e7e22c57976975ef6"></a>
## with_truncated_rows

`function` · `datafusion_datasource_csv::file_format::CsvFormat::with_truncated_rows` · datafusion-datasource-csv 55.1.0

```rust
fn with_truncated_rows(self, truncated_rows: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_csv::file_format::CsvFormat", "path": "CsvFormat"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [139, 1], "end": [322, 2], "filename": "src/file_format.rs"}, "trait": null, "trait_path": null}`

Source: `src/file_format.rs:227`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-csv/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
