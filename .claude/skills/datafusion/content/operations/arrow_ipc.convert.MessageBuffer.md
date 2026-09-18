# `arrow_ipc::convert::MessageBuffer`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_ipc.convert.MessageBuffer.json).

<a id="op-7a5292c47445524589fead4b"></a>
## MessageBuffer

`struct` · `arrow_ipc::convert::MessageBuffer` · arrow-ipc 59.3.0

```rust
struct MessageBuffer
```

Source: `src/convert.rs:979`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

An owned container for a validated [`Message`](../operations/arrow_ipc.gen.Message.Message.md#op-17c445fa1e26f0fa22a08030)

Safely decoding a flatbuffer requires validating the various embedded offsets,
see [`Verifier`]. This is a potentially expensive operation, and it is therefore desirable
to only do this once. [`crate::root_as_message`](../operations/arrow_ipc.gen.Message.root_as_message.md#op-e9c11d0f9e47e1f1dd2350d7) performs this validation on construction,
however, it returns a [`Message`](../operations/arrow_ipc.gen.Message.Message.md#op-17c445fa1e26f0fa22a08030) borrowing the provided byte slice. This prevents
storing this [`Message`](../operations/arrow_ipc.gen.Message.Message.md#op-17c445fa1e26f0fa22a08030) in the same data structure that owns the buffer, as this
would require self-referential borrows.

[`MessageBuffer`](../operations/arrow_ipc.convert.MessageBuffer.md#op-7a5292c47445524589fead4b) solves this problem by providing a safe API for a [`Message`](../operations/arrow_ipc.gen.Message.Message.md#op-17c445fa1e26f0fa22a08030)
without a lifetime bound.

Unresolved upstream links (retained, not inferred): ``Verifier``.

<a id="op-621990a502062be839a96712"></a>
## as_ref

`function` · `arrow_ipc::convert::MessageBuffer::as_ref` · arrow-ipc 59.3.0

```rust
fn as_ref(&self) -> Message<'_>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_ipc::convert::MessageBuffer", "path": "MessageBuffer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [987, 1], "end": [1004, 2], "filename": "src/convert.rs"}, "trait": null, "trait_path": null}`

Source: `src/convert.rs:1000`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

Return the [`Message`](../operations/arrow_ipc.gen.Message.Message.md#op-17c445fa1e26f0fa22a08030)

<a id="op-4dbfa86766f1c0219189260a"></a>
## clone

`function` · `arrow_ipc::convert::MessageBuffer::clone` · arrow-ipc 59.3.0

```rust
fn clone(&self) -> MessageBuffer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_ipc::convert::MessageBuffer", "path": "MessageBuffer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [978, 10], "end": [978, 15], "filename": "src/convert.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/convert.rs:978`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-daab8bc56cf50acfa6086f90"></a>
## fmt

`function` · `arrow_ipc::convert::MessageBuffer::fmt` · arrow-ipc 59.3.0

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_ipc::convert::MessageBuffer", "path": "MessageBuffer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [981, 1], "end": [985, 2], "filename": "src/convert.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/convert.rs:982`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9b9c1e60db1587e676b7b918"></a>
## try_new

`function` · `arrow_ipc::convert::MessageBuffer::try_new` · arrow-ipc 59.3.0

```rust
fn try_new(buf: Buffer) -> Result<Self, ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_ipc::convert::MessageBuffer", "path": "MessageBuffer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [987, 1], "end": [1004, 2], "filename": "src/convert.rs"}, "trait": null, "trait_path": null}`

Source: `src/convert.rs:989`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

Try to create a [`MessageBuffer`](../operations/arrow_ipc.convert.MessageBuffer.md#op-7a5292c47445524589fead4b) from the provided [`Buffer`](../operations/arrow_buffer.buffer.immutable.Buffer.md#op-f54755e677e7b3529b3edb8b)
