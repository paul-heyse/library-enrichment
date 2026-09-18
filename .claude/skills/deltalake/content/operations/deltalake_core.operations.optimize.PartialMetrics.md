# `deltalake_core::operations::optimize::PartialMetrics`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.operations.optimize.PartialMetrics.json).

<a id="op-b7a0138eb53e05c6978314a8"></a>
## PartialMetrics

`struct` · `deltalake_core::operations::optimize::PartialMetrics` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct PartialMetrics
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/optimize.rs#L224).

Source: `crates/core/src/operations/optimize.rs:224`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Metrics for a single partition

<a id="op-353839997ea7668815069eb6"></a>
## files_added

`struct_field` · `deltalake_core::operations::optimize::PartialMetrics::files_added` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
files_added: MetricDetails
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/optimize.rs#L230).

Source: `crates/core/src/operations/optimize.rs:230`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Detailed metrics for the add operation

<a id="op-15671acbcb2887d038472b43"></a>
## files_removed

`struct_field` · `deltalake_core::operations::optimize::PartialMetrics::files_removed` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
files_removed: MetricDetails
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/optimize.rs#L232).

Source: `crates/core/src/operations/optimize.rs:232`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Detailed metrics for the remove operation

<a id="op-650adb5ac671133a21771cd8"></a>
## fmt

`function` · `deltalake_core::operations::optimize::PartialMetrics::fmt` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/optimize.rs#L222).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::optimize::PartialMetrics", "path": "PartialMetrics"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [222, 10], "end": [222, 15], "filename": "crates/core/src/operations/optimize.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `crates/core/src/operations/optimize.rs:222`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c1b134042942660d38a8dc16"></a>
## num_batches

`struct_field` · `deltalake_core::operations::optimize::PartialMetrics::num_batches` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
num_batches: u64
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/optimize.rs#L234).

Source: `crates/core/src/operations/optimize.rs:234`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

The number of batches written

<a id="op-1973e329a7ab6a9a76fa241e"></a>
## num_files_added

`struct_field` · `deltalake_core::operations::optimize::PartialMetrics::num_files_added` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
num_files_added: u64
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/optimize.rs#L226).

Source: `crates/core/src/operations/optimize.rs:226`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Number of optimized files added

<a id="op-e463023cc34376e92f73355f"></a>
## num_files_removed

`struct_field` · `deltalake_core::operations::optimize::PartialMetrics::num_files_removed` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
num_files_removed: u64
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/optimize.rs#L228).

Source: `crates/core/src/operations/optimize.rs:228`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Number of unoptimized files removed
