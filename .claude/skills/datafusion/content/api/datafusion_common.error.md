# `datafusion_common::error`

Crate `datafusion-common` · 9 public items · structured records in [`model/datafusion_common.error.json`](../model/datafusion_common.error.json)

## DataFusionError

`enum` · `datafusion_common::error::DataFusionError`

Also reachable as `datafusion::common::DataFusionError`, `datafusion::error::DataFusionError`, `datafusion_common::DataFusionError`

```rust
enum DataFusionError
```

**Variants**: `ArrowError`, `ParquetError`, `ObjectStore`, `IoError`, `SQL`, `NotImplemented`, `Internal`, `Plan`, `Configuration`, `SchemaError`, `Execution`, `ExecutionJoin`, `ResourcesExhausted`, `External`, `Context`, `Substrait`, `Diagnostic`, `Collection`, `Shared`, `Ffi`

**Implements**: `core::convert::From`, `core::error::Error`, `core::fmt::Display`

**Derives**: Debug

**Methods** (10)

```rust
fn builder() -> DataFusionErrorBuilder
fn context(self, description: impl Into<String>) -> Self
fn diagnostic(&self) -> Option<&Diagnostic>
fn find_root(&self) -> &Self
fn get_back_trace() -> String
fn iter(&self) -> impl Iterator<Item = &DataFusionError>
fn message(&self) -> Cow<'_, str>
fn strip_backtrace(&self) -> String
fn with_diagnostic(self, diagnostic: Diagnostic) -> Self
fn with_diagnostic_fn<F: FnOnce(&DataFusionError) -> Diagnostic>(self, f: F) -> Self
```

**via `core::convert::From`**

```rust
fn from(e: ArrowError) -> Self
fn from(e: ParquetError) -> Self
fn from(_e: std::fmt::Error) -> Self
fn from(err: GenericError) -> Self
fn from(e: object_store::Error) -> Self
fn from(e: object_store::path::Error) -> Self
fn from(e: io::Error) -> Self
fn from(e: &Arc<DataFusionError>) -> Self
fn from(e: ParserError) -> Self
```

**via `core::error::Error`**

```rust
fn source(&self) -> Option<&dyn Error + 'static>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result
```

DataFusion error

---

## SchemaError

`enum` · `datafusion_common::error::SchemaError`

Also reachable as `datafusion::common::SchemaError`, `datafusion_common::SchemaError`

```rust
enum SchemaError
```

**Variants**: `AmbiguousReference`, `DuplicateQualifiedField`, `DuplicateUnqualifiedField`, `FieldNotFound`

**Implements**: `core::error::Error`, `core::fmt::Display`

**Derives**: Debug

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result
```

Schema-related errors

---

## add_possible_columns_to_diag

`function` · `datafusion_common::error::add_possible_columns_to_diag`

```rust
fn add_possible_columns_to_diag(diagnostic: &mut Diagnostic, field: &Column, valid_fields: &[Column])
```

---

## field_not_found

`function` · `datafusion_common::error::field_not_found`

Also reachable as `datafusion::common::field_not_found`, `datafusion_common::field_not_found`

```rust
fn field_not_found<R: Into<TableReference>>(qualifier: Option<R>, name: &str, schema: &DFSchema) -> DataFusionError
```

Create a "field not found" DataFusion::SchemaError

---

## unqualified_field_not_found

`function` · `datafusion_common::error::unqualified_field_not_found`

Also reachable as `datafusion::common::unqualified_field_not_found`, `datafusion_common::unqualified_field_not_found`

```rust
fn unqualified_field_not_found(name: &str, schema: &DFSchema) -> DataFusionError
```

Convenience wrapper over [`field_not_found`] for when there is no qualifier

---

## DataFusionErrorBuilder

`struct` · `datafusion_common::error::DataFusionErrorBuilder`

```rust
struct DataFusionErrorBuilder
```

**Derives**: Debug, Default

**Methods** (4)

```rust
fn add_error(&mut self, error: DataFusionError)
fn error_or<T>(self, ok: T) -> Result<T, DataFusionError>
fn new() -> Self
fn with_error(self, error: DataFusionError) -> Self
```

A builder for [`DataFusionError`]

This builder can be used to collect multiple errors and return them as a
[`DataFusionError::Collection`].

# Example: no errors
```
# use datafusion_common::DataFusionError;
let mut builder = DataFusionError::builder();
// ok_or returns the value if no errors have been added
assert_eq!(builder.error_or(42).unwrap(), 42);
```

# Example: with errors
```
# use datafusion_common::{assert_contains, DataFusionError};
let mut builder = DataFusionError::builder();
builder.add_error(DataFusionError::Internal("foo".to_owned()));
// ok_or returns the value if no errors have been added
assert_contains!(
    builder.error_or(42).unwrap_err().to_string(),
    "Internal error: foo"
);
```

---

## GenericError

`type_alias` · `datafusion_common::error::GenericError`

```rust
type GenericError = Box<dyn Error + Send + Sync>
```

Error type for generic operations that could result in DataFusionError::External

---

## Result

`type_alias` · `datafusion_common::error::Result`

Also reachable as `datafusion::common::Result`, `datafusion::error::Result`, `datafusion_common::Result`

```rust
type Result<T, E = DataFusionError> = result::Result<T, E>
```

**Implements**: `datafusion_common::tree_node::TransformedResult`, `datafusion_physical_expr_common::metrics::baseline::RecordOutput`

**via `datafusion_common::tree_node::TransformedResult`**

```rust
fn data(self) -> Result<T>
fn tnr(self) -> Result<TreeNodeRecursion>
fn transformed(self) -> Result<bool>
```

Result type for operations that could result in an [DataFusionError]

---

## SharedResult

`type_alias` · `datafusion_common::error::SharedResult`

Also reachable as `datafusion::common::SharedResult`, `datafusion::error::SharedResult`, `datafusion_common::SharedResult`

```rust
type SharedResult<T> = result::Result<T, std::sync::Arc<DataFusionError>>
```

Result type for operations that could result in an [DataFusionError] and needs to be shared (wrapped into `Arc`).

---
