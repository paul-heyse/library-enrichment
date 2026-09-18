# `buoyant_kernel::engine::arrow_utils::to_json_bytes`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.engine.arrow_utils.to_json_bytes.json).

<a id="op-207a5f83411374afbb17a766"></a>
## to_json_bytes

`function` · `buoyant_kernel::engine::arrow_utils::to_json_bytes` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn to_json_bytes(data: impl Iterator<Item = DeltaResult<engine_data::FilteredEngineData>> + Send) -> DeltaResult<Vec<u8>>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/engine/arrow_utils/mod.rs#L1409).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/engine/arrow_utils/mod.rs:1409`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

serialize an arrow RecordBatch to a JSON string by appending to a buffer.
