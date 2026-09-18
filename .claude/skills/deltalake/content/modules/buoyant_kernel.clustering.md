# `buoyant_kernel::clustering`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.clustering.json).

<a id="op-4f7b678e5c02cf59b5b1cece"></a>
## clustering

`module` · `buoyant_kernel::clustering` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_module**. Canonical source location is not automatically a valid import path.

```rust
mod clustering
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/clustering.rs#L1).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/clustering.rs:1`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Clustering column support for Delta tables.

This module provides functionality for reading and writing clustering columns
via domain metadata. Per the Delta protocol, writers MUST write per-file statistics
for clustering columns.

Clustering columns are stored in domain metadata under the `delta.clustering` domain
as a JSON object with a `clusteringColumns` field containing an array of column paths,
where each path is an array of field names (to handle nested columns).
