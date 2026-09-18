# `deltalake_core::table::builder::DeltaTableConfig`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.table.builder.DeltaTableConfig.json).

<a id="op-ec99b2a2fc5ed2c08aa17e2f"></a>
## DeltaTableConfig

`struct` · `deltalake_core::table::builder::DeltaTableConfig` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct DeltaTableConfig
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/table/builder.rs#L35).

Source: `crates/core/src/table/builder.rs:35`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Configuration options for delta table

<a id="op-0bebbe7fd0eccd2bf20a73aa"></a>
## clone

`function` · `deltalake_core::table::builder::DeltaTableConfig::clone` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn clone(&self) -> DeltaTableConfig
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/table/builder.rs#L33).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::table::builder::DeltaTableConfig", "path": "DeltaTableConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [33, 41], "end": [33, 46], "filename": "crates/core/src/table/builder.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `crates/core/src/table/builder.rs:33`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1691d7e86b44572f2390f9e6"></a>
## default

`function` · `deltalake_core::table::builder::DeltaTableConfig::default` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn default() -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/table/builder.rs#L72).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::table::builder::DeltaTableConfig", "path": "DeltaTableConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [71, 1], "end": [84, 2], "filename": "crates/core/src/table/builder.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `crates/core/src/table/builder.rs:72`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-30a42ed063b436c8407f01e9"></a>
## deserialize

`function` · `deltalake_core::table::builder::DeltaTableConfig::deserialize` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/table/builder.rs#L33).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::table::builder::DeltaTableConfig", "path": "DeltaTableConfig"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [33, 28], "end": [33, 39], "filename": "crates/core/src/table/builder.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `crates/core/src/table/builder.rs:33`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-93cfcbd0b8d507aece8aa2c2"></a>
## eq

`function` · `deltalake_core::table::builder::DeltaTableConfig::eq` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn eq(&self, other: &Self) -> bool
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/table/builder.rs#L87).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::table::builder::DeltaTableConfig", "path": "DeltaTableConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [86, 1], "end": [93, 2], "filename": "crates/core/src/table/builder.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `crates/core/src/table/builder.rs:87`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fa61e8c24ff4b8c15d393aea"></a>
## fmt

`function` · `deltalake_core::table::builder::DeltaTableConfig::fmt` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/table/builder.rs#L33).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::table::builder::DeltaTableConfig", "path": "DeltaTableConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [33, 10], "end": [33, 15], "filename": "crates/core/src/table/builder.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `crates/core/src/table/builder.rs:33`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d17fd44c7b36d8d68a7c4b75"></a>
## from_iter

`function` · `deltalake_core::table::builder::DeltaTableConfig::from_iter` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn from_iter<I: IntoIterator<Item = (K, V)>>(iter: I) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/table/builder.rs#L33).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::table::builder::DeltaTableConfig", "path": "DeltaTableConfig"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "K"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "V"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"type": {"primitive": "str"}}], "constraints": []}}, "id": "core::convert::AsRef", "path": "AsRef"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "alloc::string::String", "path": "String"}}}], "constraints": []}}, "id": "core::convert::Into", "path": "Into"}}}], "generic_params": [], "type": {"generic": "K"}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"type": {"primitive": "str"}}], "constraints": []}}, "id": "core::convert::AsRef", "path": "AsRef"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "alloc::string::String", "path": "String"}}}], "constraints": []}}, "id": "core::convert::Into", "path": "Into"}}}], "generic_params": [], "type": {"generic": "V"}}}]}, "is_negative": false, "span": {"begin": [33, 48], "end": [33, 59], "filename": "crates/core/src/table/builder.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"tuple": [{"generic": "K"}, {"generic": "V"}]}}], "constraints": []}}, "id": "core::iter::traits::collect::FromIterator", "path": "FromIterator"}, "trait_path": "core::iter::traits::collect::FromIterator"}`

Source: `crates/core/src/table/builder.rs:33`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5acdff8250ea48f98726ad79"></a>
## io_runtime

`struct_field` · `deltalake_core::table::builder::DeltaTableConfig::io_runtime` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
io_runtime: Option<logstore::storage::IORuntime>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/table/builder.rs#L68).

Source: `crates/core/src/table/builder.rs:68`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

When a runtime handler is provided, all IO tasks are spawn in that handle

<a id="op-610c3221dbfa7641fb1ae650"></a>
## load_from_environment

`function` · `deltalake_core::table::builder::DeltaTableConfig::load_from_environment` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn load_from_environment(&mut self) -> DeltaResult<()>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/table/builder.rs#L33).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::table::builder::DeltaTableConfig", "path": "DeltaTableConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [33, 48], "end": [33, 59], "filename": "crates/core/src/table/builder.rs"}, "trait": {"args": null, "id": "deltalake_core::logstore::config::TryUpdateKey", "path": "TryUpdateKey"}, "trait_path": "deltalake_core::logstore::config::TryUpdateKey"}`

Source: `crates/core/src/table/builder.rs:33`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bca17df2bd8bf3012ef14a89"></a>
## log_batch_size

`struct_field` · `deltalake_core::table::builder::DeltaTableConfig::log_batch_size` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
log_batch_size: usize
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/table/builder.rs#L54).

Source: `crates/core/src/table/builder.rs:54`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Control the number of records to read / process from the commit / checkpoint files
when processing record batches.

<a id="op-52e93533010e304760017553"></a>
## log_buffer_size

`struct_field` · `deltalake_core::table::builder::DeltaTableConfig::log_buffer_size` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
log_buffer_size: usize
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/table/builder.rs#L50).

Source: `crates/core/src/table/builder.rs:50`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Controls how many files to buffer from the commit log when updating the table.
This defaults to 4 * number of cpus

Setting a value greater than 1 results in concurrent calls to the storage api.
This can decrease latency if there are many files in the log since the
last checkpoint, but will also increase memory usage. Possible rate limits of the storage backend should
also be considered for optimal performance.

<a id="op-c4702f2ac5ce6e2089f02532"></a>
## require_files

`struct_field` · `deltalake_core::table::builder::DeltaTableConfig::require_files` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
require_files: bool
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/table/builder.rs#L41).

Source: `crates/core/src/table/builder.rs:41`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Indicates whether DeltaTable should track files.
This defaults to `true`

Some append-only applications might have no need of tracking any files.
Hence, DeltaTable will be loaded with significant memory reduction.

<a id="op-20f728eaa71038e5920295a2"></a>
## serialize

`function` · `deltalake_core::table::builder::DeltaTableConfig::serialize` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/table/builder.rs#L33).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::table::builder::DeltaTableConfig", "path": "DeltaTableConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [33, 17], "end": [33, 26], "filename": "crates/core/src/table/builder.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `crates/core/src/table/builder.rs:33`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0fb203006d2c47dd36967e0a"></a>
## skip_stats

`struct_field` · `deltalake_core::table::builder::DeltaTableConfig::skip_stats` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
skip_stats: bool
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/table/builder.rs#L63).

Source: `crates/core/src/table/builder.rs:63`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Skip parsing file statistics while opening the table.
This defaults to `false`.

Use this option for maintenance and append workflows that do not need file pruning.
Queries with predicates scan each file because the kernel disables statistics and
partition pruning.

<a id="op-755dd191b1fd9f5181a7b6dc"></a>
## try_update_key

`function` · `deltalake_core::table::builder::DeltaTableConfig::try_update_key` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn try_update_key(&mut self, key: &str, v: &str) -> DeltaResult<Option<()>>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/table/builder.rs#L33).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::table::builder::DeltaTableConfig", "path": "DeltaTableConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [33, 48], "end": [33, 59], "filename": "crates/core/src/table/builder.rs"}, "trait": {"args": null, "id": "deltalake_core::logstore::config::TryUpdateKey", "path": "TryUpdateKey"}, "trait_path": "deltalake_core::logstore::config::TryUpdateKey"}`

Source: `crates/core/src/table/builder.rs:33`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.
