# `arrow_avro::errors`

Crate `arrow-avro` · 1 public items · structured records in [`model/arrow_avro.errors.json`](../model/arrow_avro.errors.json)

## AvroError

`enum` · `arrow_avro::errors::AvroError`

```rust
enum AvroError
```

**Variants**: `General`, `NYI`, `EOF`, `ArrowError`, `IndexOutOfBound`, `InvalidArgument`, `ParseError`, `SchemaError`, `External`, `IoError`, `NeedMoreData`, `NeedMoreDataRange`

**Implements**: `core::convert::From`, `core::error::Error`, `core::fmt::Display`

**Derives**: Debug

**via `core::convert::From`**

```rust
fn from(e: ArrowError) -> Self
fn from(e: FromUtf8Error) -> AvroError
fn from(e: io::Error) -> AvroError
fn from(e: TryFromIntError) -> AvroError
fn from(e: str::Utf8Error) -> AvroError
```

**via `core::error::Error`**

```rust
fn source(&self) -> Option<&dyn Error + 'static>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

[Full member, field, variant and typed contracts](../operations/arrow_avro.errors.AvroError.md).


Avro error enumeration

---
