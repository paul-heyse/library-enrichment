# `parquet::file::metadata::options`

Crate `parquet` · 2 public items · structured records in [`model/parquet.file.metadata.options.json`](../model/parquet.file.metadata.options.json)

## ParquetStatisticsPolicy

`enum` · `parquet::file::metadata::options::ParquetStatisticsPolicy`

Also reachable as `parquet::file::metadata::ParquetStatisticsPolicy`

```rust
enum ParquetStatisticsPolicy
```

**Variants**: `KeepAll`, `SkipAll`, `SkipExcept`

**Derives**: Clone, Debug, Default

**Methods** (1)

```rust
fn skip_except(keep: &[usize]) -> Self
```

Enum to control decoding of some Parquet statistics fields.

# Example
```rust
use parquet::file::metadata::ParquetStatisticsPolicy;
use parquet::file::serialized_reader::ReadOptionsBuilder;
use parquet::arrow::arrow_reader::ArrowReaderOptions;

// Set arrow options to skip encoding statistics for all columns.
let options =
    ArrowReaderOptions::new().with_encoding_stats_policy(ParquetStatisticsPolicy::SkipAll);

// Set serialized reader options to decode encoding statistics for all columns.
let options =
    ReadOptionsBuilder::new().with_encoding_stats_policy(ParquetStatisticsPolicy::KeepAll)
    .build();

// Set arrow options to skip encoding statistics for all columns, but to decode statistics
// for columns 0 and 1.
let options = ArrowReaderOptions::new()
    .with_encoding_stats_policy(ParquetStatisticsPolicy::skip_except(&[0, 1]));
```

---

## ParquetMetaDataOptions

`struct` · `parquet::file::metadata::options::ParquetMetaDataOptions`

Also reachable as `parquet::file::metadata::ParquetMetaDataOptions`

```rust
struct ParquetMetaDataOptions
```

**Derives**: Clone, Debug, Default

**Methods** (16)

```rust
fn encoding_stats_as_mask(&self) -> bool
fn new() -> Self
fn schema(&self) -> Option<&SchemaDescPtr>
fn set_column_stats_policy(&mut self, policy: ParquetStatisticsPolicy)
fn set_encoding_stats_as_mask(&mut self, val: bool)
fn set_encoding_stats_policy(&mut self, policy: ParquetStatisticsPolicy)
fn set_schema(&mut self, val: SchemaDescPtr)
fn set_size_stats_policy(&mut self, policy: ParquetStatisticsPolicy)
fn skip_column_stats(&self, col_index: usize) -> bool
fn skip_encoding_stats(&self, col_index: usize) -> bool
fn skip_size_stats(&self, col_index: usize) -> bool
fn with_column_stats_policy(self, policy: ParquetStatisticsPolicy) -> Self
fn with_encoding_stats_as_mask(self, val: bool) -> Self
fn with_encoding_stats_policy(self, policy: ParquetStatisticsPolicy) -> Self
fn with_schema(self, val: SchemaDescPtr) -> Self
fn with_size_stats_policy(self, policy: ParquetStatisticsPolicy) -> Self
```

Options that can be set to control what parts of the Parquet file footer
metadata will be decoded and made present in the [`ParquetMetaData`] returned
by [`ParquetMetaDataReader`] and [`ParquetMetaDataPushDecoder`].

[`ParquetMetaData`]: crate::file::metadata::ParquetMetaData
[`ParquetMetaDataReader`]: crate::file::metadata::ParquetMetaDataReader
[`ParquetMetaDataPushDecoder`]: crate::file::metadata::ParquetMetaDataPushDecoder

---
