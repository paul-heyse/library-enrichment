# `arrow_flight::IpcMessage`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_flight.IpcMessage.json).

<a id="op-3a71cafefafa5417c1426128"></a>
## IpcMessage

`struct` · `arrow_flight::IpcMessage` · arrow-flight 59.3.0

```rust
struct IpcMessage
```

Source: `src/lib.rs:146`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

IpcMessage represents a `Schema` in the format expected in
`FlightInfo.schema`

<a id="op-0530370665675336314a27dd"></a>
## 0

`struct_field` · `arrow_flight::IpcMessage::0` · arrow-flight 59.3.0

```rust
0: bytes::Bytes
```

Source: `src/lib.rs:146`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1af10a222616adfb0b7abf89"></a>
## Error

`assoc_type` · `arrow_flight::IpcMessage::Error` · arrow-flight 59.3.0

```rust
Error
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::IpcMessage", "path": "IpcMessage"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [400, 1], "end": [406, 2], "filename": "src/lib.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}], "constraints": []}}, "id": "arrow_flight::SchemaAsIpc", "path": "SchemaAsIpc"}}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/lib.rs:401`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e7cf2615ec33c32d1e304de2"></a>
## Target

`assoc_type` · `arrow_flight::IpcMessage::Target` · arrow-flight 59.3.0

```rust
Target
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::IpcMessage", "path": "IpcMessage"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [166, 1], "end": [172, 2], "filename": "src/lib.rs"}, "trait": {"args": null, "id": "core::ops::deref::Deref", "path": "Deref"}, "trait_path": "core::ops::deref::Deref"}`

Source: `src/lib.rs:167`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a9c442af4b3012409740ec19"></a>
## deref

`function` · `arrow_flight::IpcMessage::deref` · arrow-flight 59.3.0

```rust
fn deref(&self) -> &Self::Target
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::IpcMessage", "path": "IpcMessage"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [166, 1], "end": [172, 2], "filename": "src/lib.rs"}, "trait": {"args": null, "id": "core::ops::deref::Deref", "path": "Deref"}, "trait_path": "core::ops::deref::Deref"}`

Source: `src/lib.rs:169`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-188a8446bf375093f3845516"></a>
## fmt

`function` · `arrow_flight::IpcMessage::fmt` · arrow-flight 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::IpcMessage", "path": "IpcMessage"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [145, 10], "end": [145, 15], "filename": "src/lib.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/lib.rs:145`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2fd2bcb5f119f6edc4dab823"></a>
## try_from

`function` · `arrow_flight::IpcMessage::try_from` · arrow-flight 59.3.0

```rust
fn try_from(schema_ipc: SchemaAsIpc<'_>) -> std::result::Result<Self, arrow_schema::ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::IpcMessage", "path": "IpcMessage"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [400, 1], "end": [406, 2], "filename": "src/lib.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}], "constraints": []}}, "id": "arrow_flight::SchemaAsIpc", "path": "SchemaAsIpc"}}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/lib.rs:403`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.
