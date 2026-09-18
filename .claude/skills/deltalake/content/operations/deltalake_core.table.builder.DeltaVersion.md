# `deltalake_core::table::builder::DeltaVersion`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.table.builder.DeltaVersion.json).

<a id="op-11b43aad5727020a04f7bc7f"></a>
## DeltaVersion

`enum` · `deltalake_core::table::builder::DeltaVersion` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
enum DeltaVersion
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/table/builder.rs#L22).

Source: `crates/core/src/table/builder.rs:22`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

possible version specifications for loading a delta table

<a id="op-066e7715ecddbc6aa4c1451c"></a>
## Newest

`variant` · `deltalake_core::table::builder::DeltaVersion::Newest` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
Newest
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/table/builder.rs#L25).

Source: `crates/core/src/table/builder.rs:25`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

load the newest version

<a id="op-de3a12cc4a2acfc43864c95e"></a>
## Timestamp

`variant` · `deltalake_core::table::builder::DeltaVersion::Timestamp` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
Timestamp
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/table/builder.rs#L29).

Source: `crates/core/src/table/builder.rs:29`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

specify the timestamp in UTC

<a id="op-68e86dd43465c5435cd9ca6b"></a>
## Version

`variant` · `deltalake_core::table::builder::DeltaVersion::Version` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
Version
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/table/builder.rs#L27).

Source: `crates/core/src/table/builder.rs:27`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

specify the version to load

<a id="op-97af8992f17dd5d41bc70bfe"></a>
## clone

`function` · `deltalake_core::table::builder::DeltaVersion::clone` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn clone(&self) -> DeltaVersion
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/table/builder.rs#L21).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::table::builder::DeltaVersion", "path": "DeltaVersion"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [21, 23], "end": [21, 28], "filename": "crates/core/src/table/builder.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `crates/core/src/table/builder.rs:21`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8403452a4e181af81ffe46ac"></a>
## default

`function` · `deltalake_core::table::builder::DeltaVersion::default` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn default() -> DeltaVersion
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/table/builder.rs#L21).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::table::builder::DeltaVersion", "path": "DeltaVersion"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [21, 45], "end": [21, 52], "filename": "crates/core/src/table/builder.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `crates/core/src/table/builder.rs:21`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8a3454312f8e31cfd7719f89"></a>
## eq

`function` · `deltalake_core::table::builder::DeltaVersion::eq` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn eq(&self, other: &DeltaVersion) -> bool
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/table/builder.rs#L21).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::table::builder::DeltaVersion", "path": "DeltaVersion"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [21, 30], "end": [21, 39], "filename": "crates/core/src/table/builder.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `crates/core/src/table/builder.rs:21`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c5b26f17d79911a2ff4ca0e5"></a>
## fmt

`function` · `deltalake_core::table::builder::DeltaVersion::fmt` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/table/builder.rs#L21).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::table::builder::DeltaVersion", "path": "DeltaVersion"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [21, 10], "end": [21, 15], "filename": "crates/core/src/table/builder.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `crates/core/src/table/builder.rs:21`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.
