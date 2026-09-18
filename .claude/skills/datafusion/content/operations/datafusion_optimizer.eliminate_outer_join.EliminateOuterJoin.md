# `datafusion_optimizer::eliminate_outer_join::EliminateOuterJoin`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_optimizer.eliminate_outer_join.EliminateOuterJoin.json).

<a id="op-c7b50fdf31b5ebcd77bf3f63"></a>
## EliminateOuterJoin

`struct` · `datafusion_optimizer::eliminate_outer_join::EliminateOuterJoin` · datafusion-optimizer 55.1.0

```rust
struct EliminateOuterJoin
```

Source: `src/eliminate_outer_join.rs:79`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

Attempt to simplify outer joins when filters make their null-padded
rows impossible to observe.

Outer joins are generally more expensive than inner joins and can block
predicate pushdown and other optimizations. When a filter above an outer
join removes every row the join would add for unmatched input rows, the
join can be changed to a cheaper join type.

For example:

```sql
SELECT ...
FROM a LEFT JOIN b ON ...
WHERE b.xx = 100
```

For unmatched rows from `a`, the LEFT JOIN would produce a row with
`b.xx` set to NULL. The predicate `b.xx = 100` does not pass for those
rows, so the query does not need the LEFT JOIN's null-padded output and
the join can be rewritten as an inner join.

The same reasoning can also simplify FULL joins to LEFT, RIGHT, or INNER
joins when filters remove the rows padded on one or both sides.

This rule looks for a filter above an outer join:

```text
Filter(predicate)
  Join(LEFT/RIGHT/FULL)
```

It also handles plan shapes where projection pruning has inserted one or
more Projection nodes between the filter and join:

```text
Filter(predicate over projection output)
  Projection(...)
    ...
      Join(LEFT/RIGHT/FULL)
```

In the projection case, the rule rewrites a copy of the predicate through
each Projection so it can analyze the predicate against the Join inputs.
The original filter predicate and Projection nodes are preserved when the
plan is rebuilt.

<a id="op-997eb32074cd11a9899c674e"></a>
## apply_order

`function` · `datafusion_optimizer::eliminate_outer_join::EliminateOuterJoin::apply_order` · datafusion-optimizer 55.1.0

```rust
fn apply_order(&self) -> Option<ApplyOrder>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_optimizer::eliminate_outer_join::EliminateOuterJoin", "path": "EliminateOuterJoin"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [89, 1], "end": [147, 2], "filename": "src/eliminate_outer_join.rs"}, "trait": {"args": null, "id": "datafusion_optimizer::optimizer::OptimizerRule", "path": "OptimizerRule"}, "trait_path": "datafusion_optimizer::optimizer::OptimizerRule"}`

Source: `src/eliminate_outer_join.rs:94`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-06aa6bd38d83eabe18c47639"></a>
## default

`function` · `datafusion_optimizer::eliminate_outer_join::EliminateOuterJoin::default` · datafusion-optimizer 55.1.0

```rust
fn default() -> EliminateOuterJoin
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_optimizer::eliminate_outer_join::EliminateOuterJoin", "path": "EliminateOuterJoin"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [78, 10], "end": [78, 17], "filename": "src/eliminate_outer_join.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/eliminate_outer_join.rs:78`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9fba7b1cf21c09730df7e207"></a>
## fmt

`function` · `datafusion_optimizer::eliminate_outer_join::EliminateOuterJoin::fmt` · datafusion-optimizer 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_optimizer::eliminate_outer_join::EliminateOuterJoin", "path": "EliminateOuterJoin"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [78, 19], "end": [78, 24], "filename": "src/eliminate_outer_join.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/eliminate_outer_join.rs:78`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-478cc2451e656049a92a21e9"></a>
## name

`function` · `datafusion_optimizer::eliminate_outer_join::EliminateOuterJoin::name` · datafusion-optimizer 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_optimizer::eliminate_outer_join::EliminateOuterJoin", "path": "EliminateOuterJoin"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [89, 1], "end": [147, 2], "filename": "src/eliminate_outer_join.rs"}, "trait": {"args": null, "id": "datafusion_optimizer::optimizer::OptimizerRule", "path": "OptimizerRule"}, "trait_path": "datafusion_optimizer::optimizer::OptimizerRule"}`

Source: `src/eliminate_outer_join.rs:90`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a1e399370b9583ebd967e061"></a>
## new

`function` · `datafusion_optimizer::eliminate_outer_join::EliminateOuterJoin::new` · datafusion-optimizer 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_optimizer::eliminate_outer_join::EliminateOuterJoin", "path": "EliminateOuterJoin"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [81, 1], "end": [86, 2], "filename": "src/eliminate_outer_join.rs"}, "trait": null, "trait_path": null}`

Source: `src/eliminate_outer_join.rs:83`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-aba634df1e23045c0e9261af"></a>
## rewrite

`function` · `datafusion_optimizer::eliminate_outer_join::EliminateOuterJoin::rewrite` · datafusion-optimizer 55.1.0

```rust
fn rewrite(&self, plan: LogicalPlan, _config: &dyn OptimizerConfig) -> Result<Transformed<LogicalPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_optimizer::eliminate_outer_join::EliminateOuterJoin", "path": "EliminateOuterJoin"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [89, 1], "end": [147, 2], "filename": "src/eliminate_outer_join.rs"}, "trait": {"args": null, "id": "datafusion_optimizer::optimizer::OptimizerRule", "path": "OptimizerRule"}, "trait_path": "datafusion_optimizer::optimizer::OptimizerRule"}`

Source: `src/eliminate_outer_join.rs:102`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-323429b1bd2fe3720771ab4b"></a>
## supports_rewrite

`function` · `datafusion_optimizer::eliminate_outer_join::EliminateOuterJoin::supports_rewrite` · datafusion-optimizer 55.1.0

```rust
fn supports_rewrite(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_optimizer::eliminate_outer_join::EliminateOuterJoin", "path": "EliminateOuterJoin"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [89, 1], "end": [147, 2], "filename": "src/eliminate_outer_join.rs"}, "trait": {"args": null, "id": "datafusion_optimizer::optimizer::OptimizerRule", "path": "OptimizerRule"}, "trait_path": "datafusion_optimizer::optimizer::OptimizerRule"}`

Source: `src/eliminate_outer_join.rs:98`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
