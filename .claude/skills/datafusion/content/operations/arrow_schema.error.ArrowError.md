# `arrow_schema::error::ArrowError`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_schema.error.ArrowError.json).

<a id="op-0b9e83026812de4ef436507a"></a>
## ArrowError

`enum` · `arrow_schema::error::ArrowError` · arrow-schema 59.3.0

```rust
enum ArrowError
```

Source: `src/error.rs:26`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

Many different operations in the `arrow` crate return this error type.

<a id="op-50b1905690ad41ab12e54aba"></a>
## ArithmeticOverflow

`variant` · `arrow_schema::error::ArrowError::ArithmeticOverflow` · arrow-schema 59.3.0

```rust
ArithmeticOverflow
```

Source: `src/error.rs:44`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

Error when an arithmetic operation overflows.

<a id="op-9f0373ed73270a19a31c7e50"></a>
## AvroError

`variant` · `arrow_schema::error::ArrowError::AvroError` · arrow-schema 59.3.0

```rust
AvroError
```

Source: `src/error.rs:50`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

Error during Avro-related operations.

<a id="op-693f4fa8ec62cd720683f279"></a>
## CDataInterface

`variant` · `arrow_schema::error::ArrowError::CDataInterface` · arrow-schema 59.3.0

```rust
CDataInterface
```

Source: `src/error.rs:60`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

Error during import or export to/from the C Data Interface

<a id="op-981e18d3f86a65557e5f3c54"></a>
## CastError

`variant` · `arrow_schema::error::ArrowError::CastError` · arrow-schema 59.3.0

```rust
CastError
```

Source: `src/error.rs:32`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

Error during casting from one type to another.

<a id="op-064f5370d360c7e18d75a87a"></a>
## ComputeError

`variant` · `arrow_schema::error::ArrowError::ComputeError` · arrow-schema 59.3.0

```rust
ComputeError
```

Source: `src/error.rs:40`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

Error during computation.

<a id="op-13b04553323ebec3adf07009"></a>
## CsvError

`variant` · `arrow_schema::error::ArrowError::CsvError` · arrow-schema 59.3.0

```rust
CsvError
```

Source: `src/error.rs:46`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

Error during CSV-related operations.

<a id="op-dcd5f08cb1876cdb753900db"></a>
## DictionaryKeyOverflowError

`variant` · `arrow_schema::error::ArrowError::DictionaryKeyOverflowError` · arrow-schema 59.3.0

```rust
DictionaryKeyOverflowError
```

Source: `src/error.rs:62`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

Error when a dictionary key is bigger than the key type

<a id="op-778bebf44953792ac6e7d097"></a>
## DivideByZero

`variant` · `arrow_schema::error::ArrowError::DivideByZero` · arrow-schema 59.3.0

```rust
DivideByZero
```

Source: `src/error.rs:42`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

Error during division by zero.

<a id="op-7743b6b7f312140a0b8be500"></a>
## ExternalError

`variant` · `arrow_schema::error::ArrowError::ExternalError` · arrow-schema 59.3.0

```rust
ExternalError
```

Source: `src/error.rs:30`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

Wraps an external error.

<a id="op-3a1a629c977ce0ac9c1d54a3"></a>
## InvalidArgumentError

`variant` · `arrow_schema::error::ArrowError::InvalidArgumentError` · arrow-schema 59.3.0

```rust
InvalidArgumentError
```

Source: `src/error.rs:56`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

Error indicating that an unexpected or bad argument was passed to a function.

<a id="op-f403017703747f52ca054b1c"></a>
## IoError

`variant` · `arrow_schema::error::ArrowError::IoError` · arrow-schema 59.3.0

```rust
IoError
```

Source: `src/error.rs:52`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

Error during IO operations.

<a id="op-e03fc895159ab98ffcfeccb3"></a>
## IpcError

`variant` · `arrow_schema::error::ArrowError::IpcError` · arrow-schema 59.3.0

```rust
IpcError
```

Source: `src/error.rs:54`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

Error during IPC operations in `arrow-ipc` or `arrow-flight`.

<a id="op-e20623a2c35bce2eda6a4e77"></a>
## JsonError

`variant` · `arrow_schema::error::ArrowError::JsonError` · arrow-schema 59.3.0

```rust
JsonError
```

Source: `src/error.rs:48`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

Error during JSON-related operations.

<a id="op-10d5877799a3d22cfb3c8801"></a>
## MemoryError

`variant` · `arrow_schema::error::ArrowError::MemoryError` · arrow-schema 59.3.0

```rust
MemoryError
```

Source: `src/error.rs:34`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

Memory or buffer error.

<a id="op-3d6ed70c2fc7ef4bac171a0e"></a>
## NotYetImplemented

`variant` · `arrow_schema::error::ArrowError::NotYetImplemented` · arrow-schema 59.3.0

```rust
NotYetImplemented
```

Source: `src/error.rs:28`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

Returned when functionality is not yet available.

<a id="op-3284e3fae47c9db33a637717"></a>
## OffsetOverflowError

`variant` · `arrow_schema::error::ArrowError::OffsetOverflowError` · arrow-schema 59.3.0

```rust
OffsetOverflowError
```

Source: `src/error.rs:66`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

Error when the offset overflows.

<a id="op-ed327b9a24f4fb5ca6dacf1f"></a>
## ParquetError

`variant` · `arrow_schema::error::ArrowError::ParquetError` · arrow-schema 59.3.0

```rust
ParquetError
```

Source: `src/error.rs:58`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

Error during Parquet operations.

<a id="op-64fc6f343d427ce342f9317d"></a>
## ParseError

`variant` · `arrow_schema::error::ArrowError::ParseError` · arrow-schema 59.3.0

```rust
ParseError
```

Source: `src/error.rs:36`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

Error during parsing from a string.

<a id="op-5190509321acbc7476deec39"></a>
## RunEndIndexOverflowError

`variant` · `arrow_schema::error::ArrowError::RunEndIndexOverflowError` · arrow-schema 59.3.0

```rust
RunEndIndexOverflowError
```

Source: `src/error.rs:64`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

Error when the run end index in a REE array is bigger than the array length

<a id="op-9d68b2acd84e98b566036e61"></a>
## SchemaError

`variant` · `arrow_schema::error::ArrowError::SchemaError` · arrow-schema 59.3.0

```rust
SchemaError
```

Source: `src/error.rs:38`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

Error during schema-related operations.

<a id="op-925008b0fb073c71a6fb9464"></a>
## fmt

`function` · `arrow_schema::error::ArrowError::fmt` · arrow-schema 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::error::ArrowError", "path": "ArrowError"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [25, 10], "end": [25, 15], "filename": "src/error.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/error.rs:25`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c613859517ced0940900af39"></a>
## fmt

`function` · `arrow_schema::error::ArrowError::fmt` · arrow-schema 59.3.0

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::error::ArrowError", "path": "ArrowError"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [100, 1], "end": [139, 2], "filename": "src/error.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/error.rs:101`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3623167aaa097139a0d9ade4"></a>
## from

`function` · `arrow_schema::error::ArrowError::from` · arrow-schema 59.3.0

```rust
fn from(error: std::string::FromUtf8Error) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::error::ArrowError", "path": "ArrowError"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [88, 1], "end": [92, 2], "filename": "src/error.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "alloc::string::FromUtf8Error", "path": "FromUtf8Error"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/error.rs:89`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7c1a58babe9f4a6668bf1a12"></a>
## from

`function` · `arrow_schema::error::ArrowError::from` · arrow-schema 59.3.0

```rust
fn from(error: std::io::IntoInnerError<W>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::error::ArrowError", "path": "ArrowError"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::io::write::Write", "path": "Write"}}}], "default": null, "is_synthetic": false}}, "name": "W"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [94, 1], "end": [98, 2], "filename": "src/error.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "W"}}], "constraints": []}}, "id": "alloc::io::buffered::IntoInnerError", "path": "IntoInnerError"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/error.rs:95`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c272e3f6126b2e71fd06f06e"></a>
## from

`function` · `arrow_schema::error::ArrowError::from` · arrow-schema 59.3.0

```rust
fn from(error: std::io::Error) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::error::ArrowError", "path": "ArrowError"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [76, 1], "end": [80, 2], "filename": "src/error.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "core::io::error::Error", "path": "Error"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/error.rs:77`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c51e8a26c849570f01b42e35"></a>
## from

`function` · `arrow_schema::error::ArrowError::from` · arrow-schema 59.3.0

```rust
fn from(error: std::str::Utf8Error) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::error::ArrowError", "path": "ArrowError"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [82, 1], "end": [86, 2], "filename": "src/error.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "core::str::error::Utf8Error", "path": "Utf8Error"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/error.rs:83`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fe670eec4f16747cbbae5fc2"></a>
## from_external_error

`function` · `arrow_schema::error::ArrowError::from_external_error` · arrow-schema 59.3.0

```rust
fn from_external_error(error: Box<dyn Error + Send + Sync>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::error::ArrowError", "path": "ArrowError"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [69, 1], "end": [74, 2], "filename": "src/error.rs"}, "trait": null, "trait_path": null}`

Source: `src/error.rs:71`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

Wraps an external error in an `ArrowError`.

<a id="op-dfb52a4d7642233ccb666cea"></a>
## source

`function` · `arrow_schema::error::ArrowError::source` · arrow-schema 59.3.0

```rust
fn source(&self) -> Option<&dyn Error + 'static>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::error::ArrowError", "path": "ArrowError"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [141, 1], "end": [149, 2], "filename": "src/error.rs"}, "trait": {"args": null, "id": "core::error::Error", "path": "Error"}, "trait_path": "core::error::Error"}`

Source: `src/error.rs:142`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.
