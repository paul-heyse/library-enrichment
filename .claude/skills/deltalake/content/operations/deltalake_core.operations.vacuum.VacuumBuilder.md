# `deltalake_core::operations::vacuum::VacuumBuilder`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.operations.vacuum.VacuumBuilder.json).

<a id="op-0927549bebad647dbf90eb16"></a>
## VacuumBuilder

`struct` · `deltalake_core::operations::vacuum::VacuumBuilder` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct VacuumBuilder
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/vacuum.rs#L186).

Source: `crates/core/src/operations/vacuum.rs:186`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Vacuum a Delta table with the given options
See this module's documentation for more information

<a id="op-5579325c627644c220dbc161"></a>
## IntoFuture

`assoc_type` · `deltalake_core::operations::vacuum::VacuumBuilder::IntoFuture` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
type IntoFuture = Pin<Box<dyn Future<Output = <VacuumBuilder as IntoFuture>::Output> + Send>>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/vacuum.rs#L559).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::vacuum::VacuumBuilder", "path": "VacuumBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [557, 1], "end": [605, 2], "filename": "crates/core/src/operations/vacuum.rs"}, "trait": {"args": null, "id": "core::future::into_future::IntoFuture", "path": "IntoFuture"}, "trait_path": "core::future::into_future::IntoFuture"}`

Source: `crates/core/src/operations/vacuum.rs:559`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9efeffefd9cbc54190d5ac4a"></a>
## Output

`assoc_type` · `deltalake_core::operations::vacuum::VacuumBuilder::Output` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
type Output = Result<(DeltaTable, VacuumMetrics), DeltaTableError>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/vacuum.rs#L558).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::vacuum::VacuumBuilder", "path": "VacuumBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [557, 1], "end": [605, 2], "filename": "crates/core/src/operations/vacuum.rs"}, "trait": {"args": null, "id": "core::future::into_future::IntoFuture", "path": "IntoFuture"}, "trait_path": "core::future::into_future::IntoFuture"}`

Source: `crates/core/src/operations/vacuum.rs:558`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-814bc079630f3dce005e8ddc"></a>
## into_future

`function` · `deltalake_core::operations::vacuum::VacuumBuilder::into_future` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn into_future(self) -> Self::IntoFuture
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/vacuum.rs#L561).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::vacuum::VacuumBuilder", "path": "VacuumBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [557, 1], "end": [605, 2], "filename": "crates/core/src/operations/vacuum.rs"}, "trait": {"args": null, "id": "core::future::into_future::IntoFuture", "path": "IntoFuture"}, "trait_path": "core::future::into_future::IntoFuture"}`

Source: `crates/core/src/operations/vacuum.rs:561`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ce67f5ab5195426cf2d0b751"></a>
## parallel_scan

`function` · `deltalake_core::operations::vacuum::VacuumBuilder::parallel_scan` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn parallel_scan(self, parallel_scan: bool) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/vacuum.rs#L302).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::vacuum::VacuumBuilder", "path": "VacuumBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [255, 1], "end": [555, 2], "filename": "crates/core/src/operations/vacuum.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/operations/vacuum.rs:302`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Flag to enable/disable the parallel multi-level partition scan used by full-mode vacuum.

By default, tables with more than one partition column use hierarchical
prefix expansion and concurrent leaf LIST calls. Calling this with `false` forces the
flat `list(None)` scan path instead.

<a id="op-ad02caeb47156c52633c1e95"></a>
## with_commit_properties

`function` · `deltalake_core::operations::vacuum::VacuumBuilder::with_commit_properties` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_commit_properties(self, commit_properties: CommitProperties) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/vacuum.rs#L341).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::vacuum::VacuumBuilder", "path": "VacuumBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [255, 1], "end": [555, 2], "filename": "crates/core/src/operations/vacuum.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/operations/vacuum.rs:341`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Additional metadata to be added to commit info

<a id="op-6fc0b3a27c7c53a257434026"></a>
## with_custom_execute_handler

`function` · `deltalake_core::operations::vacuum::VacuumBuilder::with_custom_execute_handler` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_custom_execute_handler(self, handler: Arc<dyn CustomExecuteHandler>) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/vacuum.rs#L347).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::vacuum::VacuumBuilder", "path": "VacuumBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [255, 1], "end": [555, 2], "filename": "crates/core/src/operations/vacuum.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/operations/vacuum.rs:347`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Set a custom execute handler, for pre and post execution

<a id="op-64b36ce2d7cda9e7d12d98d1"></a>
## with_dry_run

`function` · `deltalake_core::operations::vacuum::VacuumBuilder::with_dry_run` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_dry_run(self, dry_run: bool) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/vacuum.rs#L322).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::vacuum::VacuumBuilder", "path": "VacuumBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [255, 1], "end": [555, 2], "filename": "crates/core/src/operations/vacuum.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/operations/vacuum.rs:322`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Only determine which files should be deleted

<a id="op-42658fd8f523636f302edf7b"></a>
## with_enforce_retention_duration

`function` · `deltalake_core::operations::vacuum::VacuumBuilder::with_enforce_retention_duration` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_enforce_retention_duration(self, enforce: bool) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/vacuum.rs#L328).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::vacuum::VacuumBuilder", "path": "VacuumBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [255, 1], "end": [555, 2], "filename": "crates/core/src/operations/vacuum.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/operations/vacuum.rs:328`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Check if the specified retention period is less than the table's minimum

<a id="op-972e70574d351c468cfb45c7"></a>
## with_keep_versions

`function` · `deltalake_core::operations::vacuum::VacuumBuilder::with_keep_versions` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_keep_versions(self, versions: &[Version]) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/vacuum.rs#L309).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::vacuum::VacuumBuilder", "path": "VacuumBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [255, 1], "end": [555, 2], "filename": "crates/core/src/operations/vacuum.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/operations/vacuum.rs:309`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Specify table versions that we want to keep for time travel.
This will prevent deletion of files required by these versions.

<a id="op-1a50ecdb4534ce3068f563dc"></a>
## with_mode

`function` · `deltalake_core::operations::vacuum::VacuumBuilder::with_mode` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_mode(self, mode: VacuumMode) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/vacuum.rs#L316).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::vacuum::VacuumBuilder", "path": "VacuumBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [255, 1], "end": [555, 2], "filename": "crates/core/src/operations/vacuum.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/operations/vacuum.rs:316`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Override the default vacuum mode (lite)

<a id="op-24691fd7c97cbd97b6a2a5e0"></a>
## with_retention_period

`function` · `deltalake_core::operations::vacuum::VacuumBuilder::with_retention_period` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_retention_period(self, retention_period: Duration) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/vacuum.rs#L275).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::vacuum::VacuumBuilder", "path": "VacuumBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [255, 1], "end": [555, 2], "filename": "crates/core/src/operations/vacuum.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/operations/vacuum.rs:275`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Override the default retention period for which files are deleted.

<a id="op-36fc7028aed97237be6e9cfb"></a>
## with_scan_concurrency

`function` · `deltalake_core::operations::vacuum::VacuumBuilder::with_scan_concurrency` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_scan_concurrency(self, concurrency: usize) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/vacuum.rs#L292).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::vacuum::VacuumBuilder", "path": "VacuumBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [255, 1], "end": [555, 2], "filename": "crates/core/src/operations/vacuum.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/operations/vacuum.rs:292`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Set the maximum number of concurrent object-store LIST operations used
during full-mode vacuum scanning (partition prefix expansion and leaf
orphan listing).

Higher values can speed up scans on high-latency object stores but may
trigger throttling (e.g. HTTP 429/503). LIST retries/backoff are handled
by the underlying object store's [`object_store::RetryConfig`], not by
vacuum itself—if a LIST ultimately fails, the vacuum operation fails.

When unset, concurrency is taken from the `DELTARS_VACUUM_LIST_CONCURRENCY`
environment variable if set to a positive integer, otherwise defaults to 10.
Values of `0` are ignored and fall back to that default resolution.

Unresolved upstream links (retained, not inferred): ``object_store::RetryConfig``.

<a id="op-c9ec70bb1ac0102f14b43c14"></a>
## clock

`struct_field` · `deltalake_core::operations::vacuum::VacuumBuilder::clock` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
clock: Option<std::sync::Arc<dyn Clock>>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/vacuum.rs#L210).

Source: `crates/core/src/operations/vacuum.rs:210`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Override the source of time

<a id="op-aff44a3e8663e7feffa212b3"></a>
## commit_properties

`struct_field` · `deltalake_core::operations::vacuum::VacuumBuilder::commit_properties` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
commit_properties: kernel::transaction::CommitProperties
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/vacuum.rs#L212).

Source: `crates/core/src/operations/vacuum.rs:212`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Additional information to add to the commit

<a id="op-49755334d101e0a34f695f3f"></a>
## custom_execute_handler

`struct_field` · `deltalake_core::operations::vacuum::VacuumBuilder::custom_execute_handler` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
custom_execute_handler: Option<std::sync::Arc<dyn CustomExecuteHandler>>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/vacuum.rs#L213).

Source: `crates/core/src/operations/vacuum.rs:213`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bd9433d4851481e6f110d3d8"></a>
## dry_run

`struct_field` · `deltalake_core::operations::vacuum::VacuumBuilder::dry_run` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
dry_run: bool
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/vacuum.rs#L198).

Source: `crates/core/src/operations/vacuum.rs:198`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Don't delete the files. Just determine which files can be deleted

<a id="op-6bef882c483c47f3bfbbef35"></a>
## enforce_retention_duration

`struct_field` · `deltalake_core::operations::vacuum::VacuumBuilder::enforce_retention_duration` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
enforce_retention_duration: bool
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/vacuum.rs#L194).

Source: `crates/core/src/operations/vacuum.rs:194`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Validate the retention period is not below the retention period configured in the table

<a id="op-ba91a4d423b4056f40e2b9db"></a>
## get_custom_execute_handler

`function` · `deltalake_core::operations::vacuum::VacuumBuilder::get_custom_execute_handler` · deltalake-core 1.0.0+58f07cd6

Access: **internal_trait**. Canonical source location is not automatically a valid import path.

```rust
fn get_custom_execute_handler(&self) -> Option<Arc<dyn CustomExecuteHandler>>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/vacuum.rs#L220).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::vacuum::VacuumBuilder", "path": "VacuumBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [216, 1], "end": [223, 2], "filename": "crates/core/src/operations/vacuum.rs"}, "trait": {"args": null, "id": "deltalake_core::operations::Operation", "path": "Operation"}, "trait_path": "deltalake_core::operations::Operation"}`

Source: `crates/core/src/operations/vacuum.rs:220`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-46dbf48af73b41b32349d4d4"></a>
## keep_versions

`struct_field` · `deltalake_core::operations::vacuum::VacuumBuilder::keep_versions` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
keep_versions: Option<Vec<kernel::Version>>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/vacuum.rs#L196).

Source: `crates/core/src/operations/vacuum.rs:196`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Keep files associated with particular versions

<a id="op-354e6595cb71bfbe55afbf68"></a>
## log_store

`function` · `deltalake_core::operations::vacuum::VacuumBuilder::log_store` · deltalake-core 1.0.0+58f07cd6

Access: **internal_trait**. Canonical source location is not automatically a valid import path.

```rust
fn log_store(&self) -> &LogStoreRef
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/vacuum.rs#L217).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::vacuum::VacuumBuilder", "path": "VacuumBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [216, 1], "end": [223, 2], "filename": "crates/core/src/operations/vacuum.rs"}, "trait": {"args": null, "id": "deltalake_core::operations::Operation", "path": "Operation"}, "trait_path": "deltalake_core::operations::Operation"}`

Source: `crates/core/src/operations/vacuum.rs:217`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5f97b92e86e7d28de99f1fb0"></a>
## log_store

`struct_field` · `deltalake_core::operations::vacuum::VacuumBuilder::log_store` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
log_store: logstore::LogStoreRef
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/vacuum.rs#L190).

Source: `crates/core/src/operations/vacuum.rs:190`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Delta object store for handling data files

<a id="op-1ddb8b3368384fab9b31405b"></a>
## mode

`struct_field` · `deltalake_core::operations::vacuum::VacuumBuilder::mode` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
mode: VacuumMode
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/vacuum.rs#L200).

Source: `crates/core/src/operations/vacuum.rs:200`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Mode of vacuum that should be run

<a id="op-d7cf2d86d9b53b49bbead040"></a>
## parallel_scan

`struct_field` · `deltalake_core::operations::vacuum::VacuumBuilder::parallel_scan` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
parallel_scan: bool
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/vacuum.rs#L208).

Source: `crates/core/src/operations/vacuum.rs:208`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

By default, true. If true, it will scan the files parallelizing
through prefix-expansion path, otherwise, if false it will use
flat `list(None)` full-mode scan.

<a id="op-952b4e4e4ff2f50f5c49f33b"></a>
## retention_period

`struct_field` · `deltalake_core::operations::vacuum::VacuumBuilder::retention_period` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
retention_period: Option<chrono::Duration>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/vacuum.rs#L192).

Source: `crates/core/src/operations/vacuum.rs:192`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Period of stale files allowed.

<a id="op-c114e8fe7ca89b373a1cc56a"></a>
## scan_concurrency

`struct_field` · `deltalake_core::operations::vacuum::VacuumBuilder::scan_concurrency` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
scan_concurrency: Option<usize>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/vacuum.rs#L204).

Source: `crates/core/src/operations/vacuum.rs:204`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Max concurrent object-store LIST operations during full-mode scan.

`None` uses [`default_vacuum_list_concurrency`] (env or built-in default).

Unresolved upstream links (retained, not inferred): ``default_vacuum_list_concurrency``.

<a id="op-daed28b1ae2ef85697b5fbcd"></a>
## snapshot

`struct_field` · `deltalake_core::operations::vacuum::VacuumBuilder::snapshot` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
snapshot: Option<kernel::EagerSnapshot>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/vacuum.rs#L188).

Source: `crates/core/src/operations/vacuum.rs:188`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

A snapshot of the to-be-vacuumed table's state
