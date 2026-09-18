# `buoyant_kernel::scan::get_transform_for_row`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.scan.get_transform_for_row.json).

<a id="op-685a69a8146c5526c6cae330"></a>
## get_transform_for_row

`function` · `buoyant_kernel::scan::get_transform_for_row` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn get_transform_for_row(row: usize, transforms: &[Option<expressions::ExpressionRef>]) -> Option<expressions::ExpressionRef>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/scan/mod.rs#L599).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/scan/mod.rs:599`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

utility method making it easy to get a transform for a particular row. If the requested row is
outside the range of the passed slice returns `None`, otherwise returns the element at the index
of the specified row
