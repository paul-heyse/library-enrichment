# `datafusion::execution::session_state::SessionStateBuilder`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion.execution.session_state.SessionStateBuilder.json).

<a id="op-99760a72dc31f296f2e30fdb"></a>
## SessionStateBuilder

`struct` · `datafusion::execution::session_state::SessionStateBuilder` · datafusion 55.1.0

```rust
struct SessionStateBuilder
```

Source: `src/execution/session_state.rs:1053`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

A builder to be used for building [`SessionState`](../operations/datafusion.execution.session_state.SessionState.md#op-3ba80ad5c25fa63e8340a601)'s. Defaults will
be used for all values unless explicitly provided.

See example on [`SessionState`](../operations/datafusion.execution.session_state.SessionState.md#op-3ba80ad5c25fa63e8340a601)

<a id="op-200eb33a7a5bb485b2d9b445"></a>
## aggregate_functions

`function` · `datafusion::execution::session_state::SessionStateBuilder::aggregate_functions` · datafusion 55.1.0

```rust
fn aggregate_functions(&mut self) -> &mut Option<Vec<Arc<AggregateUDF>>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::session_state::SessionStateBuilder", "path": "SessionStateBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1087, 1], "end": [1905, 2], "filename": "src/execution/session_state.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution/session_state.rs:1829`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Returns the current aggregate_functions value

<a id="op-11eea1f2b5680d0164a211a3"></a>
## analyzer

`function` · `datafusion::execution::session_state::SessionStateBuilder::analyzer` · datafusion 55.1.0

```rust
fn analyzer(&mut self) -> &mut Option<Analyzer>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::session_state::SessionStateBuilder", "path": "SessionStateBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1087, 1], "end": [1905, 2], "filename": "src/execution/session_state.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution/session_state.rs:1770`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Returns the current analyzer value

<a id="op-025f45e7627e5a3df0ba9b30"></a>
## analyzer_rules

`function` · `datafusion::execution::session_state::SessionStateBuilder::analyzer_rules` · datafusion 55.1.0

```rust
fn analyzer_rules(&mut self) -> &mut Option<Vec<Arc<dyn AnalyzerRule + Send + Sync>>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::session_state::SessionStateBuilder", "path": "SessionStateBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1087, 1], "end": [1905, 2], "filename": "src/execution/session_state.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution/session_state.rs:1886`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Returns the current analyzer_rules value

<a id="op-9a61179476601b3e7635de9f"></a>
## build

`function` · `datafusion::execution::session_state::SessionStateBuilder::build` · datafusion 55.1.0

```rust
fn build(self) -> SessionState
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::session_state::SessionStateBuilder", "path": "SessionStateBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1087, 1], "end": [1905, 2], "filename": "src/execution/session_state.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution/session_state.rs:1568`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Builds a [`SessionState`](../operations/datafusion.execution.session_state.SessionState.md#op-3ba80ad5c25fa63e8340a601) with the current configuration.

Note that there is an explicit option for enabling catalog and schema defaults
in [SessionConfig::create_default_catalog_and_schema] which if enabled
will be built here.

Unresolved upstream links (retained, not inferred): `SessionConfig::create_default_catalog_and_schema`.

<a id="op-c53b20e220a14dd69bf04698"></a>
## cache_factory

`function` · `datafusion::execution::session_state::SessionStateBuilder::cache_factory` · datafusion 55.1.0

```rust
fn cache_factory(&mut self) -> &mut Option<Arc<dyn CacheFactory>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::session_state::SessionStateBuilder", "path": "SessionStateBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1087, 1], "end": [1905, 2], "filename": "src/execution/session_state.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution/session_state.rs:1881`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Returns the cache factory

<a id="op-8f8a512e82d8aff9f28d2dd0"></a>
## catalog_list

`function` · `datafusion::execution::session_state::SessionStateBuilder::catalog_list` · datafusion 55.1.0

```rust
fn catalog_list(&mut self) -> &mut Option<Arc<dyn CatalogProviderList>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::session_state::SessionStateBuilder", "path": "SessionStateBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1087, 1], "end": [1905, 2], "filename": "src/execution/session_state.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution/session_state.rs:1807`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Returns the current catalog_list value

<a id="op-d906389861b53e53799850b9"></a>
## clone

`function` · `datafusion::execution::session_state::SessionStateBuilder::clone` · datafusion 55.1.0

```rust
fn clone(&self) -> SessionStateBuilder
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::session_state::SessionStateBuilder", "path": "SessionStateBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1052, 10], "end": [1052, 15], "filename": "src/execution/session_state.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/execution/session_state.rs:1052`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7c25eedeeb3c34b9f8c55e48"></a>
## config

`function` · `datafusion::execution::session_state::SessionStateBuilder::config` · datafusion 55.1.0

```rust
fn config(&mut self) -> &mut Option<SessionConfig>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::session_state::SessionStateBuilder", "path": "SessionStateBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1087, 1], "end": [1905, 2], "filename": "src/execution/session_state.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution/session_state.rs:1849`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Returns the current session_config value

<a id="op-f77cf249b2cb959e2755b9b7"></a>
## default

`function` · `datafusion::execution::session_state::SessionStateBuilder::default` · datafusion 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::session_state::SessionStateBuilder", "path": "SessionStateBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1943, 1], "end": [1947, 2], "filename": "src/execution/session_state.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/execution/session_state.rs:1944`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-864976366c2e727b159e48b3"></a>
## execution_props

`function` · `datafusion::execution::session_state::SessionStateBuilder::execution_props` · datafusion 55.1.0

```rust
fn execution_props(&mut self) -> &mut Option<ExecutionProps>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::session_state::SessionStateBuilder", "path": "SessionStateBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1087, 1], "end": [1905, 2], "filename": "src/execution/session_state.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution/session_state.rs:1859`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Returns the current execution_props value

<a id="op-9e4d78964c0e50059c39a218"></a>
## expr_planners

`function` · `datafusion::execution::session_state::SessionStateBuilder::expr_planners` · datafusion 55.1.0

```rust
fn expr_planners(&mut self) -> &mut Option<Vec<Arc<dyn ExprPlanner>>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::session_state::SessionStateBuilder", "path": "SessionStateBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1087, 1], "end": [1905, 2], "filename": "src/execution/session_state.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution/session_state.rs:1775`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Returns the current expr_planners value

<a id="op-fc5f2f91eab74c44815ff14e"></a>
## file_formats

`function` · `datafusion::execution::session_state::SessionStateBuilder::file_formats` · datafusion 55.1.0

```rust
fn file_formats(&mut self) -> &mut Option<Vec<Arc<dyn FileFormatFactory>>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::session_state::SessionStateBuilder", "path": "SessionStateBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1087, 1], "end": [1905, 2], "filename": "src/execution/session_state.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution/session_state.rs:1844`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Returns the current file_formats value

<a id="op-98523767e1c404f30b18f227"></a>
## fmt

`function` · `datafusion::execution::session_state::SessionStateBuilder::fmt` · datafusion 55.1.0

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::session_state::SessionStateBuilder", "path": "SessionStateBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1907, 1], "end": [1941, 2], "filename": "src/execution/session_state.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/execution/session_state.rs:1910`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Prefer having short fields at the top and long vector fields near the end
Group fields by

<a id="op-01a684374aafd3a4c28ab41f"></a>
## from

`function` · `datafusion::execution::session_state::SessionStateBuilder::from` · datafusion 55.1.0

```rust
fn from(session: SessionContext) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::session_state::SessionStateBuilder", "path": "crate::execution::session_state::SessionStateBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2177, 1], "end": [2181, 2], "filename": "src/execution/context/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "datafusion::execution::context::SessionContext", "path": "SessionContext"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/execution/context/mod.rs:2178`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ac9c1d61cc98c140e4416ba6"></a>
## from

`function` · `datafusion::execution::session_state::SessionStateBuilder::from` · datafusion 55.1.0

```rust
fn from(state: SessionState) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::session_state::SessionStateBuilder", "path": "SessionStateBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1949, 1], "end": [1953, 2], "filename": "src/execution/session_state.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "datafusion::execution::session_state::SessionState", "path": "SessionState"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/execution/session_state.rs:1950`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-126e1d4ed3b39049501359f2"></a>
## function_factory

`function` · `datafusion::execution::session_state::SessionStateBuilder::function_factory` · datafusion 55.1.0

```rust
fn function_factory(&mut self) -> &mut Option<Arc<dyn FunctionFactory>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::session_state::SessionStateBuilder", "path": "SessionStateBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1087, 1], "end": [1905, 2], "filename": "src/execution/session_state.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution/session_state.rs:1876`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Returns the current function_factory value

<a id="op-e2535ba15228dc6c009dfbab"></a>
## higher_order_functions

`function` · `datafusion::execution::session_state::SessionStateBuilder::higher_order_functions` · datafusion 55.1.0

```rust
fn higher_order_functions(&mut self) -> &mut Option<Vec<Arc<HigherOrderUDF>>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::session_state::SessionStateBuilder", "path": "SessionStateBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1087, 1], "end": [1905, 2], "filename": "src/execution/session_state.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution/session_state.rs:1824`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Returns the current scalar_functions value

<a id="op-536405370b9523134d1a39cf"></a>
## new

`function` · `datafusion::execution::session_state::SessionStateBuilder::new` · datafusion 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::session_state::SessionStateBuilder", "path": "SessionStateBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1087, 1], "end": [1905, 2], "filename": "src/execution/session_state.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution/session_state.rs:1095`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Returns a new empty [`SessionStateBuilder`](../operations/datafusion.execution.session_state.SessionStateBuilder.md#op-99760a72dc31f296f2e30fdb).

See [`Self::with_default_features`](../operations/datafusion.execution.session_state.SessionStateBuilder.md#op-244c16e543881cb24f1bebae) to install the default set of functions,
catalogs, etc.

To create a `SessionStateBuilder` with default features such as functions,
please see [`Self::new_with_default_features`](../operations/datafusion.execution.session_state.SessionStateBuilder.md#op-0d7e0ac2c8b000cbfeae104a).

<a id="op-b4137304f5b16aa78b15dd49"></a>
## new_from_existing

`function` · `datafusion::execution::session_state::SessionStateBuilder::new_from_existing` · datafusion 55.1.0

```rust
fn new_from_existing(existing: SessionState) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::session_state::SessionStateBuilder", "path": "SessionStateBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1087, 1], "end": [1905, 2], "filename": "src/execution/session_state.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution/session_state.rs:1137`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Returns a new [SessionStateBuilder](../operations/datafusion.execution.session_state.SessionStateBuilder.md#op-99760a72dc31f296f2e30fdb) based on an existing [SessionState](../operations/datafusion.execution.session_state.SessionState.md#op-3ba80ad5c25fa63e8340a601).

The session id for the new builder will be unset; all other fields will
be cloned from `existing`. If the default
catalog exists in existing session state, the new session state will not
create default catalog and schema.

<a id="op-0d7e0ac2c8b000cbfeae104a"></a>
## new_with_default_features

`function` · `datafusion::execution::session_state::SessionStateBuilder::new_with_default_features` · datafusion 55.1.0

```rust
fn new_with_default_features() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::session_state::SessionStateBuilder", "path": "SessionStateBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1087, 1], "end": [1905, 2], "filename": "src/execution/session_state.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution/session_state.rs:1252`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Returns a new [`SessionStateBuilder`](../operations/datafusion.execution.session_state.SessionStateBuilder.md#op-99760a72dc31f296f2e30fdb) with default features.

This is equivalent to calling [`Self::new()`](../operations/datafusion.execution.session_state.SessionStateBuilder.md#op-536405370b9523134d1a39cf) followed by [`Self::with_default_features()`](../operations/datafusion.execution.session_state.SessionStateBuilder.md#op-244c16e543881cb24f1bebae).

```
use datafusion::execution::session_state::SessionStateBuilder;

// Create a new SessionState with default features
let session_state = SessionStateBuilder::new_with_default_features()
    .with_session_id("my_session".to_string())
    .build();
```

<a id="op-68c38c7e05b1ec8c869b0a51"></a>
## optimizer

`function` · `datafusion::execution::session_state::SessionStateBuilder::optimizer` · datafusion 55.1.0

```rust
fn optimizer(&mut self) -> &mut Option<Optimizer>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::session_state::SessionStateBuilder", "path": "SessionStateBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1087, 1], "end": [1905, 2], "filename": "src/execution/session_state.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution/session_state.rs:1792`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Returns the current optimizer value

<a id="op-2ea212c5a783eeabd0c3ca60"></a>
## optimizer_rules

`function` · `datafusion::execution::session_state::SessionStateBuilder::optimizer_rules` · datafusion 55.1.0

```rust
fn optimizer_rules(&mut self) -> &mut Option<Vec<Arc<dyn OptimizerRule + Send + Sync>>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::session_state::SessionStateBuilder", "path": "SessionStateBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1087, 1], "end": [1905, 2], "filename": "src/execution/session_state.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution/session_state.rs:1893`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Returns the current optimizer_rules value

<a id="op-7abb73587d6ed80bcead0a3e"></a>
## physical_optimizer_rules

`function` · `datafusion::execution::session_state::SessionStateBuilder::physical_optimizer_rules` · datafusion 55.1.0

```rust
fn physical_optimizer_rules(&mut self) -> &mut Option<Vec<Arc<dyn PhysicalOptimizerRule + Send + Sync>>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::session_state::SessionStateBuilder", "path": "SessionStateBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1087, 1], "end": [1905, 2], "filename": "src/execution/session_state.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution/session_state.rs:1900`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Returns the current physical_optimizer_rules value

<a id="op-85855b08334a58651b45566d"></a>
## physical_optimizers

`function` · `datafusion::execution::session_state::SessionStateBuilder::physical_optimizers` · datafusion 55.1.0

```rust
fn physical_optimizers(&mut self) -> &mut Option<PhysicalOptimizer>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::session_state::SessionStateBuilder", "path": "SessionStateBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1087, 1], "end": [1905, 2], "filename": "src/execution/session_state.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution/session_state.rs:1797`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Returns the current physical_optimizers value

<a id="op-40b2f610088463bba07ef7fc"></a>
## query_planner

`function` · `datafusion::execution::session_state::SessionStateBuilder::query_planner` · datafusion 55.1.0

```rust
fn query_planner(&mut self) -> &mut Option<Arc<dyn QueryPlanner + Send + Sync>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::session_state::SessionStateBuilder", "path": "SessionStateBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1087, 1], "end": [1905, 2], "filename": "src/execution/session_state.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution/session_state.rs:1802`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Returns the current query_planner value

<a id="op-2f4a54c3d812e421d8a15572"></a>
## relation_planners

`function` · `datafusion::execution::session_state::SessionStateBuilder::relation_planners` · datafusion 55.1.0

```rust
fn relation_planners(&mut self) -> &mut Option<Vec<Arc<dyn RelationPlanner>>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::session_state::SessionStateBuilder", "path": "SessionStateBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1087, 1], "end": [1905, 2], "filename": "src/execution/session_state.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution/session_state.rs:1781`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Returns a mutable reference to the current [`RelationPlanner`](../operations/datafusion_expr.planner.RelationPlanner.md#op-1be858ef3f752319489e5fc4) list.

<a id="op-3a1001ad723f8d3e297f7ef7"></a>
## runtime_env

`function` · `datafusion::execution::session_state::SessionStateBuilder::runtime_env` · datafusion 55.1.0

```rust
fn runtime_env(&mut self) -> &mut Option<Arc<RuntimeEnv>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::session_state::SessionStateBuilder", "path": "SessionStateBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1087, 1], "end": [1905, 2], "filename": "src/execution/session_state.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution/session_state.rs:1871`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Returns the current runtime_env value

<a id="op-dc7d0c96db28b34a4ec9b2ea"></a>
## scalar_functions

`function` · `datafusion::execution::session_state::SessionStateBuilder::scalar_functions` · datafusion 55.1.0

```rust
fn scalar_functions(&mut self) -> &mut Option<Vec<Arc<ScalarUDF>>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::session_state::SessionStateBuilder", "path": "SessionStateBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1087, 1], "end": [1905, 2], "filename": "src/execution/session_state.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution/session_state.rs:1819`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Returns the current scalar_functions value

<a id="op-2bd55ab9ef1711d03584fe03"></a>
## serializer_registry

`function` · `datafusion::execution::session_state::SessionStateBuilder::serializer_registry` · datafusion 55.1.0

```rust
fn serializer_registry(&mut self) -> &mut Option<Arc<dyn SerializerRegistry>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::session_state::SessionStateBuilder", "path": "SessionStateBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1087, 1], "end": [1905, 2], "filename": "src/execution/session_state.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution/session_state.rs:1839`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Returns the current serializer_registry value

<a id="op-96435e899389c2ff106e7581"></a>
## session_id

`function` · `datafusion::execution::session_state::SessionStateBuilder::session_id` · datafusion 55.1.0

```rust
fn session_id(&self) -> &Option<String>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::session_state::SessionStateBuilder", "path": "SessionStateBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1087, 1], "end": [1905, 2], "filename": "src/execution/session_state.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution/session_state.rs:1765`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Returns the current session_id value

<a id="op-a564250b616662fd132f6787"></a>
## table_factories

`function` · `datafusion::execution::session_state::SessionStateBuilder::table_factories` · datafusion 55.1.0

```rust
fn table_factories(&mut self) -> &mut Option<HashMap<String, Arc<dyn TableProviderFactory>>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::session_state::SessionStateBuilder", "path": "SessionStateBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1087, 1], "end": [1905, 2], "filename": "src/execution/session_state.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution/session_state.rs:1864`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Returns the current table_factories value

<a id="op-4fa3411626cc2f8576b0155b"></a>
## table_functions

`function` · `datafusion::execution::session_state::SessionStateBuilder::table_functions` · datafusion 55.1.0

```rust
fn table_functions(&mut self) -> &mut Option<HashMap<String, Arc<TableFunction>>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::session_state::SessionStateBuilder", "path": "SessionStateBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1087, 1], "end": [1905, 2], "filename": "src/execution/session_state.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution/session_state.rs:1812`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Returns the current table_functions value

<a id="op-bf04606006011c30bfe503e1"></a>
## table_options

`function` · `datafusion::execution::session_state::SessionStateBuilder::table_options` · datafusion 55.1.0

```rust
fn table_options(&mut self) -> &mut Option<TableOptions>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::session_state::SessionStateBuilder", "path": "SessionStateBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1087, 1], "end": [1905, 2], "filename": "src/execution/session_state.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution/session_state.rs:1854`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Returns the current table_options value

<a id="op-9a509fc7f8800050634e09d6"></a>
## type_planner

`function` · `datafusion::execution::session_state::SessionStateBuilder::type_planner` · datafusion 55.1.0

```rust
fn type_planner(&mut self) -> &mut Option<Arc<dyn TypePlanner>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::session_state::SessionStateBuilder", "path": "SessionStateBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1087, 1], "end": [1905, 2], "filename": "src/execution/session_state.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution/session_state.rs:1787`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Returns the current type_planner value

<a id="op-e194cb827938c043c10befbf"></a>
## window_functions

`function` · `datafusion::execution::session_state::SessionStateBuilder::window_functions` · datafusion 55.1.0

```rust
fn window_functions(&mut self) -> &mut Option<Vec<Arc<WindowUDF>>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::session_state::SessionStateBuilder", "path": "SessionStateBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1087, 1], "end": [1905, 2], "filename": "src/execution/session_state.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution/session_state.rs:1834`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Returns the current window_functions value

<a id="op-14dfbf905cdd87a5d6ec96e5"></a>
## with_aggregate_functions

`function` · `datafusion::execution::session_state::SessionStateBuilder::with_aggregate_functions` · datafusion 55.1.0

```rust
fn with_aggregate_functions(self, aggregate_functions: Vec<Arc<AggregateUDF>>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::session_state::SessionStateBuilder", "path": "SessionStateBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1087, 1], "end": [1905, 2], "filename": "src/execution/session_state.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution/session_state.rs:1411`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Set the map of [`AggregateUDF`](../operations/datafusion_expr.udaf.AggregateUDF.md#op-d90e5a97479981a539718379)s

<a id="op-6648117378d4dbadfba832b7"></a>
## with_analyzer_rule

`function` · `datafusion::execution::session_state::SessionStateBuilder::with_analyzer_rule` · datafusion 55.1.0

```rust
fn with_analyzer_rule(self, analyzer_rule: Arc<dyn AnalyzerRule + Send + Sync>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::session_state::SessionStateBuilder", "path": "SessionStateBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1087, 1], "end": [1905, 2], "filename": "src/execution/session_state.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution/session_state.rs:1273`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Add `analyzer_rule` to the end of the list of
[`AnalyzerRule`](../operations/datafusion_optimizer.analyzer.AnalyzerRule.md#op-cff859d3fc5687654af4ac1d)s used to rewrite queries.

<a id="op-0890996d70d7662032b1dc81"></a>
## with_analyzer_rules

`function` · `datafusion::execution::session_state::SessionStateBuilder::with_analyzer_rules` · datafusion 55.1.0

```rust
fn with_analyzer_rules(self, rules: Vec<Arc<dyn AnalyzerRule + Send + Sync>>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::session_state::SessionStateBuilder", "path": "SessionStateBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1087, 1], "end": [1905, 2], "filename": "src/execution/session_state.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution/session_state.rs:1263`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Set the [`AnalyzerRule`](../operations/datafusion_optimizer.analyzer.AnalyzerRule.md#op-cff859d3fc5687654af4ac1d)s optimizer plan rules.

<a id="op-5df813b16e88ed435b4cd0b1"></a>
## with_cache_factory

`function` · `datafusion::execution::session_state::SessionStateBuilder::with_cache_factory` · datafusion 55.1.0

```rust
fn with_cache_factory(self, cache_factory: Option<Arc<dyn CacheFactory>>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::session_state::SessionStateBuilder", "path": "SessionStateBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1087, 1], "end": [1905, 2], "filename": "src/execution/session_state.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution/session_state.rs:1510`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Set a [`CacheFactory`](../operations/datafusion.execution.session_state.CacheFactory.md#op-05346466af099911c46feb7a) for custom caching strategy

<a id="op-f522231fa05555d6ccf0fc1b"></a>
## with_catalog_list

`function` · `datafusion::execution::session_state::SessionStateBuilder::with_catalog_list` · datafusion 55.1.0

```rust
fn with_catalog_list(self, catalog_list: Arc<dyn CatalogProviderList>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::session_state::SessionStateBuilder", "path": "SessionStateBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1087, 1], "end": [1905, 2], "filename": "src/execution/session_state.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution/session_state.rs:1362`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Set the [`CatalogProviderList`](../operations/datafusion_session.catalog.CatalogProviderList.md#op-d1c9ece1dd28ba403a6492b6)

<a id="op-1aed571a69dd7249b22c171a"></a>
## with_config

`function` · `datafusion::execution::session_state::SessionStateBuilder::with_config` · datafusion 55.1.0

```rust
fn with_config(self, config: SessionConfig) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::session_state::SessionStateBuilder", "path": "SessionStateBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1087, 1], "end": [1905, 2], "filename": "src/execution/session_state.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution/session_state.rs:1456`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Set the [`SessionConfig`](../operations/datafusion_execution.config.SessionConfig.md#op-5db676088685c6e8b0496c08)

<a id="op-244c16e543881cb24f1bebae"></a>
## with_default_features

`function` · `datafusion::execution::session_state::SessionStateBuilder::with_default_features` · datafusion 55.1.0

```rust
fn with_default_features(self) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::session_state::SessionStateBuilder", "path": "SessionStateBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1087, 1], "end": [1905, 2], "filename": "src/execution/session_state.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution/session_state.rs:1195`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Adds defaults for table_factories, file formats, expr_planners and builtin
scalar, aggregate and windows functions.

Note overwrites any previously registered items with the same name.

<a id="op-45b1ce49c985a5377f6faacd"></a>
## with_execution_props

`function` · `datafusion::execution::session_state::SessionStateBuilder::with_execution_props` · datafusion 55.1.0

```rust
fn with_execution_props(self, execution_props: ExecutionProps) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::session_state::SessionStateBuilder", "path": "SessionStateBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1087, 1], "end": [1905, 2], "filename": "src/execution/session_state.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution/session_state.rs:1468`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Set the [`ExecutionProps`](../operations/datafusion_expr.execution_props.ExecutionProps.md#op-52e4deb424494aeff92bf188)

<a id="op-695dc384f6b6a50f30ac4097"></a>
## with_expr_planners

`function` · `datafusion::execution::session_state::SessionStateBuilder::with_expr_planners` · datafusion 55.1.0

```rust
fn with_expr_planners(self, expr_planners: Vec<Arc<dyn ExprPlanner>>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::session_state::SessionStateBuilder", "path": "SessionStateBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1087, 1], "end": [1905, 2], "filename": "src/execution/session_state.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution/session_state.rs:1305`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Set the [`ExprPlanner`](../operations/datafusion_expr.planner.ExprPlanner.md#op-c0ce2d948f3fc34627ff15dd)s used to customize the behavior of the SQL planner.

<a id="op-10b45fd3853e89fecbb2d7f9"></a>
## with_extension_type_registry

`function` · `datafusion::execution::session_state::SessionStateBuilder::with_extension_type_registry` · datafusion 55.1.0

```rust
fn with_extension_type_registry(self, registry: ExtensionTypeRegistryRef) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::session_state::SessionStateBuilder", "path": "SessionStateBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1087, 1], "end": [1905, 2], "filename": "src/execution/session_state.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution/session_state.rs:1429`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Sets the [`ExtensionTypeRegistry`](datafusion_expr::registry::ExtensionTypeRegistry).

<a id="op-8678b1d98d0146172d3805d8"></a>
## with_file_formats

`function` · `datafusion::execution::session_state::SessionStateBuilder::with_file_formats` · datafusion 55.1.0

```rust
fn with_file_formats(self, file_formats: Vec<Arc<dyn FileFormatFactory>>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::session_state::SessionStateBuilder", "path": "SessionStateBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1087, 1], "end": [1905, 2], "filename": "src/execution/session_state.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution/session_state.rs:1447`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Set the map of [`FileFormatFactory`](../operations/datafusion_datasource.file_format.FileFormatFactory.md#op-f967009d6b2c0c52e0070973)s

<a id="op-56e4084fe6934004b150e647"></a>
## with_function_factory

`function` · `datafusion::execution::session_state::SessionStateBuilder::with_function_factory` · datafusion 55.1.0

```rust
fn with_function_factory(self, function_factory: Option<Arc<dyn FunctionFactory>>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::session_state::SessionStateBuilder", "path": "SessionStateBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1087, 1], "end": [1905, 2], "filename": "src/execution/session_state.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution/session_state.rs:1501`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Set a [`FunctionFactory`](../operations/datafusion.execution.context.FunctionFactory.md#op-4f2bfbb3e4aa340eb4071d97) to handle `CREATE FUNCTION` statements

<a id="op-513733c8d3653b65a310d484"></a>
## with_higher_order_functions

`function` · `datafusion::execution::session_state::SessionStateBuilder::with_higher_order_functions` · datafusion 55.1.0

```rust
fn with_higher_order_functions(self, higher_order_functions: Vec<Arc<HigherOrderUDF>>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::session_state::SessionStateBuilder", "path": "SessionStateBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1087, 1], "end": [1905, 2], "filename": "src/execution/session_state.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution/session_state.rs:1402`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Set the map of [`HigherOrderUDF`](../operations/datafusion_expr.higher_order_function.HigherOrderUDF.md#op-67b8632773d35bc58334e2bb)s

<a id="op-a93f65671a3eea89f5fc9ddb"></a>
## with_object_store

`function` · `datafusion::execution::session_state::SessionStateBuilder::with_object_store` · datafusion 55.1.0

```rust
fn with_object_store(self, url: &Url, object_store: Arc<dyn ObjectStore>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::session_state::SessionStateBuilder", "path": "SessionStateBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1087, 1], "end": [1905, 2], "filename": "src/execution/session_state.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution/session_state.rs:1548`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Register an `ObjectStore` to the [`RuntimeEnv`](../operations/datafusion_execution.runtime_env.RuntimeEnv.md#op-c598f4df4cf51824ae4b9a67). See [`RuntimeEnv::register_object_store`]
for more details.

Note that this creates a default [`RuntimeEnv`](../operations/datafusion_execution.runtime_env.RuntimeEnv.md#op-c598f4df4cf51824ae4b9a67) if  there isn't one passed in already.

```
# use datafusion::prelude::*;
# use datafusion::execution::session_state::SessionStateBuilder;
# use datafusion_execution::runtime_env::RuntimeEnv;
# use url::Url;
# use std::sync::Arc;
# let http_store = object_store::local::LocalFileSystem::new();
let url = Url::try_from("file://").unwrap();
let object_store = object_store::local::LocalFileSystem::new();
let state = SessionStateBuilder::new()
    .with_config(SessionConfig::new())
    .with_object_store(&url, Arc::new(object_store))
    .with_default_features()
    .build();
```

Unresolved upstream links (retained, not inferred): ``RuntimeEnv::register_object_store``.

<a id="op-f10586b489ca96f2ba896398"></a>
## with_optimizer_rule

`function` · `datafusion::execution::session_state::SessionStateBuilder::with_optimizer_rule` · datafusion 55.1.0

```rust
fn with_optimizer_rule(self, optimizer_rule: Arc<dyn OptimizerRule + Send + Sync>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::session_state::SessionStateBuilder", "path": "SessionStateBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1087, 1], "end": [1905, 2], "filename": "src/execution/session_state.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution/session_state.rs:1294`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Add `optimizer_rule` to the end of the list of
[`OptimizerRule`](../operations/datafusion_optimizer.optimizer.OptimizerRule.md#op-16265e807b19887d2b41f187)s used to rewrite queries.

<a id="op-f5484523a850d316b9ea753e"></a>
## with_optimizer_rules

`function` · `datafusion::execution::session_state::SessionStateBuilder::with_optimizer_rules` · datafusion 55.1.0

```rust
fn with_optimizer_rules(self, rules: Vec<Arc<dyn OptimizerRule + Send + Sync>>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::session_state::SessionStateBuilder", "path": "SessionStateBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1087, 1], "end": [1905, 2], "filename": "src/execution/session_state.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution/session_state.rs:1284`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Set the [`OptimizerRule`](../operations/datafusion_optimizer.optimizer.OptimizerRule.md#op-16265e807b19887d2b41f187)s used to optimize plans.

<a id="op-66b2bcb65a29560ad311b601"></a>
## with_physical_optimizer_rule

`function` · `datafusion::execution::session_state::SessionStateBuilder::with_physical_optimizer_rule` · datafusion 55.1.0

```rust
fn with_physical_optimizer_rule(self, physical_optimizer_rule: Arc<dyn PhysicalOptimizerRule + Send + Sync>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::session_state::SessionStateBuilder", "path": "SessionStateBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1087, 1], "end": [1905, 2], "filename": "src/execution/session_state.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution/session_state.rs:1342`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Add `physical_optimizer_rule` to the end of the list of
[`PhysicalOptimizerRule`](../operations/datafusion_session.physical_optimizer.PhysicalOptimizerRule.md#op-266e99a7574020b03ac0e686)s used to rewrite queries.

<a id="op-5d49794e2e2cb522be8c1e62"></a>
## with_physical_optimizer_rules

`function` · `datafusion::execution::session_state::SessionStateBuilder::with_physical_optimizer_rules` · datafusion 55.1.0

```rust
fn with_physical_optimizer_rules(self, physical_optimizers: Vec<Arc<dyn PhysicalOptimizerRule + Send + Sync>>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::session_state::SessionStateBuilder", "path": "SessionStateBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1087, 1], "end": [1905, 2], "filename": "src/execution/session_state.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution/session_state.rs:1331`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Set the [`PhysicalOptimizerRule`](../operations/datafusion_session.physical_optimizer.PhysicalOptimizerRule.md#op-266e99a7574020b03ac0e686)s used to optimize plans.

<a id="op-72f1936ca44a5682ec530890"></a>
## with_query_planner

`function` · `datafusion::execution::session_state::SessionStateBuilder::with_query_planner` · datafusion 55.1.0

```rust
fn with_query_planner(self, query_planner: Arc<dyn QueryPlanner + Send + Sync>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::session_state::SessionStateBuilder", "path": "SessionStateBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1087, 1], "end": [1905, 2], "filename": "src/execution/session_state.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution/session_state.rs:1353`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Set the [`QueryPlanner`](../operations/datafusion_session.planner.QueryPlanner.md#op-d105e63a68841dd69dcdcc42)

<a id="op-9384b514d66f8e381d714848"></a>
## with_relation_planners

`function` · `datafusion::execution::session_state::SessionStateBuilder::with_relation_planners` · datafusion 55.1.0

```rust
fn with_relation_planners(self, relation_planners: Vec<Arc<dyn RelationPlanner>>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::session_state::SessionStateBuilder", "path": "SessionStateBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1087, 1], "end": [1905, 2], "filename": "src/execution/session_state.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution/session_state.rs:1315`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Sets the [`RelationPlanner`](../operations/datafusion_expr.planner.RelationPlanner.md#op-1be858ef3f752319489e5fc4)s used to customize SQL relation planning.

<a id="op-1e2f1996425e80b1842b3617"></a>
## with_runtime_env

`function` · `datafusion::execution::session_state::SessionStateBuilder::with_runtime_env` · datafusion 55.1.0

```rust
fn with_runtime_env(self, runtime_env: Arc<RuntimeEnv>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::session_state::SessionStateBuilder", "path": "SessionStateBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1087, 1], "end": [1905, 2], "filename": "src/execution/session_state.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution/session_state.rs:1495`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Set the [`RuntimeEnv`](../operations/datafusion_execution.runtime_env.RuntimeEnv.md#op-c598f4df4cf51824ae4b9a67)

<a id="op-98380118484a21ee3d56ab9e"></a>
## with_scalar_functions

`function` · `datafusion::execution::session_state::SessionStateBuilder::with_scalar_functions` · datafusion 55.1.0

```rust
fn with_scalar_functions(self, scalar_functions: Vec<Arc<ScalarUDF>>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::session_state::SessionStateBuilder", "path": "SessionStateBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1087, 1], "end": [1905, 2], "filename": "src/execution/session_state.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution/session_state.rs:1393`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Set the map of [`ScalarUDF`](../operations/datafusion_expr.udf.ScalarUDF.md#op-12eb8b815a2294fbbbd085a0)s

<a id="op-eb3d0595afb6f6350259c459"></a>
## with_serializer_registry

`function` · `datafusion::execution::session_state::SessionStateBuilder::with_serializer_registry` · datafusion 55.1.0

```rust
fn with_serializer_registry(self, serializer_registry: Arc<dyn SerializerRegistry>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::session_state::SessionStateBuilder", "path": "SessionStateBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1087, 1], "end": [1905, 2], "filename": "src/execution/session_state.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution/session_state.rs:1438`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Set the [`SerializerRegistry`](../operations/datafusion_expr.registry.SerializerRegistry.md#op-815a1f89b969d5eb430d928d)

<a id="op-7278ceca3c47bb220408d2a4"></a>
## with_session_id

`function` · `datafusion::execution::session_state::SessionStateBuilder::with_session_id` · datafusion 55.1.0

```rust
fn with_session_id(self, session_id: String) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::session_state::SessionStateBuilder", "path": "SessionStateBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1087, 1], "end": [1905, 2], "filename": "src/execution/session_state.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution/session_state.rs:1257`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Set the session id.

<a id="op-eb20692fdeda045c10f77b4a"></a>
## with_statistics_registry

`function` · `datafusion::execution::session_state::SessionStateBuilder::with_statistics_registry` · datafusion 55.1.0

```rust
fn with_statistics_registry(self, registry: StatisticsRegistry) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::session_state::SessionStateBuilder", "path": "SessionStateBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1087, 1], "end": [1905, 2], "filename": "src/execution/session_state.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution/session_state.rs:1523`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Set a [`StatisticsRegistry`](../operations/datafusion_physical_plan.operator_statistics.StatisticsRegistry.md#op-058fa5423a9fbd82dda192ea) for pluggable statistics providers.

The registry allows physical optimizer rules to access enhanced statistics
(e.g., NDV overrides, histograms) beyond what is available from
`ExecutionPlan::partition_statistics()`.

<a id="op-49827da5cdc2dd7f87f510e7"></a>
## with_table_factories

`function` · `datafusion::execution::session_state::SessionStateBuilder::with_table_factories` · datafusion 55.1.0

```rust
fn with_table_factories(self, table_factories: HashMap<String, Arc<dyn TableProviderFactory>>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::session_state::SessionStateBuilder", "path": "SessionStateBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1087, 1], "end": [1905, 2], "filename": "src/execution/session_state.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution/session_state.rs:1486`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Set the map of [`TableProviderFactory`](../operations/datafusion_session.table.TableProviderFactory.md#op-69227d35dfbf2f4aef45aae7)s

<a id="op-0e4a28c48315d28cd5d37a92"></a>
## with_table_factory

`function` · `datafusion::execution::session_state::SessionStateBuilder::with_table_factory` · datafusion 55.1.0

```rust
fn with_table_factory(self, key: String, table_factory: Arc<dyn TableProviderFactory>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::session_state::SessionStateBuilder", "path": "SessionStateBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1087, 1], "end": [1905, 2], "filename": "src/execution/session_state.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution/session_state.rs:1474`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Add a [`TableProviderFactory`](../operations/datafusion_session.table.TableProviderFactory.md#op-69227d35dfbf2f4aef45aae7) to the map of factories

<a id="op-9ec6e55f73d4a1071ba08afe"></a>
## with_table_function_list

`function` · `datafusion::execution::session_state::SessionStateBuilder::with_table_function_list` · datafusion 55.1.0

```rust
fn with_table_function_list(self, table_functions: Vec<Arc<TableFunction>>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::session_state::SessionStateBuilder", "path": "SessionStateBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1087, 1], "end": [1905, 2], "filename": "src/execution/session_state.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution/session_state.rs:1380`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Set the list of [`TableFunction`](../operations/datafusion_session.table.TableFunction.md#op-5f9bc702571844223c6d4a37)s

<a id="op-a7098545b0948a0ffca675ba"></a>
## with_table_functions

`function` · `datafusion::execution::session_state::SessionStateBuilder::with_table_functions` · datafusion 55.1.0

```rust
fn with_table_functions(self, table_functions: HashMap<String, Arc<TableFunction>>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::session_state::SessionStateBuilder", "path": "SessionStateBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1087, 1], "end": [1905, 2], "filename": "src/execution/session_state.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution/session_state.rs:1371`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Set the map of [`TableFunction`](../operations/datafusion_session.table.TableFunction.md#op-5f9bc702571844223c6d4a37)s

<a id="op-6118c0a870acb543ac1fb736"></a>
## with_table_options

`function` · `datafusion::execution::session_state::SessionStateBuilder::with_table_options` · datafusion 55.1.0

```rust
fn with_table_options(self, table_options: TableOptions) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::session_state::SessionStateBuilder", "path": "SessionStateBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1087, 1], "end": [1905, 2], "filename": "src/execution/session_state.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution/session_state.rs:1462`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Set the [`TableOptions`](../operations/datafusion_common.config.TableOptions.md#op-0523542c8dd1656aea13ac4e)

<a id="op-5ab6c0210fcc271b2b57e0a8"></a>
## with_type_planner

`function` · `datafusion::execution::session_state::SessionStateBuilder::with_type_planner` · datafusion 55.1.0

```rust
fn with_type_planner(self, type_planner: Arc<dyn TypePlanner>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::session_state::SessionStateBuilder", "path": "SessionStateBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1087, 1], "end": [1905, 2], "filename": "src/execution/session_state.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution/session_state.rs:1325`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Set the [`TypePlanner`](../operations/datafusion_expr.planner.TypePlanner.md#op-00ae998bdef35c3a41bed932) used to customize the behavior of the SQL planner.

<a id="op-a611e53373587c86d5cc6965"></a>
## with_window_functions

`function` · `datafusion::execution::session_state::SessionStateBuilder::with_window_functions` · datafusion 55.1.0

```rust
fn with_window_functions(self, window_functions: Vec<Arc<WindowUDF>>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::session_state::SessionStateBuilder", "path": "SessionStateBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1087, 1], "end": [1905, 2], "filename": "src/execution/session_state.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution/session_state.rs:1420`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Set the map of [`WindowUDF`](../operations/datafusion_expr.udwf.WindowUDF.md#op-42e8216e20a1c833692a7e65)s
