# `datafusion_common::parsers`

Crate `datafusion-common` · 2 public items · structured records in [`model/datafusion_common.parsers.json`](../model/datafusion_common.parsers.json)

## CompressionTypeVariant

`enum` · `datafusion_common::parsers::CompressionTypeVariant`

```rust
enum CompressionTypeVariant
```

**Variants**: `GZIP`, `BZIP2`, `XZ`, `ZSTD`, `UNCOMPRESSED`

**Implements**: `core::convert::From`, `core::fmt::Display`, `core::str::traits::FromStr`, `datafusion_common::config::ConfigField`

**Derives**: Clone, Copy, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
const fn is_compressed(&self) -> bool
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
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

[Full member, field, variant and typed contracts](../operations/datafusion_common.parsers.CompressionTypeVariant.md).


Readable file compression type

---

## CsvQuoteStyle

`enum` · `datafusion_common::parsers::CsvQuoteStyle`

```rust
enum CsvQuoteStyle
```

**Variants**: `Always`, `Necessary`, `NonNumeric`, `Never`

**Implements**: `core::convert::From`, `core::fmt::Display`, `core::str::traits::FromStr`, `datafusion_common::config::ConfigField`

**Derives**: Clone, Copy, Debug, Default, Eq, Hash, PartialEq, StructuralPartialEq

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
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

[Full member, field, variant and typed contracts](../operations/datafusion_common.parsers.CsvQuoteStyle.md).


CSV quote style

Controls when fields are quoted when writing CSV files.
Corresponds to [`arrow::csv::QuoteStyle`].

---
