# `deltalake_core::datafile::properties`

Crate `deltalake-core` · 1 public items · structured records in [`model/deltalake_core.datafile.properties.json`](../model/deltalake_core.datafile.properties.json)

## ReaderProperties

`struct` · `deltalake_core::datafile::properties::ReaderProperties`

Also reachable as `deltalake::datafile::ReaderProperties`, `deltalake::datafile::properties::ReaderProperties`, `deltalake_core::datafile::ReaderProperties`

```rust
struct ReaderProperties
```

**Derives**: Clone, Debug, Default

**Methods** (1)

```rust
fn to_table_parquet_options(&self, session: &dyn datafusion::catalog::Session) -> datafusion::config::TableParquetOptions
```

Engine-agnostic parquet read configuration for a Delta scan.

---
