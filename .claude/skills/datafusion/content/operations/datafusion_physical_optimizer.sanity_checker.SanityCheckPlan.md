# `datafusion_physical_optimizer::sanity_checker::SanityCheckPlan`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_optimizer.sanity_checker.SanityCheckPlan.json).

<a id="op-02a18f59b304936d153b65a1"></a>
## SanityCheckPlan

`struct` · `datafusion_physical_optimizer::sanity_checker::SanityCheckPlan` · datafusion-physical-optimizer 55.1.0

```rust
struct SanityCheckPlan
```

Source: `src/sanity_checker.rs:51`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-optimizer/55.1.0/json).

The SanityCheckPlan rule rejects the following query plans:
1. Invalid plans containing nodes whose order and/or distribution requirements
   are not satisfied by their children.
2. Plans that use pipeline-breaking operators on infinite input(s),
   it is impossible to execute such queries (they will never generate output nor finish)

<a id="op-18c770ca6870a68d120e2313"></a>
## default

`function` · `datafusion_physical_optimizer::sanity_checker::SanityCheckPlan::default` · datafusion-physical-optimizer 55.1.0

```rust
fn default() -> SanityCheckPlan
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_optimizer::sanity_checker::SanityCheckPlan", "path": "SanityCheckPlan"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [50, 10], "end": [50, 17], "filename": "src/sanity_checker.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/sanity_checker.rs:50`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-790c25f48d27eb0c92fbda1b"></a>
## fmt

`function` · `datafusion_physical_optimizer::sanity_checker::SanityCheckPlan::fmt` · datafusion-physical-optimizer 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_optimizer::sanity_checker::SanityCheckPlan", "path": "SanityCheckPlan"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [50, 19], "end": [50, 24], "filename": "src/sanity_checker.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/sanity_checker.rs:50`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-21e468596e41eb3a4e27f30f"></a>
## name

`function` · `datafusion_physical_optimizer::sanity_checker::SanityCheckPlan::name` · datafusion-physical-optimizer 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_optimizer::sanity_checker::SanityCheckPlan", "path": "SanityCheckPlan"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [60, 1], "end": [77, 2], "filename": "src/sanity_checker.rs"}, "trait": {"args": null, "id": "datafusion_session::physical_optimizer::PhysicalOptimizerRule", "path": "PhysicalOptimizerRule"}, "trait_path": "datafusion_session::physical_optimizer::PhysicalOptimizerRule"}`

Source: `src/sanity_checker.rs:70`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-63ff44a10a22ad93ebeb76d7"></a>
## new

`function` · `datafusion_physical_optimizer::sanity_checker::SanityCheckPlan::new` · datafusion-physical-optimizer 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_optimizer::sanity_checker::SanityCheckPlan", "path": "SanityCheckPlan"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [53, 1], "end": [58, 2], "filename": "src/sanity_checker.rs"}, "trait": null, "trait_path": null}`

Source: `src/sanity_checker.rs:55`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-73b7a44a226fb390e4bc69d2"></a>
## optimize

`function` · `datafusion_physical_optimizer::sanity_checker::SanityCheckPlan::optimize` · datafusion-physical-optimizer 55.1.0

```rust
fn optimize(&self, plan: Arc<dyn ExecutionPlan>, config: &ConfigOptions) -> Result<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_optimizer::sanity_checker::SanityCheckPlan", "path": "SanityCheckPlan"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [60, 1], "end": [77, 2], "filename": "src/sanity_checker.rs"}, "trait": {"args": null, "id": "datafusion_session::physical_optimizer::PhysicalOptimizerRule", "path": "PhysicalOptimizerRule"}, "trait_path": "datafusion_session::physical_optimizer::PhysicalOptimizerRule"}`

Source: `src/sanity_checker.rs:61`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ac4c24e3ab3c43c9b9db47ce"></a>
## schema_check

`function` · `datafusion_physical_optimizer::sanity_checker::SanityCheckPlan::schema_check` · datafusion-physical-optimizer 55.1.0

```rust
fn schema_check(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_optimizer::sanity_checker::SanityCheckPlan", "path": "SanityCheckPlan"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [60, 1], "end": [77, 2], "filename": "src/sanity_checker.rs"}, "trait": {"args": null, "id": "datafusion_session::physical_optimizer::PhysicalOptimizerRule", "path": "PhysicalOptimizerRule"}, "trait_path": "datafusion_session::physical_optimizer::PhysicalOptimizerRule"}`

Source: `src/sanity_checker.rs:74`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
