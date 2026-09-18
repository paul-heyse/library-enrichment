# `datafusion_physical_plan::spill::spill_pool::SpillPoolWriter`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.spill.spill_pool.SpillPoolWriter.json).

<a id="op-e6eb74ac5d1a8f5d6893d42a"></a>
## SpillPoolWriter

`struct` · `datafusion_physical_plan::spill::spill_pool::SpillPoolWriter` · datafusion-physical-plan 55.1.0

```rust
struct SpillPoolWriter
```

Source: `src/spill/spill_pool.rs:98`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Writer for a spill pool that can be cloned to produce additional writers.

Created by [`mpsc_channel`](../operations/datafusion_physical_plan.spill.spill_pool.mpsc_channel.md#op-03f7c46cd235c8bd43a84cfe). See that function for architecture diagrams and usage
examples.

<a id="op-6dfa07fa5410e6e4debe6f75"></a>
## clone

`function` · `datafusion_physical_plan::spill::spill_pool::SpillPoolWriter::clone` · datafusion-physical-plan 55.1.0

```rust
fn clone(&self) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::spill::spill_pool::SpillPoolWriter", "path": "SpillPoolWriter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [134, 1], "end": [140, 2], "filename": "src/spill/spill_pool.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/spill/spill_pool.rs:135`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b6a3696516d3c5df0dc5e42f"></a>
## new_sink

`function` · `datafusion_physical_plan::spill::spill_pool::SpillPoolWriter::new_sink` · datafusion-physical-plan 55.1.0

```rust
fn new_sink(&self) -> SpillPoolSink
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::spill::spill_pool::SpillPoolWriter", "path": "SpillPoolWriter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [117, 1], "end": [132, 2], "filename": "src/spill/spill_pool.rs"}, "trait": null, "trait_path": null}`

Source: `src/spill/spill_pool.rs:123`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Returns a new sink that can be used to spill batches to the pool.

As an alternative to this function, it is also possible to clone the writer. The benefit
of this method is that the output type matches the type used by [`spsc_channel`](../operations/datafusion_physical_plan.spill.spill_pool.spsc_channel.md#op-334be7f1fdba1ab78f028f38). This
enables cost-free abstraction for producers over SPSC and MPSC channels.

<a id="op-edcfd740844bfe1bbe9927cb"></a>
## push_batch

`function` · `datafusion_physical_plan::spill::spill_pool::SpillPoolWriter::push_batch` · datafusion-physical-plan 55.1.0

```rust
fn push_batch(&self, batch: &RecordBatch) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::spill::spill_pool::SpillPoolWriter", "path": "SpillPoolWriter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [104, 1], "end": [115, 2], "filename": "src/spill/spill_pool.rs"}, "trait": null, "trait_path": null}`

Source: `src/spill/spill_pool.rs:112`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Spills a batch to the pool, rotating files when necessary.

See [`mpsc_channel`](../operations/datafusion_physical_plan.spill.spill_pool.mpsc_channel.md#op-03f7c46cd235c8bd43a84cfe) for the rotation semantics.

# Errors

Returns an error if disk I/O fails or disk quota is exceeded.
