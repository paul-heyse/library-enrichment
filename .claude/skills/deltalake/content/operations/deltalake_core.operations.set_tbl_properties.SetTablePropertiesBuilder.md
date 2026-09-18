# `deltalake_core::operations::set_tbl_properties::SetTablePropertiesBuilder`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.operations.set_tbl_properties.SetTablePropertiesBuilder.json).

<a id="op-19b887b899a44ecdffc006e3"></a>
## SetTablePropertiesBuilder

`struct` · `deltalake_core::operations::set_tbl_properties::SetTablePropertiesBuilder` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct SetTablePropertiesBuilder
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/set_tbl_properties.rs#L22).

Source: `crates/core/src/operations/set_tbl_properties.rs:22`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Remove constraints from the table

<a id="op-bb29ba46c9d53433f57ec2cd"></a>
## IntoFuture

`assoc_type` · `deltalake_core::operations::set_tbl_properties::SetTablePropertiesBuilder::IntoFuture` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
type IntoFuture = Pin<Box<dyn Future<Output = <SetTablePropertiesBuilder as IntoFuture>::Output> + Send>>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/set_tbl_properties.rs#L121).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::set_tbl_properties::SetTablePropertiesBuilder", "path": "SetTablePropertiesBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [118, 1], "end": [156, 2], "filename": "crates/core/src/operations/set_tbl_properties.rs"}, "trait": {"args": null, "id": "core::future::into_future::IntoFuture", "path": "IntoFuture"}, "trait_path": "core::future::into_future::IntoFuture"}`

Source: `crates/core/src/operations/set_tbl_properties.rs:121`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-61c8cc899ecd1ff8064668b1"></a>
## Output

`assoc_type` · `deltalake_core::operations::set_tbl_properties::SetTablePropertiesBuilder::Output` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
type Output = Result<DeltaTable, DeltaTableError>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/set_tbl_properties.rs#L119).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::set_tbl_properties::SetTablePropertiesBuilder", "path": "SetTablePropertiesBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [118, 1], "end": [156, 2], "filename": "crates/core/src/operations/set_tbl_properties.rs"}, "trait": {"args": null, "id": "core::future::into_future::IntoFuture", "path": "IntoFuture"}, "trait_path": "core::future::into_future::IntoFuture"}`

Source: `crates/core/src/operations/set_tbl_properties.rs:119`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-65d3783bf0531cfd84c39a0b"></a>
## into_future

`function` · `deltalake_core::operations::set_tbl_properties::SetTablePropertiesBuilder::into_future` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn into_future(self) -> Self::IntoFuture
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/set_tbl_properties.rs#L123).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::set_tbl_properties::SetTablePropertiesBuilder", "path": "SetTablePropertiesBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [118, 1], "end": [156, 2], "filename": "crates/core/src/operations/set_tbl_properties.rs"}, "trait": {"args": null, "id": "core::future::into_future::IntoFuture", "path": "IntoFuture"}, "trait_path": "core::future::into_future::IntoFuture"}`

Source: `crates/core/src/operations/set_tbl_properties.rs:123`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8d4e877294fd02e90698b361"></a>
## with_commit_properties

`function` · `deltalake_core::operations::set_tbl_properties::SetTablePropertiesBuilder::with_commit_properties` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_commit_properties(self, commit_properties: CommitProperties) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/set_tbl_properties.rs#L71).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::set_tbl_properties::SetTablePropertiesBuilder", "path": "SetTablePropertiesBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [45, 1], "end": [81, 2], "filename": "crates/core/src/operations/set_tbl_properties.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/operations/set_tbl_properties.rs:71`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Additional metadata to be added to commit info

<a id="op-c835196c4e026e77fd846d83"></a>
## with_custom_execute_handler

`function` · `deltalake_core::operations::set_tbl_properties::SetTablePropertiesBuilder::with_custom_execute_handler` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_custom_execute_handler(self, handler: Arc<dyn CustomExecuteHandler>) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/set_tbl_properties.rs#L77).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::set_tbl_properties::SetTablePropertiesBuilder", "path": "SetTablePropertiesBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [45, 1], "end": [81, 2], "filename": "crates/core/src/operations/set_tbl_properties.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/operations/set_tbl_properties.rs:77`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Set a custom execute handler, for pre and post execution

<a id="op-4baaf99248e6fe5648fab9c5"></a>
## with_properties

`function` · `deltalake_core::operations::set_tbl_properties::SetTablePropertiesBuilder::with_properties` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_properties(self, table_properties: HashMap<String, String>) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/set_tbl_properties.rs#L59).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::set_tbl_properties::SetTablePropertiesBuilder", "path": "SetTablePropertiesBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [45, 1], "end": [81, 2], "filename": "crates/core/src/operations/set_tbl_properties.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/operations/set_tbl_properties.rs:59`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Specify the properties to be removed

<a id="op-7c1e18c27982c7c415e5e0bc"></a>
## with_raise_if_not_exists

`function` · `deltalake_core::operations::set_tbl_properties::SetTablePropertiesBuilder::with_raise_if_not_exists` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_raise_if_not_exists(self, raise: bool) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/set_tbl_properties.rs#L65).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::set_tbl_properties::SetTablePropertiesBuilder", "path": "SetTablePropertiesBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [45, 1], "end": [81, 2], "filename": "crates/core/src/operations/set_tbl_properties.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/operations/set_tbl_properties.rs:65`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Specify if you want to raise if the property does not exist

<a id="op-6b8b48bccd217a1ce27294f9"></a>
## commit_properties

`struct_field` · `deltalake_core::operations::set_tbl_properties::SetTablePropertiesBuilder::commit_properties` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
commit_properties: kernel::transaction::CommitProperties
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/set_tbl_properties.rs#L32).

Source: `crates/core/src/operations/set_tbl_properties.rs:32`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Additional information to add to the commit

<a id="op-964b31ba0cd9f8bf43fd1424"></a>
## custom_execute_handler

`struct_field` · `deltalake_core::operations::set_tbl_properties::SetTablePropertiesBuilder::custom_execute_handler` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
custom_execute_handler: Option<std::sync::Arc<dyn CustomExecuteHandler>>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/set_tbl_properties.rs#L33).

Source: `crates/core/src/operations/set_tbl_properties.rs:33`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d00eac16d508c7521cd405d1"></a>
## get_custom_execute_handler

`function` · `deltalake_core::operations::set_tbl_properties::SetTablePropertiesBuilder::get_custom_execute_handler` · deltalake-core 1.0.0+58f07cd6

Access: **internal_trait**. Canonical source location is not automatically a valid import path.

```rust
fn get_custom_execute_handler(&self) -> Option<Arc<dyn CustomExecuteHandler>>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/set_tbl_properties.rs#L40).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::set_tbl_properties::SetTablePropertiesBuilder", "path": "SetTablePropertiesBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [36, 1], "end": [43, 2], "filename": "crates/core/src/operations/set_tbl_properties.rs"}, "trait": {"args": null, "id": "deltalake_core::operations::Operation", "path": "Operation"}, "trait_path": "deltalake_core::operations::Operation"}`

Source: `crates/core/src/operations/set_tbl_properties.rs:40`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-019aa349ad8d7141cb72c64b"></a>
## log_store

`struct_field` · `deltalake_core::operations::set_tbl_properties::SetTablePropertiesBuilder::log_store` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
log_store: logstore::LogStoreRef
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/set_tbl_properties.rs#L30).

Source: `crates/core/src/operations/set_tbl_properties.rs:30`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Delta object store for handling data files

<a id="op-243561d1b7ea29a30b20256c"></a>
## log_store

`function` · `deltalake_core::operations::set_tbl_properties::SetTablePropertiesBuilder::log_store` · deltalake-core 1.0.0+58f07cd6

Access: **internal_trait**. Canonical source location is not automatically a valid import path.

```rust
fn log_store(&self) -> &LogStoreRef
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/set_tbl_properties.rs#L37).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::set_tbl_properties::SetTablePropertiesBuilder", "path": "SetTablePropertiesBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [36, 1], "end": [43, 2], "filename": "crates/core/src/operations/set_tbl_properties.rs"}, "trait": {"args": null, "id": "deltalake_core::operations::Operation", "path": "Operation"}, "trait_path": "deltalake_core::operations::Operation"}`

Source: `crates/core/src/operations/set_tbl_properties.rs:37`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-493f22da4ece14f96d106b5d"></a>
## properties

`struct_field` · `deltalake_core::operations::set_tbl_properties::SetTablePropertiesBuilder::properties` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
properties: std::collections::HashMap<String, String>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/set_tbl_properties.rs#L26).

Source: `crates/core/src/operations/set_tbl_properties.rs:26`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Name of the property

<a id="op-9421af892b15708f32d9f442"></a>
## raise_if_not_exists

`struct_field` · `deltalake_core::operations::set_tbl_properties::SetTablePropertiesBuilder::raise_if_not_exists` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
raise_if_not_exists: bool
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/set_tbl_properties.rs#L28).

Source: `crates/core/src/operations/set_tbl_properties.rs:28`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Raise if property doesn't exist

<a id="op-84e4160e5cffc09d80ab7ab9"></a>
## snapshot

`struct_field` · `deltalake_core::operations::set_tbl_properties::SetTablePropertiesBuilder::snapshot` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
snapshot: Option<kernel::EagerSnapshot>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/set_tbl_properties.rs#L24).

Source: `crates/core/src/operations/set_tbl_properties.rs:24`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

A snapshot of the table's state
