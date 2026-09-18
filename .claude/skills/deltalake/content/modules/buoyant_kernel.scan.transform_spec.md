# `buoyant_kernel::scan::transform_spec`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.scan.transform_spec.json).

<a id="op-be87a817b2fb175fa65f8707"></a>
## transform_spec

`module` · `buoyant_kernel::scan::transform_spec` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_module**. Canonical source location is not automatically a valid import path.

```rust
mod transform_spec
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/scan/transform_spec.rs#L1).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/scan/transform_spec.rs:1`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Transform-related types and utilities for Delta Kernel.

This module contains specs and helpers for translating physical file data into the logical scan
output shape, including partition value processing and expression generation. The current
implementation lowers those specs into `Expression::StructPatch` expressions.
