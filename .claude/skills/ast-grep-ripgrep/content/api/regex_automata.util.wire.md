# `regex_automata::util::wire`

Crate `regex-automata` · 3 public items · structured records in [`model/regex_automata.util.wire.json`](../model/regex_automata.util.wire.json)

## AlignAs

`struct` · `regex_automata::util::wire::AlignAs`

```rust
struct AlignAs<B: ?Sized, T>
```

**Fields**: `_align`, `bytes`

**Derives**: Debug

A hack to align a smaller type `B` with a bigger type `T`.

The usual use of this is with `B = [u8]` and `T = u32`. That is,
it permits aligning a sequence of bytes on a 4-byte boundary. This
is useful in contexts where one wants to embed a serialized [dense
DFA](crate::dfa::dense::DFA) into a Rust a program while guaranteeing the
alignment required for the DFA.

See [`dense::DFA::from_bytes`](crate::dfa::dense::DFA::from_bytes) for an
example of how to use this type.

---

## DeserializeError

`struct` · `regex_automata::util::wire::DeserializeError`

```rust
struct DeserializeError
```

**Implements**: `core::error::Error`, `core::fmt::Display`

**Derives**: Debug

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
```

An error that occurs when deserializing an object defined in this crate.

Serialization, as used in this crate, universally refers to the process
of transforming a structure (like a DFA) into a custom binary format
represented by `&[u8]`. Deserialization, then, refers to the process of
cheaply converting this binary format back to the object's in-memory
representation as defined in this crate. To the extent possible,
deserialization will report this error whenever this process fails.

A `DeserializeError` provides no introspection capabilities. Its only
supported operation is conversion to a human readable error message.

This error type implements the `std::error::Error` trait only when the
`std` feature is enabled. Otherwise, this type is defined in all
configurations.

---

## SerializeError

`struct` · `regex_automata::util::wire::SerializeError`

```rust
struct SerializeError
```

**Implements**: `core::error::Error`, `core::fmt::Display`

**Derives**: Debug

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
```

An error that occurs when serializing an object from this crate.

Serialization, as used in this crate, universally refers to the process
of transforming a structure (like a DFA) into a custom binary format
represented by `&[u8]`. To this end, serialization is generally infallible.
However, it can fail when caller provided buffer sizes are too small. When
that occurs, a serialization error is reported.

A `SerializeError` provides no introspection capabilities. Its only
supported operation is conversion to a human readable error message.

This error type implements the `std::error::Error` trait only when the
`std` feature is enabled. Otherwise, this type is defined in all
configurations.

---
