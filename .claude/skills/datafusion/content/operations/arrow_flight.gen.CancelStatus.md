# `arrow_flight::gen::CancelStatus`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_flight.gen.CancelStatus.json).

<a id="op-6439dc2a63894bfa19325874"></a>
## CancelStatus

`enum` · `arrow_flight::gen::CancelStatus` · arrow-flight 59.3.0

```rust
enum CancelStatus
```

Source: `src/arrow.flight.protocol.rs:379`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).


The result of a cancel operation.

This is used by CancelFlightInfoResult.status.

<a id="op-c55949ced1f0fc69feb77894"></a>
## Cancelled

`variant` · `arrow_flight::gen::CancelStatus::Cancelled` · arrow-flight 59.3.0

```rust
Cancelled
```

Source: `src/arrow.flight.protocol.rs:386`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

The cancellation request is complete. Subsequent requests with
the same payload may return CANCELLED or a NOT_FOUND error.

<a id="op-ab008dff3be82ca4b6f22696"></a>
## Cancelling

`variant` · `arrow_flight::gen::CancelStatus::Cancelling` · arrow-flight 59.3.0

```rust
Cancelling
```

Source: `src/arrow.flight.protocol.rs:389`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

The cancellation request is in progress. The client may retry
the cancellation request.

<a id="op-c04a153314eb44589c03302e"></a>
## Error

`assoc_type` · `arrow_flight::gen::CancelStatus::Error` · arrow-flight 59.3.0

```rust
Error
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::gen::CancelStatus", "path": "CancelStatus"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [377, 68], "end": [377, 88], "filename": "src/arrow.flight.protocol.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"primitive": "i32"}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/arrow.flight.protocol.rs:377`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cdfdb85a8067fb8d2c549c86"></a>
## NotCancellable

`variant` · `arrow_flight::gen::CancelStatus::NotCancellable` · arrow-flight 59.3.0

```rust
NotCancellable
```

Source: `src/arrow.flight.protocol.rs:392`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

The query is not cancellable. The client should not retry the
cancellation request.

<a id="op-57a62063ddd0b4bb34a0c504"></a>
## Unspecified

`variant` · `arrow_flight::gen::CancelStatus::Unspecified` · arrow-flight 59.3.0

```rust
Unspecified
```

Source: `src/arrow.flight.protocol.rs:383`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

The cancellation status is unknown. Servers should avoid using
this value (send a NOT_FOUND error if the requested query is
not known). Clients can retry the request.

<a id="op-b5f94136ff208e4a86e9ac89"></a>
## as_str_name

`function` · `arrow_flight::gen::CancelStatus::as_str_name` · arrow-flight 59.3.0

```rust
fn as_str_name(&self) -> &'static str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::gen::CancelStatus", "path": "CancelStatus"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [394, 1], "end": [417, 2], "filename": "src/arrow.flight.protocol.rs"}, "trait": null, "trait_path": null}`

Source: `src/arrow.flight.protocol.rs:399`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

String value of the enum field names used in the ProtoBuf definition.

The values are not transformed in any way and thus are considered stable
(if the ProtoBuf definition does not change) and safe for programmatic use.

<a id="op-83dbe1d4e09afc4c7be60c51"></a>
## clone

`function` · `arrow_flight::gen::CancelStatus::clone` · arrow-flight 59.3.0

```rust
fn clone(&self) -> CancelStatus
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::gen::CancelStatus", "path": "CancelStatus"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [377, 10], "end": [377, 15], "filename": "src/arrow.flight.protocol.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/arrow.flight.protocol.rs:377`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d4f6283d011698e0d66b948d"></a>
## cmp

`function` · `arrow_flight::gen::CancelStatus::cmp` · arrow-flight 59.3.0

```rust
fn cmp(&self, other: &CancelStatus) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::gen::CancelStatus", "path": "CancelStatus"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [377, 63], "end": [377, 66], "filename": "src/arrow.flight.protocol.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/arrow.flight.protocol.rs:377`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fe3c343d9264aaf729d93404"></a>
## default

`function` · `arrow_flight::gen::CancelStatus::default` · arrow-flight 59.3.0

```rust
fn default() -> CancelStatus
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::gen::CancelStatus", "path": "CancelStatus"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [377, 68], "end": [377, 88], "filename": "src/arrow.flight.protocol.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/arrow.flight.protocol.rs:377`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0e998dde98bf731806d7081f"></a>
## eq

`function` · `arrow_flight::gen::CancelStatus::eq` · arrow-flight 59.3.0

```rust
fn eq(&self, other: &CancelStatus) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::gen::CancelStatus", "path": "CancelStatus"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [377, 30], "end": [377, 39], "filename": "src/arrow.flight.protocol.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/arrow.flight.protocol.rs:377`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b949a746043507dc1c757125"></a>
## fmt

`function` · `arrow_flight::gen::CancelStatus::fmt` · arrow-flight 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::gen::CancelStatus", "path": "CancelStatus"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [377, 23], "end": [377, 28], "filename": "src/arrow.flight.protocol.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/arrow.flight.protocol.rs:377`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2c873598f4dffdc8a881ef92"></a>
## from_i32

`function` · `arrow_flight::gen::CancelStatus::from_i32` · arrow-flight 59.3.0

```rust
fn from_i32(value: i32) -> ::core::option::Option<CancelStatus>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::gen::CancelStatus", "path": "CancelStatus"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [377, 68], "end": [377, 88], "filename": "src/arrow.flight.protocol.rs"}, "trait": null, "trait_path": null}`

Source: `src/arrow.flight.protocol.rs:377`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Converts an `i32` to a `CancelStatus`, or `None` if `value` is not a valid variant.

<a id="op-0a1fad84ec0b029032b11baa"></a>
## from_str_name

`function` · `arrow_flight::gen::CancelStatus::from_str_name` · arrow-flight 59.3.0

```rust
fn from_str_name(value: &str) -> ::core::option::Option<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::gen::CancelStatus", "path": "CancelStatus"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [394, 1], "end": [417, 2], "filename": "src/arrow.flight.protocol.rs"}, "trait": null, "trait_path": null}`

Source: `src/arrow.flight.protocol.rs:408`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Creates an enum from field names used in the ProtoBuf definition.

<a id="op-95b62e50c949ba2615a11626"></a>
## hash

`function` · `arrow_flight::gen::CancelStatus::hash` · arrow-flight 59.3.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::gen::CancelStatus", "path": "CancelStatus"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [377, 45], "end": [377, 49], "filename": "src/arrow.flight.protocol.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/arrow.flight.protocol.rs:377`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8b06178b040e4739c99d0578"></a>
## is_valid

`function` · `arrow_flight::gen::CancelStatus::is_valid` · arrow-flight 59.3.0

```rust
fn is_valid(value: i32) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::gen::CancelStatus", "path": "CancelStatus"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [377, 68], "end": [377, 88], "filename": "src/arrow.flight.protocol.rs"}, "trait": null, "trait_path": null}`

Source: `src/arrow.flight.protocol.rs:377`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Returns `true` if `value` is a variant of `CancelStatus`.

<a id="op-01ed7e790abe9aa7d7995c67"></a>
## partial_cmp

`function` · `arrow_flight::gen::CancelStatus::partial_cmp` · arrow-flight 59.3.0

```rust
fn partial_cmp(&self, other: &CancelStatus) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::gen::CancelStatus", "path": "CancelStatus"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [377, 51], "end": [377, 61], "filename": "src/arrow.flight.protocol.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/arrow.flight.protocol.rs:377`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6845ae28a34e61754abbdb2e"></a>
## try_from

`function` · `arrow_flight::gen::CancelStatus::try_from` · arrow-flight 59.3.0

```rust
fn try_from(value: i32) -> ::core::result::Result<CancelStatus, ::prost::UnknownEnumValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::gen::CancelStatus", "path": "CancelStatus"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [377, 68], "end": [377, 88], "filename": "src/arrow.flight.protocol.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"primitive": "i32"}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/arrow.flight.protocol.rs:377`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.
