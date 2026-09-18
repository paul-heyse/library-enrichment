# `buoyant_kernel::log_replay::deduplicator`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.log_replay.deduplicator.json).

<a id="op-ca1880a9eb9efe28a2b07013"></a>
## deduplicator

`module` · `buoyant_kernel::log_replay::deduplicator` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_module**. Canonical source location is not automatically a valid import path.

```rust
mod deduplicator
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/log_replay/deduplicator.rs#L1).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/log_replay/deduplicator.rs:1`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Deduplication abstraction for log replay processors.

The [`Deduplicator`] trait supports two deduplication strategies:

- **JSON commit files** (`is_log_batch = true`): Tracks (path, dv_unique_id) and updates the
  hashmap as files are seen. Implementation: [`FileActionDeduplicator`]

- **Checkpoint files** (`is_log_batch = false`): Uses (path, dv_unique_id) to filter actions
  using a read-only hashmap pre-populated from the commit log phase. Future implementation.

[`FileActionDeduplicator`]: crate::log_replay::FileActionDeduplicator

Unresolved upstream links (retained, not inferred): `crate::log_replay::FileActionDeduplicator`, ``Deduplicator``.
