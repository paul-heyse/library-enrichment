# `arrow_avro::errors::AvroError`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_avro.errors.AvroError.json).

<a id="op-e8f91afb12856102fcf8416d"></a>
## AvroError

`enum` · `arrow_avro::errors::AvroError` · arrow-avro 59.3.0

```rust
enum AvroError
```

Source: `src/errors.rs:30`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

Avro error enumeration

<a id="op-ea7370af84898a5d9ad85710"></a>
## ArrowError

`variant` · `arrow_avro::errors::AvroError::ArrowError` · arrow-avro 59.3.0

```rust
ArrowError
```

Source: `src/errors.rs:43`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

Arrow error.
Returned when reading into arrow or writing from arrow.

<a id="op-4d81ab67e06add2bb9675603"></a>
## EOF

`variant` · `arrow_avro::errors::AvroError::EOF` · arrow-avro 59.3.0

```rust
EOF
```

Source: `src/errors.rs:40`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

"End of file" Avro error.
Returned when IO related failures occur, e.g. when there are not enough bytes to
decode.

<a id="op-56e69ec4ee0438fcca1bcb60"></a>
## External

`variant` · `arrow_avro::errors::AvroError::External` · arrow-avro 59.3.0

```rust
External
```

Source: `src/errors.rs:54`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

An external error variant

<a id="op-4e3f6ae909edbd6af3a09d00"></a>
## General

`variant` · `arrow_avro::errors::AvroError::General` · arrow-avro 59.3.0

```rust
General
```

Source: `src/errors.rs:33`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

General Avro error.
Returned when code violates normal workflow of working with Avro data.

<a id="op-2ebb143a7f5b4f8394a760e8"></a>
## IndexOutOfBound

`variant` · `arrow_avro::errors::AvroError::IndexOutOfBound` · arrow-avro 59.3.0

```rust
IndexOutOfBound
```

Source: `src/errors.rs:46`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

Error when the requested index is more than the
number of items expected

<a id="op-93da658b1b22c1f4ee7957f5"></a>
## InvalidArgument

`variant` · `arrow_avro::errors::AvroError::InvalidArgument` · arrow-avro 59.3.0

```rust
InvalidArgument
```

Source: `src/errors.rs:48`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

Error indicating that an unexpected or bad argument was passed to a function.

<a id="op-b26212fcc0633c4468905400"></a>
## IoError

`variant` · `arrow_avro::errors::AvroError::IoError` · arrow-avro 59.3.0

```rust
IoError
```

Source: `src/errors.rs:56`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

Error during IO operations

<a id="op-afc5caa1b6ccaa8bddce15da"></a>
## NYI

`variant` · `arrow_avro::errors::AvroError::NYI` · arrow-avro 59.3.0

```rust
NYI
```

Source: `src/errors.rs:36`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

"Not yet implemented" Avro error.
Returned when functionality is not yet available.

<a id="op-542157239592d0597ca097c8"></a>
## NeedMoreData

`variant` · `arrow_avro::errors::AvroError::NeedMoreData` · arrow-avro 59.3.0

```rust
NeedMoreData
```

Source: `src/errors.rs:59`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

Returned when a function needs more data to complete properly. The `usize` field indicates
the total number of bytes required, not the number of additional bytes.

<a id="op-3fb600835e1322649402bfa5"></a>
## NeedMoreDataRange

`variant` · `arrow_avro::errors::AvroError::NeedMoreDataRange` · arrow-avro 59.3.0

```rust
NeedMoreDataRange
```

Source: `src/errors.rs:62`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

Returned when a function needs more data to complete properly.
The `Range<u64>` indicates the range of bytes that are needed.

<a id="op-eed7596a5d8e401215ad4480"></a>
## ParseError

`variant` · `arrow_avro::errors::AvroError::ParseError` · arrow-avro 59.3.0

```rust
ParseError
```

Source: `src/errors.rs:50`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

Error indicating that a value could not be parsed.

<a id="op-f47de4445b56ae30f2de8413"></a>
## SchemaError

`variant` · `arrow_avro::errors::AvroError::SchemaError` · arrow-avro 59.3.0

```rust
SchemaError
```

Source: `src/errors.rs:52`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

Error indicating that a schema is invalid.

<a id="op-4524dcd2c65eda4d2ec39c8f"></a>
## fmt

`function` · `arrow_avro::errors::AvroError::fmt` · arrow-avro 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_avro::errors::AvroError", "path": "AvroError"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [28, 10], "end": [28, 15], "filename": "src/errors.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/errors.rs:28`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d52e41559e38c38e39bf01c6"></a>
## fmt

`function` · `arrow_avro::errors::AvroError::fmt` · arrow-avro 59.3.0

```rust
fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_avro::errors::AvroError", "path": "AvroError"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [65, 1], "end": [90, 2], "filename": "src/errors.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/errors.rs:66`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-07d1d5cdeace97bf504c66f4"></a>
## from

`function` · `arrow_avro::errors::AvroError::from` · arrow-avro 59.3.0

```rust
fn from(e: TryFromIntError) -> AvroError
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_avro::errors::AvroError", "path": "AvroError"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [103, 1], "end": [107, 2], "filename": "src/errors.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "core::num::error::TryFromIntError", "path": "TryFromIntError"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/errors.rs:104`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1fa76ec09d0bfe7923c09c2e"></a>
## from

`function` · `arrow_avro::errors::AvroError::from` · arrow-avro 59.3.0

```rust
fn from(e: ArrowError) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_avro::errors::AvroError", "path": "AvroError"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [127, 1], "end": [131, 2], "filename": "src/errors.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "arrow_schema::error::ArrowError", "path": "ArrowError"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/errors.rs:128`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4af1636a6d31e36c80ddc1d9"></a>
## from

`function` · `arrow_avro::errors::AvroError::from` · arrow-avro 59.3.0

```rust
fn from(e: io::Error) -> AvroError
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_avro::errors::AvroError", "path": "AvroError"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [109, 1], "end": [113, 2], "filename": "src/errors.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "core::io::error::Error", "path": "Error"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/errors.rs:110`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8fcac380ed0897406b6f108c"></a>
## from

`function` · `arrow_avro::errors::AvroError::from` · arrow-avro 59.3.0

```rust
fn from(e: FromUtf8Error) -> AvroError
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_avro::errors::AvroError", "path": "AvroError"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [121, 1], "end": [125, 2], "filename": "src/errors.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "alloc::string::FromUtf8Error", "path": "FromUtf8Error"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/errors.rs:122`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fd151efa23535e53fa2b8f44"></a>
## from

`function` · `arrow_avro::errors::AvroError::from` · arrow-avro 59.3.0

```rust
fn from(e: str::Utf8Error) -> AvroError
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_avro::errors::AvroError", "path": "AvroError"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [115, 1], "end": [119, 2], "filename": "src/errors.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "core::str::error::Utf8Error", "path": "Utf8Error"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/errors.rs:116`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cfbf644723e4481e293fae69"></a>
## source

`function` · `arrow_avro::errors::AvroError::source` · arrow-avro 59.3.0

```rust
fn source(&self) -> Option<&dyn Error + 'static>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_avro::errors::AvroError", "path": "AvroError"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [92, 1], "end": [101, 2], "filename": "src/errors.rs"}, "trait": {"args": null, "id": "core::error::Error", "path": "Error"}, "trait_path": "core::error::Error"}`

Source: `src/errors.rs:93`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.
