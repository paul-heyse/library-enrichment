# `datafusion_optimizer::rewrite_set_comparison::RewriteSetComparison`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_optimizer.rewrite_set_comparison.RewriteSetComparison.json).

<a id="op-fb9e4646eb7c9f0708a8883c"></a>
## RewriteSetComparison

`struct` · `datafusion_optimizer::rewrite_set_comparison::RewriteSetComparison` · datafusion-optimizer 55.1.0

```rust
struct RewriteSetComparison
```

Source: `src/rewrite_set_comparison.rs:38`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

Rewrite `SetComparison` expressions to scalar subqueries that return the
correct boolean value (including SQL NULL semantics). After this rule
runs, later rules such as `ScalarSubqueryToJoin` can decorrelate and
remove the remaining subquery.

<a id="op-2b480aa672058e83d2f0c977"></a>
## default

`function` · `datafusion_optimizer::rewrite_set_comparison::RewriteSetComparison::default` · datafusion-optimizer 55.1.0

```rust
fn default() -> RewriteSetComparison
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_optimizer::rewrite_set_comparison::RewriteSetComparison", "path": "RewriteSetComparison"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [37, 17], "end": [37, 24], "filename": "src/rewrite_set_comparison.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/rewrite_set_comparison.rs:37`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6c83e50b0a24a829e0e01e9d"></a>
## fmt

`function` · `datafusion_optimizer::rewrite_set_comparison::RewriteSetComparison::fmt` · datafusion-optimizer 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_optimizer::rewrite_set_comparison::RewriteSetComparison", "path": "RewriteSetComparison"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [37, 10], "end": [37, 15], "filename": "src/rewrite_set_comparison.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/rewrite_set_comparison.rs:37`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a4c62f3b8a0a1506b244099e"></a>
## name

`function` · `datafusion_optimizer::rewrite_set_comparison::RewriteSetComparison::name` · datafusion-optimizer 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_optimizer::rewrite_set_comparison::RewriteSetComparison", "path": "RewriteSetComparison"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [66, 1], "end": [78, 2], "filename": "src/rewrite_set_comparison.rs"}, "trait": {"args": null, "id": "datafusion_optimizer::optimizer::OptimizerRule", "path": "OptimizerRule"}, "trait_path": "datafusion_optimizer::optimizer::OptimizerRule"}`

Source: `src/rewrite_set_comparison.rs:67`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-69a023c118c0a3ed1de5ef62"></a>
## new

`function` · `datafusion_optimizer::rewrite_set_comparison::RewriteSetComparison::new` · datafusion-optimizer 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_optimizer::rewrite_set_comparison::RewriteSetComparison", "path": "RewriteSetComparison"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [40, 1], "end": [64, 2], "filename": "src/rewrite_set_comparison.rs"}, "trait": null, "trait_path": null}`

Source: `src/rewrite_set_comparison.rs:42`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

Create a new `RewriteSetComparison` optimizer rule.

<a id="op-b36c2c422370a8d43acdbaf2"></a>
## rewrite

`function` · `datafusion_optimizer::rewrite_set_comparison::RewriteSetComparison::rewrite` · datafusion-optimizer 55.1.0

```rust
fn rewrite(&self, plan: LogicalPlan, _config: &dyn OptimizerConfig) -> Result<Transformed<LogicalPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_optimizer::rewrite_set_comparison::RewriteSetComparison", "path": "RewriteSetComparison"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [66, 1], "end": [78, 2], "filename": "src/rewrite_set_comparison.rs"}, "trait": {"args": null, "id": "datafusion_optimizer::optimizer::OptimizerRule", "path": "OptimizerRule"}, "trait_path": "datafusion_optimizer::optimizer::OptimizerRule"}`

Source: `src/rewrite_set_comparison.rs:71`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
