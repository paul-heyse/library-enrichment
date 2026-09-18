# `datafusion_expr::registry::FunctionRegistry`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.registry.FunctionRegistry.json).

<a id="op-3a0de62f03e60cdeb963c7b3"></a>
## FunctionRegistry

`trait` · `datafusion_expr::registry::FunctionRegistry` · datafusion-expr 55.1.0

```rust
trait FunctionRegistry
```

Source: `src/registry.rs:40`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

A registry knows how to build logical expressions out of user-defined function' names

<a id="op-2da21d97ac93a9a9a1d749f6"></a>
## deregister_higher_order_function

`function` · `datafusion_expr::registry::FunctionRegistry::deregister_higher_order_function` · datafusion-expr 55.1.0

```rust
fn deregister_higher_order_function(&mut self, _name: &str) -> Result<Option<Arc<HigherOrderUDF>>>
```

Source: `src/registry.rs:122`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Deregisters a [`HigherOrderUDF`](../operations/datafusion_expr.higher_order_function.HigherOrderUDF.md#op-67b8632773d35bc58334e2bb), returning the implementation that was
deregistered.

Returns an error (the default) if the function can not be deregistered,
for example if the registry is read only.

<a id="op-114bbae8f8ba4e8491c46f43"></a>
## deregister_udaf

`function` · `datafusion_expr::registry::FunctionRegistry::deregister_udaf` · datafusion-expr 55.1.0

```rust
fn deregister_udaf(&mut self, _name: &str) -> Result<Option<Arc<AggregateUDF>>>
```

Source: `src/registry.rs:134`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Deregisters a [`AggregateUDF`](../operations/datafusion_expr.udaf.AggregateUDF.md#op-d90e5a97479981a539718379), returning the implementation that was
deregistered.

Returns an error (the default) if the function can not be deregistered,
for example if the registry is read only.

<a id="op-f4ca476c70f7ed90efd5ce82"></a>
## deregister_udf

`function` · `datafusion_expr::registry::FunctionRegistry::deregister_udf` · datafusion-expr 55.1.0

```rust
fn deregister_udf(&mut self, _name: &str) -> Result<Option<Arc<ScalarUDF>>>
```

Source: `src/registry.rs:113`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Deregisters a [`ScalarUDF`](../operations/datafusion_expr.udf.ScalarUDF.md#op-12eb8b815a2294fbbbd085a0), returning the implementation that was
deregistered.

Returns an error (the default) if the function can not be deregistered,
for example if the registry is read only.

<a id="op-63ccc3a16c0e1a5743d69e47"></a>
## deregister_udwf

`function` · `datafusion_expr::registry::FunctionRegistry::deregister_udwf` · datafusion-expr 55.1.0

```rust
fn deregister_udwf(&mut self, _name: &str) -> Result<Option<Arc<WindowUDF>>>
```

Source: `src/registry.rs:143`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Deregisters a [`WindowUDF`](../operations/datafusion_expr.udwf.WindowUDF.md#op-42e8216e20a1c833692a7e65), returning the implementation that was
deregistered.

Returns an error (the default) if the function can not be deregistered,
for example if the registry is read only.

<a id="op-2fddc70520041da40b956933"></a>
## expr_planners

`function` · `datafusion_expr::registry::FunctionRegistry::expr_planners` · datafusion-expr 55.1.0

```rust
fn expr_planners(&self) -> Vec<Arc<dyn ExprPlanner>>
```

Source: `src/registry.rs:162`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Set of all registered [`ExprPlanner`](../operations/datafusion_expr.planner.ExprPlanner.md#op-c0ce2d948f3fc34627ff15dd)s

<a id="op-e8628c04e00b6c4f2fd700c8"></a>
## higher_order_function

`function` · `datafusion_expr::registry::FunctionRegistry::higher_order_function` · datafusion-expr 55.1.0

```rust
fn higher_order_function(&self, name: &str) -> Result<Arc<HigherOrderUDF>>
```

Source: `src/registry.rs:59`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Returns a reference to the user defined higher order function named
`name`.

<a id="op-9666bfb489249e5d805a7d73"></a>
## higher_order_function_names

`function` · `datafusion_expr::registry::FunctionRegistry::higher_order_function_names` · datafusion-expr 55.1.0

```rust
fn higher_order_function_names(&self) -> HashSet<String>
```

Source: `src/registry.rs:45`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Returns names of all available higher order user defined functions.

<a id="op-01e6b9c8fd2453680c90e1b9"></a>
## register_expr_planner

`function` · `datafusion_expr::registry::FunctionRegistry::register_expr_planner` · datafusion-expr 55.1.0

```rust
fn register_expr_planner(&mut self, _expr_planner: Arc<dyn ExprPlanner>) -> Result<()>
```

Source: `src/registry.rs:165`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Registers a new [`ExprPlanner`](../operations/datafusion_expr.planner.ExprPlanner.md#op-c0ce2d948f3fc34627ff15dd) with the registry.

<a id="op-ab1ab205ccd0a5d8aa9c6b66"></a>
## register_function_rewrite

`function` · `datafusion_expr::registry::FunctionRegistry::register_function_rewrite` · datafusion-expr 55.1.0

```rust
fn register_function_rewrite(&mut self, _rewrite: Arc<dyn FunctionRewrite + Send + Sync>) -> Result<()>
```

Source: `src/registry.rs:154`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Registers a new [`FunctionRewrite`](../operations/datafusion_expr.expr_rewriter.FunctionRewrite.md#op-d16387f6b2d1bd76e4ec29b7) with the registry.

`FunctionRewrite` rules are used to rewrite certain / operators in the
logical plan to function calls.  For example `a || b` might be written to
`array_concat(a, b)`.

This allows the behavior of operators to be customized by the user.

<a id="op-84aa3a0e6c1c13d29b4ebd30"></a>
## register_higher_order_function

`function` · `datafusion_expr::registry::FunctionRegistry::register_higher_order_function` · datafusion-expr 55.1.0

```rust
fn register_higher_order_function(&mut self, _function: Arc<HigherOrderUDF>) -> Result<Option<Arc<HigherOrderUDF>>>
```

Source: `src/registry.rs:82`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Registers a new [`HigherOrderUDF`](../operations/datafusion_expr.higher_order_function.HigherOrderUDF.md#op-67b8632773d35bc58334e2bb), returning any previously registered
implementation.

Returns an error (the default) if the function can not be registered,
for example if the registry is read only.

<a id="op-af1ba3df638cc1f3437de919"></a>
## register_udaf

`function` · `datafusion_expr::registry::FunctionRegistry::register_udaf` · datafusion-expr 55.1.0

```rust
fn register_udaf(&mut self, _udaf: Arc<AggregateUDF>) -> Result<Option<Arc<AggregateUDF>>>
```

Source: `src/registry.rs:93`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Registers a new [`AggregateUDF`](../operations/datafusion_expr.udaf.AggregateUDF.md#op-d90e5a97479981a539718379), returning any previously registered
implementation.

Returns an error (the default) if the function can not be registered,
for example if the registry is read only.

<a id="op-cf242d25676839b5466be50f"></a>
## register_udf

`function` · `datafusion_expr::registry::FunctionRegistry::register_udf` · datafusion-expr 55.1.0

```rust
fn register_udf(&mut self, _udf: Arc<ScalarUDF>) -> Result<Option<Arc<ScalarUDF>>>
```

Source: `src/registry.rs:74`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Registers a new [`ScalarUDF`](../operations/datafusion_expr.udf.ScalarUDF.md#op-12eb8b815a2294fbbbd085a0), returning any previously registered
implementation.

Returns an error (the default) if the function can not be registered,
for example if the registry is read only.

<a id="op-85c2077b85f2c32d7b562833"></a>
## register_udwf

`function` · `datafusion_expr::registry::FunctionRegistry::register_udwf` · datafusion-expr 55.1.0

```rust
fn register_udwf(&mut self, _udaf: Arc<WindowUDF>) -> Result<Option<Arc<WindowUDF>>>
```

Source: `src/registry.rs:104`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Registers a new [`WindowUDF`](../operations/datafusion_expr.udwf.WindowUDF.md#op-42e8216e20a1c833692a7e65), returning any previously registered
implementation.

Returns an error (the default) if the function can not be registered,
for example if the registry is read only.

<a id="op-22ca781cda14eddb17a6b63a"></a>
## udaf

`function` · `datafusion_expr::registry::FunctionRegistry::udaf` · datafusion-expr 55.1.0

```rust
fn udaf(&self, name: &str) -> Result<Arc<AggregateUDF>>
```

Source: `src/registry.rs:63`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Returns a reference to the user defined aggregate function (udaf) named
`name`.

<a id="op-e17d372e74daa6d2f91b95a1"></a>
## udafs

`function` · `datafusion_expr::registry::FunctionRegistry::udafs` · datafusion-expr 55.1.0

```rust
fn udafs(&self) -> HashSet<String>
```

Source: `src/registry.rs:48`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Returns names of all available aggregate user defined functions.

<a id="op-d677afc3b1116b9b08e45b49"></a>
## udf

`function` · `datafusion_expr::registry::FunctionRegistry::udf` · datafusion-expr 55.1.0

```rust
fn udf(&self, name: &str) -> Result<Arc<ScalarUDF>>
```

Source: `src/registry.rs:55`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Returns a reference to the user defined scalar function (udf) named
`name`.

<a id="op-52a7aea5170a15359efb6e81"></a>
## udfs

`function` · `datafusion_expr::registry::FunctionRegistry::udfs` · datafusion-expr 55.1.0

```rust
fn udfs(&self) -> HashSet<String>
```

Source: `src/registry.rs:42`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Returns names of all available scalar user defined functions.

<a id="op-6a5eb8c3f016ecc70a1249a1"></a>
## udwf

`function` · `datafusion_expr::registry::FunctionRegistry::udwf` · datafusion-expr 55.1.0

```rust
fn udwf(&self, name: &str) -> Result<Arc<WindowUDF>>
```

Source: `src/registry.rs:67`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Returns a reference to the user defined window function (udwf) named
`name`.

<a id="op-006f1c2881e3279cde6a266b"></a>
## udwfs

`function` · `datafusion_expr::registry::FunctionRegistry::udwfs` · datafusion-expr 55.1.0

```rust
fn udwfs(&self) -> HashSet<String>
```

Source: `src/registry.rs:51`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Returns names of all available window user defined functions.
