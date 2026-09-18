# `datafusion_optimizer::eliminate_cross_join::EliminateCrossJoin`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_optimizer.eliminate_cross_join.EliminateCrossJoin.json).

<a id="op-595298539f8047ce374a99b2"></a>
## EliminateCrossJoin

`struct` · `datafusion_optimizer::eliminate_cross_join::EliminateCrossJoin` · datafusion-optimizer 55.1.0

```rust
struct EliminateCrossJoin
```

Source: `src/eliminate_cross_join.rs:33`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-95fd71365bfec3af9ad3d76d"></a>
## default

`function` · `datafusion_optimizer::eliminate_cross_join::EliminateCrossJoin::default` · datafusion-optimizer 55.1.0

```rust
fn default() -> EliminateCrossJoin
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_optimizer::eliminate_cross_join::EliminateCrossJoin", "path": "EliminateCrossJoin"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [32, 10], "end": [32, 17], "filename": "src/eliminate_cross_join.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/eliminate_cross_join.rs:32`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-24ea0913c6d55097cd26b93b"></a>
## fmt

`function` · `datafusion_optimizer::eliminate_cross_join::EliminateCrossJoin::fmt` · datafusion-optimizer 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_optimizer::eliminate_cross_join::EliminateCrossJoin", "path": "EliminateCrossJoin"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [32, 19], "end": [32, 24], "filename": "src/eliminate_cross_join.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/eliminate_cross_join.rs:32`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-73c0c157c6753c99b9f5b1f6"></a>
## name

`function` · `datafusion_optimizer::eliminate_cross_join::EliminateCrossJoin::name` · datafusion-optimizer 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_optimizer::eliminate_cross_join::EliminateCrossJoin", "path": "EliminateCrossJoin"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [77, 1], "end": [219, 2], "filename": "src/eliminate_cross_join.rs"}, "trait": {"args": null, "id": "datafusion_optimizer::optimizer::OptimizerRule", "path": "OptimizerRule"}, "trait_path": "datafusion_optimizer::optimizer::OptimizerRule"}`

Source: `src/eliminate_cross_join.rs:216`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1ae953797104ba519173bb30"></a>
## new

`function` · `datafusion_optimizer::eliminate_cross_join::EliminateCrossJoin::new` · datafusion-optimizer 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_optimizer::eliminate_cross_join::EliminateCrossJoin", "path": "EliminateCrossJoin"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [35, 1], "end": [40, 2], "filename": "src/eliminate_cross_join.rs"}, "trait": null, "trait_path": null}`

Source: `src/eliminate_cross_join.rs:37`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-afc1fdfd3d767030a219dbe9"></a>
## rewrite

`function` · `datafusion_optimizer::eliminate_cross_join::EliminateCrossJoin::rewrite` · datafusion-optimizer 55.1.0

```rust
fn rewrite(&self, plan: LogicalPlan, config: &dyn OptimizerConfig) -> Result<Transformed<LogicalPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_optimizer::eliminate_cross_join::EliminateCrossJoin", "path": "EliminateCrossJoin"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [77, 1], "end": [219, 2], "filename": "src/eliminate_cross_join.rs"}, "trait": {"args": null, "id": "datafusion_optimizer::optimizer::OptimizerRule", "path": "OptimizerRule"}, "trait_path": "datafusion_optimizer::optimizer::OptimizerRule"}`

Source: `src/eliminate_cross_join.rs:82`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5cefd7f12fbd916d13584a3f"></a>
## supports_rewrite

`function` · `datafusion_optimizer::eliminate_cross_join::EliminateCrossJoin::supports_rewrite` · datafusion-optimizer 55.1.0

```rust
fn supports_rewrite(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_optimizer::eliminate_cross_join::EliminateCrossJoin", "path": "EliminateCrossJoin"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [77, 1], "end": [219, 2], "filename": "src/eliminate_cross_join.rs"}, "trait": {"args": null, "id": "datafusion_optimizer::optimizer::OptimizerRule", "path": "OptimizerRule"}, "trait_path": "datafusion_optimizer::optimizer::OptimizerRule"}`

Source: `src/eliminate_cross_join.rs:78`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
