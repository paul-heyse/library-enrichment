# `datafusion_common::utils::memory::RecordBatchMemoryCounter`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.utils.memory.RecordBatchMemoryCounter.json).

<a id="op-b28d63549f04a619d24f155c"></a>
## RecordBatchMemoryCounter

`struct` · `datafusion_common::utils::memory::RecordBatchMemoryCounter` · datafusion-common 55.1.0

```rust
struct RecordBatchMemoryCounter
```

Source: `src/utils/memory.rs:151`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Tracks the memory used by a sequence of [`RecordBatch`](../operations/arrow_array.record_batch.RecordBatch.md#op-87f977a95cb312da9259aa34)es that may share
underlying buffers, counting each buffer exactly once.

Use this instead of [`get_record_batch_memory_size`](../operations/datafusion_common.utils.memory.get_record_batch_memory_size.md#op-96c226571bfad24b0ab66e2c) to account for the
total memory of a sequence of batches, e.g. when buffering the batches of
an input stream. Such batches can share buffers (for example, operators
like aggregates emit one large batch as multiple zero-copy slices), and
calling [`get_record_batch_memory_size`](../operations/datafusion_common.utils.memory.get_record_batch_memory_size.md#op-96c226571bfad24b0ab66e2c) per batch counts the shared
buffers once per batch, while this counter counts them exactly once. A
batch's buffers are kept alive by the batch even when only a sub-range is
referenced, so counting unique buffers in full reflects the memory the
batches actually retain.

<a id="op-25bcf8e416c5d8b2393915f6"></a>
## count_batch

`function` · `datafusion_common::utils::memory::RecordBatchMemoryCounter::count_batch` · datafusion-common 55.1.0

```rust
fn count_batch(&mut self, batch: &RecordBatch) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::utils::memory::RecordBatchMemoryCounter", "path": "RecordBatchMemoryCounter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [159, 1], "end": [186, 2], "filename": "src/utils/memory.rs"}, "trait": null, "trait_path": null}`

Source: `src/utils/memory.rs:166`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Count `batch`, returning the memory used by its buffers that have not
been counted before.

<a id="op-e34c9c6cca97e0b7642ac36f"></a>
## default

`function` · `datafusion_common::utils::memory::RecordBatchMemoryCounter::default` · datafusion-common 55.1.0

```rust
fn default() -> RecordBatchMemoryCounter
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::utils::memory::RecordBatchMemoryCounter", "path": "RecordBatchMemoryCounter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [150, 17], "end": [150, 24], "filename": "src/utils/memory.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/utils/memory.rs:150`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c6c67479ba0907867afd31b6"></a>
## fmt

`function` · `datafusion_common::utils::memory::RecordBatchMemoryCounter::fmt` · datafusion-common 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::utils::memory::RecordBatchMemoryCounter", "path": "RecordBatchMemoryCounter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [150, 10], "end": [150, 15], "filename": "src/utils/memory.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/utils/memory.rs:150`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-dbf2f6967caef03d054f4c30"></a>
## memory_usage

`function` · `datafusion_common::utils::memory::RecordBatchMemoryCounter::memory_usage` · datafusion-common 55.1.0

```rust
fn memory_usage(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::utils::memory::RecordBatchMemoryCounter", "path": "RecordBatchMemoryCounter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [159, 1], "end": [186, 2], "filename": "src/utils/memory.rs"}, "trait": null, "trait_path": null}`

Source: `src/utils/memory.rs:183`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Total memory of the unique buffers of all batches counted so far.

<a id="op-99e718dd91fb423b27d555c9"></a>
## new

`function` · `datafusion_common::utils::memory::RecordBatchMemoryCounter::new` · datafusion-common 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::utils::memory::RecordBatchMemoryCounter", "path": "RecordBatchMemoryCounter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [159, 1], "end": [186, 2], "filename": "src/utils/memory.rs"}, "trait": null, "trait_path": null}`

Source: `src/utils/memory.rs:160`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
