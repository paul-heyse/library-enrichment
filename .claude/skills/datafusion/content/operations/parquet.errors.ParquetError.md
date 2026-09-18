# `parquet::errors::ParquetError`

Full upstream contracts; raw type trees and source locators in [structured records](parquet.errors.ParquetError.json).

<a id="op-3b1497ee1cedeb43fa24a7b9"></a>
## ParquetError

`enum` · `parquet::errors::ParquetError` · parquet 59.3.0

```rust
enum ParquetError
```

Source: `src/errors.rs:33`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Parquet error enumeration

<a id="op-31d04a5100dfe840c71a2d09"></a>
## ArrowError

`variant` · `parquet::errors::ParquetError::ArrowError` · parquet 59.3.0

```rust
ArrowError
```

Source: `src/errors.rs:47`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Arrow error.
Returned when reading into arrow or writing from arrow.

<a id="op-2830373c06954b892e473eff"></a>
## EOF

`variant` · `parquet::errors::ParquetError::EOF` · parquet 59.3.0

```rust
EOF
```

Source: `src/errors.rs:43`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

"End of file" Parquet error.
Returned when IO related failures occur, e.g. when there are not enough bytes to
decode.

<a id="op-9fd2ae1f5f4d398df0cb996d"></a>
## External

`variant` · `parquet::errors::ParquetError::External` · parquet 59.3.0

```rust
External
```

Source: `src/errors.rs:52`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

An external error variant

<a id="op-aba3bc6a5e60d075362ac1d8"></a>
## General

`variant` · `parquet::errors::ParquetError::General` · parquet 59.3.0

```rust
General
```

Source: `src/errors.rs:36`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

General Parquet error.
Returned when code violates normal workflow of working with Parquet files.

<a id="op-86643015ae0cfec1b148d03c"></a>
## IndexOutOfBound

`variant` · `parquet::errors::ParquetError::IndexOutOfBound` · parquet 59.3.0

```rust
IndexOutOfBound
```

Source: `src/errors.rs:50`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Error when the requested column index is more than the
number of columns in the row group

<a id="op-d6e126d65d6fffe7876d0f37"></a>
## NYI

`variant` · `parquet::errors::ParquetError::NYI` · parquet 59.3.0

```rust
NYI
```

Source: `src/errors.rs:39`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

"Not yet implemented" Parquet error.
Returned when functionality is not yet available.

<a id="op-4738b9ae6e77ab1806f3cac9"></a>
## NeedMoreData

`variant` · `parquet::errors::ParquetError::NeedMoreData` · parquet 59.3.0

```rust
NeedMoreData
```

Source: `src/errors.rs:55`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returned when a function needs more data to complete properly. The `usize` field indicates
the total number of bytes required, not the number of additional bytes.

<a id="op-55590510d4b86487ba0ba699"></a>
## NeedMoreDataRange

`variant` · `parquet::errors::ParquetError::NeedMoreDataRange` · parquet 59.3.0

```rust
NeedMoreDataRange
```

Source: `src/errors.rs:58`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returned when a function needs more data to complete properly.
The `Range<u64>` indicates the range of bytes that are needed.

<a id="op-2ad7e15b17f6c7eb013cc0f9"></a>
## fmt

`function` · `parquet::errors::ParquetError::fmt` · parquet 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::errors::ParquetError", "path": "ParquetError"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [31, 10], "end": [31, 15], "filename": "src/errors.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/errors.rs:31`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cd52593fec4badfd5c67eb44"></a>
## fmt

`function` · `parquet::errors::ParquetError::fmt` · parquet 59.3.0

```rust
fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::errors::ParquetError", "path": "ParquetError"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [61, 1], "end": [81, 2], "filename": "src/errors.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/errors.rs:62`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-383b28610c3d67a40738ee24"></a>
## from

`function` · `parquet::errors::ParquetError::from` · parquet 59.3.0

```rust
fn from(e: object_store::Error) -> ParquetError
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::errors::ParquetError", "path": "ParquetError"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [137, 1], "end": [141, 2], "filename": "src/errors.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "object_store::Error", "path": "Error"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/errors.rs:138`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7132eedfaeab58f02293642b"></a>
## from

`function` · `parquet::errors::ParquetError::from` · parquet 59.3.0

```rust
fn from(e: cell::BorrowMutError) -> ParquetError
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::errors::ParquetError", "path": "ParquetError"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [111, 1], "end": [115, 2], "filename": "src/errors.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "core::cell::BorrowMutError", "path": "BorrowMutError"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/errors.rs:112`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8f6c4682609595e1f14c4a5a"></a>
## from

`function` · `parquet::errors::ParquetError::from` · parquet 59.3.0

```rust
fn from(e: FromUtf8Error) -> ParquetError
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::errors::ParquetError", "path": "ParquetError"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [123, 1], "end": [127, 2], "filename": "src/errors.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "alloc::string::FromUtf8Error", "path": "FromUtf8Error"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/errors.rs:124`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-90d9cc7dddd6da050da59388"></a>
## from

`function` · `parquet::errors::ParquetError::from` · parquet 59.3.0

```rust
fn from(e: str::Utf8Error) -> ParquetError
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::errors::ParquetError", "path": "ParquetError"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [117, 1], "end": [121, 2], "filename": "src/errors.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "core::str::error::Utf8Error", "path": "Utf8Error"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/errors.rs:118`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a2822eba7a4820d1991c6c9b"></a>
## from

`function` · `parquet::errors::ParquetError::from` · parquet 59.3.0

```rust
fn from(e: ArrowError) -> ParquetError
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::errors::ParquetError", "path": "ParquetError"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [130, 1], "end": [134, 2], "filename": "src/errors.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "arrow_schema::error::ArrowError", "path": "ArrowError"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/errors.rs:131`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a9379cb6c1e665f285a1204c"></a>
## from

`function` · `parquet::errors::ParquetError::from` · parquet 59.3.0

```rust
fn from(e: ring::error::Unspecified) -> ParquetError
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::errors::ParquetError", "path": "ParquetError"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [144, 1], "end": [148, 2], "filename": "src/errors.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "ring::error::unspecified::Unspecified", "path": "Unspecified"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/errors.rs:145`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b9fb8ca956589124f60da16f"></a>
## from

`function` · `parquet::errors::ParquetError::from` · parquet 59.3.0

```rust
fn from(e: snap::Error) -> ParquetError
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::errors::ParquetError", "path": "ParquetError"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [105, 1], "end": [109, 2], "filename": "src/errors.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "snap::error::Error", "path": "Error"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/errors.rs:106`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c8e90083d2cadc17364a9301"></a>
## from

`function` · `parquet::errors::ParquetError::from` · parquet 59.3.0

```rust
fn from(e: TryFromIntError) -> ParquetError
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::errors::ParquetError", "path": "ParquetError"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [92, 1], "end": [96, 2], "filename": "src/errors.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "core::num::error::TryFromIntError", "path": "TryFromIntError"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/errors.rs:93`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ecc2cbca4fac241ef403ba92"></a>
## from

`function` · `parquet::errors::ParquetError::from` · parquet 59.3.0

```rust
fn from(e: io::Error) -> ParquetError
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::errors::ParquetError", "path": "ParquetError"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [98, 1], "end": [102, 2], "filename": "src/errors.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "core::io::error::Error", "path": "Error"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/errors.rs:99`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ef157da265c816349a0f1fa4"></a>
## source

`function` · `parquet::errors::ParquetError::source` · parquet 59.3.0

```rust
fn source(&self) -> Option<&dyn Error + 'static>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::errors::ParquetError", "path": "ParquetError"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [83, 1], "end": [90, 2], "filename": "src/errors.rs"}, "trait": {"args": null, "id": "core::error::Error", "path": "Error"}, "trait_path": "core::error::Error"}`

Source: `src/errors.rs:84`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.
