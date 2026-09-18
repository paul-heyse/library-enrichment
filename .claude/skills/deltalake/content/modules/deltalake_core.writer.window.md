# `deltalake_core::writer::window`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.writer.window.json).

<a id="op-0865c83fdd2b647d541c8d84"></a>
## window

`module` · `deltalake_core::writer::window` · deltalake-core 1.0.0+58f07cd6

Access: **internal_module**. Canonical source location is not automatically a valid import path.

```rust
mod window
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/writer/window.rs#L1).

Source: `crates/core/src/writer/window.rs:1`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

The write-window abstraction shared by the legacy [`RecordBatchWriter`] and
[`JsonWriter`].

[`WriteWindow`] is the single owner of the mutable state between flushes (open
sink, sealed rotations, current schema, batch count), so the writers' core
invariants hold by construction rather than by error-path discipline:

1. **A flush window commits all-or-nothing.** Any IO error aborts the whole
   window ([`WriteWindow::abort`]), so a later flush can't commit a partial
   subset of a failed write.
2. **Schema advances only with its data.** The committed `baseline` advances
   ([`WriteWindow::committed`]) only after the log commit succeeds, and an abort
   reverts to it — so a failed write can't evolve the table schema.

[`RecordBatchWriter`]: super::record_batch::RecordBatchWriter
[`JsonWriter`]: super::json::JsonWriter

Unresolved upstream links (retained, not inferred): ``WriteWindow::abort``, ``WriteWindow::committed``, ``WriteWindow``.
