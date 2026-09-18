# `arrow_flight::error::FlightError`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_flight.error.FlightError.json).

<a id="op-6b7a963265f48d7ac8e47cdf"></a>
## FlightError

`enum` · `arrow_flight::error::FlightError` · arrow-flight 59.3.0

```rust
enum FlightError
```

Source: `src/error.rs:24`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Errors for the Apache Arrow Flight crate

<a id="op-b0271bfba1ce86e87a2d4a91"></a>
## Arrow

`variant` · `arrow_flight::error::FlightError::Arrow` · arrow-flight 59.3.0

```rust
Arrow
```

Source: `src/error.rs:26`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Underlying arrow error

<a id="op-51fe384692d1615599d87637"></a>
## DecodeError

`variant` · `arrow_flight::error::FlightError::DecodeError` · arrow-flight 59.3.0

```rust
DecodeError
```

Source: `src/error.rs:34`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

An error occurred during decoding

<a id="op-74f3984f65fe32c70cbcbd4f"></a>
## ExternalError

`variant` · `arrow_flight::error::FlightError::ExternalError` · arrow-flight 59.3.0

```rust
ExternalError
```

Source: `src/error.rs:36`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

External error that can provide source of error by calling `Error::source`.

<a id="op-3c8d85dee5bd6463a73fed33"></a>
## NotYetImplemented

`variant` · `arrow_flight::error::FlightError::NotYetImplemented` · arrow-flight 59.3.0

```rust
NotYetImplemented
```

Source: `src/error.rs:28`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Returned when functionality is not yet available.

<a id="op-2d4fb1b7c2a152b626f02d12"></a>
## ProtocolError

`variant` · `arrow_flight::error::FlightError::ProtocolError` · arrow-flight 59.3.0

```rust
ProtocolError
```

Source: `src/error.rs:32`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Some unexpected message was received

<a id="op-9174d5e36a58f3dad45ec44b"></a>
## Tonic

`variant` · `arrow_flight::error::FlightError::Tonic` · arrow-flight 59.3.0

```rust
Tonic
```

Source: `src/error.rs:30`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Error from the underlying tonic library

<a id="op-4be7bbbf4fbbe7ea363404a0"></a>
## fmt

`function` · `arrow_flight::error::FlightError::fmt` · arrow-flight 59.3.0

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::error::FlightError", "path": "FlightError"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [51, 1], "end": [62, 2], "filename": "src/error.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/error.rs:52`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c08a7b8ce14e8527a76d82b1"></a>
## fmt

`function` · `arrow_flight::error::FlightError::fmt` · arrow-flight 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::error::FlightError", "path": "FlightError"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [23, 10], "end": [23, 15], "filename": "src/error.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/error.rs:23`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0a4d220cee812edd8d12582f"></a>
## from

`function` · `arrow_flight::error::FlightError::from` · arrow-flight 59.3.0

```rust
fn from(status: tonic::Status) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::error::FlightError", "path": "FlightError"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [75, 1], "end": [79, 2], "filename": "src/error.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "tonic::status::Status", "path": "Status"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/error.rs:76`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-363f6a7ba0fab3923406da59"></a>
## from

`function` · `arrow_flight::error::FlightError::from` · arrow-flight 59.3.0

```rust
fn from(value: ArrowError) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::error::FlightError", "path": "FlightError"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [87, 1], "end": [91, 2], "filename": "src/error.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "arrow_schema::error::ArrowError", "path": "ArrowError"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/error.rs:88`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7489d34cc71e215b2a1231e6"></a>
## from

`function` · `arrow_flight::error::FlightError::from` · arrow-flight 59.3.0

```rust
fn from(error: prost::DecodeError) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::error::FlightError", "path": "FlightError"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [81, 1], "end": [85, 2], "filename": "src/error.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "prost::error::DecodeError", "path": "DecodeError"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/error.rs:82`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9944e0ebb389604dd9009d77"></a>
## from_external_error

`function` · `arrow_flight::error::FlightError::from_external_error` · arrow-flight 59.3.0

```rust
fn from_external_error(error: Box<dyn Error + Send + Sync>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::error::FlightError", "path": "FlightError"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [39, 1], "end": [49, 2], "filename": "src/error.rs"}, "trait": null, "trait_path": null}`

Source: `src/error.rs:46`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Wraps an external error in an `ArrowError`.

<a id="op-78e8c83ad271435b1a78f86b"></a>
## protocol

`function` · `arrow_flight::error::FlightError::protocol` · arrow-flight 59.3.0

```rust
fn protocol(message: impl Into<String>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::error::FlightError", "path": "FlightError"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [39, 1], "end": [49, 2], "filename": "src/error.rs"}, "trait": null, "trait_path": null}`

Source: `src/error.rs:41`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Generate a new `FlightError::ProtocolError` variant.

<a id="op-117f80f09ccee56f12b1efc7"></a>
## source

`function` · `arrow_flight::error::FlightError::source` · arrow-flight 59.3.0

```rust
fn source(&self) -> Option<&dyn Error + 'static>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::error::FlightError", "path": "FlightError"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [64, 1], "end": [73, 2], "filename": "src/error.rs"}, "trait": {"args": null, "id": "core::error::Error", "path": "Error"}, "trait_path": "core::error::Error"}`

Source: `src/error.rs:65`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.
