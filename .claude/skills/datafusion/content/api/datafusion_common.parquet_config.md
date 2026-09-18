# `datafusion_common::parquet_config`

Crate `datafusion-common` · 1 public items · structured records in [`model/datafusion_common.parquet_config.json`](../model/datafusion_common.parquet_config.json)

## DFParquetWriterVersion

`enum` · `datafusion_common::parquet_config::DFParquetWriterVersion`

```rust
enum DFParquetWriterVersion
```

**Variants**: `V1_0`, `V2_0`

**Implements**: `core::convert::From`, `core::fmt::Display`, `core::str::traits::FromStr`, `datafusion_common::config::ConfigField`

**Derives**: Clone, Copy, Debug, Default, Eq, PartialEq, StructuralPartialEq

**via `core::convert::From`**

```rust
fn from(version: parquet::file::properties::WriterVersion) -> Self
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

**via `core::str::traits::FromStr`**

```rust
fn from_str(s: &str) -> Result<Self, Self::Err>
```

**via `datafusion_common::config::ConfigField`**

```rust
fn set(&mut self, _: &str, value: &str) -> Result<()>
fn visit<V: Visit>(&self, v: &mut V, key: &str, description: &'static str)
```

[Full member, field, variant and typed contracts](../operations/datafusion_common.parquet_config.DFParquetWriterVersion.md).


Parquet writer version options for controlling the Parquet file format version

This enum validates parquet writer version values at configuration time,
ensuring only valid versions ("1.0" or "2.0") can be set via `SET` commands
or proto deserialization.

---
