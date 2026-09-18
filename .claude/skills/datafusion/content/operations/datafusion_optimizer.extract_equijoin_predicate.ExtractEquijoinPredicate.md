# `datafusion_optimizer::extract_equijoin_predicate::ExtractEquijoinPredicate`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_optimizer.extract_equijoin_predicate.ExtractEquijoinPredicate.json).

<a id="op-8f630378351e6f751de1efb7"></a>
## ExtractEquijoinPredicate

`struct` · `datafusion_optimizer::extract_equijoin_predicate::ExtractEquijoinPredicate` · datafusion-optimizer 55.1.0

```rust
struct ExtractEquijoinPredicate
```

Source: `src/extract_equijoin_predicate.rs:42`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

Optimizer that splits conjunctive join predicates into equijoin
predicates and (other) filter predicates.

Join algorithms are often highly optimized for equality predicates such as `x = y`,
often called `equijoin` predicates, so it is important to locate such predicates
and treat them specially.

For example, `SELECT ... FROM A JOIN B ON (A.x = B.y AND B.z > 50)`
has one equijoin predicate (`A.x = B.y`) and one filter predicate (`B.z > 50`).
See [find_valid_equijoin_key_pair](../operations/datafusion_expr.utils.find_valid_equijoin_key_pair.md#op-998a5a4863b0b64ce5e9c26d) for more information on what predicates
are considered equijoins.

<a id="op-65dcea4619776d5c0ba6a09e"></a>
## apply_order

`function` · `datafusion_optimizer::extract_equijoin_predicate::ExtractEquijoinPredicate::apply_order` · datafusion-optimizer 55.1.0

```rust
fn apply_order(&self) -> Option<ApplyOrder>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_optimizer::extract_equijoin_predicate::ExtractEquijoinPredicate", "path": "ExtractEquijoinPredicate"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [51, 1], "end": [156, 2], "filename": "src/extract_equijoin_predicate.rs"}, "trait": {"args": null, "id": "datafusion_optimizer::optimizer::OptimizerRule", "path": "OptimizerRule"}, "trait_path": "datafusion_optimizer::optimizer::OptimizerRule"}`

Source: `src/extract_equijoin_predicate.rs:60`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a1e1ae7bed63671ebdc0b99e"></a>
## default

`function` · `datafusion_optimizer::extract_equijoin_predicate::ExtractEquijoinPredicate::default` · datafusion-optimizer 55.1.0

```rust
fn default() -> ExtractEquijoinPredicate
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_optimizer::extract_equijoin_predicate::ExtractEquijoinPredicate", "path": "ExtractEquijoinPredicate"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [41, 10], "end": [41, 17], "filename": "src/extract_equijoin_predicate.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/extract_equijoin_predicate.rs:41`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c51a272756341fdd05557e9c"></a>
## fmt

`function` · `datafusion_optimizer::extract_equijoin_predicate::ExtractEquijoinPredicate::fmt` · datafusion-optimizer 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_optimizer::extract_equijoin_predicate::ExtractEquijoinPredicate", "path": "ExtractEquijoinPredicate"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [41, 19], "end": [41, 24], "filename": "src/extract_equijoin_predicate.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/extract_equijoin_predicate.rs:41`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-784805dc290c557e88fc8726"></a>
## name

`function` · `datafusion_optimizer::extract_equijoin_predicate::ExtractEquijoinPredicate::name` · datafusion-optimizer 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_optimizer::extract_equijoin_predicate::ExtractEquijoinPredicate", "path": "ExtractEquijoinPredicate"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [51, 1], "end": [156, 2], "filename": "src/extract_equijoin_predicate.rs"}, "trait": {"args": null, "id": "datafusion_optimizer::optimizer::OptimizerRule", "path": "OptimizerRule"}, "trait_path": "datafusion_optimizer::optimizer::OptimizerRule"}`

Source: `src/extract_equijoin_predicate.rs:56`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4856962be926b9389e66b352"></a>
## new

`function` · `datafusion_optimizer::extract_equijoin_predicate::ExtractEquijoinPredicate::new` · datafusion-optimizer 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_optimizer::extract_equijoin_predicate::ExtractEquijoinPredicate", "path": "ExtractEquijoinPredicate"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [44, 1], "end": [49, 2], "filename": "src/extract_equijoin_predicate.rs"}, "trait": null, "trait_path": null}`

Source: `src/extract_equijoin_predicate.rs:46`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-956b9beb4a0e93e553046324"></a>
## rewrite

`function` · `datafusion_optimizer::extract_equijoin_predicate::ExtractEquijoinPredicate::rewrite` · datafusion-optimizer 55.1.0

```rust
fn rewrite(&self, plan: LogicalPlan, _config: &dyn OptimizerConfig) -> Result<Transformed<LogicalPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_optimizer::extract_equijoin_predicate::ExtractEquijoinPredicate", "path": "ExtractEquijoinPredicate"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [51, 1], "end": [156, 2], "filename": "src/extract_equijoin_predicate.rs"}, "trait": {"args": null, "id": "datafusion_optimizer::optimizer::OptimizerRule", "path": "OptimizerRule"}, "trait_path": "datafusion_optimizer::optimizer::OptimizerRule"}`

Source: `src/extract_equijoin_predicate.rs:64`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-eee007fe3b3eb18d7c4ea0fe"></a>
## supports_rewrite

`function` · `datafusion_optimizer::extract_equijoin_predicate::ExtractEquijoinPredicate::supports_rewrite` · datafusion-optimizer 55.1.0

```rust
fn supports_rewrite(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_optimizer::extract_equijoin_predicate::ExtractEquijoinPredicate", "path": "ExtractEquijoinPredicate"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [51, 1], "end": [156, 2], "filename": "src/extract_equijoin_predicate.rs"}, "trait": {"args": null, "id": "datafusion_optimizer::optimizer::OptimizerRule", "path": "OptimizerRule"}, "trait_path": "datafusion_optimizer::optimizer::OptimizerRule"}`

Source: `src/extract_equijoin_predicate.rs:52`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
