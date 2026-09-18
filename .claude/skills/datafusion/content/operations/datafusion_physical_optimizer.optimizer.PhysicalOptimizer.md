# `datafusion_physical_optimizer::optimizer::PhysicalOptimizer`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_optimizer.optimizer.PhysicalOptimizer.json).

<a id="op-3d1363a1dee00a89b1536157"></a>
## PhysicalOptimizer

`struct` · `datafusion_physical_optimizer::optimizer::PhysicalOptimizer` · datafusion-physical-optimizer 55.1.0

```rust
struct PhysicalOptimizer
```

Source: `src/optimizer.rs:71`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-optimizer/55.1.0/json).

A rule-based physical optimizer.

<a id="op-ac8cda3113ebc8d95ae49e92"></a>
## clone

`function` · `datafusion_physical_optimizer::optimizer::PhysicalOptimizer::clone` · datafusion-physical-optimizer 55.1.0

```rust
fn clone(&self) -> PhysicalOptimizer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_optimizer::optimizer::PhysicalOptimizer", "path": "PhysicalOptimizer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [70, 10], "end": [70, 15], "filename": "src/optimizer.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/optimizer.rs:70`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-48c777a22ff087b7f468dc7a"></a>
## default

`function` · `datafusion_physical_optimizer::optimizer::PhysicalOptimizer::default` · datafusion-physical-optimizer 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_optimizer::optimizer::PhysicalOptimizer", "path": "PhysicalOptimizer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [76, 1], "end": [80, 2], "filename": "src/optimizer.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/optimizer.rs:77`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-af90b47395df7b63f6df08b5"></a>
## fmt

`function` · `datafusion_physical_optimizer::optimizer::PhysicalOptimizer::fmt` · datafusion-physical-optimizer 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_optimizer::optimizer::PhysicalOptimizer", "path": "PhysicalOptimizer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [70, 17], "end": [70, 22], "filename": "src/optimizer.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/optimizer.rs:70`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bcb8fdc514a6eac2b91563e5"></a>
## new

`function` · `datafusion_physical_optimizer::optimizer::PhysicalOptimizer::new` · datafusion-physical-optimizer 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_optimizer::optimizer::PhysicalOptimizer", "path": "PhysicalOptimizer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [82, 1], "end": [200, 2], "filename": "src/optimizer.rs"}, "trait": null, "trait_path": null}`

Source: `src/optimizer.rs:84`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-optimizer/55.1.0/json).

Create a new optimizer using the recommended list of rules

<a id="op-4486e1bc8e25ea200e9382d7"></a>
## rules

`struct_field` · `datafusion_physical_optimizer::optimizer::PhysicalOptimizer::rules` · datafusion-physical-optimizer 55.1.0

```rust
rules: Vec<std::sync::Arc<dyn PhysicalOptimizerRule + Send + Sync>>
```

Source: `src/optimizer.rs:73`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-optimizer/55.1.0/json).

All rules to apply

<a id="op-b601b003afec363a4cf5d6e0"></a>
## with_rules

`function` · `datafusion_physical_optimizer::optimizer::PhysicalOptimizer::with_rules` · datafusion-physical-optimizer 55.1.0

```rust
fn with_rules(rules: Vec<Arc<dyn PhysicalOptimizerRule + Send + Sync>>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_optimizer::optimizer::PhysicalOptimizer", "path": "PhysicalOptimizer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [82, 1], "end": [200, 2], "filename": "src/optimizer.rs"}, "trait": null, "trait_path": null}`

Source: `src/optimizer.rs:197`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-optimizer/55.1.0/json).

Create a new optimizer with the given rules
