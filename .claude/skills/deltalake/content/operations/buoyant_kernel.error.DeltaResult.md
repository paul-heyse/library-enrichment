# `buoyant_kernel::error::DeltaResult`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.error.DeltaResult.json).

<a id="op-3db788f17aa90cfeefaa890f"></a>
## DeltaResult

`type_alias` · `buoyant_kernel::error::DeltaResult` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
type DeltaResult<T, E = Error> = std::result::Result<T, E>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/error.rs#L17).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/error.rs:17`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

A [`std::result::Result`] that has the kernel [`Error`](../operations/buoyant_kernel.error.Error.md#op-a6fe8c009eabf996f29109bf) as the error variant

Unresolved upstream links (retained, not inferred): ``std::result::Result``.

<a id="op-f97a4921f6595e3b534e21d0"></a>
## try_into_record_batch

`function` · `buoyant_kernel::error::DeltaResult::try_into_record_batch` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn try_into_record_batch(self) -> DeltaResult<RecordBatch>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/engine/arrow_data.rs#L57).

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"dyn_trait": {"lifetime": null, "traits": [{"generic_params": [], "trait": {"args": null, "id": "buoyant_kernel::engine_data::EngineData", "path": "EngineData"}}]}}}], "constraints": []}}, "id": "alloc::boxed::Box", "path": "Box"}}}], "constraints": []}}, "id": "buoyant_kernel::error::DeltaResult", "path": "crate::DeltaResult"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [56, 1], "end": [64, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/engine/arrow_data.rs"}, "trait": {"args": null, "id": "buoyant_kernel::engine::arrow_data::EngineDataArrowExt", "path": "EngineDataArrowExt"}, "trait_path": "buoyant_kernel::engine::arrow_data::EngineDataArrowExt"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/engine/arrow_data.rs:57`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.
