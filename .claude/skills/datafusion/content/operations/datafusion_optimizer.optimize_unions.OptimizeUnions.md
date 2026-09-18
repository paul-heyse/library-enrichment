# `datafusion_optimizer::optimize_unions::OptimizeUnions`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_optimizer.optimize_unions.OptimizeUnions.json).

<a id="op-8dd052790e27654b621c7998"></a>
## OptimizeUnions

`struct` · `datafusion_optimizer::optimize_unions::OptimizeUnions` · datafusion-optimizer 55.1.0

```rust
struct OptimizeUnions
```

Source: `src/optimize_unions.rs:32`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

An optimization rule that
1. replaces nested unions with a single union.
2. removes unions with a single input.

<a id="op-6e0e4e2ad68cfb1477b0df95"></a>
## apply_order

`function` · `datafusion_optimizer::optimize_unions::OptimizeUnions::apply_order` · datafusion-optimizer 55.1.0

```rust
fn apply_order(&self) -> Option<ApplyOrder>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_optimizer::optimize_unions::OptimizeUnions", "path": "OptimizeUnions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [41, 1], "end": [100, 2], "filename": "src/optimize_unions.rs"}, "trait": {"args": null, "id": "datafusion_optimizer::optimizer::OptimizerRule", "path": "OptimizerRule"}, "trait_path": "datafusion_optimizer::optimizer::OptimizerRule"}`

Source: `src/optimize_unions.rs:46`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d5af58ba1c0211d91007c16e"></a>
## default

`function` · `datafusion_optimizer::optimize_unions::OptimizeUnions::default` · datafusion-optimizer 55.1.0

```rust
fn default() -> OptimizeUnions
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_optimizer::optimize_unions::OptimizeUnions", "path": "OptimizeUnions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [28, 10], "end": [28, 17], "filename": "src/optimize_unions.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/optimize_unions.rs:28`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-69fa17d2e6323092784149b4"></a>
## fmt

`function` · `datafusion_optimizer::optimize_unions::OptimizeUnions::fmt` · datafusion-optimizer 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_optimizer::optimize_unions::OptimizeUnions", "path": "OptimizeUnions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [28, 19], "end": [28, 24], "filename": "src/optimize_unions.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/optimize_unions.rs:28`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6845cced393e091c5e5a6223"></a>
## name

`function` · `datafusion_optimizer::optimize_unions::OptimizeUnions::name` · datafusion-optimizer 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_optimizer::optimize_unions::OptimizeUnions", "path": "OptimizeUnions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [41, 1], "end": [100, 2], "filename": "src/optimize_unions.rs"}, "trait": {"args": null, "id": "datafusion_optimizer::optimizer::OptimizerRule", "path": "OptimizerRule"}, "trait_path": "datafusion_optimizer::optimizer::OptimizerRule"}`

Source: `src/optimize_unions.rs:42`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-dfae2f0d18c0c0d7699a08de"></a>
## new

`function` · `datafusion_optimizer::optimize_unions::OptimizeUnions::new` · datafusion-optimizer 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_optimizer::optimize_unions::OptimizeUnions", "path": "OptimizeUnions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [34, 1], "end": [39, 2], "filename": "src/optimize_unions.rs"}, "trait": null, "trait_path": null}`

Source: `src/optimize_unions.rs:36`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8b638291ce4a46c0b021eaa9"></a>
## rewrite

`function` · `datafusion_optimizer::optimize_unions::OptimizeUnions::rewrite` · datafusion-optimizer 55.1.0

```rust
fn rewrite(&self, plan: LogicalPlan, _config: &dyn OptimizerConfig) -> Result<Transformed<LogicalPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_optimizer::optimize_unions::OptimizeUnions", "path": "OptimizeUnions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [41, 1], "end": [100, 2], "filename": "src/optimize_unions.rs"}, "trait": {"args": null, "id": "datafusion_optimizer::optimizer::OptimizerRule", "path": "OptimizerRule"}, "trait_path": "datafusion_optimizer::optimizer::OptimizerRule"}`

Source: `src/optimize_unions.rs:54`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c145b4f84265b2e11443329a"></a>
## supports_rewrite

`function` · `datafusion_optimizer::optimize_unions::OptimizeUnions::supports_rewrite` · datafusion-optimizer 55.1.0

```rust
fn supports_rewrite(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_optimizer::optimize_unions::OptimizeUnions", "path": "OptimizeUnions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [41, 1], "end": [100, 2], "filename": "src/optimize_unions.rs"}, "trait": {"args": null, "id": "datafusion_optimizer::optimizer::OptimizerRule", "path": "OptimizerRule"}, "trait_path": "datafusion_optimizer::optimizer::OptimizerRule"}`

Source: `src/optimize_unions.rs:50`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
