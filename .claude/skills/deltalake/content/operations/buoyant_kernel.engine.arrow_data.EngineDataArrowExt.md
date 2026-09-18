# `buoyant_kernel::engine::arrow_data::EngineDataArrowExt`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.engine.arrow_data.EngineDataArrowExt.json).

<a id="op-540b6baa85a9e36ccdf5398f"></a>
## EngineDataArrowExt

`trait` · `buoyant_kernel::engine::arrow_data::EngineDataArrowExt` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
trait EngineDataArrowExt
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/engine/arrow_data.rs#L42).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/engine/arrow_data.rs:42`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

A trait to allow easy conversion from [`EngineData`](../operations/buoyant_kernel.engine_data.EngineData.md#op-f9036ab52623c01f96e1f809) to an arrow [``RecordBatch`]. Returns an
error if called on an `EngineData` that is not an `ArrowEngineData`.

<a id="op-f4f9adcf35e62323d99158ea"></a>
## try_into_record_batch

`function` · `buoyant_kernel::engine::arrow_data::EngineDataArrowExt::try_into_record_batch` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn try_into_record_batch(self) -> DeltaResult<RecordBatch>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/engine/arrow_data.rs#L43).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/engine/arrow_data.rs:43`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.
