# `datafusion_physical_plan::spill::spill_pool::SpillPoolSink`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.spill.spill_pool.SpillPoolSink.json).

<a id="op-4c81dc7e9ce14f175a89458a"></a>
## SpillPoolSink

`struct` · `datafusion_physical_plan::spill::spill_pool::SpillPoolSink` · datafusion-physical-plan 55.1.0

```rust
struct SpillPoolSink
```

Source: `src/spill/spill_pool.rs:188`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Single writer for a spill pool that cannot be cloned.

Created by [`spsc_channel`](../operations/datafusion_physical_plan.spill.spill_pool.spsc_channel.md#op-334be7f1fdba1ab78f028f38) and [`SpillPoolWriter::new_sink`](../operations/datafusion_physical_plan.spill.spill_pool.SpillPoolWriter.md#op-b6a3696516d3c5df0dc5e42f).

<a id="op-5d62faec422c87c72be6472b"></a>
## drop

`function` · `datafusion_physical_plan::spill::spill_pool::SpillPoolSink::drop` · datafusion-physical-plan 55.1.0

```rust
fn drop(&mut self)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::spill::spill_pool::SpillPoolSink", "path": "SpillPoolSink"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [142, 1], "end": [183, 2], "filename": "src/spill/spill_pool.rs"}, "trait": {"args": null, "id": "core::ops::drop::Drop", "path": "Drop"}, "trait_path": "core::ops::drop::Drop"}`

Source: `src/spill/spill_pool.rs:143`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d0f3dd2bf71d5ada348a1f87"></a>
## push_batch

`function` · `datafusion_physical_plan::spill::spill_pool::SpillPoolSink::push_batch` · datafusion-physical-plan 55.1.0

```rust
fn push_batch(&self, batch: &RecordBatch) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::spill::spill_pool::SpillPoolSink", "path": "SpillPoolSink"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [196, 1], "end": [288, 2], "filename": "src/spill/spill_pool.rs"}, "trait": null, "trait_path": null}`

Source: `src/spill/spill_pool.rs:204`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Spills a batch to the pool, rotating files when necessary.

See [`spsc_channel`](../operations/datafusion_physical_plan.spill.spill_pool.spsc_channel.md#op-334be7f1fdba1ab78f028f38) for overall architecture and examples.

# Errors

Returns an error if disk I/O fails or disk quota is exceeded.
