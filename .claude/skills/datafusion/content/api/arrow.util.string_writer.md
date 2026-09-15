# `arrow::util::string_writer`

Crate `arrow` · 1 public items · structured records in [`model/arrow.util.string_writer.json`](../model/arrow.util.string_writer.json)

## StringWriter

`struct` · `arrow::util::string_writer::StringWriter`

```rust
struct StringWriter
```

**Implements**: `core::fmt::Display`, `core::io::write::Write`

**Derives**: Debug, Default

**Methods** (1)

```rust
fn new() -> Self
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result
```

**via `core::io::write::Write`**

```rust
fn flush(&mut self) -> Result<()>
fn write(&mut self, buf: &[u8]) -> Result<usize>
```

A writer that allows writing to a `String`
like an `std::io::Write` object.

---
