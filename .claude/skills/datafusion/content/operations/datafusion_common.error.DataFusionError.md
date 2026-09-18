# `datafusion_common::error::DataFusionError`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.error.DataFusionError.json).

<a id="op-d335eb18d3c98363748854bb"></a>
## DataFusionError

`enum` · `datafusion_common::error::DataFusionError` · datafusion-common 55.1.0

```rust
enum DataFusionError
```

Source: `src/error.rs:69`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

DataFusion error

<a id="op-5ea8745e5c5bfce4154f3662"></a>
## ArrowError

`variant` · `datafusion_common::error::DataFusionError::ArrowError` · datafusion-common 55.1.0

```rust
ArrowError
```

Source: `src/error.rs:73`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Error returned by arrow.

2nd argument is for optional backtrace

<a id="op-3d8a2e1f1b76fa4706127b5c"></a>
## BACK_TRACE_SEP

`assoc_const` · `datafusion_common::error::DataFusionError::BACK_TRACE_SEP` · datafusion-common 55.1.0

```rust
BACK_TRACE_SEP
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::error::DataFusionError", "path": "DataFusionError"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [483, 1], "end": [736, 2], "filename": "src/error.rs"}, "trait": null, "trait_path": null}`

Source: `src/error.rs:485`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

The separator between the error message and the backtrace

<a id="op-4abb12a883183e8b5af99955"></a>
## Collection

`variant` · `datafusion_common::error::DataFusionError::Collection` · datafusion-common 55.1.0

```rust
Collection
```

Source: `src/error.rs:162`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

A collection of one or more [`DataFusionError`](../operations/datafusion_common.error.DataFusionError.md#op-d335eb18d3c98363748854bb). Useful in cases where
DataFusion can recover from an erroneous state, and produce more errors
before terminating. e.g. when planning a SELECT clause, DataFusion can
synchronize to the next `SelectItem` if the previous one had errors. The
end result is that the user can see errors about all `SelectItem`,
instead of just the first one.

<a id="op-17ff7df69f5dd95d0efed3f7"></a>
## Configuration

`variant` · `datafusion_common::error::DataFusionError::Configuration` · datafusion-common 55.1.0

```rust
Configuration
```

Source: `src/error.rs:117`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Error for invalid or unsupported configuration options.

<a id="op-d434c697b09f7fc03c5d9425"></a>
## Context

`variant` · `datafusion_common::error::DataFusionError::Context` · datafusion-common 55.1.0

```rust
Context
```

Source: `src/error.rs:147`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Error with additional context

<a id="op-9ed96321fb7bc0373c59f2bb"></a>
## Diagnostic

`variant` · `datafusion_common::error::DataFusionError::Diagnostic` · datafusion-common 55.1.0

```rust
Diagnostic
```

Source: `src/error.rs:155`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Error wrapped together with additional contextual information intended
for end users, to help them understand what went wrong by providing
human-readable messages, and locations in the source query that relate
to the error in some way.

<a id="op-03029cb87c98330b2e3da49c"></a>
## Execution

`variant` · `datafusion_common::error::DataFusionError::Execution` · datafusion-common 55.1.0

```rust
Execution
```

Source: `src/error.rs:132`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Error during execution of the query.

This error is returned when an error happens during execution due to a
malformed input. For example, the user passed malformed arguments to a
SQL method, opened a CSV file that is broken, or tried to divide an
integer by zero.

<a id="op-ddb4c42889418b066869f831"></a>
## ExecutionJoin

`variant` · `datafusion_common::error::DataFusionError::ExecutionJoin` · datafusion-common 55.1.0

```rust
ExecutionJoin
```

Source: `src/error.rs:136`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

[`JoinError`] during execution of the query.

This error can't occur for unjoined tasks, such as execution shutdown.

Unresolved upstream links (retained, not inferred): ``JoinError``.

<a id="op-dcf6dc7c6f3ca68a2eb3ad3f"></a>
## External

`variant` · `datafusion_common::error::DataFusionError::External` · datafusion-common 55.1.0

```rust
External
```

Source: `src/error.rs:145`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Errors originating from outside DataFusion's core codebase.

For example, a custom S3Error from the crate datafusion-objectstore-s3

<a id="op-63542ef531584ca4757c6d83"></a>
## Ffi

`variant` · `datafusion_common::error::DataFusionError::Ffi` · datafusion-common 55.1.0

```rust
Ffi
```

Source: `src/error.rs:172`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

An error that originated during a foreign function interface call.
Transferring errors across the FFI boundary is difficult, so the original
error will be converted to a string.

<a id="op-bdc3e1c5a07020f01c5a1dc4"></a>
## Internal

`variant` · `datafusion_common::error::DataFusionError::Internal` · datafusion-common 55.1.0

```rust
Internal
```

Source: `src/error.rs:109`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Error due to bugs in DataFusion

This error should not happen in normal usage of DataFusion. It results
from something that wasn't expected/anticipated by the implementation
and that is most likely a bug (the error message even encourages users
to open a bug report). A user should not be able to trigger internal
errors under normal circumstances by feeding in malformed queries, bad
data, etc.

Note that I/O errors (or any error that happens due to external systems)
do NOT fall under this category. See other variants such as
[`Self::IoError`](../operations/datafusion_common.error.DataFusionError.md#op-ed9d0eb2b7b6eb692654571f) and [`Self::External`](../operations/datafusion_common.error.DataFusionError.md#op-dcf6dc7c6f3ca68a2eb3ad3f).

DataFusions has internal invariants that the compiler is not always able
to check. This error is raised when one of those invariants does not
hold for some reason.

<a id="op-ed9d0eb2b7b6eb692654571f"></a>
## IoError

`variant` · `datafusion_common::error::DataFusionError::IoError` · datafusion-common 55.1.0

```rust
IoError
```

Source: `src/error.rs:81`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Error when an I/O operation fails

<a id="op-a9f69b5c9245545c3cf33247"></a>
## NotImplemented

`variant` · `datafusion_common::error::DataFusionError::NotImplemented` · datafusion-common 55.1.0

```rust
NotImplemented
```

Source: `src/error.rs:92`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Error when a feature is not yet implemented.

These errors are sometimes returned for features that are still in
development and are not entirely complete. Often, these errors are
tracked in our issue tracker.

<a id="op-852b280edf134c9e7c583125"></a>
## ObjectStore

`variant` · `datafusion_common::error::DataFusionError::ObjectStore` · datafusion-common 55.1.0

```rust
ObjectStore
```

Source: `src/error.rs:79`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Error when reading / writing to / from an object_store (e.g. S3 or LocalFile)

<a id="op-0374f34b5793887c34f92dd2"></a>
## ParquetError

`variant` · `datafusion_common::error::DataFusionError::ParquetError` · datafusion-common 55.1.0

```rust
ParquetError
```

Source: `src/error.rs:76`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Error when reading / writing Parquet data.

<a id="op-73ccb4a88f06e80cf93af715"></a>
## Plan

`variant` · `datafusion_common::error::DataFusionError::Plan` · datafusion-common 55.1.0

```rust
Plan
```

Source: `src/error.rs:115`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Error during planning of the query.

This error happens when the user provides a bad query or plan, for
example the user attempts to call a function that doesn't exist, or if
the types of a function call are not supported.

<a id="op-34eaf91f0bb9b35d94bf98e7"></a>
## ResourcesExhausted

`variant` · `datafusion_common::error::DataFusionError::ResourcesExhausted` · datafusion-common 55.1.0

```rust
ResourcesExhausted
```

Source: `src/error.rs:141`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Error when resources (such as memory of scratch disk space) are exhausted.

This error is thrown when a consumer cannot acquire additional memory
or other resources needed to execute the query from the Memory Manager.

<a id="op-787af9712871c60989902c80"></a>
## SQL

`variant` · `datafusion_common::error::DataFusionError::SQL` · datafusion-common 55.1.0

```rust
SQL
```

Source: `src/error.rs:86`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Error when SQL is syntactically incorrect.

2nd argument is for optional backtrace

<a id="op-193dc5e1320cf59cb8295ff2"></a>
## SchemaError

`variant` · `datafusion_common::error::DataFusionError::SchemaError` · datafusion-common 55.1.0

```rust
SchemaError
```

Source: `src/error.rs:125`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Error when there is a problem with the query related to schema.

This error can be returned in cases such as when schema inference is not
possible and when column names are not unique.

2nd argument is for optional backtrace
Boxing the optional backtrace to prevent <https://rust-lang.github.io/rust-clippy/master/index.html#/result_large_err>

<a id="op-6c75ecec0d5e954e4f067eb1"></a>
## Shared

`variant` · `datafusion_common::error::DataFusionError::Shared` · datafusion-common 55.1.0

```rust
Shared
```

Source: `src/error.rs:168`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

A [`DataFusionError`](../operations/datafusion_common.error.DataFusionError.md#op-d335eb18d3c98363748854bb) which shares an underlying [`DataFusionError`](../operations/datafusion_common.error.DataFusionError.md#op-d335eb18d3c98363748854bb).

This is useful when the same underlying [`DataFusionError`](../operations/datafusion_common.error.DataFusionError.md#op-d335eb18d3c98363748854bb) is passed
to multiple receivers. For example, when the source of a repartition
errors and the error is propagated to multiple consumers.

<a id="op-b88d8538a645afba3a72ddad"></a>
## Substrait

`variant` · `datafusion_common::error::DataFusionError::Substrait` · datafusion-common 55.1.0

```rust
Substrait
```

Source: `src/error.rs:150`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Errors from either mapping LogicalPlans to/from Substrait plans
or serializing/deserializing protobytes to Substrait plans

<a id="op-4bb1b66ce3bf5bdeb8abbeae"></a>
## builder

`function` · `datafusion_common::error::DataFusionError::builder` · datafusion-common 55.1.0

```rust
fn builder() -> DataFusionErrorBuilder
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::error::DataFusionError", "path": "DataFusionError"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [483, 1], "end": [736, 2], "filename": "src/error.rs"}, "trait": null, "trait_path": null}`

Source: `src/error.rs:566`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Return a [`DataFusionErrorBuilder`](../operations/datafusion_common.error.DataFusionErrorBuilder.md#op-a001bb0074aab0c30a3f7bff) to build a [`DataFusionError`](../operations/datafusion_common.error.DataFusionError.md#op-d335eb18d3c98363748854bb)

<a id="op-8370176ec495d6e553b5ec86"></a>
## context

`function` · `datafusion_common::error::DataFusionError::context` · datafusion-common 55.1.0

```rust
fn context(self, description: impl Into<String>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::error::DataFusionError", "path": "DataFusionError"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [483, 1], "end": [736, 2], "filename": "src/error.rs"}, "trait": null, "trait_path": null}`

Source: `src/error.rs:525`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

wraps self in Self::Context with a description

<a id="op-38c884d856f78a8c4cd2c207"></a>
## diagnostic

`function` · `datafusion_common::error::DataFusionError::diagnostic` · datafusion-common 55.1.0

```rust
fn diagnostic(&self) -> Option<&Diagnostic>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::error::DataFusionError", "path": "DataFusionError"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [483, 1], "end": [736, 2], "filename": "src/error.rs"}, "trait": null, "trait_path": null}`

Source: `src/error.rs:675`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Gets the [`Diagnostic`](../operations/datafusion_common.diagnostic.Diagnostic.md#op-c0ce5c2af0812acb9f56efc1) associated with the error, if any. If there is
more than one, only the outermost [`Diagnostic`](../operations/datafusion_common.diagnostic.Diagnostic.md#op-c0ce5c2af0812acb9f56efc1) is returned.

<a id="op-98f42eb90942f14aaa91115e"></a>
## find_root

`function` · `datafusion_common::error::DataFusionError::find_root` · datafusion-common 55.1.0

```rust
fn find_root(&self) -> &Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::error::DataFusionError", "path": "DataFusionError"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [483, 1], "end": [736, 2], "filename": "src/error.rs"}, "trait": null, "trait_path": null}`

Source: `src/error.rs:502`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Get deepest underlying [`DataFusionError`](../operations/datafusion_common.error.DataFusionError.md#op-d335eb18d3c98363748854bb)

[`DataFusionError`](../operations/datafusion_common.error.DataFusionError.md#op-d335eb18d3c98363748854bb)s sometimes form a chain, such as `DataFusionError::ArrowError()` in order to conform
to the correct error signature. Thus sometimes there is a chain several layers deep that can obscure the
original error. This function finds the lowest level DataFusionError possible.

For example,  `find_root` will return`DataFusionError::ResourceExhausted` given the input
```text
DataFusionError::ArrowError
  ArrowError::External
   Box(DataFusionError::Context)
     DataFusionError::ResourceExhausted
```

This may be the same as `self`.

<a id="op-17793451de50a224a7900543"></a>
## fmt

`function` · `datafusion_common::error::DataFusionError::fmt` · datafusion-common 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::error::DataFusionError", "path": "DataFusionError"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [68, 10], "end": [68, 15], "filename": "src/error.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/error.rs:68`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1ce1ecf395683392575fa9da"></a>
## fmt

`function` · `datafusion_common::error::DataFusionError::fmt` · datafusion-common 55.1.0

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::error::DataFusionError", "path": "DataFusionError"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [432, 1], "end": [438, 2], "filename": "src/error.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/error.rs:433`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-008ecabcf00517eadebe98b3"></a>
## from

`function` · `datafusion_common::error::DataFusionError::from` · datafusion-common 55.1.0

```rust
fn from(e: object_store::Error) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::error::DataFusionError", "path": "DataFusionError"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [397, 1], "end": [401, 2], "filename": "src/error.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "object_store::Error", "path": "Error"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/error.rs:398`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0605918ab46426cf90a74596"></a>
## from

`function` · `datafusion_common::error::DataFusionError::from` · datafusion-common 55.1.0

```rust
fn from(e: object_store::path::Error) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::error::DataFusionError", "path": "DataFusionError"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [404, 1], "end": [408, 2], "filename": "src/error.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "object_store::path::Error", "path": "Error"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/error.rs:405`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4d4a4e119f620d14456e9b5a"></a>
## from

`function` · `datafusion_common::error::DataFusionError::from` · datafusion-common 55.1.0

```rust
fn from(e: ParquetError) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::error::DataFusionError", "path": "DataFusionError"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [390, 1], "end": [394, 2], "filename": "src/error.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "parquet::errors::ParquetError", "path": "ParquetError"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/error.rs:391`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6fae50d45a946dbe56014127"></a>
## from

`function` · `datafusion_common::error::DataFusionError::from` · datafusion-common 55.1.0

```rust
fn from(e: io::Error) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::error::DataFusionError", "path": "DataFusionError"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [356, 1], "end": [360, 2], "filename": "src/error.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "core::io::error::Error", "path": "Error"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/error.rs:357`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-997833a804dc5a0d6e5b3f11"></a>
## from

`function` · `datafusion_common::error::DataFusionError::from` · datafusion-common 55.1.0

```rust
fn from(_e: std::fmt::Error) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::error::DataFusionError", "path": "DataFusionError"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [350, 1], "end": [354, 2], "filename": "src/error.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "core::fmt::Error", "path": "Error"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/error.rs:351`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a61c99c2c5402a7da910b8a4"></a>
## from

`function` · `datafusion_common::error::DataFusionError::from` · datafusion-common 55.1.0

```rust
fn from(e: ArrowError) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::error::DataFusionError", "path": "DataFusionError"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [362, 1], "end": [366, 2], "filename": "src/error.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "arrow_schema::error::ArrowError", "path": "ArrowError"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/error.rs:363`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d9f98fc27f535785a7270517"></a>
## from

`function` · `datafusion_common::error::DataFusionError::from` · datafusion-common 55.1.0

```rust
fn from(e: ParserError) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::error::DataFusionError", "path": "DataFusionError"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [411, 1], "end": [415, 2], "filename": "src/error.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "sqlparser::parser::ParserError", "path": "ParserError"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/error.rs:412`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ef81598d79467c94cdce22f9"></a>
## from

`function` · `datafusion_common::error::DataFusionError::from` · datafusion-common 55.1.0

```rust
fn from(err: GenericError) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::error::DataFusionError", "path": "DataFusionError"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [417, 1], "end": [430, 2], "filename": "src/error.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"dyn_trait": {"lifetime": null, "traits": [{"generic_params": [], "trait": {"args": null, "id": "core::error::Error", "path": "Error"}}, {"generic_params": [], "trait": {"args": null, "id": "core::marker::Send", "path": "Send"}}, {"generic_params": [], "trait": {"args": null, "id": "core::marker::Sync", "path": "Sync"}}]}}}], "constraints": []}}, "id": "alloc::boxed::Box", "path": "Box"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/error.rs:418`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fb526b18281cd06011b736d3"></a>
## from

`function` · `datafusion_common::error::DataFusionError::from` · datafusion-common 55.1.0

```rust
fn from(e: &Arc<DataFusionError>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::error::DataFusionError", "path": "DataFusionError"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [378, 1], "end": [387, 2], "filename": "src/error.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"borrowed_ref": {"is_mutable": false, "lifetime": null, "type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "datafusion_common::error::DataFusionError", "path": "DataFusionError"}}}], "constraints": []}}, "id": "alloc::sync::Arc", "path": "Arc"}}}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/error.rs:379`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-dcd01cfd1602f60cf910a261"></a>
## get_back_trace

`function` · `datafusion_common::error::DataFusionError::get_back_trace` · datafusion-common 55.1.0

```rust
fn get_back_trace() -> String
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::error::DataFusionError", "path": "DataFusionError"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [483, 1], "end": [736, 2], "filename": "src/error.rs"}, "trait": null, "trait_path": null}`

Source: `src/error.rs:550`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

To enable optional rust backtrace in DataFusion:
- [`Setup Env Variables`]<https://doc.rust-lang.org/std/backtrace/index.html#environment-variables>
- Enable `backtrace` cargo feature

Example:
cargo build --features 'backtrace'
RUST_BACKTRACE=1 ./app

<a id="op-c82fd848e234aca550961a51"></a>
## iter

`function` · `datafusion_common::error::DataFusionError::iter` · datafusion-common 55.1.0

```rust
fn iter(&self) -> impl Iterator<Item = &DataFusionError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::error::DataFusionError", "path": "DataFusionError"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [483, 1], "end": [736, 2], "filename": "src/error.rs"}, "trait": null, "trait_path": null}`

Source: `src/error.rs:713`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Return an iterator over this [`DataFusionError`](../operations/datafusion_common.error.DataFusionError.md#op-d335eb18d3c98363748854bb) and any other
[`DataFusionError`](../operations/datafusion_common.error.DataFusionError.md#op-d335eb18d3c98363748854bb)s in a [`DataFusionError::Collection`](../operations/datafusion_common.error.DataFusionError.md#op-4abb12a883183e8b5af99955).

Sometimes DataFusion is able to collect multiple errors in a SQL query
before terminating, e.g. across different expressions in a SELECT
statements or different sides of a UNION. This method returns an
iterator over all the errors in the collection.

For this to work, the top-level error must be a
`DataFusionError::Collection`, not something that contains it.

<a id="op-2dd187bd240633832a973d1c"></a>
## message

`function` · `datafusion_common::error::DataFusionError::message` · datafusion-common 55.1.0

```rust
fn message(&self) -> Cow<'_, str>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::error::DataFusionError", "path": "DataFusionError"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [483, 1], "end": [736, 2], "filename": "src/error.rs"}, "trait": null, "trait_path": null}`

Source: `src/error.rs:606`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-436190951de4266d42c87964"></a>
## source

`function` · `datafusion_common::error::DataFusionError::source` · datafusion-common 55.1.0

```rust
fn source(&self) -> Option<&dyn Error + 'static>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::error::DataFusionError", "path": "DataFusionError"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [440, 1], "end": [475, 2], "filename": "src/error.rs"}, "trait": {"args": null, "id": "core::error::Error", "path": "Error"}, "trait_path": "core::error::Error"}`

Source: `src/error.rs:441`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6a9807f4f84136e52e81220d"></a>
## strip_backtrace

`function` · `datafusion_common::error::DataFusionError::strip_backtrace` · datafusion-common 55.1.0

```rust
fn strip_backtrace(&self) -> String
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::error::DataFusionError", "path": "DataFusionError"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [483, 1], "end": [736, 2], "filename": "src/error.rs"}, "trait": null, "trait_path": null}`

Source: `src/error.rs:532`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Strips backtrace out of the error message
If backtrace enabled then error has a format "message" [`Self::BACK_TRACE_SEP`](../operations/datafusion_common.error.DataFusionError.md#op-3d8a2e1f1b76fa4706127b5c) "backtrace"
The method strips the backtrace and outputs "message"

<a id="op-a3663a9f727972a537333a21"></a>
## with_diagnostic

`function` · `datafusion_common::error::DataFusionError::with_diagnostic` · datafusion-common 55.1.0

```rust
fn with_diagnostic(self, diagnostic: Diagnostic) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::error::DataFusionError", "path": "DataFusionError"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [483, 1], "end": [736, 2], "filename": "src/error.rs"}, "trait": null, "trait_path": null}`

Source: `src/error.rs:658`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Wraps the error with contextual information intended for end users

<a id="op-0a47e80a28a100d5fe3772b4"></a>
## with_diagnostic_fn

`function` · `datafusion_common::error::DataFusionError::with_diagnostic_fn` · datafusion-common 55.1.0

```rust
fn with_diagnostic_fn<F: FnOnce(&DataFusionError) -> Diagnostic>(self, f: F) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::error::DataFusionError", "path": "DataFusionError"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [483, 1], "end": [736, 2], "filename": "src/error.rs"}, "trait": null, "trait_path": null}`

Source: `src/error.rs:665`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Wraps the error with contextual information intended for end users.
Takes a function that inspects the error and returns the diagnostic to
wrap it with.
