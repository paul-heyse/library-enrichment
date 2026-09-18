# `deltalake_core::operations::merge::MergeMetrics`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.operations.merge.MergeMetrics.json).

<a id="op-d2fd2e333d2f06aec0bd8e02"></a>
## MergeMetrics

`struct` · `deltalake_core::operations::merge::MergeMetrics` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct MergeMetrics
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/merge/mod.rs#L678).

Source: `crates/core/src/operations/merge/mod.rs:678`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Metrics for the Merge Operation

<a id="op-eb607889ae6f449c7092223a"></a>
## default

`function` · `deltalake_core::operations::merge::MergeMetrics::default` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn default() -> MergeMetrics
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/merge/mod.rs#L676).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::merge::MergeMetrics", "path": "MergeMetrics"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [676, 10], "end": [676, 17], "filename": "crates/core/src/operations/merge/mod.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `crates/core/src/operations/merge/mod.rs:676`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b90ff5ec14c18459eb176edd"></a>
## execution_time_ms

`struct_field` · `deltalake_core::operations::merge::MergeMetrics::execution_time_ms` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
execution_time_ms: u64
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/merge/mod.rs#L700).

Source: `crates/core/src/operations/merge/mod.rs:700`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Time taken to execute the entire operation

<a id="op-51db5c472b56abc877257c1d"></a>
## fmt

`function` · `deltalake_core::operations::merge::MergeMetrics::fmt` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/merge/mod.rs#L676).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::merge::MergeMetrics", "path": "MergeMetrics"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [676, 30], "end": [676, 35], "filename": "crates/core/src/operations/merge/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `crates/core/src/operations/merge/mod.rs:676`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-394a005219269920a0891f20"></a>
## num_output_rows

`struct_field` · `deltalake_core::operations::merge::MergeMetrics::num_output_rows` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
num_output_rows: usize
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/merge/mod.rs#L690).

Source: `crates/core/src/operations/merge/mod.rs:690`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Total number of rows written out

<a id="op-3b4e7fc8b1716ba3e4100a05"></a>
## num_source_rows

`struct_field` · `deltalake_core::operations::merge::MergeMetrics::num_source_rows` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
num_source_rows: usize
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/merge/mod.rs#L680).

Source: `crates/core/src/operations/merge/mod.rs:680`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Number of rows in the source data

<a id="op-911c8d0f892bf9d3dbffc1ba"></a>
## num_target_files_added

`struct_field` · `deltalake_core::operations::merge::MergeMetrics::num_target_files_added` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
num_target_files_added: usize
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/merge/mod.rs#L696).

Source: `crates/core/src/operations/merge/mod.rs:696`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Number of files added to the sink(target)

<a id="op-81a6dacb59821756a574029d"></a>
## num_target_files_removed

`struct_field` · `deltalake_core::operations::merge::MergeMetrics::num_target_files_removed` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
num_target_files_removed: usize
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/merge/mod.rs#L698).

Source: `crates/core/src/operations/merge/mod.rs:698`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Number of files removed from the sink(target)

<a id="op-f8ce10b6c53f556fe3a41135"></a>
## num_target_files_scanned

`struct_field` · `deltalake_core::operations::merge::MergeMetrics::num_target_files_scanned` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
num_target_files_scanned: usize
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/merge/mod.rs#L692).

Source: `crates/core/src/operations/merge/mod.rs:692`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Amount of files considered during table scan

<a id="op-3a6badb3eb50a50414264f4a"></a>
## num_target_files_skipped_during_scan

`struct_field` · `deltalake_core::operations::merge::MergeMetrics::num_target_files_skipped_during_scan` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
num_target_files_skipped_during_scan: usize
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/merge/mod.rs#L694).

Source: `crates/core/src/operations/merge/mod.rs:694`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Amount of files not considered (pruned) during table scan

<a id="op-8eef48db5c418d0d03f9fc26"></a>
## num_target_rows_copied

`struct_field` · `deltalake_core::operations::merge::MergeMetrics::num_target_rows_copied` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
num_target_rows_copied: usize
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/merge/mod.rs#L688).

Source: `crates/core/src/operations/merge/mod.rs:688`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Number of target rows copied

<a id="op-d4df0574b1815dfec6f5e99a"></a>
## num_target_rows_deleted

`struct_field` · `deltalake_core::operations::merge::MergeMetrics::num_target_rows_deleted` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
num_target_rows_deleted: usize
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/merge/mod.rs#L686).

Source: `crates/core/src/operations/merge/mod.rs:686`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Number of rows deleted in the target table

<a id="op-a2b51cddf0cabfd3dace6782"></a>
## num_target_rows_inserted

`struct_field` · `deltalake_core::operations::merge::MergeMetrics::num_target_rows_inserted` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
num_target_rows_inserted: usize
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/merge/mod.rs#L682).

Source: `crates/core/src/operations/merge/mod.rs:682`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Number of rows inserted into the target table

<a id="op-c480514dfda93a4bd74d23d8"></a>
## num_target_rows_updated

`struct_field` · `deltalake_core::operations::merge::MergeMetrics::num_target_rows_updated` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
num_target_rows_updated: usize
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/merge/mod.rs#L684).

Source: `crates/core/src/operations/merge/mod.rs:684`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Number of rows updated in the target table

<a id="op-7fcfe2ec64550729e15069a0"></a>
## rewrite_time_ms

`struct_field` · `deltalake_core::operations::merge::MergeMetrics::rewrite_time_ms` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
rewrite_time_ms: u64
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/merge/mod.rs#L704).

Source: `crates/core/src/operations/merge/mod.rs:704`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Time taken to rewrite the matched files

<a id="op-34e4a8bc8f421018c9db10a9"></a>
## scan_time_ms

`struct_field` · `deltalake_core::operations::merge::MergeMetrics::scan_time_ms` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
scan_time_ms: u64
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/merge/mod.rs#L702).

Source: `crates/core/src/operations/merge/mod.rs:702`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Time taken to scan the files for matches

<a id="op-9e1c185496a96162edea9064"></a>
## serialize

`function` · `deltalake_core::operations::merge::MergeMetrics::serialize` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/merge/mod.rs#L676).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::merge::MergeMetrics", "path": "MergeMetrics"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [676, 19], "end": [676, 28], "filename": "crates/core/src/operations/merge/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `crates/core/src/operations/merge/mod.rs:676`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.
