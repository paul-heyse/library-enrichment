# `deltalake_core::datafile::datafusion_ext::ScanOptions`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.datafile.datafusion_ext.ScanOptions.json).

<a id="op-c811602b2fd615a8d8dc0270"></a>
## ScanOptions

`struct` · `deltalake_core::datafile::datafusion_ext::ScanOptions` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct ScanOptions
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/datafile/datafusion_ext.rs#L45).

Source: `crates/core/src/datafile/datafusion_ext.rs:45`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Options controlling a DataFusion-backed scan.

<a id="op-d53f99947201cef6d145290f"></a>
## clone

`function` · `deltalake_core::datafile::datafusion_ext::ScanOptions::clone` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn clone(&self) -> ScanOptions
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/datafile/datafusion_ext.rs#L44).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::datafile::datafusion_ext::ScanOptions", "path": "ScanOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [44, 26], "end": [44, 31], "filename": "crates/core/src/datafile/datafusion_ext.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `crates/core/src/datafile/datafusion_ext.rs:44`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a490a25426adffb5c383be27"></a>
## default

`function` · `deltalake_core::datafile::datafusion_ext::ScanOptions::default` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn default() -> ScanOptions
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/datafile/datafusion_ext.rs#L44).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::datafile::datafusion_ext::ScanOptions", "path": "ScanOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [44, 17], "end": [44, 24], "filename": "crates/core/src/datafile/datafusion_ext.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `crates/core/src/datafile/datafusion_ext.rs:44`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5bf42c4737b134b27a27318c"></a>
## fmt

`function` · `deltalake_core::datafile::datafusion_ext::ScanOptions::fmt` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/datafile/datafusion_ext.rs#L44).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::datafile::datafusion_ext::ScanOptions", "path": "ScanOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [44, 10], "end": [44, 15], "filename": "crates/core/src/datafile/datafusion_ext.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `crates/core/src/datafile/datafusion_ext.rs:44`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-850421ba68db6867474712fa"></a>
## from

`function` · `deltalake_core::datafile::datafusion_ext::ScanOptions::from` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn from(value: ReadOptions) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/datafile/datafusion_ext.rs#L53).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::datafile::datafusion_ext::ScanOptions", "path": "ScanOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [52, 1], "end": [59, 2], "filename": "crates/core/src/datafile/datafusion_ext.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "deltalake_core::datafile::ReadOptions", "path": "ReadOptions"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `crates/core/src/datafile/datafusion_ext.rs:53`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-541e3471c44fa87d30b89998"></a>
## limit

`struct_field` · `deltalake_core::datafile::datafusion_ext::ScanOptions::limit` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
limit: Option<usize>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/datafile/datafusion_ext.rs#L49).

Source: `crates/core/src/datafile/datafusion_ext.rs:49`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Stop after returning at least this many rows. `None` reads the whole table.

<a id="op-4890701dc5da40b7b4a43e88"></a>
## projection

`struct_field` · `deltalake_core::datafile::datafusion_ext::ScanOptions::projection` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
projection: Option<Vec<String>>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/datafile/datafusion_ext.rs#L47).

Source: `crates/core/src/datafile/datafusion_ext.rs:47`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Project to this subset of (logical) column names. `None` reads all columns.
