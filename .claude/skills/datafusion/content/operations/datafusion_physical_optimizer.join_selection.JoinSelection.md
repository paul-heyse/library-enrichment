# `datafusion_physical_optimizer::join_selection::JoinSelection`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_optimizer.join_selection.JoinSelection.json).

<a id="op-33eb2e7b16aaea008290f269"></a>
## JoinSelection

`struct` · `datafusion_physical_optimizer::join_selection::JoinSelection` · datafusion-physical-optimizer 55.1.0

```rust
struct JoinSelection
```

Source: `src/join_selection.rs:51`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-optimizer/55.1.0/json).

The [`JoinSelection`](../operations/datafusion_physical_optimizer.join_selection.JoinSelection.md#op-33eb2e7b16aaea008290f269) rule tries to modify a given plan so that it can
accommodate infinite sources and optimize joins in the plan according to
available statistical information, if there is any.

<a id="op-7554aac494043d14bd1bf1d0"></a>
## default

`function` · `datafusion_physical_optimizer::join_selection::JoinSelection::default` · datafusion-physical-optimizer 55.1.0

```rust
fn default() -> JoinSelection
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_optimizer::join_selection::JoinSelection", "path": "JoinSelection"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [50, 10], "end": [50, 17], "filename": "src/join_selection.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/join_selection.rs:50`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3699726c0bf0941a6ee16d58"></a>
## fmt

`function` · `datafusion_physical_optimizer::join_selection::JoinSelection::fmt` · datafusion-physical-optimizer 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_optimizer::join_selection::JoinSelection", "path": "JoinSelection"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [50, 19], "end": [50, 24], "filename": "src/join_selection.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/join_selection.rs:50`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f8ea4229d6ae29eb3d64f2ab"></a>
## name

`function` · `datafusion_physical_optimizer::join_selection::JoinSelection::name` · datafusion-physical-optimizer 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_optimizer::join_selection::JoinSelection", "path": "JoinSelection"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [141, 1], "end": [187, 2], "filename": "src/join_selection.rs"}, "trait": {"args": null, "id": "datafusion_session::physical_optimizer::PhysicalOptimizerRule", "path": "PhysicalOptimizerRule"}, "trait_path": "datafusion_session::physical_optimizer::PhysicalOptimizerRule"}`

Source: `src/join_selection.rs:180`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-723cff14d6096e64d1a36bd3"></a>
## new

`function` · `datafusion_physical_optimizer::join_selection::JoinSelection::new` · datafusion-physical-optimizer 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_optimizer::join_selection::JoinSelection", "path": "JoinSelection"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [53, 1], "end": [58, 2], "filename": "src/join_selection.rs"}, "trait": null, "trait_path": null}`

Source: `src/join_selection.rs:55`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-aca7bb99c8522155bfc45589"></a>
## optimize

`function` · `datafusion_physical_optimizer::join_selection::JoinSelection::optimize` · datafusion-physical-optimizer 55.1.0

```rust
fn optimize(&self, plan: Arc<dyn ExecutionPlan>, config: &ConfigOptions) -> Result<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_optimizer::join_selection::JoinSelection", "path": "JoinSelection"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [141, 1], "end": [187, 2], "filename": "src/join_selection.rs"}, "trait": {"args": null, "id": "datafusion_session::physical_optimizer::PhysicalOptimizerRule", "path": "PhysicalOptimizerRule"}, "trait_path": "datafusion_session::physical_optimizer::PhysicalOptimizerRule"}`

Source: `src/join_selection.rs:142`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-74e8ab81c9759f7cbc3958da"></a>
## optimize_with_context

`function` · `datafusion_physical_optimizer::join_selection::JoinSelection::optimize_with_context` · datafusion-physical-optimizer 55.1.0

```rust
fn optimize_with_context(&self, plan: Arc<dyn ExecutionPlan>, context: &dyn PhysicalOptimizerContext) -> Result<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_optimizer::join_selection::JoinSelection", "path": "JoinSelection"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [141, 1], "end": [187, 2], "filename": "src/join_selection.rs"}, "trait": {"args": null, "id": "datafusion_session::physical_optimizer::PhysicalOptimizerRule", "path": "PhysicalOptimizerRule"}, "trait_path": "datafusion_session::physical_optimizer::PhysicalOptimizerRule"}`

Source: `src/join_selection.rs:150`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-29938004e31eeb5c4c946109"></a>
## schema_check

`function` · `datafusion_physical_optimizer::join_selection::JoinSelection::schema_check` · datafusion-physical-optimizer 55.1.0

```rust
fn schema_check(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_optimizer::join_selection::JoinSelection", "path": "JoinSelection"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [141, 1], "end": [187, 2], "filename": "src/join_selection.rs"}, "trait": {"args": null, "id": "datafusion_session::physical_optimizer::PhysicalOptimizerRule", "path": "PhysicalOptimizerRule"}, "trait_path": "datafusion_session::physical_optimizer::PhysicalOptimizerRule"}`

Source: `src/join_selection.rs:184`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
