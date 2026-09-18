# `datafusion_datasource_csv::source::CsvSource`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_datasource_csv.source.CsvSource.json).

<a id="op-6bce2d9b2687e195f4c66c00"></a>
## CsvSource

`struct` · `datafusion_datasource_csv::source::CsvSource` · datafusion-datasource-csv 55.1.0

```rust
struct CsvSource
```

Source: `src/source.rs:87`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-csv/55.1.0/json).

A Config for [`CsvOpener`](../operations/datafusion_datasource_csv.source.CsvOpener.md#op-7aec00729af8c1d605c18193)

# Example: create a `DataSourceExec` for CSV
```
# use std::sync::Arc;
# use arrow::datatypes::Schema;
# use datafusion_datasource::file_scan_config::{FileScanConfig, FileScanConfigBuilder};
# use datafusion_datasource::PartitionedFile;
# use datafusion_datasource_csv::source::CsvSource;
# use datafusion_execution::object_store::ObjectStoreUrl;
# use datafusion_datasource::source::DataSourceExec;
# use datafusion_common::config::CsvOptions;

# let object_store_url = ObjectStoreUrl::local_filesystem();
# let file_schema = Arc::new(Schema::empty());

let options = CsvOptions {
    has_header: Some(true),
    delimiter: b',',
    quote: b'"',
    newlines_in_values: Some(true), // The file contains newlines in values
    ..Default::default()
};
let source = Arc::new(CsvSource::new(file_schema.clone())
    .with_csv_options(options)
    .with_terminator(Some(b'#'))
);
// Create a DataSourceExec for reading the first 100MB of `file1.csv`
let config = FileScanConfigBuilder::new(object_store_url, source)
    .with_file(PartitionedFile::new("file1.csv", 100*1024*1024))
    .build();
let exec = (DataSourceExec::from_data_source(config));
```

<a id="op-a59e7daf0d734d7584db80e9"></a>
## apply_expressions

`function` · `datafusion_datasource_csv::source::CsvSource::apply_expressions` · datafusion-datasource-csv 55.1.0

```rust
fn apply_expressions(&self, f: &mut dyn FnMut(&Arc<dyn datafusion_physical_plan::PhysicalExpr>) -> Result<TreeNodeRecursion>) -> Result<TreeNodeRecursion>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_csv::source::CsvSource", "path": "CsvSource"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [243, 1], "end": [364, 2], "filename": "src/source.rs"}, "trait": {"args": null, "id": "datafusion_datasource::file::FileSource", "path": "FileSource"}, "trait_path": "datafusion_datasource::file::FileSource"}`

Source: `src/source.rs:313`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-csv/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b89351e4ae7abaf2c081100e"></a>
## clone

`function` · `datafusion_datasource_csv::source::CsvSource::clone` · datafusion-datasource-csv 55.1.0

```rust
fn clone(&self) -> CsvSource
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_csv::source::CsvSource", "path": "CsvSource"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [86, 17], "end": [86, 22], "filename": "src/source.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/source.rs:86`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-csv/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0a3a9713dcf8a91648e6c056"></a>
## comment

`function` · `datafusion_datasource_csv::source::CsvSource::comment` · datafusion-datasource-csv 55.1.0

```rust
fn comment(&self) -> Option<u8>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_csv::source::CsvSource", "path": "CsvSource"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [95, 1], "end": [180, 2], "filename": "src/source.rs"}, "trait": null, "trait_path": null}`

Source: `src/source.rs:139`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-csv/55.1.0/json).

Lines beginning with this byte are ignored.

<a id="op-97cd386fddfb6c65304a7251"></a>
## create_file_opener

`function` · `datafusion_datasource_csv::source::CsvSource::create_file_opener` · datafusion-datasource-csv 55.1.0

```rust
fn create_file_opener(&self, object_store: Arc<dyn ObjectStore>, base_config: &FileScanConfig, partition_index: usize) -> Result<Arc<dyn FileOpener>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_csv::source::CsvSource", "path": "CsvSource"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [243, 1], "end": [364, 2], "filename": "src/source.rs"}, "trait": {"args": null, "id": "datafusion_datasource::file::FileSource", "path": "FileSource"}, "trait_path": "datafusion_datasource::file::FileSource"}`

Source: `src/source.rs:244`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-csv/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6225d1c243cc6aa5d44e1b91"></a>
## delimiter

`function` · `datafusion_datasource_csv::source::CsvSource::delimiter` · datafusion-datasource-csv 55.1.0

```rust
fn delimiter(&self) -> u8
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_csv::source::CsvSource", "path": "CsvSource"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [95, 1], "end": [180, 2], "filename": "src/source.rs"}, "trait": null, "trait_path": null}`

Source: `src/source.rs:124`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-csv/55.1.0/json).

A column delimiter

<a id="op-ce9cc476a069224ec2ab6023"></a>
## escape

`function` · `datafusion_datasource_csv::source::CsvSource::escape` · datafusion-datasource-csv 55.1.0

```rust
fn escape(&self) -> Option<u8>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_csv::source::CsvSource", "path": "CsvSource"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [95, 1], "end": [180, 2], "filename": "src/source.rs"}, "trait": null, "trait_path": null}`

Source: `src/source.rs:144`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-csv/55.1.0/json).

The escape character

<a id="op-58c19814ef08fc6b16116aae"></a>
## file_type

`function` · `datafusion_datasource_csv::source::CsvSource::file_type` · datafusion-datasource-csv 55.1.0

```rust
fn file_type(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_csv::source::CsvSource", "path": "CsvSource"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [243, 1], "end": [364, 2], "filename": "src/source.rs"}, "trait": {"args": null, "id": "datafusion_datasource::file::FileSource", "path": "FileSource"}, "trait_path": "datafusion_datasource::file::FileSource"}`

Source: `src/source.rs:294`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-csv/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3e86e0122b59a301ac05f29d"></a>
## fmt

`function` · `datafusion_datasource_csv::source::CsvSource::fmt` · datafusion-datasource-csv 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_csv::source::CsvSource", "path": "CsvSource"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [86, 10], "end": [86, 15], "filename": "src/source.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/source.rs:86`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-csv/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bf8910ead1a36e28b8641862"></a>
## fmt_extra

`function` · `datafusion_datasource_csv::source::CsvSource::fmt_extra` · datafusion-datasource-csv 55.1.0

```rust
fn fmt_extra(&self, t: DisplayFormatType, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_csv::source::CsvSource", "path": "CsvSource"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [243, 1], "end": [364, 2], "filename": "src/source.rs"}, "trait": {"args": null, "id": "datafusion_datasource::file::FileSource", "path": "FileSource"}, "trait_path": "datafusion_datasource::file::FileSource"}`

Source: `src/source.rs:304`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-csv/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-798cb1990d287f644ff8f2eb"></a>
## has_header

`function` · `datafusion_datasource_csv::source::CsvSource::has_header` · datafusion-datasource-csv 55.1.0

```rust
fn has_header(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_csv::source::CsvSource", "path": "CsvSource"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [95, 1], "end": [180, 2], "filename": "src/source.rs"}, "trait": null, "trait_path": null}`

Source: `src/source.rs:115`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-csv/55.1.0/json).

true if the first line of each file is a header

<a id="op-90fa63ef5c950ad24bd094ec"></a>
## metrics

`function` · `datafusion_datasource_csv::source::CsvSource::metrics` · datafusion-datasource-csv 55.1.0

```rust
fn metrics(&self) -> &ExecutionPlanMetricsSet
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_csv::source::CsvSource", "path": "CsvSource"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [243, 1], "end": [364, 2], "filename": "src/source.rs"}, "trait": {"args": null, "id": "datafusion_datasource::file::FileSource", "path": "FileSource"}, "trait_path": "datafusion_datasource::file::FileSource"}`

Source: `src/source.rs:290`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-csv/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-243fd1f05ee65dfb763f6d81"></a>
## new

`function` · `datafusion_datasource_csv::source::CsvSource::new` · datafusion-datasource-csv 55.1.0

```rust
fn new(table_schema: impl Into<TableSchema>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_csv::source::CsvSource", "path": "CsvSource"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [95, 1], "end": [180, 2], "filename": "src/source.rs"}, "trait": null, "trait_path": null}`

Source: `src/source.rs:97`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-csv/55.1.0/json).

Returns a [`CsvSource`](../operations/datafusion_datasource_csv.source.CsvSource.md#op-6bce2d9b2687e195f4c66c00)

<a id="op-7a753393915a9a001ddebedb"></a>
## newlines_in_values

`function` · `datafusion_datasource_csv::source::CsvSource::newlines_in_values` · datafusion-datasource-csv 55.1.0

```rust
fn newlines_in_values(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_csv::source::CsvSource", "path": "CsvSource"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [95, 1], "end": [180, 2], "filename": "src/source.rs"}, "trait": null, "trait_path": null}`

Source: `src/source.rs:177`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-csv/55.1.0/json).

Whether values may contain newline characters

<a id="op-63e522ac1daf6cccb6aaf095"></a>
## projection

`function` · `datafusion_datasource_csv::source::CsvSource::projection` · datafusion-datasource-csv 55.1.0

```rust
fn projection(&self) -> Option<&ProjectionExprs>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_csv::source::CsvSource", "path": "CsvSource"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [243, 1], "end": [364, 2], "filename": "src/source.rs"}, "trait": {"args": null, "id": "datafusion_datasource::file::FileSource", "path": "FileSource"}, "trait_path": "datafusion_datasource::file::FileSource"}`

Source: `src/source.rs:286`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-csv/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5e67f85e54e013cb28adc0a5"></a>
## quote

`function` · `datafusion_datasource_csv::source::CsvSource::quote` · datafusion-datasource-csv 55.1.0

```rust
fn quote(&self) -> u8
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_csv::source::CsvSource", "path": "CsvSource"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [95, 1], "end": [180, 2], "filename": "src/source.rs"}, "trait": null, "trait_path": null}`

Source: `src/source.rs:129`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-csv/55.1.0/json).

The quote character

<a id="op-4b4397a6b36832927ce24df4"></a>
## supports_repartitioning

`function` · `datafusion_datasource_csv::source::CsvSource::supports_repartitioning` · datafusion-datasource-csv 55.1.0

```rust
fn supports_repartitioning(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_csv::source::CsvSource", "path": "CsvSource"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [243, 1], "end": [364, 2], "filename": "src/source.rs"}, "trait": {"args": null, "id": "datafusion_datasource::file::FileSource", "path": "FileSource"}, "trait_path": "datafusion_datasource::file::FileSource"}`

Source: `src/source.rs:298`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-csv/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-dd096e62388acc16dffa9ec7"></a>
## table_schema

`function` · `datafusion_datasource_csv::source::CsvSource::table_schema` · datafusion-datasource-csv 55.1.0

```rust
fn table_schema(&self) -> &TableSchema
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_csv::source::CsvSource", "path": "CsvSource"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [243, 1], "end": [364, 2], "filename": "src/source.rs"}, "trait": {"args": null, "id": "datafusion_datasource::file::FileSource", "path": "FileSource"}, "trait_path": "datafusion_datasource::file::FileSource"}`

Source: `src/source.rs:264`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-csv/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ddae998942eb3b4ddaa367b6"></a>
## terminator

`function` · `datafusion_datasource_csv::source::CsvSource::terminator` · datafusion-datasource-csv 55.1.0

```rust
fn terminator(&self) -> Option<u8>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_csv::source::CsvSource", "path": "CsvSource"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [95, 1], "end": [180, 2], "filename": "src/source.rs"}, "trait": null, "trait_path": null}`

Source: `src/source.rs:134`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-csv/55.1.0/json).

The line terminator

<a id="op-8531d74e1f45111258a84295"></a>
## truncate_rows

`function` · `datafusion_datasource_csv::source::CsvSource::truncate_rows` · datafusion-datasource-csv 55.1.0

```rust
fn truncate_rows(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_csv::source::CsvSource", "path": "CsvSource"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [95, 1], "end": [180, 2], "filename": "src/source.rs"}, "trait": null, "trait_path": null}`

Source: `src/source.rs:120`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-csv/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c82000efe83d56c841dcaa69"></a>
## try_from_proto

`function` · `datafusion_datasource_csv::source::CsvSource::try_from_proto` · datafusion-datasource-csv 55.1.0

```rust
fn try_from_proto(node: &datafusion_proto_models::protobuf::PhysicalPlanNode, ctx: &datafusion_physical_plan::proto::ExecutionPlanDecodeCtx<'_>) -> Result<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_csv::source::CsvSource", "path": "CsvSource"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [580, 1], "end": [650, 2], "filename": "src/source.rs"}, "trait": null, "trait_path": null}`

Source: `src/source.rs:584`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-csv/55.1.0/json).

Reconstructs a `DataSourceExec` from a protobuf `CsvScan`.

Custom line terminators are not represented in the wire format.

<a id="op-f5d02a383e78cfc6e39f4c54"></a>
## try_pushdown_projection

`function` · `datafusion_datasource_csv::source::CsvSource::try_pushdown_projection` · datafusion-datasource-csv 55.1.0

```rust
fn try_pushdown_projection(&self, projection: &ProjectionExprs) -> Result<Option<Arc<dyn FileSource>>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_csv::source::CsvSource", "path": "CsvSource"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [243, 1], "end": [364, 2], "filename": "src/source.rs"}, "trait": {"args": null, "id": "datafusion_datasource::file::FileSource", "path": "FileSource"}, "trait_path": "datafusion_datasource::file::FileSource"}`

Source: `src/source.rs:274`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-csv/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2a259f61f8fab930b0693524"></a>
## try_to_proto

`function` · `datafusion_datasource_csv::source::CsvSource::try_to_proto` · datafusion-datasource-csv 55.1.0

```rust
fn try_to_proto(&self, base: &FileScanConfig, ctx: &datafusion_physical_plan::proto::ExecutionPlanEncodeCtx<'_>) -> Result<Option<datafusion_proto_models::protobuf::PhysicalPlanNode>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_csv::source::CsvSource", "path": "CsvSource"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [243, 1], "end": [364, 2], "filename": "src/source.rs"}, "trait": {"args": null, "id": "datafusion_datasource::file::FileSource", "path": "FileSource"}, "trait_path": "datafusion_datasource::file::FileSource"}`

Source: `src/source.rs:324`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-csv/55.1.0/json).

Emit a `CsvScan` node wrapping the shared base config and CSV options.

<a id="op-22a6c8eecf93228cb8a9b99a"></a>
## with_batch_size

`function` · `datafusion_datasource_csv::source::CsvSource::with_batch_size` · datafusion-datasource-csv 55.1.0

```rust
fn with_batch_size(&self, batch_size: usize) -> Arc<dyn FileSource>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_csv::source::CsvSource", "path": "CsvSource"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [243, 1], "end": [364, 2], "filename": "src/source.rs"}, "trait": {"args": null, "id": "datafusion_datasource::file::FileSource", "path": "FileSource"}, "trait_path": "datafusion_datasource::file::FileSource"}`

Source: `src/source.rs:268`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-csv/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-91e2bdbdd5c8b5ddbcab06f1"></a>
## with_comment

`function` · `datafusion_datasource_csv::source::CsvSource::with_comment` · datafusion-datasource-csv 55.1.0

```rust
fn with_comment(&self, comment: Option<u8>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_csv::source::CsvSource", "path": "CsvSource"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [95, 1], "end": [180, 2], "filename": "src/source.rs"}, "trait": null, "trait_path": null}`

Source: `src/source.rs:163`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-csv/55.1.0/json).

Initialize a CsvSource with comment

<a id="op-7fd6ebae534d521b2053f3fc"></a>
## with_csv_options

`function` · `datafusion_datasource_csv::source::CsvSource::with_csv_options` · datafusion-datasource-csv 55.1.0

```rust
fn with_csv_options(self, options: CsvOptions) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_csv::source::CsvSource", "path": "CsvSource"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [95, 1], "end": [180, 2], "filename": "src/source.rs"}, "trait": null, "trait_path": null}`

Source: `src/source.rs:109`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-csv/55.1.0/json).

Sets the CSV options

<a id="op-5d5934cbbded2b626d036e80"></a>
## with_escape

`function` · `datafusion_datasource_csv::source::CsvSource::with_escape` · datafusion-datasource-csv 55.1.0

```rust
fn with_escape(&self, escape: Option<u8>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_csv::source::CsvSource", "path": "CsvSource"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [95, 1], "end": [180, 2], "filename": "src/source.rs"}, "trait": null, "trait_path": null}`

Source: `src/source.rs:149`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-csv/55.1.0/json).

Initialize a CsvSource with escape

<a id="op-1c103938bfc2be7e310f716c"></a>
## with_terminator

`function` · `datafusion_datasource_csv::source::CsvSource::with_terminator` · datafusion-datasource-csv 55.1.0

```rust
fn with_terminator(&self, terminator: Option<u8>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_csv::source::CsvSource", "path": "CsvSource"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [95, 1], "end": [180, 2], "filename": "src/source.rs"}, "trait": null, "trait_path": null}`

Source: `src/source.rs:156`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-csv/55.1.0/json).

Initialize a CsvSource with terminator

<a id="op-b374622d10c466d35a3a1177"></a>
## with_truncate_rows

`function` · `datafusion_datasource_csv::source::CsvSource::with_truncate_rows` · datafusion-datasource-csv 55.1.0

```rust
fn with_truncate_rows(&self, truncate_rows: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_csv::source::CsvSource", "path": "CsvSource"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [95, 1], "end": [180, 2], "filename": "src/source.rs"}, "trait": null, "trait_path": null}`

Source: `src/source.rs:170`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-csv/55.1.0/json).

Whether to support truncate rows when read csv file
