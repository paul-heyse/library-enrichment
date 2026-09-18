# `datafusion_optimizer::optimizer::Optimizer`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_optimizer.optimizer.Optimizer.json).

<a id="op-111f177db94ba9d05d0d598a"></a>
## Optimizer

`struct` · `datafusion_optimizer::optimizer::Optimizer` · datafusion-optimizer 55.1.0

```rust
struct Optimizer
```

Source: `src/optimizer.rs:255`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

A rule-based optimizer.

<a id="op-671f299e3434c39a0ca54bd8"></a>
## clone

`function` · `datafusion_optimizer::optimizer::Optimizer::clone` · datafusion-optimizer 55.1.0

```rust
fn clone(&self) -> Optimizer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_optimizer::optimizer::Optimizer", "path": "Optimizer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [254, 10], "end": [254, 15], "filename": "src/optimizer.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/optimizer.rs:254`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c3618e5dd63e267cd08a8990"></a>
## default

`function` · `datafusion_optimizer::optimizer::Optimizer::default` · datafusion-optimizer 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_optimizer::optimizer::Optimizer", "path": "Optimizer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [272, 1], "end": [276, 2], "filename": "src/optimizer.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/optimizer.rs:273`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-eefffdd8d72c14f784ffa7a2"></a>
## fmt

`function` · `datafusion_optimizer::optimizer::Optimizer::fmt` · datafusion-optimizer 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_optimizer::optimizer::Optimizer", "path": "Optimizer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [254, 17], "end": [254, 22], "filename": "src/optimizer.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/optimizer.rs:254`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6d0867a66bcb85ce215cd49c"></a>
## new

`function` · `datafusion_optimizer::optimizer::Optimizer::new` · datafusion-optimizer 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_optimizer::optimizer::Optimizer", "path": "Optimizer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [278, 1], "end": [328, 2], "filename": "src/optimizer.rs"}, "trait": null, "trait_path": null}`

Source: `src/optimizer.rs:280`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

Create a new optimizer using the recommended list of rules

<a id="op-85c78c427d9e989a042bb82b"></a>
## optimize

`function` · `datafusion_optimizer::optimizer::Optimizer::optimize` · datafusion-optimizer 55.1.0

```rust
fn optimize<F>(&self, plan: LogicalPlan, config: &dyn OptimizerConfig, observer: F) -> Result<LogicalPlan> where F: FnMut(&LogicalPlan, &dyn OptimizerRule)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_optimizer::optimizer::Optimizer", "path": "Optimizer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [591, 1], "end": [765, 2], "filename": "src/optimizer.rs"}, "trait": null, "trait_path": null}`

Source: `src/optimizer.rs:594`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

Optimizes the logical plan by applying optimizer rules, and
invoking observer function after each call

<a id="op-203893e5193fad7a1de1eda9"></a>
## rules

`struct_field` · `datafusion_optimizer::optimizer::Optimizer::rules` · datafusion-optimizer 55.1.0

```rust
rules: Vec<std::sync::Arc<dyn OptimizerRule + Send + Sync>>
```

Source: `src/optimizer.rs:257`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

All optimizer rules to apply

<a id="op-c2956c07704c56b046b0051e"></a>
## with_rules

`function` · `datafusion_optimizer::optimizer::Optimizer::with_rules` · datafusion-optimizer 55.1.0

```rust
fn with_rules(rules: Vec<Arc<dyn OptimizerRule + Send + Sync>>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_optimizer::optimizer::Optimizer", "path": "Optimizer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [278, 1], "end": [328, 2], "filename": "src/optimizer.rs"}, "trait": null, "trait_path": null}`

Source: `src/optimizer.rs:325`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

Create a new optimizer with the given rules
