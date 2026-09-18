# `buoyant_kernel::actions::visitors::visit_protocol_at`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.actions.visitors.visit_protocol_at.json).

<a id="op-86bc9c9f19fdeddf4f81689d"></a>
## visit_protocol_at

`function` · `buoyant_kernel::actions::visitors::visit_protocol_at` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn visit_protocol_at<'a>(row_index: usize, getters: &[&'a dyn GetData<'a>]) -> DeltaResult<Option<Protocol>>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/actions/visitors.rs#L600).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/actions/visitors.rs:600`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Get a Protocol out of some engine data. Note that Ok(None) is returned if there is no Protocol
found. The caller is responsible for slicing the `getters` slice such that the first element
contains the `min_reader_version` element of the protocol.
