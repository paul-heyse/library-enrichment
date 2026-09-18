# `datafusion_optimizer::push_down_limit::PushDownLimit`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_optimizer.push_down_limit.PushDownLimit.json).

<a id="op-819ff66890778fc0d2b7f39f"></a>
## PushDownLimit

`struct` · `datafusion_optimizer::push_down_limit::PushDownLimit` · datafusion-optimizer 55.1.0

```rust
struct PushDownLimit
```

Source: `src/push_down_limit.rs:35`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

Optimization rule that tries to push down `LIMIT`.

<a id="op-c377ac5af01845fa50d493f6"></a>
## apply_order

`function` · `datafusion_optimizer::push_down_limit::PushDownLimit::apply_order` · datafusion-optimizer 55.1.0

```rust
fn apply_order(&self) -> Option<ApplyOrder>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_optimizer::push_down_limit::PushDownLimit", "path": "PushDownLimit"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [45, 1], "end": [191, 2], "filename": "src/push_down_limit.rs"}, "trait": {"args": null, "id": "datafusion_optimizer::optimizer::OptimizerRule", "path": "OptimizerRule"}, "trait_path": "datafusion_optimizer::optimizer::OptimizerRule"}`

Source: `src/push_down_limit.rs:188`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7d57d43f75a264ff29388075"></a>
## default

`function` · `datafusion_optimizer::push_down_limit::PushDownLimit::default` · datafusion-optimizer 55.1.0

```rust
fn default() -> PushDownLimit
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_optimizer::push_down_limit::PushDownLimit", "path": "PushDownLimit"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [34, 10], "end": [34, 17], "filename": "src/push_down_limit.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/push_down_limit.rs:34`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-52c201385fd096dc82a4c719"></a>
## fmt

`function` · `datafusion_optimizer::push_down_limit::PushDownLimit::fmt` · datafusion-optimizer 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_optimizer::push_down_limit::PushDownLimit", "path": "PushDownLimit"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [34, 19], "end": [34, 24], "filename": "src/push_down_limit.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/push_down_limit.rs:34`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-519dd359807bd38b87158fd3"></a>
## name

`function` · `datafusion_optimizer::push_down_limit::PushDownLimit::name` · datafusion-optimizer 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_optimizer::push_down_limit::PushDownLimit", "path": "PushDownLimit"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [45, 1], "end": [191, 2], "filename": "src/push_down_limit.rs"}, "trait": {"args": null, "id": "datafusion_optimizer::optimizer::OptimizerRule", "path": "OptimizerRule"}, "trait_path": "datafusion_optimizer::optimizer::OptimizerRule"}`

Source: `src/push_down_limit.rs:184`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c834f2a61e3950ccd3e59e1c"></a>
## new

`function` · `datafusion_optimizer::push_down_limit::PushDownLimit::new` · datafusion-optimizer 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_optimizer::push_down_limit::PushDownLimit", "path": "PushDownLimit"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [37, 1], "end": [42, 2], "filename": "src/push_down_limit.rs"}, "trait": null, "trait_path": null}`

Source: `src/push_down_limit.rs:39`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-adbda4e09946b1e6a4be59a6"></a>
## rewrite

`function` · `datafusion_optimizer::push_down_limit::PushDownLimit::rewrite` · datafusion-optimizer 55.1.0

```rust
fn rewrite(&self, plan: LogicalPlan, config: &dyn OptimizerConfig) -> Result<Transformed<LogicalPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_optimizer::push_down_limit::PushDownLimit", "path": "PushDownLimit"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [45, 1], "end": [191, 2], "filename": "src/push_down_limit.rs"}, "trait": {"args": null, "id": "datafusion_optimizer::optimizer::OptimizerRule", "path": "OptimizerRule"}, "trait_path": "datafusion_optimizer::optimizer::OptimizerRule"}`

Source: `src/push_down_limit.rs:51`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-73054290a3f73cb23872ef2b"></a>
## supports_rewrite

`function` · `datafusion_optimizer::push_down_limit::PushDownLimit::supports_rewrite` · datafusion-optimizer 55.1.0

```rust
fn supports_rewrite(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_optimizer::push_down_limit::PushDownLimit", "path": "PushDownLimit"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [45, 1], "end": [191, 2], "filename": "src/push_down_limit.rs"}, "trait": {"args": null, "id": "datafusion_optimizer::optimizer::OptimizerRule", "path": "OptimizerRule"}, "trait_path": "datafusion_optimizer::optimizer::OptimizerRule"}`

Source: `src/push_down_limit.rs:46`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
