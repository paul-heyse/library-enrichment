# `deltalake_core::kernel::snapshot::scan::ScanBuilder`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.kernel.snapshot.scan.ScanBuilder.json).

<a id="op-a287f3a037e75c6307dca12c"></a>
## ScanBuilder

`struct` · `deltalake_core::kernel::snapshot::scan::ScanBuilder` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct ScanBuilder
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/snapshot/scan.rs#L28).

Source: `crates/core/src/kernel/snapshot/scan.rs:28`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Builder to scan a snapshot of a table.

<a id="op-f96293b28ce16cb63f815ada"></a>
## build

`function` · `deltalake_core::kernel::snapshot::scan::ScanBuilder::build` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn build(self) -> DeltaResult<Scan>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/snapshot/scan.rs#L108).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::snapshot::scan::ScanBuilder", "path": "ScanBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [35, 1], "end": [138, 2], "filename": "crates/core/src/kernel/snapshot/scan.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/kernel/snapshot/scan.rs:108`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Finalize the builder into a [`Scan`](../operations/deltalake_core.kernel.snapshot.scan.Scan.md#op-ef5be9790dda876312ea948b), validating the configured schema and predicate.

<a id="op-244c14511a7a68ef0f6ad7c0"></a>
## fmt

`function` · `deltalake_core::kernel::snapshot::scan::ScanBuilder::fmt` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/snapshot/scan.rs#L27).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::snapshot::scan::ScanBuilder", "path": "ScanBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [27, 10], "end": [27, 15], "filename": "crates/core/src/kernel/snapshot/scan.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `crates/core/src/kernel/snapshot/scan.rs:27`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9e405827ae4091bd2f1ae440"></a>
## new

`function` · `deltalake_core::kernel::snapshot::scan::ScanBuilder::new` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn new(snapshot: impl Into<Arc<KernelSnapshot>>) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/snapshot/scan.rs#L37).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::snapshot::scan::ScanBuilder", "path": "ScanBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [35, 1], "end": [138, 2], "filename": "crates/core/src/kernel/snapshot/scan.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/kernel/snapshot/scan.rs:37`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Create a new [`ScanBuilder`](../operations/deltalake_core.kernel.snapshot.scan.ScanBuilder.md#op-a287f3a037e75c6307dca12c) instance.

<a id="op-51944eed065a9c99d435ec79"></a>
## with_kernel_all_struct_stats

`function` · `deltalake_core::kernel::snapshot::scan::ScanBuilder::with_kernel_all_struct_stats` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_kernel_all_struct_stats(self) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/snapshot/scan.rs#L132).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::snapshot::scan::ScanBuilder", "path": "ScanBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [35, 1], "end": [138, 2], "filename": "crates/core/src/kernel/snapshot/scan.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/kernel/snapshot/scan.rs:132`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Experimental: Use kernel's all_struct statistics mode (AFFECTS ULP).
This is a preview of the modernization - use for experimentation.

<a id="op-c91e7a877f498e3017555c3d"></a>
## with_predicate

`function` · `deltalake_core::kernel::snapshot::scan::ScanBuilder::with_predicate` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_predicate(self, predicate: impl Into<Option<PredicateRef>>) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/snapshot/scan.rs#L75).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::snapshot::scan::ScanBuilder", "path": "ScanBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [35, 1], "end": [138, 2], "filename": "crates/core/src/kernel/snapshot/scan.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/kernel/snapshot/scan.rs:75`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Optionally provide an expression to filter rows. For example, using the predicate `x <
4` to return a subset of the rows in the scan which satisfy the filter. If `predicate_opt`
is `None`, this is a no-op.

NOTE: The filtering is best-effort and can produce false positives (rows that should should
have been filtered out but were kept).

<a id="op-5d558e4c63344ad0573e7aba"></a>
## with_schema

`function` · `deltalake_core::kernel::snapshot::scan::ScanBuilder::with_schema` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_schema(self, schema: SchemaRef) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/snapshot/scan.rs#L53).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::snapshot::scan::ScanBuilder", "path": "ScanBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [35, 1], "end": [138, 2], "filename": "crates/core/src/kernel/snapshot/scan.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/kernel/snapshot/scan.rs:53`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Provide [`Schema`] for columns to select from the [`Snapshot`].

A table with columns `[a, b, c]` could have a scan which reads only the first
two columns by using the schema `[a, b]`.

[`Schema`]: crate::schema::Schema
[`Snapshot`]: crate::snapshot::Snapshot

<a id="op-ea73ae821ed0576a07af7c4e"></a>
## with_schema_opt

`function` · `deltalake_core::kernel::snapshot::scan::ScanBuilder::with_schema_opt` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_schema_opt(self, schema_opt: Option<SchemaRef>) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/snapshot/scan.rs#L62).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::snapshot::scan::ScanBuilder", "path": "ScanBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [35, 1], "end": [138, 2], "filename": "crates/core/src/kernel/snapshot/scan.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/kernel/snapshot/scan.rs:62`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Optionally provide a [`SchemaRef`](../operations/buoyant_kernel.schema.SchemaRef.md#op-bcbc708676b5e88d4183ee59) for columns to select from the [`Snapshot`]. See
[`ScanBuilder::with_schema`](../operations/deltalake_core.kernel.snapshot.scan.ScanBuilder.md#op-5d558e4c63344ad0573e7aba) for details. If `schema_opt` is `None` this is a no-op.

[`Snapshot`]: crate::Snapshot

<a id="op-34d9bfcbcd4538224770a2da"></a>
## with_skip_stats

`function` · `deltalake_core::kernel::snapshot::scan::ScanBuilder::with_skip_stats` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_skip_stats(self, skip_stats: bool) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/snapshot/scan.rs#L87).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::snapshot::scan::ScanBuilder", "path": "ScanBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [35, 1], "end": [138, 2], "filename": "crates/core/src/kernel/snapshot/scan.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/kernel/snapshot/scan.rs:87`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Skip file statistics during kernel log replay.

Set `true` to make the kernel skip minimum and maximum values plus null counts.
The kernel disables predicate pruning, including partition pruning. Scan metadata keeps
each active file for predicates that depend on file data. Readers must apply the predicate.
Set `false` to remove a prior override. The builder infers statistics materialization and
enables kernel pruning.

<a id="op-e72930f403e1ace7711d58db"></a>
## predicate

`struct_field` · `deltalake_core::kernel::snapshot::scan::ScanBuilder::predicate` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
predicate: Option<delta_kernel::PredicateRef>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/snapshot/scan.rs#L31).

Source: `crates/core/src/kernel/snapshot/scan.rs:31`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4f5654d2203af8b7b692bed8"></a>
## schema

`struct_field` · `deltalake_core::kernel::snapshot::scan::ScanBuilder::schema` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
schema: Option<delta_kernel::schema::SchemaRef>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/snapshot/scan.rs#L30).

Source: `crates/core/src/kernel/snapshot/scan.rs:30`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-651bd9526595240e6e6f3b08"></a>
## snapshot

`struct_field` · `deltalake_core::kernel::snapshot::scan::ScanBuilder::snapshot` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
snapshot: std::sync::Arc<delta_kernel::snapshot::Snapshot>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/snapshot/scan.rs#L29).

Source: `crates/core/src/kernel/snapshot/scan.rs:29`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-685021650cea5ab2d4e5ea8b"></a>
## stats_materialization

`struct_field` · `deltalake_core::kernel::snapshot::scan::ScanBuilder::stats_materialization` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
stats_materialization: Option<super::stats_projection::FileStatsMaterialization>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/snapshot/scan.rs#L32).

Source: `crates/core/src/kernel/snapshot/scan.rs:32`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.
