# `deltalake_core::operations::merge::MergeBuilder`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.operations.merge.MergeBuilder.json).

<a id="op-178a56da87bb85e396423d0b"></a>
## MergeBuilder

`struct` · `deltalake_core::operations::merge::MergeBuilder` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct MergeBuilder
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/merge/mod.rs#L151).

Source: `crates/core/src/operations/merge/mod.rs:151`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Merge records into a Delta Table.

<a id="op-7cb2105c56e2b0ee1eb12030"></a>
## IntoFuture

`assoc_type` · `deltalake_core::operations::merge::MergeBuilder::IntoFuture` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
type IntoFuture = Pin<Box<dyn Future<Output = <MergeBuilder as IntoFuture>::Output> + Send>>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/merge/mod.rs#L1879).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::merge::MergeBuilder", "path": "MergeBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1877, 1], "end": [1944, 2], "filename": "crates/core/src/operations/merge/mod.rs"}, "trait": {"args": null, "id": "core::future::into_future::IntoFuture", "path": "IntoFuture"}, "trait_path": "core::future::into_future::IntoFuture"}`

Source: `crates/core/src/operations/merge/mod.rs:1879`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-246dbe6c66042a837a13c116"></a>
## Output

`assoc_type` · `deltalake_core::operations::merge::MergeBuilder::Output` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
type Output = Result<(DeltaTable, MergeMetrics), DeltaTableError>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/merge/mod.rs#L1878).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::merge::MergeBuilder", "path": "MergeBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1877, 1], "end": [1944, 2], "filename": "crates/core/src/operations/merge/mod.rs"}, "trait": {"args": null, "id": "core::future::into_future::IntoFuture", "path": "IntoFuture"}, "trait_path": "core::future::into_future::IntoFuture"}`

Source: `crates/core/src/operations/merge/mod.rs:1878`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-48e2baaa903296b6728d5b1a"></a>
## into_future

`function` · `deltalake_core::operations::merge::MergeBuilder::into_future` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn into_future(self) -> Self::IntoFuture
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/merge/mod.rs#L1881).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::merge::MergeBuilder", "path": "MergeBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1877, 1], "end": [1944, 2], "filename": "crates/core/src/operations/merge/mod.rs"}, "trait": {"args": null, "id": "core::future::into_future::IntoFuture", "path": "IntoFuture"}, "trait_path": "core::future::into_future::IntoFuture"}`

Source: `crates/core/src/operations/merge/mod.rs:1881`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8117bf6773ca7d3f6eee4205"></a>
## new

`function` · `deltalake_core::operations::merge::MergeBuilder::new` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn new<E: Into<Expression>>(log_store: LogStoreRef, snapshot: Option<EagerSnapshot>, predicate: E, source: DataFrame) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/merge/mod.rs#L198).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::merge::MergeBuilder", "path": "MergeBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [196, 1], "end": [476, 2], "filename": "crates/core/src/operations/merge/mod.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/operations/merge/mod.rs:198`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Create a new [`MergeBuilder`](../operations/deltalake_core.operations.merge.MergeBuilder.md#op-178a56da87bb85e396423d0b)

<a id="op-55a55bdc3317d18a78a15343"></a>
## when_matched_delete

`function` · `deltalake_core::operations::merge::MergeBuilder::when_matched_delete` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn when_matched_delete<F>(self, builder: F) -> DeltaResult<MergeBuilder> where F: FnOnce(DeleteBuilder) -> DeleteBuilder
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/merge/mod.rs#L285).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::merge::MergeBuilder", "path": "MergeBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [196, 1], "end": [476, 2], "filename": "crates/core/src/operations/merge/mod.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/operations/merge/mod.rs:285`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Delete a target record when it matches with a source record

Multiple match clauses can be specified and their predicates are
evaluated to determine if the corresponding operation are performed.
Only the first clause that results in a satisfy predicate is executed.
The order of match clauses matter.

#Example
```rust ignore
let table = open_table(Url::from_directory_path("/abs/path/to/table").unwrap())?;
let (table, metrics) = table
    .merge(source, col("target.id").eq(col("source.id")))
    .with_source_alias("source")
    .with_target_alias("target")
    .when_matched_delete(|delete| {
        delete.predicate(col("source.delete"))
    })?
    .await?
```

<a id="op-3b956eb522c0b07669f78099"></a>
## when_matched_update

`function` · `deltalake_core::operations::merge::MergeBuilder::when_matched_update` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn when_matched_update<F>(self, builder: F) -> DeltaResult<MergeBuilder> where F: FnOnce(UpdateBuilder) -> UpdateBuilder
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/merge/mod.rs#L255).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::merge::MergeBuilder", "path": "MergeBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [196, 1], "end": [476, 2], "filename": "crates/core/src/operations/merge/mod.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/operations/merge/mod.rs:255`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Update a target record when it matches with a source record

The update expressions can specify both source and target columns.

Multiple match clauses can be specified and their predicates are
evaluated to determine if the corresponding operation are performed.
Only the first clause that results in a satisfy predicate is executed.
The order of match clauses matter.

#Example
```rust ignore
let table = open_table(Url::from_directory_path("/abs/path/to/table").unwrap())?;
let (table, metrics) = table
    .merge(source, col("target.id").eq(col("source.id")))
    .with_source_alias("source")
    .with_target_alias("target")
    .when_matched_update(|update| {
        update
            .predicate(col("source.value").lt(lit(0)))
            .update("value", lit(0))
            .update("modified", col("source.modified"))
    })?
    .when_matched_update(|update| {
        update
            .update("value", col("source.value") + lit(1))
            .update("modified", col("source.modified"))
    })?
    .await?
```

<a id="op-c47b2f8a582c539f8251a343"></a>
## when_not_matched_by_source_delete

`function` · `deltalake_core::operations::merge::MergeBuilder::when_not_matched_by_source_delete` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn when_not_matched_by_source_delete<F>(self, builder: F) -> DeltaResult<MergeBuilder> where F: FnOnce(DeleteBuilder) -> DeleteBuilder
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/merge/mod.rs#L385).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::merge::MergeBuilder", "path": "MergeBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [196, 1], "end": [476, 2], "filename": "crates/core/src/operations/merge/mod.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/operations/merge/mod.rs:385`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Delete a target record when it does not match with a source record

Multiple source "not match" clauses can be specified and their predicates
are evaluated to determine if the corresponding operations are performed.
Only the first clause that results in a satisfy predicate is executed.
The order of source "not match" clauses matter.

#Example
```rust ignore
let table = open_table(Url::from_directory_path("/abs/path/to/table").unwrap())?;
let (table, metrics) = table
    .merge(source, col("target.id").eq(col("source.id")))
    .with_source_alias("source")
    .with_target_alias("target")
    .when_not_matched_by_source_delete(|delete| {
        delete
    })?
    .await?
```

<a id="op-67a34c63043c7d57ef779175"></a>
## when_not_matched_by_source_update

`function` · `deltalake_core::operations::merge::MergeBuilder::when_not_matched_by_source_update` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn when_not_matched_by_source_update<F>(self, builder: F) -> DeltaResult<MergeBuilder> where F: FnOnce(UpdateBuilder) -> UpdateBuilder
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/merge/mod.rs#L355).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::merge::MergeBuilder", "path": "MergeBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [196, 1], "end": [476, 2], "filename": "crates/core/src/operations/merge/mod.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/operations/merge/mod.rs:355`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Update a target record when it does not match with a
source record

The update expressions can specify only target columns.

Multiple source not match clauses can be specified and their predicates
are evaluated to determine if the corresponding operation are performed.
Only the first clause that results in a satisfy predicate is executed.
The order of source not match clauses matter.

#Example
```rust ignore
let table = open_table(Url::from_directory_path("/abs/path/to/table").unwrap())?;
let (table, metrics) = table
    .merge(source, col("target.id").eq(col("source.id")))
    .with_source_alias("source")
    .with_target_alias("target")
    .when_not_matched_by_source_update(|update| {
        update
            .update("active", lit(false))
            .update("to_dt", lit("2023-07-11"))
    })?
    .await?
```

<a id="op-5df71d61188e1549d37abb11"></a>
## when_not_matched_insert

`function` · `deltalake_core::operations::merge::MergeBuilder::when_not_matched_insert` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn when_not_matched_insert<F>(self, builder: F) -> DeltaResult<MergeBuilder> where F: FnOnce(InsertBuilder) -> InsertBuilder
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/merge/mod.rs#L321).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::merge::MergeBuilder", "path": "MergeBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [196, 1], "end": [476, 2], "filename": "crates/core/src/operations/merge/mod.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/operations/merge/mod.rs:321`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Insert a source record when it does not match with a target record

Multiple not match clauses can be specified and their predicates are
evaluated to determine if the corresponding operation are performed.
Only the first clause that results in a satisfy predicate is executed.
The order of not match clauses matter.

#Example
```rust ignore
let table = open_table(Url::from_directory_path("/abs/path/to/table").unwrap())?;
let (table, metrics) = table
    .merge(source, col("target.id").eq(col("source.id")))
    .with_source_alias("source")
    .with_target_alias("target")
    .when_not_matched_insert(|insert| {
        insert
            .set("id", col("source.id"))
            .set("value", col("source.value"))
            .set("modified", col("source.modified"))
    })?
    .await?
```

<a id="op-94f3a94e8429e8002f400438"></a>
## with_commit_properties

`function` · `deltalake_core::operations::merge::MergeBuilder::with_commit_properties` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_commit_properties(self, commit_properties: CommitProperties) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/merge/mod.rs#L439).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::merge::MergeBuilder", "path": "MergeBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [196, 1], "end": [476, 2], "filename": "crates/core/src/operations/merge/mod.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/operations/merge/mod.rs:439`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Additional metadata to be added to commit info

<a id="op-02d2a4061d1e97078ad09bed"></a>
## with_custom_execute_handler

`function` · `deltalake_core::operations::merge::MergeBuilder::with_custom_execute_handler` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_custom_execute_handler(self, handler: Arc<dyn CustomExecuteHandler>) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/merge/mod.rs#L472).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::merge::MergeBuilder", "path": "MergeBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [196, 1], "end": [476, 2], "filename": "crates/core/src/operations/merge/mod.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/operations/merge/mod.rs:472`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Set a custom execute handler, for pre and post execution

<a id="op-397ec7652a5deaea329b3cbf"></a>
## with_merge_schema

`function` · `deltalake_core::operations::merge::MergeBuilder::with_merge_schema` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_merge_schema(self, merge_schema: bool) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/merge/mod.rs#L411).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::merge::MergeBuilder", "path": "MergeBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [196, 1], "end": [476, 2], "filename": "crates/core/src/operations/merge/mod.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/operations/merge/mod.rs:411`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Add Schema Write Mode

<a id="op-1e68915a42890152686e34b6"></a>
## with_safe_cast

`function` · `deltalake_core::operations::merge::MergeBuilder::with_safe_cast` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_safe_cast(self, safe_cast: bool) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/merge/mod.rs#L460).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::merge::MergeBuilder", "path": "MergeBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [196, 1], "end": [476, 2], "filename": "crates/core/src/operations/merge/mod.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/operations/merge/mod.rs:460`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Specify whether MERGE uses safe casts when casting update and insert
expressions to the table's schema. When enabled, failed casts yield null
for target columns that allow null values. When disabled, failed casts
return an error. A failed safe cast into a column that does not allow
null values still fails the write constraint check.

Example (column's type is int):
Input               Output
123         ->      123
Test123     ->      null

<a id="op-a936b68d8b1ed9677331ae2f"></a>
## with_session_fallback_policy

`function` · `deltalake_core::operations::merge::MergeBuilder::with_session_fallback_policy` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_session_fallback_policy(self, policy: SessionFallbackPolicy) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/merge/mod.rs#L433).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::merge::MergeBuilder", "path": "MergeBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [196, 1], "end": [476, 2], "filename": "crates/core/src/operations/merge/mod.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/operations/merge/mod.rs:433`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Control how delta-rs resolves the provided session when it is not a concrete `SessionState`.

Defaults to `SessionFallbackPolicy::InternalDefaults` to preserve existing behavior.

<a id="op-7342330b6d7efcf14936a543"></a>
## with_session_state

`function` · `deltalake_core::operations::merge::MergeBuilder::with_session_state` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_session_state(self, state: Arc<dyn Session>) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/merge/mod.rs#L425).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::merge::MergeBuilder", "path": "MergeBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [196, 1], "end": [476, 2], "filename": "crates/core/src/operations/merge/mod.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/operations/merge/mod.rs:425`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Set the DataFusion session used for planning and execution.

The provided `state` should wrap a concrete `datafusion::execution::context::SessionState`.

If `state` is not a `SessionState`, the default policy is to log a warning and fall back to
internal defaults. To make this strict (error instead), set
`with_session_fallback_policy(SessionFallbackPolicy::RequireSessionState)`.

Example: `Arc::new(create_session().state())`.

<a id="op-37aa639372a82018e14b3546"></a>
## with_source_alias

`function` · `deltalake_core::operations::merge::MergeBuilder::with_source_alias` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_source_alias<S: ToString>(self, alias: S) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/merge/mod.rs#L400).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::merge::MergeBuilder", "path": "MergeBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [196, 1], "end": [476, 2], "filename": "crates/core/src/operations/merge/mod.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/operations/merge/mod.rs:400`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Rename columns in the source dataset to have a prefix of `alias`.`original column name`

<a id="op-bd19392bca2f51b9e5dfbe75"></a>
## with_streaming

`function` · `deltalake_core::operations::merge::MergeBuilder::with_streaming` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_streaming(self, streaming: bool) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/merge/mod.rs#L466).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::merge::MergeBuilder", "path": "MergeBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [196, 1], "end": [476, 2], "filename": "crates/core/src/operations/merge/mod.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/operations/merge/mod.rs:466`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Set streaming mode execution

<a id="op-10a9f1c5868928d1a732e3ec"></a>
## with_target_alias

`function` · `deltalake_core::operations::merge::MergeBuilder::with_target_alias` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_target_alias<S: ToString>(self, alias: S) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/merge/mod.rs#L406).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::merge::MergeBuilder", "path": "MergeBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [196, 1], "end": [476, 2], "filename": "crates/core/src/operations/merge/mod.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/operations/merge/mod.rs:406`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Rename columns in the target dataset to have a prefix of `alias`.`original column name`

<a id="op-45291697791d293d921bb4ac"></a>
## with_writer_properties

`function` · `deltalake_core::operations::merge::MergeBuilder::with_writer_properties` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_writer_properties(self, writer_properties: WriterProperties) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/merge/mod.rs#L445).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::merge::MergeBuilder", "path": "MergeBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [196, 1], "end": [476, 2], "filename": "crates/core/src/operations/merge/mod.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/operations/merge/mod.rs:445`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Writer properties passed to parquet writer for when files are rewritten

<a id="op-81571f9dfeab1bd00a439144"></a>
## commit_properties

`struct_field` · `deltalake_core::operations::merge::MergeBuilder::commit_properties` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
commit_properties: kernel::transaction::CommitProperties
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/merge/mod.rs#L180).

Source: `crates/core/src/operations/merge/mod.rs:180`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Additional information to add to the commit

<a id="op-e3deb2139a65bed2dcd7ac9a"></a>
## custom_execute_handler

`struct_field` · `deltalake_core::operations::merge::MergeBuilder::custom_execute_handler` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
custom_execute_handler: Option<std::sync::Arc<dyn CustomExecuteHandler>>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/merge/mod.rs#L184).

Source: `crates/core/src/operations/merge/mod.rs:184`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-83594adcf98f347fb961a047"></a>
## get_custom_execute_handler

`function` · `deltalake_core::operations::merge::MergeBuilder::get_custom_execute_handler` · deltalake-core 1.0.0+58f07cd6

Access: **internal_trait**. Canonical source location is not automatically a valid import path.

```rust
fn get_custom_execute_handler(&self) -> Option<Arc<dyn CustomExecuteHandler>>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/merge/mod.rs#L191).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::merge::MergeBuilder", "path": "MergeBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [187, 1], "end": [194, 2], "filename": "crates/core/src/operations/merge/mod.rs"}, "trait": {"args": null, "id": "deltalake_core::operations::Operation", "path": "Operation"}, "trait_path": "deltalake_core::operations::Operation"}`

Source: `crates/core/src/operations/merge/mod.rs:191`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3272ea6da97c19e195209b2b"></a>
## log_store

`struct_field` · `deltalake_core::operations::merge::MergeBuilder::log_store` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
log_store: logstore::LogStoreRef
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/merge/mod.rs#L173).

Source: `crates/core/src/operations/merge/mod.rs:173`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Delta object store for handling data files

<a id="op-5e7a3325680d147d39292aa9"></a>
## log_store

`function` · `deltalake_core::operations::merge::MergeBuilder::log_store` · deltalake-core 1.0.0+58f07cd6

Access: **internal_trait**. Canonical source location is not automatically a valid import path.

```rust
fn log_store(&self) -> &LogStoreRef
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/merge/mod.rs#L188).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::merge::MergeBuilder", "path": "MergeBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [187, 1], "end": [194, 2], "filename": "crates/core/src/operations/merge/mod.rs"}, "trait": {"args": null, "id": "deltalake_core::operations::Operation", "path": "Operation"}, "trait_path": "deltalake_core::operations::Operation"}`

Source: `crates/core/src/operations/merge/mod.rs:188`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cecc5344c899798ab12fb27c"></a>
## match_operations

`struct_field` · `deltalake_core::operations::merge::MergeBuilder::match_operations` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
match_operations: Vec<MergeOperationConfig>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/merge/mod.rs#L155).

Source: `crates/core/src/operations/merge/mod.rs:155`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Operations to perform when a source record and target record match

<a id="op-45ec08968f287435f74895e3"></a>
## merge_schema

`struct_field` · `deltalake_core::operations::merge::MergeBuilder::merge_schema` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
merge_schema: bool
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/merge/mod.rs#L171).

Source: `crates/core/src/operations/merge/mod.rs:171`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Enable merge schema evolution

<a id="op-bceddf0e3fa999d6632f7bf4"></a>
## not_match_operations

`struct_field` · `deltalake_core::operations::merge::MergeBuilder::not_match_operations` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
not_match_operations: Vec<MergeOperationConfig>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/merge/mod.rs#L157).

Source: `crates/core/src/operations/merge/mod.rs:157`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Operations to perform on source records when they do not pair with a target record

<a id="op-5e3d5991bd0c50b4c66cc947"></a>
## not_match_source_operations

`struct_field` · `deltalake_core::operations::merge::MergeBuilder::not_match_source_operations` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
not_match_source_operations: Vec<MergeOperationConfig>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/merge/mod.rs#L159).

Source: `crates/core/src/operations/merge/mod.rs:159`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Operations to perform on target records when they do not pair with a source record

<a id="op-36fab2a878b0ccd96568cf35"></a>
## predicate

`struct_field` · `deltalake_core::operations::merge::MergeBuilder::predicate` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
predicate: delta_datafusion::Expression
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/merge/mod.rs#L153).

Source: `crates/core/src/operations/merge/mod.rs:153`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

The join predicate

<a id="op-1b9d89db3671ad1ae10f3829"></a>
## safe_cast

`struct_field` · `deltalake_core::operations::merge::MergeBuilder::safe_cast` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
safe_cast: bool
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/merge/mod.rs#L183).

Source: `crates/core/src/operations/merge/mod.rs:183`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

safe_cast determines how data types that do not match the underlying table are handled
By default an error is returned

<a id="op-743db32d748df3ea8515cb3c"></a>
## session_fallback_policy

`struct_field` · `deltalake_core::operations::merge::MergeBuilder::session_fallback_policy` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
session_fallback_policy: delta_datafusion::SessionFallbackPolicy
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/merge/mod.rs#L176).

Source: `crates/core/src/operations/merge/mod.rs:176`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-34ca57c3f2e7ab6d1cfc4329"></a>
## snapshot

`struct_field` · `deltalake_core::operations::merge::MergeBuilder::snapshot` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
snapshot: Option<kernel::EagerSnapshot>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/merge/mod.rs#L165).

Source: `crates/core/src/operations/merge/mod.rs:165`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

A snapshot of the table's state. AKA the target table in the operation

<a id="op-e076ecce6f22592e5f82b9ba"></a>
## source

`struct_field` · `deltalake_core::operations::merge::MergeBuilder::source` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
source: datafusion::prelude::DataFrame
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/merge/mod.rs#L167).

Source: `crates/core/src/operations/merge/mod.rs:167`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

The source data

<a id="op-f18d2024efa2fe82bc1a1d09"></a>
## source_alias

`struct_field` · `deltalake_core::operations::merge::MergeBuilder::source_alias` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
source_alias: Option<String>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/merge/mod.rs#L161).

Source: `crates/core/src/operations/merge/mod.rs:161`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Prefix the source columns with a user provided prefix

<a id="op-c3f532bc829c613cd7c3b6eb"></a>
## state

`struct_field` · `deltalake_core::operations::merge::MergeBuilder::state` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
state: Option<std::sync::Arc<dyn Session>>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/merge/mod.rs#L175).

Source: `crates/core/src/operations/merge/mod.rs:175`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Datafusion session state relevant for executing the input plan

<a id="op-2fe93b100231a23169252959"></a>
## streaming

`struct_field` · `deltalake_core::operations::merge::MergeBuilder::streaming` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
streaming: bool
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/merge/mod.rs#L169).

Source: `crates/core/src/operations/merge/mod.rs:169`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Whether the source is a streaming source (if true, stats deducing to prune target is disabled)

<a id="op-46c43f47e235054ae1ae9681"></a>
## target_alias

`struct_field` · `deltalake_core::operations::merge::MergeBuilder::target_alias` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
target_alias: Option<String>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/merge/mod.rs#L163).

Source: `crates/core/src/operations/merge/mod.rs:163`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Prefix target columns with a user provided prefix

<a id="op-585a07af64fc1e98a77799b1"></a>
## writer_properties

`struct_field` · `deltalake_core::operations::merge::MergeBuilder::writer_properties` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
writer_properties: Option<parquet::file::properties::WriterProperties>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/merge/mod.rs#L178).

Source: `crates/core/src/operations/merge/mod.rs:178`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Properties passed to underlying parquet writer for when files are rewritten
