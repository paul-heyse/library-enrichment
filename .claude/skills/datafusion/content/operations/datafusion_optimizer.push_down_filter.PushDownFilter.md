# `datafusion_optimizer::push_down_filter::PushDownFilter`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_optimizer.push_down_filter.PushDownFilter.json).

<a id="op-d159bfe46e1b0f3280277ada"></a>
## PushDownFilter

`struct` · `datafusion_optimizer::push_down_filter::PushDownFilter` · datafusion-optimizer 55.1.0

```rust
struct PushDownFilter
```

Source: `src/push_down_filter.rs:143`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

Optimizer rule for pushing (moving) filter expressions down in a plan so
they are applied as early as possible.

# Introduction

The goal of this rule is to improve query performance by eliminating
redundant work.

For example, given a plan that sorts all values where `a > 10`:

```text
 Filter (a > 10)
   Sort (a, b)
```

A better plan is to filter the data *before* the Sort, which sorts fewer
rows and therefore does less work overall:

```text
 Sort (a, b)
   Filter (a > 10)  <-- Filter is moved before the sort
```

However it is not always possible to push filters down. For example, given a
plan that finds the top 3 values and then keeps only those that are greater
than 10, if the filter is pushed below the limit it would produce a
different result.

```text
 Filter (a > 10)   <-- cannot move this Filter before the limit
   Limit (fetch=3)
     Sort (a, b)
```


More formally, a filter-commutative operation is an operation `op` that
satisfies `filter(op(data)) = op(filter(data))`.

The filter-commutative property is plan and column-specific. A filter on `a`
can be pushed through a `Aggregate(group_by = [a], agg=[sum(b)])`. However, a
filter on `sum(b)` cannot be pushed through the same aggregate.

# Handling Conjunctions

It is possible to only push down **part** of a filter expression if it is
connected with `AND`s (more formally if it is a "conjunction").

For example, given the following plan:

```text
Filter(a > 10 AND sum(b) < 5)
  Aggregate(group_by = [a], agg = [sum(b)])
```

The `a > 10` is commutative with the `Aggregate` but `sum(b) < 5` is not.
Therefore it is possible to only push down part of the expression, resulting in:

```text
Filter(sum(b) < 5)
  Aggregate(group_by = [a], agg = [sum(b)])
    Filter(a > 10)
```

# Handling Column Aliases

This optimizer must sometimes handle rewriting filter expressions when they are
pushed. For example, consider a projection that aliases `a+1` to `"b"`:

```text
Filter (b > 10)
    Projection: [a+1 AS "b"]  <-- changes the name of `a+1` to `b`
```

To push this filter below the `Projection`, all references to `b` must be
rewritten to `a+1`:

```text
Projection: [a+1 AS "b"]
    Filter: (a+1 > 10)  <--- changed from b to a+1
```
# Implementation Notes

This implementation performs a single pass through the plan, "pushing" down
filters. When it passes through a filter, it stores that filter, and when it
reaches a plan node that does not commute with that filter, it adds the
filter to that place. When it passes through a projection, it re-writes the
filter's expression taking into account that projection.

<a id="op-e3b64bcb06cea37381198b98"></a>
## apply_order

`function` · `datafusion_optimizer::push_down_filter::PushDownFilter::apply_order` · datafusion-optimizer 55.1.0

```rust
fn apply_order(&self) -> Option<ApplyOrder>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_optimizer::push_down_filter::PushDownFilter", "path": "PushDownFilter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [772, 1], "end": [1266, 2], "filename": "src/push_down_filter.rs"}, "trait": {"args": null, "id": "datafusion_optimizer::optimizer::OptimizerRule", "path": "OptimizerRule"}, "trait_path": "datafusion_optimizer::optimizer::OptimizerRule"}`

Source: `src/push_down_filter.rs:777`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fe91147c2cae7bb1fb0982ba"></a>
## default

`function` · `datafusion_optimizer::push_down_filter::PushDownFilter::default` · datafusion-optimizer 55.1.0

```rust
fn default() -> PushDownFilter
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_optimizer::push_down_filter::PushDownFilter", "path": "PushDownFilter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [142, 10], "end": [142, 17], "filename": "src/push_down_filter.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/push_down_filter.rs:142`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-318efdfff080bab5fd656f50"></a>
## fmt

`function` · `datafusion_optimizer::push_down_filter::PushDownFilter::fmt` · datafusion-optimizer 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_optimizer::push_down_filter::PushDownFilter", "path": "PushDownFilter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [142, 19], "end": [142, 24], "filename": "src/push_down_filter.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/push_down_filter.rs:142`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4a2f676179b458bf22ab0a45"></a>
## name

`function` · `datafusion_optimizer::push_down_filter::PushDownFilter::name` · datafusion-optimizer 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_optimizer::push_down_filter::PushDownFilter", "path": "PushDownFilter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [772, 1], "end": [1266, 2], "filename": "src/push_down_filter.rs"}, "trait": {"args": null, "id": "datafusion_optimizer::optimizer::OptimizerRule", "path": "OptimizerRule"}, "trait_path": "datafusion_optimizer::optimizer::OptimizerRule"}`

Source: `src/push_down_filter.rs:773`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0304b3831562142c960feeca"></a>
## new

`function` · `datafusion_optimizer::push_down_filter::PushDownFilter::new` · datafusion-optimizer 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_optimizer::push_down_filter::PushDownFilter", "path": "PushDownFilter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1351, 1], "end": [1356, 2], "filename": "src/push_down_filter.rs"}, "trait": null, "trait_path": null}`

Source: `src/push_down_filter.rs:1353`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a9a961cad9aa054f314effc0"></a>
## rewrite

`function` · `datafusion_optimizer::push_down_filter::PushDownFilter::rewrite` · datafusion-optimizer 55.1.0

```rust
fn rewrite(&self, plan: LogicalPlan, config: &dyn OptimizerConfig) -> Result<Transformed<LogicalPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_optimizer::push_down_filter::PushDownFilter", "path": "PushDownFilter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [772, 1], "end": [1266, 2], "filename": "src/push_down_filter.rs"}, "trait": {"args": null, "id": "datafusion_optimizer::optimizer::OptimizerRule", "path": "OptimizerRule"}, "trait_path": "datafusion_optimizer::optimizer::OptimizerRule"}`

Source: `src/push_down_filter.rs:785`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0ec1c77ccfe8405dc12b0156"></a>
## supports_rewrite

`function` · `datafusion_optimizer::push_down_filter::PushDownFilter::supports_rewrite` · datafusion-optimizer 55.1.0

```rust
fn supports_rewrite(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_optimizer::push_down_filter::PushDownFilter", "path": "PushDownFilter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [772, 1], "end": [1266, 2], "filename": "src/push_down_filter.rs"}, "trait": {"args": null, "id": "datafusion_optimizer::optimizer::OptimizerRule", "path": "OptimizerRule"}, "trait_path": "datafusion_optimizer::optimizer::OptimizerRule"}`

Source: `src/push_down_filter.rs:781`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
