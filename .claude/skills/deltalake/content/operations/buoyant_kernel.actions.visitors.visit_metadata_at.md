# `buoyant_kernel::actions::visitors::visit_metadata_at`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.actions.visitors.visit_metadata_at.json).

<a id="op-3518de553a77817688e6cf91"></a>
## visit_metadata_at

`function` · `buoyant_kernel::actions::visitors::visit_metadata_at` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn visit_metadata_at<'a>(row_index: usize, getters: &[&'a dyn GetData<'a>]) -> DeltaResult<Option<Metadata>>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/actions/visitors.rs#L552).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/actions/visitors.rs:552`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Get a Metadata out of some engine data. Note that Ok(None) is returned if there is no Metadata
found. The caller is responsible for slicing the `getters` slice such that the first element
contains the `id` element of the metadata.
