# `datafusion_optimizer::eliminate_join::EliminateJoin`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_optimizer.eliminate_join.EliminateJoin.json).

<a id="op-0b7e6d6f9696103143bd302f"></a>
## EliminateJoin

`struct` · `datafusion_optimizer::eliminate_join::EliminateJoin` · datafusion-optimizer 55.1.0

```rust
struct EliminateJoin
```

Source: `src/eliminate_join.rs:163`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

Rewrites an inner join to a semi join when one input only filters the
other, removes an outer join whose non-preserved side is unused and cannot
multiply the preserved side's rows, and replaces an always-false inner join
with an empty relation.

<a id="op-7e118f114ff7a9dd15064c5f"></a>
## default

`function` · `datafusion_optimizer::eliminate_join::EliminateJoin::default` · datafusion-optimizer 55.1.0

```rust
fn default() -> EliminateJoin
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_optimizer::eliminate_join::EliminateJoin", "path": "EliminateJoin"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [162, 10], "end": [162, 17], "filename": "src/eliminate_join.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/eliminate_join.rs:162`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8d6ad31a091eec5c96a36674"></a>
## fmt

`function` · `datafusion_optimizer::eliminate_join::EliminateJoin::fmt` · datafusion-optimizer 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_optimizer::eliminate_join::EliminateJoin", "path": "EliminateJoin"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [162, 19], "end": [162, 24], "filename": "src/eliminate_join.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/eliminate_join.rs:162`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-71f0e0e736dfb8ec1f03c0cf"></a>
## name

`function` · `datafusion_optimizer::eliminate_join::EliminateJoin::name` · datafusion-optimizer 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_optimizer::eliminate_join::EliminateJoin", "path": "EliminateJoin"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [171, 1], "end": [184, 2], "filename": "src/eliminate_join.rs"}, "trait": {"args": null, "id": "datafusion_optimizer::optimizer::OptimizerRule", "path": "OptimizerRule"}, "trait_path": "datafusion_optimizer::optimizer::OptimizerRule"}`

Source: `src/eliminate_join.rs:172`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-06e52346d02e1ecc4b758429"></a>
## new

`function` · `datafusion_optimizer::eliminate_join::EliminateJoin::new` · datafusion-optimizer 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_optimizer::eliminate_join::EliminateJoin", "path": "EliminateJoin"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [165, 1], "end": [169, 2], "filename": "src/eliminate_join.rs"}, "trait": null, "trait_path": null}`

Source: `src/eliminate_join.rs:166`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-50ec1eaf82d0f6e9fadc7d32"></a>
## rewrite

`function` · `datafusion_optimizer::eliminate_join::EliminateJoin::rewrite` · datafusion-optimizer 55.1.0

```rust
fn rewrite(&self, plan: LogicalPlan, _config: &dyn OptimizerConfig) -> Result<Transformed<LogicalPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_optimizer::eliminate_join::EliminateJoin", "path": "EliminateJoin"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [171, 1], "end": [184, 2], "filename": "src/eliminate_join.rs"}, "trait": {"args": null, "id": "datafusion_optimizer::optimizer::OptimizerRule", "path": "OptimizerRule"}, "trait_path": "datafusion_optimizer::optimizer::OptimizerRule"}`

Source: `src/eliminate_join.rs:176`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
