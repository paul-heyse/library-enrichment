# `datafusion_datasource_parquet::virtual_column`

Crate `datafusion-datasource-parquet` · 1 public items · structured records in [`model/datafusion_datasource_parquet.virtual_column.json`](../model/datafusion_datasource_parquet.virtual_column.json)

## ParquetVirtualColumn

`enum` · `datafusion_datasource_parquet::virtual_column::ParquetVirtualColumn`

Also reachable as `datafusion::datasource::physical_plan::parquet::ParquetVirtualColumn`, `datafusion_datasource_parquet::ParquetVirtualColumn`

```rust
enum ParquetVirtualColumn
```

**Variants**: `RowNumber`

**Implements**: `core::convert::TryFrom`

**Derives**: Clone, Debug

**Methods** (1)

```rust
fn field(&self) -> &FieldRef
```

**via `core::convert::TryFrom`**

```rust
fn try_from(field: &FieldRef) -> Result<Self>
```

[Full member, field, variant and typed contracts](../operations/datafusion_datasource_parquet.virtual_column.ParquetVirtualColumn.md).


A parquet virtual column validated to have a supported arrow extension
type.

Construct via [`TryFrom<&FieldRef>`]; add a new variant (and update the
`TryFrom` impl) when DataFusion gains support for another arrow-rs virtual
extension type.

---
