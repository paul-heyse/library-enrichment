# `datafusion_optimizer::propagate_empty_relation::PropagateEmptyRelation`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_optimizer.propagate_empty_relation.PropagateEmptyRelation.json).

<a id="op-6b0973979afc49ec79f86d0f"></a>
## PropagateEmptyRelation

`struct` · `datafusion_optimizer::propagate_empty_relation::PropagateEmptyRelation` · datafusion-optimizer 55.1.0

```rust
struct PropagateEmptyRelation
```

Source: `src/propagate_empty_relation.rs:33`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

Optimization rule that bottom-up to eliminate plan by propagating empty_relation.

<a id="op-789c3b445d2519429f1122cc"></a>
## apply_order

`function` · `datafusion_optimizer::propagate_empty_relation::PropagateEmptyRelation::apply_order` · datafusion-optimizer 55.1.0

```rust
fn apply_order(&self) -> Option<ApplyOrder>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_optimizer::propagate_empty_relation::PropagateEmptyRelation", "path": "PropagateEmptyRelation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [42, 1], "end": [235, 2], "filename": "src/propagate_empty_relation.rs"}, "trait": {"args": null, "id": "datafusion_optimizer::optimizer::OptimizerRule", "path": "OptimizerRule"}, "trait_path": "datafusion_optimizer::optimizer::OptimizerRule"}`

Source: `src/propagate_empty_relation.rs:47`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-30d153115f49ea2954377dbd"></a>
## default

`function` · `datafusion_optimizer::propagate_empty_relation::PropagateEmptyRelation::default` · datafusion-optimizer 55.1.0

```rust
fn default() -> PropagateEmptyRelation
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_optimizer::propagate_empty_relation::PropagateEmptyRelation", "path": "PropagateEmptyRelation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [32, 10], "end": [32, 17], "filename": "src/propagate_empty_relation.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/propagate_empty_relation.rs:32`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1d33be45cf562caa63f77ea9"></a>
## fmt

`function` · `datafusion_optimizer::propagate_empty_relation::PropagateEmptyRelation::fmt` · datafusion-optimizer 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_optimizer::propagate_empty_relation::PropagateEmptyRelation", "path": "PropagateEmptyRelation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [32, 19], "end": [32, 24], "filename": "src/propagate_empty_relation.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/propagate_empty_relation.rs:32`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-aa5f0f1864eba18a27b8a2b5"></a>
## name

`function` · `datafusion_optimizer::propagate_empty_relation::PropagateEmptyRelation::name` · datafusion-optimizer 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_optimizer::propagate_empty_relation::PropagateEmptyRelation", "path": "PropagateEmptyRelation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [42, 1], "end": [235, 2], "filename": "src/propagate_empty_relation.rs"}, "trait": {"args": null, "id": "datafusion_optimizer::optimizer::OptimizerRule", "path": "OptimizerRule"}, "trait_path": "datafusion_optimizer::optimizer::OptimizerRule"}`

Source: `src/propagate_empty_relation.rs:43`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ddae7f17adca8e1d8d56a143"></a>
## new

`function` · `datafusion_optimizer::propagate_empty_relation::PropagateEmptyRelation::new` · datafusion-optimizer 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_optimizer::propagate_empty_relation::PropagateEmptyRelation", "path": "PropagateEmptyRelation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [35, 1], "end": [40, 2], "filename": "src/propagate_empty_relation.rs"}, "trait": null, "trait_path": null}`

Source: `src/propagate_empty_relation.rs:37`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e29c9b9ab07b0fdae3bcfb6c"></a>
## rewrite

`function` · `datafusion_optimizer::propagate_empty_relation::PropagateEmptyRelation::rewrite` · datafusion-optimizer 55.1.0

```rust
fn rewrite(&self, plan: LogicalPlan, _config: &dyn OptimizerConfig) -> Result<Transformed<LogicalPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_optimizer::propagate_empty_relation::PropagateEmptyRelation", "path": "PropagateEmptyRelation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [42, 1], "end": [235, 2], "filename": "src/propagate_empty_relation.rs"}, "trait": {"args": null, "id": "datafusion_optimizer::optimizer::OptimizerRule", "path": "OptimizerRule"}, "trait_path": "datafusion_optimizer::optimizer::OptimizerRule"}`

Source: `src/propagate_empty_relation.rs:55`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1c067e96f56eee65d1af6cf8"></a>
## supports_rewrite

`function` · `datafusion_optimizer::propagate_empty_relation::PropagateEmptyRelation::supports_rewrite` · datafusion-optimizer 55.1.0

```rust
fn supports_rewrite(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_optimizer::propagate_empty_relation::PropagateEmptyRelation", "path": "PropagateEmptyRelation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [42, 1], "end": [235, 2], "filename": "src/propagate_empty_relation.rs"}, "trait": {"args": null, "id": "datafusion_optimizer::optimizer::OptimizerRule", "path": "OptimizerRule"}, "trait_path": "datafusion_optimizer::optimizer::OptimizerRule"}`

Source: `src/propagate_empty_relation.rs:51`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
