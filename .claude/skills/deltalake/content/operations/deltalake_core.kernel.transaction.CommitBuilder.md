# `deltalake_core::kernel::transaction::CommitBuilder`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.kernel.transaction.CommitBuilder.json).

<a id="op-8e35b7f8912ddf52f59854e0"></a>
## CommitBuilder

`struct` · `deltalake_core::kernel::transaction::CommitBuilder` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct CommitBuilder
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/transaction/mod.rs#L627).

Source: `crates/core/src/kernel/transaction/mod.rs:627`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Prepare data to be committed to the Delta log and control how the commit is performed

<a id="op-d53693381c75904ce089c98e"></a>
## build

`function` · `deltalake_core::kernel::transaction::CommitBuilder::build` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn build(self, table_data: Option<&'a dyn TableReference>, log_store: LogStoreRef, operation: DeltaOperation) -> PreCommit<'a>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/transaction/mod.rs#L692).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::transaction::CommitBuilder", "path": "CommitBuilder"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [651, 1], "end": [714, 2], "filename": "crates/core/src/kernel/transaction/mod.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/kernel/transaction/mod.rs:692`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Prepare a Commit operation using the configured builder

<a id="op-2d5eff7516f499dee10371d9"></a>
## default

`function` · `deltalake_core::kernel::transaction::CommitBuilder::default` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn default() -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/transaction/mod.rs#L638).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::transaction::CommitBuilder", "path": "CommitBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [637, 1], "end": [649, 2], "filename": "crates/core/src/kernel/transaction/mod.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `crates/core/src/kernel/transaction/mod.rs:638`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-97728fda63dced79e1e06d71"></a>
## from

`function` · `deltalake_core::kernel::transaction::CommitBuilder::from` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn from(value: CommitProperties) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/transaction/mod.rs#L612).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::transaction::CommitBuilder", "path": "CommitBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [611, 1], "end": [624, 2], "filename": "crates/core/src/kernel/transaction/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::transaction::CommitProperties", "path": "CommitProperties"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `crates/core/src/kernel/transaction/mod.rs:612`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-aaf5bae9a8c76a6a0a27dfa3"></a>
## with_actions

`function` · `deltalake_core::kernel::transaction::CommitBuilder::with_actions` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_actions(self, actions: Vec<Action>) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/transaction/mod.rs#L653).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::transaction::CommitBuilder", "path": "CommitBuilder"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [651, 1], "end": [714, 2], "filename": "crates/core/src/kernel/transaction/mod.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/kernel/transaction/mod.rs:653`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Actions to be included in the commit

<a id="op-35c53585ea181baa294c6b4b"></a>
## with_app_metadata

`function` · `deltalake_core::kernel::transaction::CommitBuilder::with_app_metadata` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_app_metadata(self, app_metadata: HashMap<String, Value>) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/transaction/mod.rs#L659).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::transaction::CommitBuilder", "path": "CommitBuilder"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [651, 1], "end": [714, 2], "filename": "crates/core/src/kernel/transaction/mod.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/kernel/transaction/mod.rs:659`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Metadata for the operation performed like metrics, user, and notebook

<a id="op-6aa30fd8b0d491cfd5844e53"></a>
## with_max_retries

`function` · `deltalake_core::kernel::transaction::CommitBuilder::with_max_retries` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_max_retries(self, max_retries: usize) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/transaction/mod.rs#L665).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::transaction::CommitBuilder", "path": "CommitBuilder"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [651, 1], "end": [714, 2], "filename": "crates/core/src/kernel/transaction/mod.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/kernel/transaction/mod.rs:665`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Maximum number of times to retry the transaction before failing to commit

<a id="op-6d9e4bef49d0a5538fedcd35"></a>
## with_operation_id

`function` · `deltalake_core::kernel::transaction::CommitBuilder::with_operation_id` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_operation_id(self, operation_id: Uuid) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/transaction/mod.rs#L677).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::transaction::CommitBuilder", "path": "CommitBuilder"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [651, 1], "end": [714, 2], "filename": "crates/core/src/kernel/transaction/mod.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/kernel/transaction/mod.rs:677`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Propagate operation id to log store

<a id="op-96d0511c177ce5952e79146e"></a>
## with_post_commit_hook

`function` · `deltalake_core::kernel::transaction::CommitBuilder::with_post_commit_hook` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_post_commit_hook(self, post_commit_hook: PostCommitHookProperties) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/transaction/mod.rs#L671).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::transaction::CommitBuilder", "path": "CommitBuilder"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [651, 1], "end": [714, 2], "filename": "crates/core/src/kernel/transaction/mod.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/kernel/transaction/mod.rs:671`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Specify all the post commit hook properties

<a id="op-28e42d4b611e91b603b61faa"></a>
## with_post_commit_hook_handler

`function` · `deltalake_core::kernel::transaction::CommitBuilder::with_post_commit_hook_handler` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_post_commit_hook_handler(self, handler: Option<Arc<dyn CustomExecuteHandler>>) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/transaction/mod.rs#L683).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::transaction::CommitBuilder", "path": "CommitBuilder"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [651, 1], "end": [714, 2], "filename": "crates/core/src/kernel/transaction/mod.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/kernel/transaction/mod.rs:683`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Set a custom execute handler, for pre and post execution

<a id="op-aa3583dde92a1551e17500a1"></a>
## actions

`struct_field` · `deltalake_core::kernel::transaction::CommitBuilder::actions` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
actions: Vec<kernel::Action>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/transaction/mod.rs#L628).

Source: `crates/core/src/kernel/transaction/mod.rs:628`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c7e4ce75b7fad3cb8120b51e"></a>
## app_metadata

`struct_field` · `deltalake_core::kernel::transaction::CommitBuilder::app_metadata` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
app_metadata: std::collections::HashMap<String, serde_json::Value>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/transaction/mod.rs#L629).

Source: `crates/core/src/kernel/transaction/mod.rs:629`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5546958466d708dbe7270406"></a>
## app_transaction

`struct_field` · `deltalake_core::kernel::transaction::CommitBuilder::app_transaction` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
app_transaction: Vec<kernel::Transaction>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/transaction/mod.rs#L630).

Source: `crates/core/src/kernel/transaction/mod.rs:630`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-94f94a17f163b7b23bf75de8"></a>
## max_retries

`struct_field` · `deltalake_core::kernel::transaction::CommitBuilder::max_retries` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
max_retries: usize
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/transaction/mod.rs#L631).

Source: `crates/core/src/kernel/transaction/mod.rs:631`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6f93ae717fc59e837c8a5b12"></a>
## operation_id

`struct_field` · `deltalake_core::kernel::transaction::CommitBuilder::operation_id` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
operation_id: uuid::Uuid
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/transaction/mod.rs#L634).

Source: `crates/core/src/kernel/transaction/mod.rs:634`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7be99c6c3f23574e17851bc9"></a>
## post_commit_hook

`struct_field` · `deltalake_core::kernel::transaction::CommitBuilder::post_commit_hook` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
post_commit_hook: Option<PostCommitHookProperties>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/transaction/mod.rs#L632).

Source: `crates/core/src/kernel/transaction/mod.rs:632`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8c7e0a0033d0404671cafc83"></a>
## post_commit_hook_handler

`struct_field` · `deltalake_core::kernel::transaction::CommitBuilder::post_commit_hook_handler` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
post_commit_hook_handler: Option<std::sync::Arc<dyn CustomExecuteHandler>>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/transaction/mod.rs#L633).

Source: `crates/core/src/kernel/transaction/mod.rs:633`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.
