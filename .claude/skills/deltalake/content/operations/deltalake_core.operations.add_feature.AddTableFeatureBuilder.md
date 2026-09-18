# `deltalake_core::operations::add_feature::AddTableFeatureBuilder`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.operations.add_feature.AddTableFeatureBuilder.json).

<a id="op-5c3716092b96ab4839dff719"></a>
## AddTableFeatureBuilder

`struct` · `deltalake_core::operations::add_feature::AddTableFeatureBuilder` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct AddTableFeatureBuilder
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/add_feature.rs#L20).

Source: `crates/core/src/operations/add_feature.rs:20`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Enable table features for a table

<a id="op-bdf985e3114be4bfd9f8c050"></a>
## IntoFuture

`assoc_type` · `deltalake_core::operations::add_feature::AddTableFeatureBuilder::IntoFuture` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
type IntoFuture = Pin<Box<dyn Future<Output = <AddTableFeatureBuilder as IntoFuture>::Output> + Send>>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/add_feature.rs#L128).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::add_feature::AddTableFeatureBuilder", "path": "AddTableFeatureBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [125, 1], "end": [164, 2], "filename": "crates/core/src/operations/add_feature.rs"}, "trait": {"args": null, "id": "core::future::into_future::IntoFuture", "path": "IntoFuture"}, "trait_path": "core::future::into_future::IntoFuture"}`

Source: `crates/core/src/operations/add_feature.rs:128`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ab0b9e6f2956284f8ea5dabd"></a>
## Output

`assoc_type` · `deltalake_core::operations::add_feature::AddTableFeatureBuilder::Output` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
type Output = Result<DeltaTable, DeltaTableError>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/add_feature.rs#L126).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::add_feature::AddTableFeatureBuilder", "path": "AddTableFeatureBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [125, 1], "end": [164, 2], "filename": "crates/core/src/operations/add_feature.rs"}, "trait": {"args": null, "id": "core::future::into_future::IntoFuture", "path": "IntoFuture"}, "trait_path": "core::future::into_future::IntoFuture"}`

Source: `crates/core/src/operations/add_feature.rs:126`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-694269a9bcae3a6b95e7ad6e"></a>
## into_future

`function` · `deltalake_core::operations::add_feature::AddTableFeatureBuilder::into_future` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn into_future(self) -> Self::IntoFuture
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/add_feature.rs#L130).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::add_feature::AddTableFeatureBuilder", "path": "AddTableFeatureBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [125, 1], "end": [164, 2], "filename": "crates/core/src/operations/add_feature.rs"}, "trait": {"args": null, "id": "core::future::into_future::IntoFuture", "path": "IntoFuture"}, "trait_path": "core::future::into_future::IntoFuture"}`

Source: `crates/core/src/operations/add_feature.rs:130`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f27f1846fc9a1545b39a3708"></a>
## with_allow_protocol_versions_increase

`function` · `deltalake_core::operations::add_feature::AddTableFeatureBuilder::with_allow_protocol_versions_increase` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_allow_protocol_versions_increase(self, allow: bool) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/add_feature.rs#L70).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::add_feature::AddTableFeatureBuilder", "path": "AddTableFeatureBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [43, 1], "end": [86, 2], "filename": "crates/core/src/operations/add_feature.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/operations/add_feature.rs:70`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Specify if you want to allow protocol version to be increased

<a id="op-74de6613516f0f36dbdb76fb"></a>
## with_commit_properties

`function` · `deltalake_core::operations::add_feature::AddTableFeatureBuilder::with_commit_properties` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_commit_properties(self, commit_properties: CommitProperties) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/add_feature.rs#L76).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::add_feature::AddTableFeatureBuilder", "path": "AddTableFeatureBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [43, 1], "end": [86, 2], "filename": "crates/core/src/operations/add_feature.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/operations/add_feature.rs:76`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Additional metadata to be added to commit info

<a id="op-cc86705429fc440375ccba78"></a>
## with_custom_execute_handler

`function` · `deltalake_core::operations::add_feature::AddTableFeatureBuilder::with_custom_execute_handler` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_custom_execute_handler(self, handler: Arc<dyn CustomExecuteHandler>) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/add_feature.rs#L82).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::add_feature::AddTableFeatureBuilder", "path": "AddTableFeatureBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [43, 1], "end": [86, 2], "filename": "crates/core/src/operations/add_feature.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/operations/add_feature.rs:82`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Set a custom execute handler, for pre and post execution

<a id="op-4054c045e316bc84f9742df9"></a>
## with_feature

`function` · `deltalake_core::operations::add_feature::AddTableFeatureBuilder::with_feature` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_feature<S: Into<TableFeatures>>(self, name: S) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/add_feature.rs#L57).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::add_feature::AddTableFeatureBuilder", "path": "AddTableFeatureBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [43, 1], "end": [86, 2], "filename": "crates/core/src/operations/add_feature.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/operations/add_feature.rs:57`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Specify the features to be added

<a id="op-19e6c4983e51a8a66060699a"></a>
## with_features

`function` · `deltalake_core::operations::add_feature::AddTableFeatureBuilder::with_features` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_features<S: Into<TableFeatures>>(self, name: Vec<S>) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/add_feature.rs#L63).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::add_feature::AddTableFeatureBuilder", "path": "AddTableFeatureBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [43, 1], "end": [86, 2], "filename": "crates/core/src/operations/add_feature.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/operations/add_feature.rs:63`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Specify the features to be added

<a id="op-c61cd0637376acfca01a6768"></a>
## allow_protocol_versions_increase

`struct_field` · `deltalake_core::operations::add_feature::AddTableFeatureBuilder::allow_protocol_versions_increase` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
allow_protocol_versions_increase: bool
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/add_feature.rs#L26).

Source: `crates/core/src/operations/add_feature.rs:26`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Allow protocol versions to be increased by setting features

<a id="op-8a2293eaed434133016f3f45"></a>
## commit_properties

`struct_field` · `deltalake_core::operations::add_feature::AddTableFeatureBuilder::commit_properties` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
commit_properties: kernel::transaction::CommitProperties
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/add_feature.rs#L30).

Source: `crates/core/src/operations/add_feature.rs:30`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Additional information to add to the commit

<a id="op-9e6a5f48221145792b12ed10"></a>
## custom_execute_handler

`struct_field` · `deltalake_core::operations::add_feature::AddTableFeatureBuilder::custom_execute_handler` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
custom_execute_handler: Option<std::sync::Arc<dyn CustomExecuteHandler>>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/add_feature.rs#L31).

Source: `crates/core/src/operations/add_feature.rs:31`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fe11a5ccb235ca916ed901e7"></a>
## get_custom_execute_handler

`function` · `deltalake_core::operations::add_feature::AddTableFeatureBuilder::get_custom_execute_handler` · deltalake-core 1.0.0+58f07cd6

Access: **internal_trait**. Canonical source location is not automatically a valid import path.

```rust
fn get_custom_execute_handler(&self) -> Option<Arc<dyn CustomExecuteHandler>>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/add_feature.rs#L38).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::add_feature::AddTableFeatureBuilder", "path": "AddTableFeatureBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [34, 1], "end": [41, 2], "filename": "crates/core/src/operations/add_feature.rs"}, "trait": {"args": null, "id": "deltalake_core::operations::Operation", "path": "Operation"}, "trait_path": "deltalake_core::operations::Operation"}`

Source: `crates/core/src/operations/add_feature.rs:38`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-261e9582126d0142b1381896"></a>
## log_store

`function` · `deltalake_core::operations::add_feature::AddTableFeatureBuilder::log_store` · deltalake-core 1.0.0+58f07cd6

Access: **internal_trait**. Canonical source location is not automatically a valid import path.

```rust
fn log_store(&self) -> &LogStoreRef
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/add_feature.rs#L35).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::add_feature::AddTableFeatureBuilder", "path": "AddTableFeatureBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [34, 1], "end": [41, 2], "filename": "crates/core/src/operations/add_feature.rs"}, "trait": {"args": null, "id": "deltalake_core::operations::Operation", "path": "Operation"}, "trait_path": "deltalake_core::operations::Operation"}`

Source: `crates/core/src/operations/add_feature.rs:35`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-55b7cbbb2048810591ba3a6f"></a>
## log_store

`struct_field` · `deltalake_core::operations::add_feature::AddTableFeatureBuilder::log_store` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
log_store: logstore::LogStoreRef
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/add_feature.rs#L28).

Source: `crates/core/src/operations/add_feature.rs:28`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Delta object store for handling data files

<a id="op-43db49de18582427af43a162"></a>
## name

`struct_field` · `deltalake_core::operations::add_feature::AddTableFeatureBuilder::name` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
name: Vec<kernel::TableFeatures>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/add_feature.rs#L24).

Source: `crates/core/src/operations/add_feature.rs:24`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Name of the feature

<a id="op-f9fc0457a7e3715e2cf3a97c"></a>
## snapshot

`struct_field` · `deltalake_core::operations::add_feature::AddTableFeatureBuilder::snapshot` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
snapshot: Option<kernel::EagerSnapshot>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/add_feature.rs#L22).

Source: `crates/core/src/operations/add_feature.rs:22`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

A snapshot of the table's state
