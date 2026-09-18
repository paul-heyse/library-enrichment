# `datafusion_optimizer::eliminate_join`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_optimizer.eliminate_join.json).

<a id="op-f595155605406f819968ad42"></a>
## eliminate_join

`module` · `datafusion_optimizer::eliminate_join` · datafusion-optimizer 55.1.0

```rust
mod eliminate_join
```

Source: `src/eliminate_join.rs:18`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

[`EliminateJoin`](../operations/datafusion_optimizer.eliminate_join.EliminateJoin.md#op-0b7e6d6f9696103143bd302f) rewrites joins to simpler forms to make them cheaper
to evaluate. We implement three distinct rewrites:

* An inner join can be rewritten to an empty relation if the join condition
  is trivially false.

* An inner join `L ⋈ R` can be rewritten to a left semi join `L ⋉ R`
  (`LeftSemi`), which keeps the rows of L that have a match in R and outputs
  only L's columns. The rewrite to `L ⋉ R` is valid when both of the
  following are true:

    1. None of R's columns are referenced above the join.
    2. R does not observably multiply L's rows. This holds when either the
       join's ancestors are duplicate-insensitive (e.g., DISTINCT) or we can use
       functional dependencies to prove that each L row matches at most one R
       row (R is provably unique on the join keys).

* A left outer join `L ⟕ R` can be removed entirely, i.e. replaced by `L`,
  under the same two conditions. Unlike an inner join, a left join
  preserves every row of L whether or not it has a match in R, so when R's
  columns are unused and R cannot multiply L's rows the join has no
  observable effect at all. Such joins commonly appear in generated SQL
  and in queries over views that join in lookup tables the query does not
  read. A join filter does not prevent this rewrite: for a left join it
  only decides whether a left row is matched or null-padded, and either
  way the row is emitted. Symmetrically, a right outer join `L ⟖ R` can be
  replaced by `R` when L's columns are unused and L cannot multiply R's
  rows.

# Overview

`rewrite_subtree` walks the plan top-down, threading two pieces of context
down to each join:

* `live` — which of the join's output columns are referenced above it. It is
  propagated top-down: each node asks its children only for the columns it
  needs from them, so a projection or aggregate asks for just the columns its
  expressions reference, dropping the rest (the narrowing); a join splits the
  set across its two inputs.
* `duplicate_insensitive` — whether emitting each row once instead of many
  times will not change the output. A duplicate-collapsing node (e.g.,
  DISTINCT, GROUP BY with no aggregate functions, or the existence side of a
  semi/anti/mark join) sets it `true` for its subtree, and it propagates
  downward until a node that makes the row count observable again (a `LIMIT`,
  a top-N sort, ...) clears it. It is therefore fixed by the nearest such
  node, not by the whole ancestor chain: a collapsing node shields its subtree,
  so a duplicate-sensitive node further above does not matter.

At each join, `rewritten_join_type` combines this context with the side's
functional dependencies to choose `Inner`, `LeftSemi`, or `RightSemi`, or
to eliminate the join entirely in favor of its preserved input. Most
node types just forward the context to their single child via
`rewrite_single_input`; nodes that alter column requirements or
duplicate-sensitivity (projection, aggregate, sort, ...) adjust it first.
