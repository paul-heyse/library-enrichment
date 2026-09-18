# `datafusion_physical_plan::memory::LazyBatchGenerator`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.memory.LazyBatchGenerator.json).

<a id="op-d98ea201c184317285bce65c"></a>
## LazyBatchGenerator

`trait` · `datafusion_physical_plan::memory::LazyBatchGenerator` · datafusion-physical-plan 55.1.0

```rust
trait LazyBatchGenerator: Send + Sync + fmt::Debug + fmt::Display
```

Source: `src/memory.rs:148`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1310e2f224adaa8440642c4f"></a>
## as_any

`function` · `datafusion_physical_plan::memory::LazyBatchGenerator::as_any` · datafusion-physical-plan 55.1.0

```rust
fn as_any(&self) -> &dyn Any
```

Source: `src/memory.rs:151`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Returns the generator as [`Any`] so that it can be
downcast to a specific implementation.

Unresolved upstream links (retained, not inferred): ``Any``.

<a id="op-9237b622999cc51fc5a2f920"></a>
## boundedness

`function` · `datafusion_physical_plan::memory::LazyBatchGenerator::boundedness` · datafusion-physical-plan 55.1.0

```rust
fn boundedness(&self) -> Boundedness
```

Source: `src/memory.rs:153`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0a1fd0190d25a3e9f56d2bcf"></a>
## generate_next_batch

`function` · `datafusion_physical_plan::memory::LazyBatchGenerator::generate_next_batch` · datafusion-physical-plan 55.1.0

```rust
fn generate_next_batch(&mut self) -> Result<Option<RecordBatch>>
```

Source: `src/memory.rs:158`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Generate the next batch, return `None` when no more batches are available

<a id="op-8635e4126ecc14105169b25b"></a>
## reset_state

`function` · `datafusion_physical_plan::memory::LazyBatchGenerator::reset_state` · datafusion-physical-plan 55.1.0

```rust
fn reset_state(&self) -> Arc<RwLock<dyn LazyBatchGenerator>>
```

Source: `src/memory.rs:161`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Returns a new instance with the state reset.
