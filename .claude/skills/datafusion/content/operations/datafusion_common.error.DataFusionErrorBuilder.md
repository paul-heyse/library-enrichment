# `datafusion_common::error::DataFusionErrorBuilder`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.error.DataFusionErrorBuilder.json).

<a id="op-a001bb0074aab0c30a3f7bff"></a>
## DataFusionErrorBuilder

`struct` · `datafusion_common::error::DataFusionErrorBuilder` · datafusion-common 55.1.0

```rust
struct DataFusionErrorBuilder
```

Source: `src/error.rs:763`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

A builder for [`DataFusionError`](../operations/datafusion_common.error.DataFusionError.md#op-d335eb18d3c98363748854bb)

This builder can be used to collect multiple errors and return them as a
[`DataFusionError::Collection`](../operations/datafusion_common.error.DataFusionError.md#op-4abb12a883183e8b5af99955).

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

<a id="op-901d5f50dd7a2e0d04ef1e62"></a>
## add_error

`function` · `datafusion_common::error::DataFusionErrorBuilder::add_error` · datafusion-common 55.1.0

```rust
fn add_error(&mut self, error: DataFusionError)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::error::DataFusionErrorBuilder", "path": "DataFusionErrorBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [765, 1], "end": [813, 2], "filename": "src/error.rs"}, "trait": null, "trait_path": null}`

Source: `src/error.rs:783`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Add an error to the in progress list

# Example
```
# use datafusion_common::{assert_contains, DataFusionError};
let mut builder = DataFusionError::builder();
builder.add_error(DataFusionError::Internal("foo".to_owned()));
assert_contains!(
    builder.error_or(42).unwrap_err().to_string(),
    "Internal error: foo"
);
```

<a id="op-3228235bd02d6146d71a2a9b"></a>
## default

`function` · `datafusion_common::error::DataFusionErrorBuilder::default` · datafusion-common 55.1.0

```rust
fn default() -> DataFusionErrorBuilder
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::error::DataFusionErrorBuilder", "path": "DataFusionErrorBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [762, 17], "end": [762, 24], "filename": "src/error.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/error.rs:762`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-13321c3faa0b17eebeac3287"></a>
## error_or

`function` · `datafusion_common::error::DataFusionErrorBuilder::error_or` · datafusion-common 55.1.0

```rust
fn error_or<T>(self, ok: T) -> Result<T, DataFusionError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::error::DataFusionErrorBuilder", "path": "DataFusionErrorBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [765, 1], "end": [813, 2], "filename": "src/error.rs"}, "trait": null, "trait_path": null}`

Source: `src/error.rs:806`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Returns `Ok(ok)` if no errors were added to the builder,
otherwise returns a `Result::Err`

<a id="op-0b3fffdd7919e2155abf830f"></a>
## fmt

`function` · `datafusion_common::error::DataFusionErrorBuilder::fmt` · datafusion-common 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::error::DataFusionErrorBuilder", "path": "DataFusionErrorBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [762, 10], "end": [762, 15], "filename": "src/error.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/error.rs:762`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b6e68bb0e8449e3232f101b6"></a>
## new

`function` · `datafusion_common::error::DataFusionErrorBuilder::new` · datafusion-common 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::error::DataFusionErrorBuilder", "path": "DataFusionErrorBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [765, 1], "end": [813, 2], "filename": "src/error.rs"}, "trait": null, "trait_path": null}`

Source: `src/error.rs:767`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Create a new [`DataFusionErrorBuilder`](../operations/datafusion_common.error.DataFusionErrorBuilder.md#op-a001bb0074aab0c30a3f7bff)

<a id="op-bea42ebbb9773567d337e205"></a>
## with_error

`function` · `datafusion_common::error::DataFusionErrorBuilder::with_error` · datafusion-common 55.1.0

```rust
fn with_error(self, error: DataFusionError) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::error::DataFusionErrorBuilder", "path": "DataFusionErrorBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [765, 1], "end": [813, 2], "filename": "src/error.rs"}, "trait": null, "trait_path": null}`

Source: `src/error.rs:799`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Add an error to the in progress list, returning the builder

# Example
```
# use datafusion_common::{assert_contains, DataFusionError};
let builder = DataFusionError::builder()
    .with_error(DataFusionError::Internal("foo".to_owned()));
assert_contains!(
    builder.error_or(42).unwrap_err().to_string(),
    "Internal error: foo"
);
```
