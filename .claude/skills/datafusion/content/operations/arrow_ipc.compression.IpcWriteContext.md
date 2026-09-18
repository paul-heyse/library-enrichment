# `arrow_ipc::compression::IpcWriteContext`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_ipc.compression.IpcWriteContext.json).

<a id="op-3bb69fdf70757681bcbe49c0"></a>
## IpcWriteContext

`struct` · `arrow_ipc::compression::IpcWriteContext` · arrow-ipc 59.3.0

```rust
struct IpcWriteContext
```

Source: `src/compression.rs:33`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

Additional context that may be needed for compression.

In the case of zstd, this will contain the zstd context, which can be reused between subsequent
compression calls to avoid the performance overhead of initialising a new context for every
compression. Also holds a [`FlatBufferBuilder`] that is reused across IPC writes.

Unresolved upstream links (retained, not inferred): ``FlatBufferBuilder``.

<a id="op-941482fb934ea3dc1a987968"></a>
## default

`function` · `arrow_ipc::compression::IpcWriteContext::default` · arrow-ipc 59.3.0

```rust
fn default() -> IpcWriteContext
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_ipc::compression::IpcWriteContext", "path": "IpcWriteContext"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [32, 10], "end": [32, 17], "filename": "src/compression.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/compression.rs:32`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-235565621577d5806c9498dd"></a>
## fmt

`function` · `arrow_ipc::compression::IpcWriteContext::fmt` · arrow-ipc 59.3.0

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_ipc::compression::IpcWriteContext", "path": "IpcWriteContext"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [72, 1], "end": [84, 2], "filename": "src/compression.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/compression.rs:73`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2188aa54d79c427343a897cd"></a>
## set_reserve_scratch

`function` · `arrow_ipc::compression::IpcWriteContext::set_reserve_scratch` · arrow-ipc 59.3.0

```rust
fn set_reserve_scratch(&mut self, reserve: bool)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_ipc::compression::IpcWriteContext", "path": "IpcWriteContext"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [41, 1], "end": [70, 2], "filename": "src/compression.rs"}, "trait": null, "trait_path": null}`

Source: `src/compression.rs:50`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

Set whether the scratch buffer capacity should be reserved after each encode for reuse
on the next call. Set to `false` for the final batch in a sequence to avoid a
pointless allocation. by default, this is set to `false`.
