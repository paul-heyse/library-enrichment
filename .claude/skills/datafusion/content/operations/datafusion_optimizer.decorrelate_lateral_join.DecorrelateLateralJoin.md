# `datafusion_optimizer::decorrelate_lateral_join::DecorrelateLateralJoin`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_optimizer.decorrelate_lateral_join.DecorrelateLateralJoin.json).

<a id="op-037af0a490d91352c3c78d20"></a>
## DecorrelateLateralJoin

`struct` · `datafusion_optimizer::decorrelate_lateral_join::DecorrelateLateralJoin` · datafusion-optimizer 55.1.0

```rust
struct DecorrelateLateralJoin
```

Source: `src/decorrelate_lateral_join.rs:38`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

Optimizer rule for rewriting lateral joins to joins

<a id="op-53cc9941b5300a0b8dc37aae"></a>
## apply_order

`function` · `datafusion_optimizer::decorrelate_lateral_join::DecorrelateLateralJoin::apply_order` · datafusion-optimizer 55.1.0

```rust
fn apply_order(&self) -> Option<ApplyOrder>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_optimizer::decorrelate_lateral_join::DecorrelateLateralJoin", "path": "DecorrelateLateralJoin"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [47, 1], "end": [72, 2], "filename": "src/decorrelate_lateral_join.rs"}, "trait": {"args": null, "id": "datafusion_optimizer::optimizer::OptimizerRule", "path": "OptimizerRule"}, "trait_path": "datafusion_optimizer::optimizer::OptimizerRule"}`

Source: `src/decorrelate_lateral_join.rs:69`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-971682d9c666f10d7ee9bd75"></a>
## default

`function` · `datafusion_optimizer::decorrelate_lateral_join::DecorrelateLateralJoin::default` · datafusion-optimizer 55.1.0

```rust
fn default() -> DecorrelateLateralJoin
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_optimizer::decorrelate_lateral_join::DecorrelateLateralJoin", "path": "DecorrelateLateralJoin"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [37, 10], "end": [37, 17], "filename": "src/decorrelate_lateral_join.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/decorrelate_lateral_join.rs:37`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e32ebb6cc78ec4aef48ef53c"></a>
## fmt

`function` · `datafusion_optimizer::decorrelate_lateral_join::DecorrelateLateralJoin::fmt` · datafusion-optimizer 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_optimizer::decorrelate_lateral_join::DecorrelateLateralJoin", "path": "DecorrelateLateralJoin"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [37, 19], "end": [37, 24], "filename": "src/decorrelate_lateral_join.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/decorrelate_lateral_join.rs:37`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c14182cf906ab037b7d65704"></a>
## name

`function` · `datafusion_optimizer::decorrelate_lateral_join::DecorrelateLateralJoin::name` · datafusion-optimizer 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_optimizer::decorrelate_lateral_join::DecorrelateLateralJoin", "path": "DecorrelateLateralJoin"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [47, 1], "end": [72, 2], "filename": "src/decorrelate_lateral_join.rs"}, "trait": {"args": null, "id": "datafusion_optimizer::optimizer::OptimizerRule", "path": "OptimizerRule"}, "trait_path": "datafusion_optimizer::optimizer::OptimizerRule"}`

Source: `src/decorrelate_lateral_join.rs:65`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ebbecd29db49d98572a082d5"></a>
## new

`function` · `datafusion_optimizer::decorrelate_lateral_join::DecorrelateLateralJoin::new` · datafusion-optimizer 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_optimizer::decorrelate_lateral_join::DecorrelateLateralJoin", "path": "DecorrelateLateralJoin"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [40, 1], "end": [45, 2], "filename": "src/decorrelate_lateral_join.rs"}, "trait": null, "trait_path": null}`

Source: `src/decorrelate_lateral_join.rs:42`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0fab27525b1f8a10e2b5a14b"></a>
## rewrite

`function` · `datafusion_optimizer::decorrelate_lateral_join::DecorrelateLateralJoin::rewrite` · datafusion-optimizer 55.1.0

```rust
fn rewrite(&self, plan: LogicalPlan, _config: &dyn OptimizerConfig) -> Result<Transformed<LogicalPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_optimizer::decorrelate_lateral_join::DecorrelateLateralJoin", "path": "DecorrelateLateralJoin"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [47, 1], "end": [72, 2], "filename": "src/decorrelate_lateral_join.rs"}, "trait": {"args": null, "id": "datafusion_optimizer::optimizer::OptimizerRule", "path": "OptimizerRule"}, "trait_path": "datafusion_optimizer::optimizer::OptimizerRule"}`

Source: `src/decorrelate_lateral_join.rs:52`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e6079d9f0cfc39d5d183b96c"></a>
## supports_rewrite

`function` · `datafusion_optimizer::decorrelate_lateral_join::DecorrelateLateralJoin::supports_rewrite` · datafusion-optimizer 55.1.0

```rust
fn supports_rewrite(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_optimizer::decorrelate_lateral_join::DecorrelateLateralJoin", "path": "DecorrelateLateralJoin"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [47, 1], "end": [72, 2], "filename": "src/decorrelate_lateral_join.rs"}, "trait": {"args": null, "id": "datafusion_optimizer::optimizer::OptimizerRule", "path": "OptimizerRule"}, "trait_path": "datafusion_optimizer::optimizer::OptimizerRule"}`

Source: `src/decorrelate_lateral_join.rs:48`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
