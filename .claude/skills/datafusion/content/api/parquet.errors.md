# `parquet::errors`

Crate `parquet` · 2 public items · structured records in [`model/parquet.errors.json`](../model/parquet.errors.json)

## ParquetError

`enum` · `parquet::errors::ParquetError`

```rust
enum ParquetError
```

**Variants**: `General`, `NYI`, `EOF`, `ArrowError`, `IndexOutOfBound`, `External`, `NeedMoreData`, `NeedMoreDataRange`

**Implements**: `core::convert::From`, `core::error::Error`, `core::fmt::Display`

**Derives**: Debug

**via `core::convert::From`**

```rust
fn from(e: TryFromIntError) -> ParquetError
fn from(e: io::Error) -> ParquetError
fn from(e: snap::Error) -> ParquetError
fn from(e: cell::BorrowMutError) -> ParquetError
fn from(e: str::Utf8Error) -> ParquetError
fn from(e: FromUtf8Error) -> ParquetError
fn from(e: ArrowError) -> ParquetError
fn from(e: object_store::Error) -> ParquetError
fn from(e: ring::error::Unspecified) -> ParquetError
```

**via `core::error::Error`**

```rust
fn source(&self) -> Option<&dyn Error + 'static>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

Parquet error enumeration

---

## Result

`type_alias` · `parquet::errors::Result`

```rust
type Result<T, E = ParquetError> = result::Result<T, E>
```

A specialized `Result` for Parquet errors.

---
