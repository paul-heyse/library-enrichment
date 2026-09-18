# `datafusion_optimizer::common_subexpr_eliminate::CommonSubexprEliminate`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_optimizer.common_subexpr_eliminate.CommonSubexprEliminate.json).

<a id="op-5495f79277c60608747ce314"></a>
## CommonSubexprEliminate

`struct` · `datafusion_optimizer::common_subexpr_eliminate::CommonSubexprEliminate` · datafusion-optimizer 55.1.0

```rust
struct CommonSubexprEliminate
```

Source: `src/common_subexpr_eliminate.rs:70`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

Performs Common Sub-expression Elimination optimization.

This optimization improves query performance by computing expressions that
appear more than once and reusing those results rather than re-computing the
same value

Currently only common sub-expressions within a single `LogicalPlan` are
eliminated.

# Example

Given a projection that computes the same expensive expression
multiple times such as parsing as string as a date with `to_date` twice:

```text
ProjectionExec(expr=[extract (day from to_date(c1)), extract (year from to_date(c1))])
```

This optimization will rewrite the plan to compute the common expression once
using a new `ProjectionExec` and then rewrite the original expressions to
refer to that new column.

```text
ProjectionExec(exprs=[extract (day from new_col), extract (year from new_col)]) <-- reuse here
  ProjectionExec(exprs=[to_date(c1) as new_col]) <-- compute to_date once
```

<a id="op-97db183241fe7b0a3b05a254"></a>
## apply_order

`function` · `datafusion_optimizer::common_subexpr_eliminate::CommonSubexprEliminate::apply_order` · datafusion-optimizer 55.1.0

```rust
fn apply_order(&self) -> Option<ApplyOrder>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_optimizer::common_subexpr_eliminate::CommonSubexprEliminate", "path": "CommonSubexprEliminate"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [542, 1], "end": [612, 2], "filename": "src/common_subexpr_eliminate.rs"}, "trait": {"args": null, "id": "datafusion_optimizer::optimizer::OptimizerRule", "path": "OptimizerRule"}, "trait_path": "datafusion_optimizer::optimizer::OptimizerRule"}`

Source: `src/common_subexpr_eliminate.rs:547`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3ecdd666ea0376ad3ec020ed"></a>
## default

`function` · `datafusion_optimizer::common_subexpr_eliminate::CommonSubexprEliminate::default` · datafusion-optimizer 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_optimizer::common_subexpr_eliminate::CommonSubexprEliminate", "path": "CommonSubexprEliminate"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [771, 1], "end": [775, 2], "filename": "src/common_subexpr_eliminate.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/common_subexpr_eliminate.rs:772`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0c9d7d2eae7c6fb23a9c6d62"></a>
## fmt

`function` · `datafusion_optimizer::common_subexpr_eliminate::CommonSubexprEliminate::fmt` · datafusion-optimizer 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_optimizer::common_subexpr_eliminate::CommonSubexprEliminate", "path": "CommonSubexprEliminate"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [69, 10], "end": [69, 15], "filename": "src/common_subexpr_eliminate.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/common_subexpr_eliminate.rs:69`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1588e929ef1739ce8365005b"></a>
## name

`function` · `datafusion_optimizer::common_subexpr_eliminate::CommonSubexprEliminate::name` · datafusion-optimizer 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_optimizer::common_subexpr_eliminate::CommonSubexprEliminate", "path": "CommonSubexprEliminate"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [542, 1], "end": [612, 2], "filename": "src/common_subexpr_eliminate.rs"}, "trait": {"args": null, "id": "datafusion_optimizer::optimizer::OptimizerRule", "path": "OptimizerRule"}, "trait_path": "datafusion_optimizer::optimizer::OptimizerRule"}`

Source: `src/common_subexpr_eliminate.rs:609`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3a0727e7fdc2dfa0094cf17e"></a>
## new

`function` · `datafusion_optimizer::common_subexpr_eliminate::CommonSubexprEliminate::new` · datafusion-optimizer 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_optimizer::common_subexpr_eliminate::CommonSubexprEliminate", "path": "CommonSubexprEliminate"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [72, 1], "end": [489, 2], "filename": "src/common_subexpr_eliminate.rs"}, "trait": null, "trait_path": null}`

Source: `src/common_subexpr_eliminate.rs:73`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-973b818589a4b3d9df57cce9"></a>
## rewrite

`function` · `datafusion_optimizer::common_subexpr_eliminate::CommonSubexprEliminate::rewrite` · datafusion-optimizer 55.1.0

```rust
fn rewrite(&self, plan: LogicalPlan, config: &dyn OptimizerConfig) -> Result<Transformed<LogicalPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_optimizer::common_subexpr_eliminate::CommonSubexprEliminate", "path": "CommonSubexprEliminate"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [542, 1], "end": [612, 2], "filename": "src/common_subexpr_eliminate.rs"}, "trait": {"args": null, "id": "datafusion_optimizer::optimizer::OptimizerRule", "path": "OptimizerRule"}, "trait_path": "datafusion_optimizer::optimizer::OptimizerRule"}`

Source: `src/common_subexpr_eliminate.rs:554`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-24c2fe1d0bf51fd5b51008f9"></a>
## supports_rewrite

`function` · `datafusion_optimizer::common_subexpr_eliminate::CommonSubexprEliminate::supports_rewrite` · datafusion-optimizer 55.1.0

```rust
fn supports_rewrite(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_optimizer::common_subexpr_eliminate::CommonSubexprEliminate", "path": "CommonSubexprEliminate"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [542, 1], "end": [612, 2], "filename": "src/common_subexpr_eliminate.rs"}, "trait": {"args": null, "id": "datafusion_optimizer::optimizer::OptimizerRule", "path": "OptimizerRule"}, "trait_path": "datafusion_optimizer::optimizer::OptimizerRule"}`

Source: `src/common_subexpr_eliminate.rs:543`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
