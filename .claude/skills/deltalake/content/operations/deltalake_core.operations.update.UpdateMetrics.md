# `deltalake_core::operations::update::UpdateMetrics`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.operations.update.UpdateMetrics.json).

<a id="op-f0fa2cbb57c95612e8e38295"></a>
## UpdateMetrics

`struct` · `deltalake_core::operations::update::UpdateMetrics` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct UpdateMetrics
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/update.rs#L111).

Source: `crates/core/src/operations/update.rs:111`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Metrics collected during the Update operation

<a id="op-2a085a482377e6088e0da2e9"></a>
## default

`function` · `deltalake_core::operations::update::UpdateMetrics::default` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn default() -> UpdateMetrics
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/update.rs#L109).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::update::UpdateMetrics", "path": "UpdateMetrics"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [109, 10], "end": [109, 17], "filename": "crates/core/src/operations/update.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `crates/core/src/operations/update.rs:109`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d3be35eab9a51b2c6f8b2dc0"></a>
## execution_time_ms

`struct_field` · `deltalake_core::operations::update::UpdateMetrics::execution_time_ms` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
execution_time_ms: u64
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/update.rs#L121).

Source: `crates/core/src/operations/update.rs:121`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Time taken to execute the entire operation.

<a id="op-d5e93c3c9f6eb7c9922378a0"></a>
## fmt

`function` · `deltalake_core::operations::update::UpdateMetrics::fmt` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/update.rs#L109).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::update::UpdateMetrics", "path": "UpdateMetrics"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [109, 30], "end": [109, 35], "filename": "crates/core/src/operations/update.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `crates/core/src/operations/update.rs:109`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0ddb84bea493306a7fee16ab"></a>
## num_added_files

`struct_field` · `deltalake_core::operations::update::UpdateMetrics::num_added_files` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
num_added_files: usize
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/update.rs#L113).

Source: `crates/core/src/operations/update.rs:113`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Number of files added.

<a id="op-f21629218e585e8452a312cb"></a>
## num_copied_rows

`struct_field` · `deltalake_core::operations::update::UpdateMetrics::num_copied_rows` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
num_copied_rows: usize
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/update.rs#L119).

Source: `crates/core/src/operations/update.rs:119`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Number of rows just copied over in the process of updating files.

<a id="op-636ece5ebc2980cc386a8ca6"></a>
## num_removed_files

`struct_field` · `deltalake_core::operations::update::UpdateMetrics::num_removed_files` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
num_removed_files: usize
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/update.rs#L115).

Source: `crates/core/src/operations/update.rs:115`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Number of files removed.

<a id="op-b7501e76c82c4d0b02f95a7b"></a>
## num_updated_rows

`struct_field` · `deltalake_core::operations::update::UpdateMetrics::num_updated_rows` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
num_updated_rows: usize
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/update.rs#L117).

Source: `crates/core/src/operations/update.rs:117`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Number of rows updated.

<a id="op-0592e40a2ad62f10c06eef12"></a>
## scan_time_ms

`struct_field` · `deltalake_core::operations::update::UpdateMetrics::scan_time_ms` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
scan_time_ms: u64
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/update.rs#L123).

Source: `crates/core/src/operations/update.rs:123`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Time taken to scan the files for matches.

<a id="op-1aa2052223d7f3d99ef9098b"></a>
## serialize

`function` · `deltalake_core::operations::update::UpdateMetrics::serialize` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/update.rs#L109).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::update::UpdateMetrics", "path": "UpdateMetrics"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [109, 19], "end": [109, 28], "filename": "crates/core/src/operations/update.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `crates/core/src/operations/update.rs:109`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.
