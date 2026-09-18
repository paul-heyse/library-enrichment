# `datafusion_physical_optimizer::topk_repartition::TopKRepartition`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_optimizer.topk_repartition.TopKRepartition.json).

<a id="op-ca9019e971b45a6946d64099"></a>
## TopKRepartition

`struct` · `datafusion_physical_optimizer::topk_repartition::TopKRepartition` · datafusion-physical-optimizer 55.1.0

```rust
struct TopKRepartition
```

Source: `src/topk_repartition.rs:66`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-optimizer/55.1.0/json).

A physical optimizer rule that pushes TopK (Sort with fetch) past
hash repartition when the partition key is a prefix of the sort key.

See module-level documentation for details.

<a id="op-6ee0e1c4f84faa632c03b4b3"></a>
## clone

`function` · `datafusion_physical_optimizer::topk_repartition::TopKRepartition::clone` · datafusion-physical-optimizer 55.1.0

```rust
fn clone(&self) -> TopKRepartition
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_optimizer::topk_repartition::TopKRepartition", "path": "TopKRepartition"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [65, 17], "end": [65, 22], "filename": "src/topk_repartition.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/topk_repartition.rs:65`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f24bb3b8daaa3c9dc74b46d4"></a>
## default

`function` · `datafusion_physical_optimizer::topk_repartition::TopKRepartition::default` · datafusion-physical-optimizer 55.1.0

```rust
fn default() -> TopKRepartition
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_optimizer::topk_repartition::TopKRepartition", "path": "TopKRepartition"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [65, 24], "end": [65, 31], "filename": "src/topk_repartition.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/topk_repartition.rs:65`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ac59eef37e1bf818e84116e2"></a>
## fmt

`function` · `datafusion_physical_optimizer::topk_repartition::TopKRepartition::fmt` · datafusion-physical-optimizer 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_optimizer::topk_repartition::TopKRepartition", "path": "TopKRepartition"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [65, 10], "end": [65, 15], "filename": "src/topk_repartition.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/topk_repartition.rs:65`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b162cc5e89ccce944e29389f"></a>
## name

`function` · `datafusion_physical_optimizer::topk_repartition::TopKRepartition::name` · datafusion-physical-optimizer 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_optimizer::topk_repartition::TopKRepartition", "path": "TopKRepartition"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [74, 1], "end": [178, 2], "filename": "src/topk_repartition.rs"}, "trait": {"args": null, "id": "datafusion_session::physical_optimizer::PhysicalOptimizerRule", "path": "PhysicalOptimizerRule"}, "trait_path": "datafusion_session::physical_optimizer::PhysicalOptimizerRule"}`

Source: `src/topk_repartition.rs:171`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c078ed293c78bef47c481cca"></a>
## new

`function` · `datafusion_physical_optimizer::topk_repartition::TopKRepartition::new` · datafusion-physical-optimizer 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_optimizer::topk_repartition::TopKRepartition", "path": "TopKRepartition"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [68, 1], "end": [72, 2], "filename": "src/topk_repartition.rs"}, "trait": null, "trait_path": null}`

Source: `src/topk_repartition.rs:69`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2fe015e79e571a7eb8b8b417"></a>
## optimize

`function` · `datafusion_physical_optimizer::topk_repartition::TopKRepartition::optimize` · datafusion-physical-optimizer 55.1.0

```rust
fn optimize(&self, plan: Arc<dyn ExecutionPlan>, config: &ConfigOptions) -> Result<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_optimizer::topk_repartition::TopKRepartition", "path": "TopKRepartition"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [74, 1], "end": [178, 2], "filename": "src/topk_repartition.rs"}, "trait": {"args": null, "id": "datafusion_session::physical_optimizer::PhysicalOptimizerRule", "path": "PhysicalOptimizerRule"}, "trait_path": "datafusion_session::physical_optimizer::PhysicalOptimizerRule"}`

Source: `src/topk_repartition.rs:76`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-922d2b4548839e143fa00983"></a>
## schema_check

`function` · `datafusion_physical_optimizer::topk_repartition::TopKRepartition::schema_check` · datafusion-physical-optimizer 55.1.0

```rust
fn schema_check(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_optimizer::topk_repartition::TopKRepartition", "path": "TopKRepartition"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [74, 1], "end": [178, 2], "filename": "src/topk_repartition.rs"}, "trait": {"args": null, "id": "datafusion_session::physical_optimizer::PhysicalOptimizerRule", "path": "PhysicalOptimizerRule"}, "trait_path": "datafusion_session::physical_optimizer::PhysicalOptimizerRule"}`

Source: `src/topk_repartition.rs:175`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
