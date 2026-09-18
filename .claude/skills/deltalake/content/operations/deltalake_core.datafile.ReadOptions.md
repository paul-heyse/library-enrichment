# `deltalake_core::datafile::ReadOptions`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.datafile.ReadOptions.json).

<a id="op-6d9528072861586dc6f53800"></a>
## ReadOptions

`struct` · `deltalake_core::datafile::ReadOptions` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct ReadOptions
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/datafile/mod.rs#L74).

Source: `crates/core/src/datafile/mod.rs:74`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Options for a basic (DataFusion-free) read. Richer predicate/projection
pushdown is the DataFusion extension's job ([`datafusion_ext::DeltaDataReaderExt`](../operations/deltalake_core.datafile.datafusion_ext.DeltaDataReaderExt.md#op-1bf8e4be6ddc21b15f13b89a)).

<a id="op-0deec5a7ab1b76291a1b44e6"></a>
## clone

`function` · `deltalake_core::datafile::ReadOptions::clone` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn clone(&self) -> ReadOptions
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/datafile/mod.rs#L73).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::datafile::ReadOptions", "path": "ReadOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [73, 26], "end": [73, 31], "filename": "crates/core/src/datafile/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `crates/core/src/datafile/mod.rs:73`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a387c455c4d1eae47b55214b"></a>
## default

`function` · `deltalake_core::datafile::ReadOptions::default` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn default() -> ReadOptions
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/datafile/mod.rs#L73).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::datafile::ReadOptions", "path": "ReadOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [73, 17], "end": [73, 24], "filename": "crates/core/src/datafile/mod.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `crates/core/src/datafile/mod.rs:73`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bb686319119dda819f2f2dc4"></a>
## fmt

`function` · `deltalake_core::datafile::ReadOptions::fmt` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/datafile/mod.rs#L73).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::datafile::ReadOptions", "path": "ReadOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [73, 10], "end": [73, 15], "filename": "crates/core/src/datafile/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `crates/core/src/datafile/mod.rs:73`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c5afbf215b884283da98d0fb"></a>
## limit

`struct_field` · `deltalake_core::datafile::ReadOptions::limit` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
limit: Option<usize>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/datafile/mod.rs#L78).

Source: `crates/core/src/datafile/mod.rs:78`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Stop after returning at least this many rows. `None` reads the whole table.

<a id="op-7f171aadacb3b93f1975f4ea"></a>
## projection

`struct_field` · `deltalake_core::datafile::ReadOptions::projection` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
projection: Option<Vec<String>>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/datafile/mod.rs#L76).

Source: `crates/core/src/datafile/mod.rs:76`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Project to this subset of (logical) column names. `None` reads all columns.

<a id="op-c4484a77ab5dfc0e05b8cffa"></a>
## with_limit

`function` · `deltalake_core::datafile::ReadOptions::with_limit` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_limit(self, limit: usize) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/datafile/mod.rs#L89).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::datafile::ReadOptions", "path": "ReadOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [81, 1], "end": [93, 2], "filename": "crates/core/src/datafile/mod.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/datafile/mod.rs:89`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Limit the number of rows returned.

<a id="op-1eb0c89f289d9b56fcb41e0e"></a>
## with_projection

`function` · `deltalake_core::datafile::ReadOptions::with_projection` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_projection(self, projection: impl Into<Vec<String>>) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/datafile/mod.rs#L83).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::datafile::ReadOptions", "path": "ReadOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [81, 1], "end": [93, 2], "filename": "crates/core/src/datafile/mod.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/datafile/mod.rs:83`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Project to the given logical column names.
