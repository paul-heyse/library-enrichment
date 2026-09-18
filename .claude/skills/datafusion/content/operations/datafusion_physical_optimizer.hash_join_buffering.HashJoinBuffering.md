# `datafusion_physical_optimizer::hash_join_buffering::HashJoinBuffering`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_optimizer.hash_join_buffering.HashJoinBuffering.json).

<a id="op-73b736a982d284d8cd5d6693"></a>
## HashJoinBuffering

`struct` · `datafusion_physical_optimizer::hash_join_buffering::HashJoinBuffering` · datafusion-physical-optimizer 55.1.0

```rust
struct HashJoinBuffering
```

Source: `src/hash_join_buffering.rs:48`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-optimizer/55.1.0/json).

Looks for all the [HashJoinExec](../operations/datafusion_physical_plan.joins.hash_join.exec.HashJoinExec.md#op-41aad8f48fa144bd1cf86810)s in the plan and places a [BufferExec](../operations/datafusion_physical_plan.buffer.BufferExec.md#op-dfabfc6e02e65fdcdbb664da) node with the
configured capacity in the probe side:

```text
           ┌───────────────────┐
           │   HashJoinExec    │
           └─────▲────────▲────┘
         ┌───────┘        └─────────┐
         │                          │
┌────────────────┐         ┌─────────────────┐
│   Build side   │       + │   BufferExec    │
└────────────────┘         └────────▲────────┘
                                    │
                           ┌────────┴────────┐
                           │   Probe side    │
                           └─────────────────┘
```

Which allows eagerly pulling it even before the build side has completely finished.

<a id="op-e09e976a920685f7143e38e6"></a>
## default

`function` · `datafusion_physical_optimizer::hash_join_buffering::HashJoinBuffering::default` · datafusion-physical-optimizer 55.1.0

```rust
fn default() -> HashJoinBuffering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_optimizer::hash_join_buffering::HashJoinBuffering", "path": "HashJoinBuffering"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [47, 17], "end": [47, 24], "filename": "src/hash_join_buffering.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/hash_join_buffering.rs:47`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c539876b7be5a632ec151fb5"></a>
## fmt

`function` · `datafusion_physical_optimizer::hash_join_buffering::HashJoinBuffering::fmt` · datafusion-physical-optimizer 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_optimizer::hash_join_buffering::HashJoinBuffering", "path": "HashJoinBuffering"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [47, 10], "end": [47, 15], "filename": "src/hash_join_buffering.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/hash_join_buffering.rs:47`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0be9bb87b5b93c81cc985281"></a>
## name

`function` · `datafusion_physical_optimizer::hash_join_buffering::HashJoinBuffering::name` · datafusion-physical-optimizer 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_optimizer::hash_join_buffering::HashJoinBuffering", "path": "HashJoinBuffering"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [56, 1], "end": [110, 2], "filename": "src/hash_join_buffering.rs"}, "trait": {"args": null, "id": "datafusion_session::physical_optimizer::PhysicalOptimizerRule", "path": "PhysicalOptimizerRule"}, "trait_path": "datafusion_session::physical_optimizer::PhysicalOptimizerRule"}`

Source: `src/hash_join_buffering.rs:103`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-eb78d48738df2120af870993"></a>
## new

`function` · `datafusion_physical_optimizer::hash_join_buffering::HashJoinBuffering::new` · datafusion-physical-optimizer 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_optimizer::hash_join_buffering::HashJoinBuffering", "path": "HashJoinBuffering"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [50, 1], "end": [54, 2], "filename": "src/hash_join_buffering.rs"}, "trait": null, "trait_path": null}`

Source: `src/hash_join_buffering.rs:51`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a4985cc70eb1b5957d7ca3f4"></a>
## optimize

`function` · `datafusion_physical_optimizer::hash_join_buffering::HashJoinBuffering::optimize` · datafusion-physical-optimizer 55.1.0

```rust
fn optimize(&self, plan: Arc<dyn ExecutionPlan>, config: &ConfigOptions) -> datafusion_common::Result<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_optimizer::hash_join_buffering::HashJoinBuffering", "path": "HashJoinBuffering"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [56, 1], "end": [110, 2], "filename": "src/hash_join_buffering.rs"}, "trait": {"args": null, "id": "datafusion_session::physical_optimizer::PhysicalOptimizerRule", "path": "PhysicalOptimizerRule"}, "trait_path": "datafusion_session::physical_optimizer::PhysicalOptimizerRule"}`

Source: `src/hash_join_buffering.rs:57`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6318f2053831362e786ff36b"></a>
## schema_check

`function` · `datafusion_physical_optimizer::hash_join_buffering::HashJoinBuffering::schema_check` · datafusion-physical-optimizer 55.1.0

```rust
fn schema_check(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_optimizer::hash_join_buffering::HashJoinBuffering", "path": "HashJoinBuffering"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [56, 1], "end": [110, 2], "filename": "src/hash_join_buffering.rs"}, "trait": {"args": null, "id": "datafusion_session::physical_optimizer::PhysicalOptimizerRule", "path": "PhysicalOptimizerRule"}, "trait_path": "datafusion_session::physical_optimizer::PhysicalOptimizerRule"}`

Source: `src/hash_join_buffering.rs:107`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
