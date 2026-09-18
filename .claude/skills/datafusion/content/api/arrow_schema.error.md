# `arrow_schema::error`

Crate `arrow-schema` · 1 public items · structured records in [`model/arrow_schema.error.json`](../model/arrow_schema.error.json)

## ArrowError

`enum` · `arrow_schema::error::ArrowError`

Also reachable as `arrow::error::ArrowError`

```rust
enum ArrowError
```

**Variants**: `NotYetImplemented`, `ExternalError`, `CastError`, `MemoryError`, `ParseError`, `SchemaError`, `ComputeError`, `DivideByZero`, `ArithmeticOverflow`, `CsvError`, `JsonError`, `AvroError`, `IoError`, `IpcError`, `InvalidArgumentError`, `ParquetError`, `CDataInterface`, `DictionaryKeyOverflowError`, `RunEndIndexOverflowError`, `OffsetOverflowError`

**Implements**: `core::convert::From`, `core::error::Error`, `core::fmt::Display`

**Derives**: Debug

**Methods** (1)

```rust
fn from_external_error(error: Box<dyn Error + Send + Sync>) -> Self
```

**via `core::convert::From`**

```rust
fn from(error: std::string::FromUtf8Error) -> Self
fn from(error: std::io::IntoInnerError<W>) -> Self
fn from(error: std::io::Error) -> Self
fn from(error: std::str::Utf8Error) -> Self
```

**via `core::error::Error`**

```rust
fn source(&self) -> Option<&dyn Error + 'static>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result
```

[Full member, field, variant and typed contracts](../operations/arrow_schema.error.ArrowError.md).


Many different operations in the `arrow` crate return this error type.

---
