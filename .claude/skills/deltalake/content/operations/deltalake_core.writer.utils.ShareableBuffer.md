# `deltalake_core::writer::utils::ShareableBuffer`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.writer.utils.ShareableBuffer.json).

<a id="op-cb6e6ceeab2a4e947b4df80a"></a>
## ShareableBuffer

`struct` · `deltalake_core::writer::utils::ShareableBuffer` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct ShareableBuffer
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/writer/utils.rs#L107).

Source: `crates/core/src/writer/utils.rs:107`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

An in memory buffer that allows for shared ownership and interior mutability.
The underlying buffer is wrapped in an `Arc` and `RwLock`, so cloning the instance
allows multiple owners to have access to the same underlying buffer.

<a id="op-bfdb12d1e992c9c3853c330a"></a>
## clone

`function` · `deltalake_core::writer::utils::ShareableBuffer::clone` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn clone(&self) -> ShareableBuffer
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/writer/utils.rs#L106).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::writer::utils::ShareableBuffer", "path": "ShareableBuffer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [106, 26], "end": [106, 31], "filename": "crates/core/src/writer/utils.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `crates/core/src/writer/utils.rs:106`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cde601b8a05870ae4443c4a8"></a>
## default

`function` · `deltalake_core::writer::utils::ShareableBuffer::default` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn default() -> ShareableBuffer
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/writer/utils.rs#L106).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::writer::utils::ShareableBuffer", "path": "ShareableBuffer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [106, 17], "end": [106, 24], "filename": "crates/core/src/writer/utils.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `crates/core/src/writer/utils.rs:106`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-aa1bacd545b6405771e4acc0"></a>
## flush

`function` · `deltalake_core::writer::utils::ShareableBuffer::flush` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn flush(&mut self) -> std::io::Result<()>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/writer/utils.rs#L152).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::writer::utils::ShareableBuffer", "path": "ShareableBuffer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [146, 1], "end": [156, 2], "filename": "crates/core/src/writer/utils.rs"}, "trait": {"args": null, "id": "core::io::write::Write", "path": "Write"}, "trait_path": "core::io::write::Write"}`

Source: `crates/core/src/writer/utils.rs:152`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ea853acb9f4e806dd116293f"></a>
## fmt

`function` · `deltalake_core::writer::utils::ShareableBuffer::fmt` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/writer/utils.rs#L106).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::writer::utils::ShareableBuffer", "path": "ShareableBuffer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [106, 10], "end": [106, 15], "filename": "crates/core/src/writer/utils.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `crates/core/src/writer/utils.rs:106`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fca880fb5757c2b90a74b6a9"></a>
## from_bytes

`function` · `deltalake_core::writer::utils::ShareableBuffer::from_bytes` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn from_bytes(bytes: &[u8]) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/writer/utils.rs#L139).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::writer::utils::ShareableBuffer", "path": "ShareableBuffer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [111, 1], "end": [144, 2], "filename": "crates/core/src/writer/utils.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/writer/utils.rs:139`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Creates a new instance with buffer initialized from the underylying bytes.

<a id="op-adecb11a394a84068bfa9ddb"></a>
## into_inner

`function` · `deltalake_core::writer::utils::ShareableBuffer::into_inner` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn into_inner(self) -> Option<Vec<u8>>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/writer/utils.rs#L114).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::writer::utils::ShareableBuffer", "path": "ShareableBuffer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [111, 1], "end": [144, 2], "filename": "crates/core/src/writer/utils.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/writer/utils.rs:114`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Consumes this instance and returns the underlying buffer.
Returns None if there are other references to the instance.

<a id="op-016dce872e42862d6bed5967"></a>
## is_empty

`function` · `deltalake_core::writer::utils::ShareableBuffer::is_empty` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn is_empty(&self) -> bool
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/writer/utils.rs#L133).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::writer::utils::ShareableBuffer", "path": "ShareableBuffer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [111, 1], "end": [144, 2], "filename": "crates/core/src/writer/utils.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/writer/utils.rs:133`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Returns true if the underlying buffer is empty.

<a id="op-43a081bab299c979cf5b1cca"></a>
## len

`function` · `deltalake_core::writer::utils::ShareableBuffer::len` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn len(&self) -> usize
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/writer/utils.rs#L127).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::writer::utils::ShareableBuffer", "path": "ShareableBuffer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [111, 1], "end": [144, 2], "filename": "crates/core/src/writer/utils.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/writer/utils.rs:127`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Returns the number of bytes in the underlying buffer.

<a id="op-bf78ac67085a9e4ad98db563"></a>
## to_vec

`function` · `deltalake_core::writer::utils::ShareableBuffer::to_vec` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn to_vec(&self) -> Vec<u8>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/writer/utils.rs#L121).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::writer::utils::ShareableBuffer", "path": "ShareableBuffer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [111, 1], "end": [144, 2], "filename": "crates/core/src/writer/utils.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/writer/utils.rs:121`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Returns a clone of the underlying buffer as a `Vec`.

<a id="op-fb8e42e6ae98a176f796a265"></a>
## write

`function` · `deltalake_core::writer::utils::ShareableBuffer::write` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn write(&mut self, buf: &[u8]) -> std::io::Result<usize>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/writer/utils.rs#L147).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::writer::utils::ShareableBuffer", "path": "ShareableBuffer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [146, 1], "end": [156, 2], "filename": "crates/core/src/writer/utils.rs"}, "trait": {"args": null, "id": "core::io::write::Write", "path": "Write"}, "trait_path": "core::io::write::Write"}`

Source: `crates/core/src/writer/utils.rs:147`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1cff9b529911665ca58dce26"></a>
## buffer

`struct_field` · `deltalake_core::writer::utils::ShareableBuffer::buffer` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
buffer: std::sync::Arc<parking_lot::RwLock<Vec<u8>>>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/writer/utils.rs#L108).

Source: `crates/core/src/writer/utils.rs:108`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.
