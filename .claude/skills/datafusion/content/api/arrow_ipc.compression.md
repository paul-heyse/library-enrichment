# `arrow_ipc::compression`

Crate `arrow-ipc` · 2 public items · structured records in [`model/arrow_ipc.compression.json`](../model/arrow_ipc.compression.json)

## IpcWriteContext

`struct` · `arrow_ipc::compression::IpcWriteContext`

Also reachable as `arrow_ipc::writer::IpcWriteContext`

```rust
struct IpcWriteContext
```

**Derives**: Debug, Default

**Methods** (1)

```rust
fn set_reserve_scratch(&mut self, reserve: bool)
```

Additional context that may be needed for compression.

In the case of zstd, this will contain the zstd context, which can be reused between subsequent
compression calls to avoid the performance overhead of initialising a new context for every
compression. Also holds a [`FlatBufferBuilder`] that is reused across IPC writes.

---

## CompressionContext

`type_alias` · `arrow_ipc::compression::CompressionContext`

> **Deprecated** — since 59.1.0: Use IpcWriteContext instead

Also reachable as `arrow_ipc::writer::CompressionContext`

```rust
type CompressionContext = IpcWriteContext
```

Deprecated alias for [`IpcWriteContext`].

---
