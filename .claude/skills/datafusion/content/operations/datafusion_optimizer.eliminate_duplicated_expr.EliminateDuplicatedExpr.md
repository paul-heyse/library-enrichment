# `datafusion_optimizer::eliminate_duplicated_expr::EliminateDuplicatedExpr`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_optimizer.eliminate_duplicated_expr.EliminateDuplicatedExpr.json).

<a id="op-208fa502350f0e25d8c505bb"></a>
## EliminateDuplicatedExpr

`struct` · `datafusion_optimizer::eliminate_duplicated_expr::EliminateDuplicatedExpr` · datafusion-optimizer 55.1.0

```rust
struct EliminateDuplicatedExpr
```

Source: `src/eliminate_duplicated_expr.rs:32`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

Optimization rule that eliminate duplicated expr.

<a id="op-769ee3b6c2f9ed5524207a76"></a>
## apply_order

`function` · `datafusion_optimizer::eliminate_duplicated_expr::EliminateDuplicatedExpr::apply_order` · datafusion-optimizer 55.1.0

```rust
fn apply_order(&self) -> Option<ApplyOrder>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_optimizer::eliminate_duplicated_expr::EliminateDuplicatedExpr", "path": "EliminateDuplicatedExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [53, 1], "end": [140, 2], "filename": "src/eliminate_duplicated_expr.rs"}, "trait": {"args": null, "id": "datafusion_optimizer::optimizer::OptimizerRule", "path": "OptimizerRule"}, "trait_path": "datafusion_optimizer::optimizer::OptimizerRule"}`

Source: `src/eliminate_duplicated_expr.rs:54`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0cc3116fd95e176a2f5c60b1"></a>
## default

`function` · `datafusion_optimizer::eliminate_duplicated_expr::EliminateDuplicatedExpr::default` · datafusion-optimizer 55.1.0

```rust
fn default() -> EliminateDuplicatedExpr
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_optimizer::eliminate_duplicated_expr::EliminateDuplicatedExpr", "path": "EliminateDuplicatedExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [31, 10], "end": [31, 17], "filename": "src/eliminate_duplicated_expr.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/eliminate_duplicated_expr.rs:31`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6ad0f5bc32198656f8f696f7"></a>
## fmt

`function` · `datafusion_optimizer::eliminate_duplicated_expr::EliminateDuplicatedExpr::fmt` · datafusion-optimizer 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_optimizer::eliminate_duplicated_expr::EliminateDuplicatedExpr", "path": "EliminateDuplicatedExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [31, 19], "end": [31, 24], "filename": "src/eliminate_duplicated_expr.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/eliminate_duplicated_expr.rs:31`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-746b6a9870a2d32719cbf6ea"></a>
## name

`function` · `datafusion_optimizer::eliminate_duplicated_expr::EliminateDuplicatedExpr::name` · datafusion-optimizer 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_optimizer::eliminate_duplicated_expr::EliminateDuplicatedExpr", "path": "EliminateDuplicatedExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [53, 1], "end": [140, 2], "filename": "src/eliminate_duplicated_expr.rs"}, "trait": {"args": null, "id": "datafusion_optimizer::optimizer::OptimizerRule", "path": "OptimizerRule"}, "trait_path": "datafusion_optimizer::optimizer::OptimizerRule"}`

Source: `src/eliminate_duplicated_expr.rs:137`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-188a12d62a7e9d894b38e932"></a>
## new

`function` · `datafusion_optimizer::eliminate_duplicated_expr::EliminateDuplicatedExpr::new` · datafusion-optimizer 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_optimizer::eliminate_duplicated_expr::EliminateDuplicatedExpr", "path": "EliminateDuplicatedExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [34, 1], "end": [39, 2], "filename": "src/eliminate_duplicated_expr.rs"}, "trait": null, "trait_path": null}`

Source: `src/eliminate_duplicated_expr.rs:36`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4b871de726b88f7f0c383a99"></a>
## rewrite

`function` · `datafusion_optimizer::eliminate_duplicated_expr::EliminateDuplicatedExpr::rewrite` · datafusion-optimizer 55.1.0

```rust
fn rewrite(&self, plan: LogicalPlan, _config: &dyn OptimizerConfig) -> Result<Transformed<LogicalPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_optimizer::eliminate_duplicated_expr::EliminateDuplicatedExpr", "path": "EliminateDuplicatedExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [53, 1], "end": [140, 2], "filename": "src/eliminate_duplicated_expr.rs"}, "trait": {"args": null, "id": "datafusion_optimizer::optimizer::OptimizerRule", "path": "OptimizerRule"}, "trait_path": "datafusion_optimizer::optimizer::OptimizerRule"}`

Source: `src/eliminate_duplicated_expr.rs:62`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3a420d70b4ff14c4cf2d0440"></a>
## supports_rewrite

`function` · `datafusion_optimizer::eliminate_duplicated_expr::EliminateDuplicatedExpr::supports_rewrite` · datafusion-optimizer 55.1.0

```rust
fn supports_rewrite(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_optimizer::eliminate_duplicated_expr::EliminateDuplicatedExpr", "path": "EliminateDuplicatedExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [53, 1], "end": [140, 2], "filename": "src/eliminate_duplicated_expr.rs"}, "trait": {"args": null, "id": "datafusion_optimizer::optimizer::OptimizerRule", "path": "OptimizerRule"}, "trait_path": "datafusion_optimizer::optimizer::OptimizerRule"}`

Source: `src/eliminate_duplicated_expr.rs:58`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
