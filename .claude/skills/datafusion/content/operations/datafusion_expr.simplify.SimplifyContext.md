# `datafusion_expr::simplify::SimplifyContext`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.simplify.SimplifyContext.json).

<a id="op-7b5998cee70553815ef2a10a"></a>
## SimplifyContext

`struct` · `datafusion_expr::simplify::SimplifyContext` · datafusion-expr 55.1.0

```rust
struct SimplifyContext
```

Source: `src/simplify.rs:37`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Provides simplification information based on schema, query execution time,
and configuration options.

# Example
See the `simplify_demo` in the [`expr_api` example]

[`expr_api` example]: https://github.com/apache/datafusion/blob/main/datafusion-examples/examples/query_planning/expr_api.rs

<a id="op-8d2274d10eab2626c55cce62"></a>
## builder

`function` · `datafusion_expr::simplify::SimplifyContext::builder` · datafusion-expr 55.1.0

```rust
fn builder() -> SimplifyContextBuilder
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::simplify::SimplifyContext", "path": "SimplifyContext"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [61, 1], "end": [140, 2], "filename": "src/simplify.rs"}, "trait": null, "trait_path": null}`

Source: `src/simplify.rs:63`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Returns a builder for [`SimplifyContext`](../operations/datafusion_expr.simplify.SimplifyContext.md#op-7b5998cee70553815ef2a10a).

<a id="op-c94f079a2a6d890006df7973"></a>
## clone

`function` · `datafusion_expr::simplify::SimplifyContext::clone` · datafusion-expr 55.1.0

```rust
fn clone(&self) -> SimplifyContext
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::simplify::SimplifyContext", "path": "SimplifyContext"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [36, 17], "end": [36, 22], "filename": "src/simplify.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/simplify.rs:36`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4be24f93063753339f6ff4c7"></a>
## config_options

`function` · `datafusion_expr::simplify::SimplifyContext::config_options` · datafusion-expr 55.1.0

```rust
fn config_options(&self) -> &Arc<ConfigOptions>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::simplify::SimplifyContext", "path": "SimplifyContext"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [61, 1], "end": [140, 2], "filename": "src/simplify.rs"}, "trait": null, "trait_path": null}`

Source: `src/simplify.rs:137`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Returns the configuration options for the session.

<a id="op-122601d9b5ddeafddbf85150"></a>
## default

`function` · `datafusion_expr::simplify::SimplifyContext::default` · datafusion-expr 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::simplify::SimplifyContext", "path": "SimplifyContext"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [51, 1], "end": [59, 2], "filename": "src/simplify.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/simplify.rs:52`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-eee8da0da3aa95c85d616b00"></a>
## fmt

`function` · `datafusion_expr::simplify::SimplifyContext::fmt` · datafusion-expr 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::simplify::SimplifyContext", "path": "SimplifyContext"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [36, 10], "end": [36, 15], "filename": "src/simplify.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/simplify.rs:36`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1787ff58e5c1c98d2155b42c"></a>
## get_data_type

`function` · `datafusion_expr::simplify::SimplifyContext::get_data_type` · datafusion-expr 55.1.0

```rust
fn get_data_type(&self, expr: &Expr) -> Result<DataType>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::simplify::SimplifyContext", "path": "SimplifyContext"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [61, 1], "end": [140, 2], "filename": "src/simplify.rs"}, "trait": null, "trait_path": null}`

Source: `src/simplify.rs:126`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Returns data type of this expr needed for determining optimized int type of a value

<a id="op-f34c3d33a52577e4cf1c8c96"></a>
## is_boolean_type

`function` · `datafusion_expr::simplify::SimplifyContext::is_boolean_type` · datafusion-expr 55.1.0

```rust
fn is_boolean_type(&self, expr: &Expr) -> Result<bool>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::simplify::SimplifyContext", "path": "SimplifyContext"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [61, 1], "end": [140, 2], "filename": "src/simplify.rs"}, "trait": null, "trait_path": null}`

Source: `src/simplify.rs:116`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Returns true if this Expr has boolean type

<a id="op-906d0845fef693539b4b6f62"></a>
## nullable

`function` · `datafusion_expr::simplify::SimplifyContext::nullable` · datafusion-expr 55.1.0

```rust
fn nullable(&self, expr: &Expr) -> Result<bool>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::simplify::SimplifyContext", "path": "SimplifyContext"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [61, 1], "end": [140, 2], "filename": "src/simplify.rs"}, "trait": null, "trait_path": null}`

Source: `src/simplify.rs:121`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Returns true if expr is nullable

<a id="op-aebe80f34c8fd24b13da8a2f"></a>
## query_execution_start_time

`function` · `datafusion_expr::simplify::SimplifyContext::query_execution_start_time` · datafusion-expr 55.1.0

```rust
fn query_execution_start_time(&self) -> Option<DateTime<Utc>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::simplify::SimplifyContext", "path": "SimplifyContext"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [61, 1], "end": [140, 2], "filename": "src/simplify.rs"}, "trait": null, "trait_path": null}`

Source: `src/simplify.rs:132`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Returns the time at which the query execution started.
If `None`, time-dependent functions like `now()` will not be simplified.

<a id="op-bf1422caac5132dbc52b249a"></a>
## schema

`function` · `datafusion_expr::simplify::SimplifyContext::schema` · datafusion-expr 55.1.0

```rust
fn schema(&self) -> &DFSchemaRef
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::simplify::SimplifyContext", "path": "SimplifyContext"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [61, 1], "end": [140, 2], "filename": "src/simplify.rs"}, "trait": null, "trait_path": null}`

Source: `src/simplify.rs:111`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Returns the schema

<a id="op-860e2b09c92b4c3aa1a08804"></a>
## with_config_options

`function` · `datafusion_expr::simplify::SimplifyContext::with_config_options` · datafusion-expr 55.1.0

```rust
fn with_config_options(self, config_options: Arc<ConfigOptions>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::simplify::SimplifyContext", "path": "SimplifyContext"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [61, 1], "end": [140, 2], "filename": "src/simplify.rs"}, "trait": null, "trait_path": null}`

Source: `src/simplify.rs:72`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Set the [`ConfigOptions`](../operations/datafusion_common.config.ConfigOptions.md#op-0fede8afa640e38e337e0cc4) for this context

<a id="op-78ce5e7da1476953e8f1ecca"></a>
## with_current_time

`function` · `datafusion_expr::simplify::SimplifyContext::with_current_time` · datafusion-expr 55.1.0

```rust
fn with_current_time(self) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::simplify::SimplifyContext", "path": "SimplifyContext"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [61, 1], "end": [140, 2], "filename": "src/simplify.rs"}, "trait": null, "trait_path": null}`

Source: `src/simplify.rs:105`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Set the query execution start to the current time

<a id="op-5c16faac8a76df061f15eab1"></a>
## with_query_execution_start_time

`function` · `datafusion_expr::simplify::SimplifyContext::with_query_execution_start_time` · datafusion-expr 55.1.0

```rust
fn with_query_execution_start_time(self, query_execution_start_time: Option<DateTime<Utc>>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::simplify::SimplifyContext", "path": "SimplifyContext"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [61, 1], "end": [140, 2], "filename": "src/simplify.rs"}, "trait": null, "trait_path": null}`

Source: `src/simplify.rs:92`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Set the query execution start time

<a id="op-9d4604922fa7c2b3295ab253"></a>
## with_schema

`function` · `datafusion_expr::simplify::SimplifyContext::with_schema` · datafusion-expr 55.1.0

```rust
fn with_schema(self, schema: DFSchemaRef) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::simplify::SimplifyContext", "path": "SimplifyContext"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [61, 1], "end": [140, 2], "filename": "src/simplify.rs"}, "trait": null, "trait_path": null}`

Source: `src/simplify.rs:82`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Set the schema for this context
