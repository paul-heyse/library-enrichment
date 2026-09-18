# `datafusion_expr::registry::MemoryFunctionRegistry`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.registry.MemoryFunctionRegistry.json).

<a id="op-79591a67ff6067dbf1fa4670"></a>
## MemoryFunctionRegistry

`struct` · `datafusion_expr::registry::MemoryFunctionRegistry` · datafusion-expr 55.1.0

```rust
struct MemoryFunctionRegistry
```

Source: `src/registry.rs:193`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

A  [`FunctionRegistry`](../operations/datafusion_expr.registry.FunctionRegistry.md#op-3a0de62f03e60cdeb963c7b3) that uses in memory [`HashMap`](../operations/datafusion_common.HashMap.md#op-12b499036bd6e05ac6fbeefd)s

<a id="op-038b4c0a6724004534c88ad5"></a>
## default

`function` · `datafusion_expr::registry::MemoryFunctionRegistry::default` · datafusion-expr 55.1.0

```rust
fn default() -> MemoryFunctionRegistry
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::registry::MemoryFunctionRegistry", "path": "MemoryFunctionRegistry"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [192, 10], "end": [192, 17], "filename": "src/registry.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/registry.rs:192`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fd9c2da4c7e5e729631d3cc5"></a>
## expr_planners

`function` · `datafusion_expr::registry::MemoryFunctionRegistry::expr_planners` · datafusion-expr 55.1.0

```rust
fn expr_planners(&self) -> Vec<Arc<dyn ExprPlanner>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::registry::MemoryFunctionRegistry", "path": "MemoryFunctionRegistry"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [210, 1], "end": [279, 2], "filename": "src/registry.rs"}, "trait": {"args": null, "id": "datafusion_expr::registry::FunctionRegistry", "path": "FunctionRegistry"}, "trait_path": "datafusion_expr::registry::FunctionRegistry"}`

Source: `src/registry.rs:264`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bb0c6c1833408cc7da0baebc"></a>
## fmt

`function` · `datafusion_expr::registry::MemoryFunctionRegistry::fmt` · datafusion-expr 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::registry::MemoryFunctionRegistry", "path": "MemoryFunctionRegistry"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [192, 19], "end": [192, 24], "filename": "src/registry.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/registry.rs:192`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4f5aff7d1b07042bf42afbf9"></a>
## higher_order_function

`function` · `datafusion_expr::registry::MemoryFunctionRegistry::higher_order_function` · datafusion-expr 55.1.0

```rust
fn higher_order_function(&self, name: &str) -> Result<Arc<HigherOrderUDF>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::registry::MemoryFunctionRegistry", "path": "MemoryFunctionRegistry"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [210, 1], "end": [279, 2], "filename": "src/registry.rs"}, "trait": {"args": null, "id": "datafusion_expr::registry::FunctionRegistry", "path": "FunctionRegistry"}, "trait_path": "datafusion_expr::registry::FunctionRegistry"}`

Source: `src/registry.rs:222`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-198bf8ffbd570d342e306a7d"></a>
## higher_order_function_names

`function` · `datafusion_expr::registry::MemoryFunctionRegistry::higher_order_function_names` · datafusion-expr 55.1.0

```rust
fn higher_order_function_names(&self) -> HashSet<String>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::registry::MemoryFunctionRegistry", "path": "MemoryFunctionRegistry"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [210, 1], "end": [279, 2], "filename": "src/registry.rs"}, "trait": {"args": null, "id": "datafusion_expr::registry::FunctionRegistry", "path": "FunctionRegistry"}, "trait_path": "datafusion_expr::registry::FunctionRegistry"}`

Source: `src/registry.rs:268`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-970a13fc83e8a1edbbb8762b"></a>
## new

`function` · `datafusion_expr::registry::MemoryFunctionRegistry::new` · datafusion-expr 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::registry::MemoryFunctionRegistry", "path": "MemoryFunctionRegistry"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [204, 1], "end": [208, 2], "filename": "src/registry.rs"}, "trait": null, "trait_path": null}`

Source: `src/registry.rs:205`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-691385b44b9546e0cb46d2ee"></a>
## register_higher_order_function

`function` · `datafusion_expr::registry::MemoryFunctionRegistry::register_higher_order_function` · datafusion-expr 55.1.0

```rust
fn register_higher_order_function(&mut self, function: Arc<HigherOrderUDF>) -> Result<Option<Arc<HigherOrderUDF>>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::registry::MemoryFunctionRegistry", "path": "MemoryFunctionRegistry"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [210, 1], "end": [279, 2], "filename": "src/registry.rs"}, "trait": {"args": null, "id": "datafusion_expr::registry::FunctionRegistry", "path": "FunctionRegistry"}, "trait_path": "datafusion_expr::registry::FunctionRegistry"}`

Source: `src/registry.rs:246`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-afdb99066a2e1828eada1922"></a>
## register_udaf

`function` · `datafusion_expr::registry::MemoryFunctionRegistry::register_udaf` · datafusion-expr 55.1.0

```rust
fn register_udaf(&mut self, udaf: Arc<AggregateUDF>) -> Result<Option<Arc<AggregateUDF>>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::registry::MemoryFunctionRegistry", "path": "MemoryFunctionRegistry"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [210, 1], "end": [279, 2], "filename": "src/registry.rs"}, "trait": {"args": null, "id": "datafusion_expr::registry::FunctionRegistry", "path": "FunctionRegistry"}, "trait_path": "datafusion_expr::registry::FunctionRegistry"}`

Source: `src/registry.rs:254`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a06aee92267e6ce05815aaf9"></a>
## register_udf

`function` · `datafusion_expr::registry::MemoryFunctionRegistry::register_udf` · datafusion-expr 55.1.0

```rust
fn register_udf(&mut self, udf: Arc<ScalarUDF>) -> Result<Option<Arc<ScalarUDF>>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::registry::MemoryFunctionRegistry", "path": "MemoryFunctionRegistry"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [210, 1], "end": [279, 2], "filename": "src/registry.rs"}, "trait": {"args": null, "id": "datafusion_expr::registry::FunctionRegistry", "path": "FunctionRegistry"}, "trait_path": "datafusion_expr::registry::FunctionRegistry"}`

Source: `src/registry.rs:243`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-924fc32f5460578ff2b67e9a"></a>
## register_udwf

`function` · `datafusion_expr::registry::MemoryFunctionRegistry::register_udwf` · datafusion-expr 55.1.0

```rust
fn register_udwf(&mut self, udaf: Arc<WindowUDF>) -> Result<Option<Arc<WindowUDF>>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::registry::MemoryFunctionRegistry", "path": "MemoryFunctionRegistry"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [210, 1], "end": [279, 2], "filename": "src/registry.rs"}, "trait": {"args": null, "id": "datafusion_expr::registry::FunctionRegistry", "path": "FunctionRegistry"}, "trait_path": "datafusion_expr::registry::FunctionRegistry"}`

Source: `src/registry.rs:260`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-02aa2e0d3531ede7321c5529"></a>
## udaf

`function` · `datafusion_expr::registry::MemoryFunctionRegistry::udaf` · datafusion-expr 55.1.0

```rust
fn udaf(&self, name: &str) -> Result<Arc<AggregateUDF>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::registry::MemoryFunctionRegistry", "path": "MemoryFunctionRegistry"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [210, 1], "end": [279, 2], "filename": "src/registry.rs"}, "trait": {"args": null, "id": "datafusion_expr::registry::FunctionRegistry", "path": "FunctionRegistry"}, "trait_path": "datafusion_expr::registry::FunctionRegistry"}`

Source: `src/registry.rs:229`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0459e90df211aff113cf3470"></a>
## udafs

`function` · `datafusion_expr::registry::MemoryFunctionRegistry::udafs` · datafusion-expr 55.1.0

```rust
fn udafs(&self) -> HashSet<String>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::registry::MemoryFunctionRegistry", "path": "MemoryFunctionRegistry"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [210, 1], "end": [279, 2], "filename": "src/registry.rs"}, "trait": {"args": null, "id": "datafusion_expr::registry::FunctionRegistry", "path": "FunctionRegistry"}, "trait_path": "datafusion_expr::registry::FunctionRegistry"}`

Source: `src/registry.rs:272`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b05a4d8ba43753e9af994a6a"></a>
## udf

`function` · `datafusion_expr::registry::MemoryFunctionRegistry::udf` · datafusion-expr 55.1.0

```rust
fn udf(&self, name: &str) -> Result<Arc<ScalarUDF>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::registry::MemoryFunctionRegistry", "path": "MemoryFunctionRegistry"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [210, 1], "end": [279, 2], "filename": "src/registry.rs"}, "trait": {"args": null, "id": "datafusion_expr::registry::FunctionRegistry", "path": "FunctionRegistry"}, "trait_path": "datafusion_expr::registry::FunctionRegistry"}`

Source: `src/registry.rs:215`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fed2ef71ee1591150bac8051"></a>
## udfs

`function` · `datafusion_expr::registry::MemoryFunctionRegistry::udfs` · datafusion-expr 55.1.0

```rust
fn udfs(&self) -> HashSet<String>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::registry::MemoryFunctionRegistry", "path": "MemoryFunctionRegistry"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [210, 1], "end": [279, 2], "filename": "src/registry.rs"}, "trait": {"args": null, "id": "datafusion_expr::registry::FunctionRegistry", "path": "FunctionRegistry"}, "trait_path": "datafusion_expr::registry::FunctionRegistry"}`

Source: `src/registry.rs:211`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4f0759cfca59e55f1a176e7d"></a>
## udwf

`function` · `datafusion_expr::registry::MemoryFunctionRegistry::udwf` · datafusion-expr 55.1.0

```rust
fn udwf(&self, name: &str) -> Result<Arc<WindowUDF>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::registry::MemoryFunctionRegistry", "path": "MemoryFunctionRegistry"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [210, 1], "end": [279, 2], "filename": "src/registry.rs"}, "trait": {"args": null, "id": "datafusion_expr::registry::FunctionRegistry", "path": "FunctionRegistry"}, "trait_path": "datafusion_expr::registry::FunctionRegistry"}`

Source: `src/registry.rs:236`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7e5972286c8729448bfd86b6"></a>
## udwfs

`function` · `datafusion_expr::registry::MemoryFunctionRegistry::udwfs` · datafusion-expr 55.1.0

```rust
fn udwfs(&self) -> HashSet<String>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::registry::MemoryFunctionRegistry", "path": "MemoryFunctionRegistry"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [210, 1], "end": [279, 2], "filename": "src/registry.rs"}, "trait": {"args": null, "id": "datafusion_expr::registry::FunctionRegistry", "path": "FunctionRegistry"}, "trait_path": "datafusion_expr::registry::FunctionRegistry"}`

Source: `src/registry.rs:276`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
