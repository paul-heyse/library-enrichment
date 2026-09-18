# `deltalake_core::operations::filesystem_check::FileSystemCheckMetrics`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.operations.filesystem_check.FileSystemCheckMetrics.json).

<a id="op-b3c265f69e8af9a294dc6525"></a>
## FileSystemCheckMetrics

`struct` · `deltalake_core::operations::filesystem_check::FileSystemCheckMetrics` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct FileSystemCheckMetrics
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/filesystem_check.rs#L57).

Source: `crates/core/src/operations/filesystem_check.rs:57`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Details of the FSCK operation including which files were removed from the log

<a id="op-22b66e12b55f4202f557d5bf"></a>
## dry_run

`struct_field` · `deltalake_core::operations::filesystem_check::FileSystemCheckMetrics::dry_run` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
dry_run: bool
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/filesystem_check.rs#L59).

Source: `crates/core/src/operations/filesystem_check.rs:59`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Was this a dry run

<a id="op-2a9c624b1be3cf988fdc7af6"></a>
## files_removed

`struct_field` · `deltalake_core::operations::filesystem_check::FileSystemCheckMetrics::files_removed` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
files_removed: Vec<String>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/filesystem_check.rs#L65).

Source: `crates/core/src/operations/filesystem_check.rs:65`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Files that were removed successfully

<a id="op-640a346c45bec002e573c1b3"></a>
## fmt

`function` · `deltalake_core::operations::filesystem_check::FileSystemCheckMetrics::fmt` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/filesystem_check.rs#L56).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::filesystem_check::FileSystemCheckMetrics", "path": "FileSystemCheckMetrics"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [56, 10], "end": [56, 15], "filename": "crates/core/src/operations/filesystem_check.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `crates/core/src/operations/filesystem_check.rs:56`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ccd1511add5acf012b13bfc5"></a>
## serialize

`function` · `deltalake_core::operations::filesystem_check::FileSystemCheckMetrics::serialize` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/filesystem_check.rs#L56).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::filesystem_check::FileSystemCheckMetrics", "path": "FileSystemCheckMetrics"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [56, 17], "end": [56, 26], "filename": "crates/core/src/operations/filesystem_check.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `crates/core/src/operations/filesystem_check.rs:56`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.
