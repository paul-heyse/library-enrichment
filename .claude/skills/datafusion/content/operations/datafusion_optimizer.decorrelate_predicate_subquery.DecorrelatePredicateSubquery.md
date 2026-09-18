# `datafusion_optimizer::decorrelate_predicate_subquery::DecorrelatePredicateSubquery`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_optimizer.decorrelate_predicate_subquery.DecorrelatePredicateSubquery.json).

<a id="op-bf2e5b87a797226bba08be61"></a>
## DecorrelatePredicateSubquery

`struct` · `datafusion_optimizer::decorrelate_predicate_subquery::DecorrelatePredicateSubquery` · datafusion-optimizer 55.1.0

```rust
struct DecorrelatePredicateSubquery
```

Source: `src/decorrelate_predicate_subquery.rs:47`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

Optimizer rule for rewriting predicate(IN/EXISTS) subquery to left semi/anti joins

<a id="op-4812b2116d8b36cc98a3d146"></a>
## apply_order

`function` · `datafusion_optimizer::decorrelate_predicate_subquery::DecorrelatePredicateSubquery::apply_order` · datafusion-optimizer 55.1.0

```rust
fn apply_order(&self) -> Option<ApplyOrder>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_optimizer::decorrelate_predicate_subquery::DecorrelatePredicateSubquery", "path": "DecorrelatePredicateSubquery"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [56, 1], "end": [136, 2], "filename": "src/decorrelate_predicate_subquery.rs"}, "trait": {"args": null, "id": "datafusion_optimizer::optimizer::OptimizerRule", "path": "OptimizerRule"}, "trait_path": "datafusion_optimizer::optimizer::OptimizerRule"}`

Source: `src/decorrelate_predicate_subquery.rs:133`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9f3afba436316cd2980a0a1c"></a>
## default

`function` · `datafusion_optimizer::decorrelate_predicate_subquery::DecorrelatePredicateSubquery::default` · datafusion-optimizer 55.1.0

```rust
fn default() -> DecorrelatePredicateSubquery
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_optimizer::decorrelate_predicate_subquery::DecorrelatePredicateSubquery", "path": "DecorrelatePredicateSubquery"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [46, 10], "end": [46, 17], "filename": "src/decorrelate_predicate_subquery.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/decorrelate_predicate_subquery.rs:46`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6c937e73348891e475d16581"></a>
## fmt

`function` · `datafusion_optimizer::decorrelate_predicate_subquery::DecorrelatePredicateSubquery::fmt` · datafusion-optimizer 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_optimizer::decorrelate_predicate_subquery::DecorrelatePredicateSubquery", "path": "DecorrelatePredicateSubquery"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [46, 19], "end": [46, 24], "filename": "src/decorrelate_predicate_subquery.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/decorrelate_predicate_subquery.rs:46`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-961f210897c80979e16525cd"></a>
## name

`function` · `datafusion_optimizer::decorrelate_predicate_subquery::DecorrelatePredicateSubquery::name` · datafusion-optimizer 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_optimizer::decorrelate_predicate_subquery::DecorrelatePredicateSubquery", "path": "DecorrelatePredicateSubquery"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [56, 1], "end": [136, 2], "filename": "src/decorrelate_predicate_subquery.rs"}, "trait": {"args": null, "id": "datafusion_optimizer::optimizer::OptimizerRule", "path": "OptimizerRule"}, "trait_path": "datafusion_optimizer::optimizer::OptimizerRule"}`

Source: `src/decorrelate_predicate_subquery.rs:129`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3541b11333960222313e816e"></a>
## new

`function` · `datafusion_optimizer::decorrelate_predicate_subquery::DecorrelatePredicateSubquery::new` · datafusion-optimizer 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_optimizer::decorrelate_predicate_subquery::DecorrelatePredicateSubquery", "path": "DecorrelatePredicateSubquery"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [49, 1], "end": [54, 2], "filename": "src/decorrelate_predicate_subquery.rs"}, "trait": null, "trait_path": null}`

Source: `src/decorrelate_predicate_subquery.rs:51`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8197707c2ffcc70c6347ced5"></a>
## rewrite

`function` · `datafusion_optimizer::decorrelate_predicate_subquery::DecorrelatePredicateSubquery::rewrite` · datafusion-optimizer 55.1.0

```rust
fn rewrite(&self, plan: LogicalPlan, config: &dyn OptimizerConfig) -> Result<Transformed<LogicalPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_optimizer::decorrelate_predicate_subquery::DecorrelatePredicateSubquery", "path": "DecorrelatePredicateSubquery"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [56, 1], "end": [136, 2], "filename": "src/decorrelate_predicate_subquery.rs"}, "trait": {"args": null, "id": "datafusion_optimizer::optimizer::OptimizerRule", "path": "OptimizerRule"}, "trait_path": "datafusion_optimizer::optimizer::OptimizerRule"}`

Source: `src/decorrelate_predicate_subquery.rs:61`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c9b8e5ca62df4697e7d0cb6c"></a>
## supports_rewrite

`function` · `datafusion_optimizer::decorrelate_predicate_subquery::DecorrelatePredicateSubquery::supports_rewrite` · datafusion-optimizer 55.1.0

```rust
fn supports_rewrite(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_optimizer::decorrelate_predicate_subquery::DecorrelatePredicateSubquery", "path": "DecorrelatePredicateSubquery"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [56, 1], "end": [136, 2], "filename": "src/decorrelate_predicate_subquery.rs"}, "trait": {"args": null, "id": "datafusion_optimizer::optimizer::OptimizerRule", "path": "OptimizerRule"}, "trait_path": "datafusion_optimizer::optimizer::OptimizerRule"}`

Source: `src/decorrelate_predicate_subquery.rs:57`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
