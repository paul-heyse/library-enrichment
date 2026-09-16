# `deltalake_core::kernel::error`

Crate `deltalake-core` · 2 public items · structured records in [`model/deltalake_core.kernel.error.json`](../model/deltalake_core.kernel.error.json)

## Error

`enum` · `deltalake_core::kernel::error::Error`

Also reachable as `deltalake::kernel::Error`, `deltalake::kernel::error::Error`, `deltalake_core::kernel::Error`

```rust
enum Error
```

**Variants**: `Arrow`, `Generic`, `GenericError`, `Parquet`, `ObjectStore`, `FileNotFound`, `MissingColumn`, `UnexpectedColumnType`, `MissingData`, `MissingVersion`, `DeletionVector`, `Schema`, `InvalidUrl`, `MalformedJson`, `MissingMetadata`, `InvalidInvariantJson`, `InvalidGenerationExpressionJson`, `MetadataError`, `Parse`

**Implements**: `core::convert::From`, `core::error::Error`, `core::fmt::Display`

**Derives**: Debug

**via `core::convert::From`**

```rust
fn from(source: arrow_schema::ArrowError) -> Self
fn from(source: url::ParseError) -> Self
fn from(source: object_store::Error) -> Self
fn from(source: parquet::errors::ParquetError) -> Self
fn from(source: serde_json::Error) -> Self
```

**via `core::error::Error`**

```rust
fn source(&self) -> ::core::option::Option<&dyn ::thiserror::__private20::Error + 'static>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, __formatter: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result
```

---

## DeltaResult

`type_alias` · `deltalake_core::kernel::error::DeltaResult`

Also reachable as `deltalake::kernel::DeltaResult`, `deltalake::kernel::error::DeltaResult`, `deltalake_core::kernel::DeltaResult`

```rust
type DeltaResult<T, E = Error> = std::result::Result<T, E>
```

A specialized [`Result`] type for Delta Lake operations.

---
