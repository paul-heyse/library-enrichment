# `datafusion_common::file_options::file_type`

Crate `datafusion-common` · 7 public items · structured records in [`model/datafusion_common.file_options.file_type.json`](../model/datafusion_common.file_options.file_type.json)

## DEFAULT_ARROW_EXTENSION

`constant` · `datafusion_common::file_options::file_type::DEFAULT_ARROW_EXTENSION`

Also reachable as `datafusion::common::DEFAULT_ARROW_EXTENSION`, `datafusion_common::DEFAULT_ARROW_EXTENSION`

```rust
const DEFAULT_ARROW_EXTENSION: &str = ".arrow"
```

The default file extension of arrow files

---

## DEFAULT_AVRO_EXTENSION

`constant` · `datafusion_common::file_options::file_type::DEFAULT_AVRO_EXTENSION`

Also reachable as `datafusion::common::DEFAULT_AVRO_EXTENSION`, `datafusion_common::DEFAULT_AVRO_EXTENSION`

```rust
const DEFAULT_AVRO_EXTENSION: &str = ".avro"
```

The default file extension of avro files

---

## DEFAULT_CSV_EXTENSION

`constant` · `datafusion_common::file_options::file_type::DEFAULT_CSV_EXTENSION`

Also reachable as `datafusion::common::DEFAULT_CSV_EXTENSION`, `datafusion_common::DEFAULT_CSV_EXTENSION`

```rust
const DEFAULT_CSV_EXTENSION: &str = ".csv"
```

The default file extension of csv files

---

## DEFAULT_JSON_EXTENSION

`constant` · `datafusion_common::file_options::file_type::DEFAULT_JSON_EXTENSION`

Also reachable as `datafusion::common::DEFAULT_JSON_EXTENSION`, `datafusion_common::DEFAULT_JSON_EXTENSION`

```rust
const DEFAULT_JSON_EXTENSION: &str = ".json"
```

The default file extension of json files

---

## DEFAULT_PARQUET_EXTENSION

`constant` · `datafusion_common::file_options::file_type::DEFAULT_PARQUET_EXTENSION`

Also reachable as `datafusion::common::DEFAULT_PARQUET_EXTENSION`, `datafusion_common::DEFAULT_PARQUET_EXTENSION`

```rust
const DEFAULT_PARQUET_EXTENSION: &str = ".parquet"
```

The default file extension of parquet files

---

## FileType

`trait` · `datafusion_common::file_options::file_type::FileType`

```rust
trait FileType: GetExt + Display + Send + Sync
```

**Implementors** (1)

- `datafusion_datasource::file_format::DefaultFileType`

**Methods** (1)

```rust
fn as_any(&self) -> &dyn Any
```

Defines the functionality needed for logical planning for
a type of file which will be read or written to storage.

---

## GetExt

`trait` · `datafusion_common::file_options::file_type::GetExt`

Also reachable as `datafusion::common::GetExt`, `datafusion_common::GetExt`

```rust
trait GetExt
```

**Implementors** (7)

- `datafusion_datasource::file_compression_type::FileCompressionType`
- `datafusion_datasource::file_format::DefaultFileType`
- `datafusion_datasource_arrow::file_format::ArrowFormatFactory`
- `datafusion_datasource_avro::file_format::AvroFormatFactory`
- `datafusion_datasource_csv::file_format::CsvFormatFactory`
- `datafusion_datasource_json::file_format::JsonFormatFactory`
- `datafusion_datasource_parquet::file_format::ParquetFormatFactory`

**Methods** (1)

```rust
fn get_ext(&self) -> String
```

Define each `FileType`/`FileCompressionType`'s extension

---
