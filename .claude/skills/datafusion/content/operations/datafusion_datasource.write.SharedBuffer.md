# `datafusion_datasource::write::SharedBuffer`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_datasource.write.SharedBuffer.json).

<a id="op-b15057b51f3b31b2a6120065"></a>
## SharedBuffer

`struct` · `datafusion_datasource::write::SharedBuffer` · datafusion-datasource 55.1.0

```rust
struct SharedBuffer
```

Source: `src/write/mod.rs:42`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

A buffer with interior mutability shared by the SerializedFileWriter and
ObjectStore writer

<a id="op-0ddb7ea13ddbf96cd0dec4e7"></a>
## buffer

`struct_field` · `datafusion_datasource::write::SharedBuffer::buffer` · datafusion-datasource 55.1.0

```rust
buffer: std::sync::Arc<futures::lock::Mutex<Vec<u8>>>
```

Source: `src/write/mod.rs:47`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

The inner buffer for reading and writing

The lock is used to obtain internal mutability, so no worry about the
lock contention.

<a id="op-504c5e5007ed01f43054d440"></a>
## clone

`function` · `datafusion_datasource::write::SharedBuffer::clone` · datafusion-datasource 55.1.0

```rust
fn clone(&self) -> SharedBuffer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::write::SharedBuffer", "path": "SharedBuffer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [41, 10], "end": [41, 15], "filename": "src/write/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/write/mod.rs:41`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-dea66263a0655d73021349ce"></a>
## flush

`function` · `datafusion_datasource::write::SharedBuffer::flush` · datafusion-datasource 55.1.0

```rust
fn flush(&mut self) -> std::io::Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::write::SharedBuffer", "path": "SharedBuffer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [58, 1], "end": [68, 2], "filename": "src/write/mod.rs"}, "trait": {"args": null, "id": "core::io::write::Write", "path": "Write"}, "trait_path": "core::io::write::Write"}`

Source: `src/write/mod.rs:64`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8959d462471962782150ecc1"></a>
## new

`function` · `datafusion_datasource::write::SharedBuffer::new` · datafusion-datasource 55.1.0

```rust
fn new(capacity: usize) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::write::SharedBuffer", "path": "SharedBuffer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [50, 1], "end": [56, 2], "filename": "src/write/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/write/mod.rs:51`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-37c0635f193fb01f14e78958"></a>
## write

`function` · `datafusion_datasource::write::SharedBuffer::write` · datafusion-datasource 55.1.0

```rust
fn write(&mut self, buf: &[u8]) -> std::io::Result<usize>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::write::SharedBuffer", "path": "SharedBuffer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [58, 1], "end": [68, 2], "filename": "src/write/mod.rs"}, "trait": {"args": null, "id": "core::io::write::Write", "path": "Write"}, "trait_path": "core::io::write::Write"}`

Source: `src/write/mod.rs:59`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
