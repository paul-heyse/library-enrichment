# `buoyant_kernel::scan::state_info`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.scan.state_info.json).

<a id="op-09db1f18d7b7d7b019999350"></a>
## state_info

`module` · `buoyant_kernel::scan::state_info` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_module**. Canonical source location is not automatically a valid import path.

```rust
mod state_info
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/scan/state_info.rs#L1).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/scan/state_info.rs:1`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

StateInfo handles the state that we use through log-replay in order to correctly construct all
the physical->logical transforms needed for each add file
