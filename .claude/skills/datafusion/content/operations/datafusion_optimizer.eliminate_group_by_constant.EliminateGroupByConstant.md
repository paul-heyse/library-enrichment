# `datafusion_optimizer::eliminate_group_by_constant::EliminateGroupByConstant`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_optimizer.eliminate_group_by_constant.EliminateGroupByConstant.json).

<a id="op-1e76476c133c630bc89505aa"></a>
## EliminateGroupByConstant

`struct` · `datafusion_optimizer::eliminate_group_by_constant::EliminateGroupByConstant` · datafusion-optimizer 55.1.0

```rust
struct EliminateGroupByConstant
```

Source: `src/eliminate_group_by_constant.rs:33`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

Optimizer rule that removes constant expressions from `GROUP BY` clause
and places additional projection on top of aggregation, to preserve
original schema

<a id="op-a61adc84b2f7f58d18078c50"></a>
## apply_order

`function` · `datafusion_optimizer::eliminate_group_by_constant::EliminateGroupByConstant::apply_order` · datafusion-optimizer 55.1.0

```rust
fn apply_order(&self) -> Option<ApplyOrder>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_optimizer::eliminate_group_by_constant::EliminateGroupByConstant", "path": "EliminateGroupByConstant"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [41, 1], "end": [104, 2], "filename": "src/eliminate_group_by_constant.rs"}, "trait": {"args": null, "id": "datafusion_optimizer::optimizer::OptimizerRule", "path": "OptimizerRule"}, "trait_path": "datafusion_optimizer::optimizer::OptimizerRule"}`

Source: `src/eliminate_group_by_constant.rs:101`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-01014b54e22949917dd5beaf"></a>
## default

`function` · `datafusion_optimizer::eliminate_group_by_constant::EliminateGroupByConstant::default` · datafusion-optimizer 55.1.0

```rust
fn default() -> EliminateGroupByConstant
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_optimizer::eliminate_group_by_constant::EliminateGroupByConstant", "path": "EliminateGroupByConstant"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [32, 10], "end": [32, 17], "filename": "src/eliminate_group_by_constant.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/eliminate_group_by_constant.rs:32`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-661f70113a52cd139e2fb01e"></a>
## fmt

`function` · `datafusion_optimizer::eliminate_group_by_constant::EliminateGroupByConstant::fmt` · datafusion-optimizer 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_optimizer::eliminate_group_by_constant::EliminateGroupByConstant", "path": "EliminateGroupByConstant"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [32, 19], "end": [32, 24], "filename": "src/eliminate_group_by_constant.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/eliminate_group_by_constant.rs:32`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cb00b9618776c9cb8e682757"></a>
## name

`function` · `datafusion_optimizer::eliminate_group_by_constant::EliminateGroupByConstant::name` · datafusion-optimizer 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_optimizer::eliminate_group_by_constant::EliminateGroupByConstant", "path": "EliminateGroupByConstant"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [41, 1], "end": [104, 2], "filename": "src/eliminate_group_by_constant.rs"}, "trait": {"args": null, "id": "datafusion_optimizer::optimizer::OptimizerRule", "path": "OptimizerRule"}, "trait_path": "datafusion_optimizer::optimizer::OptimizerRule"}`

Source: `src/eliminate_group_by_constant.rs:97`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a7d3d997eb2bef456b8fcdfc"></a>
## new

`function` · `datafusion_optimizer::eliminate_group_by_constant::EliminateGroupByConstant::new` · datafusion-optimizer 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_optimizer::eliminate_group_by_constant::EliminateGroupByConstant", "path": "EliminateGroupByConstant"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [35, 1], "end": [39, 2], "filename": "src/eliminate_group_by_constant.rs"}, "trait": null, "trait_path": null}`

Source: `src/eliminate_group_by_constant.rs:36`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a569b81128e6bd7b3d0a5e41"></a>
## rewrite

`function` · `datafusion_optimizer::eliminate_group_by_constant::EliminateGroupByConstant::rewrite` · datafusion-optimizer 55.1.0

```rust
fn rewrite(&self, plan: LogicalPlan, _config: &dyn OptimizerConfig) -> Result<Transformed<LogicalPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_optimizer::eliminate_group_by_constant::EliminateGroupByConstant", "path": "EliminateGroupByConstant"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [41, 1], "end": [104, 2], "filename": "src/eliminate_group_by_constant.rs"}, "trait": {"args": null, "id": "datafusion_optimizer::optimizer::OptimizerRule", "path": "OptimizerRule"}, "trait_path": "datafusion_optimizer::optimizer::OptimizerRule"}`

Source: `src/eliminate_group_by_constant.rs:46`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-27c21fe62c44be5326dd3d41"></a>
## supports_rewrite

`function` · `datafusion_optimizer::eliminate_group_by_constant::EliminateGroupByConstant::supports_rewrite` · datafusion-optimizer 55.1.0

```rust
fn supports_rewrite(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_optimizer::eliminate_group_by_constant::EliminateGroupByConstant", "path": "EliminateGroupByConstant"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [41, 1], "end": [104, 2], "filename": "src/eliminate_group_by_constant.rs"}, "trait": {"args": null, "id": "datafusion_optimizer::optimizer::OptimizerRule", "path": "OptimizerRule"}, "trait_path": "datafusion_optimizer::optimizer::OptimizerRule"}`

Source: `src/eliminate_group_by_constant.rs:42`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
