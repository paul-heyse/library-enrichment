# `datafusion::execution::session_state_defaults::SessionStateDefaults`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion.execution.session_state_defaults.SessionStateDefaults.json).

<a id="op-5e0edf79cb605531ad6983b6"></a>
## SessionStateDefaults

`struct` · `datafusion::execution::session_state_defaults::SessionStateDefaults` · datafusion 55.1.0

```rust
struct SessionStateDefaults
```

Source: `src/execution/session_state_defaults.rs:47`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Defaults that are used as part of creating a SessionState such as table providers,
file formats, registering of builtin functions, etc.

<a id="op-e84e9e9b00d45c62f309d73a"></a>
## default_aggregate_functions

`function` · `datafusion::execution::session_state_defaults::SessionStateDefaults::default_aggregate_functions` · datafusion 55.1.0

```rust
fn default_aggregate_functions() -> Vec<Arc<AggregateUDF>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::session_state_defaults::SessionStateDefaults", "path": "SessionStateDefaults"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [49, 1], "end": [243, 2], "filename": "src/execution/session_state_defaults.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution/session_state_defaults.rs:126`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

returns the list of default [`AggregateUDF`](../operations/datafusion_expr.udaf.AggregateUDF.md#op-d90e5a97479981a539718379)s

<a id="op-bd14a5196b656d82ae1afbdb"></a>
## default_catalog

`function` · `datafusion::execution::session_state_defaults::SessionStateDefaults::default_catalog` · datafusion 55.1.0

```rust
fn default_catalog(config: &SessionConfig, table_factories: &HashMap<String, Arc<dyn TableProviderFactory>>, runtime: &Arc<RuntimeEnv>) -> MemoryCatalogProvider
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::session_state_defaults::SessionStateDefaults", "path": "SessionStateDefaults"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [49, 1], "end": [243, 2], "filename": "src/execution/session_state_defaults.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution/session_state_defaults.rs:66`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

returns the default MemoryCatalogProvider

<a id="op-cc89e4314e34abd87ca88f52"></a>
## default_expr_planners

`function` · `datafusion::execution::session_state_defaults::SessionStateDefaults::default_expr_planners` · datafusion 55.1.0

```rust
fn default_expr_planners() -> Vec<Arc<dyn ExprPlanner>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::session_state_defaults::SessionStateDefaults", "path": "SessionStateDefaults"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [49, 1], "end": [243, 2], "filename": "src/execution/session_state_defaults.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution/session_state_defaults.rs:86`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

returns the list of default [`ExprPlanner`](../operations/datafusion_expr.planner.ExprPlanner.md#op-c0ce2d948f3fc34627ff15dd)s

<a id="op-32891a8bc736fcc7fb4ecf7f"></a>
## default_extension_types

`function` · `datafusion::execution::session_state_defaults::SessionStateDefaults::default_extension_types` · datafusion 55.1.0

```rust
fn default_extension_types() -> Vec<ExtensionTypeRegistrationRef>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::session_state_defaults::SessionStateDefaults", "path": "SessionStateDefaults"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [49, 1], "end": [243, 2], "filename": "src/execution/session_state_defaults.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution/session_state_defaults.rs:138`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Returns the list of default extension types.

For now, we do not register any extension types by default.

<a id="op-fde1577cf31eae44c5f28cf1"></a>
## default_file_formats

`function` · `datafusion::execution::session_state_defaults::SessionStateDefaults::default_file_formats` · datafusion 55.1.0

```rust
fn default_file_formats() -> Vec<Arc<dyn FileFormatFactory>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::session_state_defaults::SessionStateDefaults", "path": "SessionStateDefaults"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [49, 1], "end": [243, 2], "filename": "src/execution/session_state_defaults.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution/session_state_defaults.rs:148`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

returns the list of default [`FileFormatFactory`](../operations/datafusion_datasource.file_format.FileFormatFactory.md#op-f967009d6b2c0c52e0070973)s

<a id="op-cf70f3d1ea7d3d6f07058f4c"></a>
## default_higher_order_functions

`function` · `datafusion::execution::session_state_defaults::SessionStateDefaults::default_higher_order_functions` · datafusion 55.1.0

```rust
fn default_higher_order_functions() -> Vec<Arc<HigherOrderUDF>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::session_state_defaults::SessionStateDefaults", "path": "SessionStateDefaults"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [49, 1], "end": [243, 2], "filename": "src/execution/session_state_defaults.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution/session_state_defaults.rs:117`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

returns the list of default [`HigherOrderUDF`](../operations/datafusion_expr.higher_order_function.HigherOrderUDF.md#op-67b8632773d35bc58334e2bb)s

<a id="op-be498bc1898a8aca9faf1d77"></a>
## default_scalar_functions

`function` · `datafusion::execution::session_state_defaults::SessionStateDefaults::default_scalar_functions` · datafusion 55.1.0

```rust
fn default_scalar_functions() -> Vec<Arc<ScalarUDF>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::session_state_defaults::SessionStateDefaults", "path": "SessionStateDefaults"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [49, 1], "end": [243, 2], "filename": "src/execution/session_state_defaults.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution/session_state_defaults.rs:106`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

returns the list of default [`ScalarUDF`](../operations/datafusion_expr.udf.ScalarUDF.md#op-12eb8b815a2294fbbbd085a0)s

<a id="op-57c5fba4d16467f521545ce2"></a>
## default_table_factories

`function` · `datafusion::execution::session_state_defaults::SessionStateDefaults::default_table_factories` · datafusion 55.1.0

```rust
fn default_table_factories() -> HashMap<String, Arc<dyn TableProviderFactory>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::session_state_defaults::SessionStateDefaults", "path": "SessionStateDefaults"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [49, 1], "end": [243, 2], "filename": "src/execution/session_state_defaults.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution/session_state_defaults.rs:51`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

returns a map of the default [`TableProviderFactory`](../operations/datafusion_session.table.TableProviderFactory.md#op-69227d35dfbf2f4aef45aae7)s

<a id="op-6fc0149b42bd067c8857fc95"></a>
## default_table_functions

`function` · `datafusion::execution::session_state_defaults::SessionStateDefaults::default_table_functions` · datafusion 55.1.0

```rust
fn default_table_functions() -> Vec<Arc<TableFunction>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::session_state_defaults::SessionStateDefaults", "path": "SessionStateDefaults"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [49, 1], "end": [243, 2], "filename": "src/execution/session_state_defaults.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution/session_state_defaults.rs:143`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

returns the list of default [`TableFunction`](../operations/datafusion_session.table.TableFunction.md#op-5f9bc702571844223c6d4a37)s

<a id="op-501717151ef4eb60d6e45faf"></a>
## default_window_functions

`function` · `datafusion::execution::session_state_defaults::SessionStateDefaults::default_window_functions` · datafusion 55.1.0

```rust
fn default_window_functions() -> Vec<Arc<WindowUDF>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::session_state_defaults::SessionStateDefaults", "path": "SessionStateDefaults"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [49, 1], "end": [243, 2], "filename": "src/execution/session_state_defaults.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution/session_state_defaults.rs:131`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

returns the list of default [`WindowUDF`](../operations/datafusion_expr.udwf.WindowUDF.md#op-42e8216e20a1c833692a7e65)s

<a id="op-ef41a46e51e525804fdb64e3"></a>
## register_aggregate_functions

`function` · `datafusion::execution::session_state_defaults::SessionStateDefaults::register_aggregate_functions` · datafusion 55.1.0

```rust
fn register_aggregate_functions(state: &mut SessionState)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::session_state_defaults::SessionStateDefaults", "path": "SessionStateDefaults"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [49, 1], "end": [243, 2], "filename": "src/execution/session_state_defaults.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution/session_state_defaults.rs:184`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

registers all the builtin aggregate functions

<a id="op-c8b15f2019edf409332c3f0b"></a>
## register_array_functions

`function` · `datafusion::execution::session_state_defaults::SessionStateDefaults::register_array_functions` · datafusion 55.1.0

```rust
fn register_array_functions(state: &mut SessionState)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::session_state_defaults::SessionStateDefaults", "path": "SessionStateDefaults"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [49, 1], "end": [243, 2], "filename": "src/execution/session_state_defaults.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution/session_state_defaults.rs:176`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

registers all the builtin array functions

<a id="op-1f02ea4a520676176842cbaf"></a>
## register_builtin_functions

`function` · `datafusion::execution::session_state_defaults::SessionStateDefaults::register_builtin_functions` · datafusion 55.1.0

```rust
fn register_builtin_functions(state: &mut SessionState)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::session_state_defaults::SessionStateDefaults", "path": "SessionStateDefaults"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [49, 1], "end": [243, 2], "filename": "src/execution/session_state_defaults.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution/session_state_defaults.rs:163`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

registers all builtin functions - scalar, array and aggregate

<a id="op-4436c8421027390162e2b0a5"></a>
## register_default_file_formats

`function` · `datafusion::execution::session_state_defaults::SessionStateDefaults::register_default_file_formats` · datafusion 55.1.0

```rust
fn register_default_file_formats(state: &mut SessionState)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::session_state_defaults::SessionStateDefaults", "path": "SessionStateDefaults"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [49, 1], "end": [243, 2], "filename": "src/execution/session_state_defaults.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution/session_state_defaults.rs:235`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

registers the default [`FileFormatFactory`](../operations/datafusion_datasource.file_format.FileFormatFactory.md#op-f967009d6b2c0c52e0070973)s

<a id="op-245600db4de7bbb216d8ede7"></a>
## register_default_schema

`function` · `datafusion::execution::session_state_defaults::SessionStateDefaults::register_default_schema` · datafusion 55.1.0

```rust
fn register_default_schema(config: &SessionConfig, table_factories: &HashMap<String, Arc<dyn TableProviderFactory>>, runtime: &Arc<RuntimeEnv>, default_catalog: &MemoryCatalogProvider)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::session_state_defaults::SessionStateDefaults", "path": "SessionStateDefaults"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [49, 1], "end": [243, 2], "filename": "src/execution/session_state_defaults.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution/session_state_defaults.rs:190`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

registers the default schema

<a id="op-10ad0329b25607f321cbabb1"></a>
## register_scalar_functions

`function` · `datafusion::execution::session_state_defaults::SessionStateDefaults::register_scalar_functions` · datafusion 55.1.0

```rust
fn register_scalar_functions(state: &mut SessionState)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::session_state_defaults::SessionStateDefaults", "path": "SessionStateDefaults"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [49, 1], "end": [243, 2], "filename": "src/execution/session_state_defaults.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution/session_state_defaults.rs:170`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

registers all the builtin scalar functions
