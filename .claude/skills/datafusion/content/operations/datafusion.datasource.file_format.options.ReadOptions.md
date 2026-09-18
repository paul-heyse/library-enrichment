# `datafusion::datasource::file_format::options::ReadOptions`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion.datasource.file_format.options.ReadOptions.json).

<a id="op-5909528fc1e7b63b814aa152"></a>
## ReadOptions

`trait` · `datafusion::datasource::file_format::options::ReadOptions` · datafusion 55.1.0

```rust
trait ReadOptions<'a>
```

Source: `src/datasource/file_format/options.rs:583`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

['ReadOptions'] is implemented by Options like ['CsvReadOptions'] that control the reading of respective files/sources.

<a id="op-3ba38fee1e4dff9b919bfb14"></a>
## _get_resolved_schema

`function` · `datafusion::datasource::file_format::options::ReadOptions::_get_resolved_schema` · datafusion 55.1.0

```rust
async fn _get_resolved_schema(&'a self, config: &SessionConfig, state: SessionState, table_path: ListingTableUrl, schema: Option<&'a Schema>) -> Result<SchemaRef>
```

Source: `src/datasource/file_format/options.rs:605`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

helper function to reduce repetitive code. Infers the schema from sources if not provided. Infinite data sources not supported through this function.

<a id="op-593d40b82e7eff02b7dc9bc6"></a>
## get_resolved_schema

`function` · `datafusion::datasource::file_format::options::ReadOptions::get_resolved_schema` · datafusion 55.1.0

```rust
async fn get_resolved_schema(&self, config: &SessionConfig, state: SessionState, table_path: ListingTableUrl) -> Result<SchemaRef>
```

Source: `src/datasource/file_format/options.rs:592`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Infer and resolve the schema from the files/sources provided.

<a id="op-242f8e239703fd58a505846f"></a>
## schema_source

`function` · `datafusion::datasource::file_format::options::ReadOptions::schema_source` · datafusion 55.1.0

```rust
fn schema_source(&self) -> SchemaSource
```

Source: `src/datasource/file_format/options.rs:600`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Returns whether the read schema was inferred or specified.

<a id="op-c560342b8f8885802335666f"></a>
## to_listing_options

`function` · `datafusion::datasource::file_format::options::ReadOptions::to_listing_options` · datafusion 55.1.0

```rust
fn to_listing_options(&self, config: &SessionConfig, table_options: TableOptions) -> ListingOptions
```

Source: `src/datasource/file_format/options.rs:585`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Helper to convert these user facing options to `ListingTable` options
