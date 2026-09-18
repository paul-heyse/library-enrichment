# `datafusion_expr::simplify::SimplifyContextBuilder`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.simplify.SimplifyContextBuilder.json).

<a id="op-d8c2f269615021b11b808b1c"></a>
## SimplifyContextBuilder

`struct` · `datafusion_expr::simplify::SimplifyContextBuilder` · datafusion-expr 55.1.0

```rust
struct SimplifyContextBuilder
```

Source: `src/simplify.rs:45`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Builder for [`SimplifyContext`](../operations/datafusion_expr.simplify.SimplifyContext.md#op-7b5998cee70553815ef2a10a).

<a id="op-a87ac53abe2c1c04b57331ac"></a>
## build

`function` · `datafusion_expr::simplify::SimplifyContextBuilder::build` · datafusion-expr 55.1.0

```rust
fn build(self) -> SimplifyContext
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::simplify::SimplifyContextBuilder", "path": "SimplifyContextBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [142, 1], "end": [180, 2], "filename": "src/simplify.rs"}, "trait": null, "trait_path": null}`

Source: `src/simplify.rs:171`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Build a [`SimplifyContext`](../operations/datafusion_expr.simplify.SimplifyContext.md#op-7b5998cee70553815ef2a10a), filling in any unspecified fields with defaults.

<a id="op-b88bec832e267bfa26bb34b2"></a>
## default

`function` · `datafusion_expr::simplify::SimplifyContextBuilder::default` · datafusion-expr 55.1.0

```rust
fn default() -> SimplifyContextBuilder
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::simplify::SimplifyContextBuilder", "path": "SimplifyContextBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [44, 17], "end": [44, 24], "filename": "src/simplify.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/simplify.rs:44`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8f245247c726a59ca8a68941"></a>
## fmt

`function` · `datafusion_expr::simplify::SimplifyContextBuilder::fmt` · datafusion-expr 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::simplify::SimplifyContextBuilder", "path": "SimplifyContextBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [44, 10], "end": [44, 15], "filename": "src/simplify.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/simplify.rs:44`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f9237b336841ed333dd0d295"></a>
## with_config_options

`function` · `datafusion_expr::simplify::SimplifyContextBuilder::with_config_options` · datafusion-expr 55.1.0

```rust
fn with_config_options(self, config_options: Arc<ConfigOptions>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::simplify::SimplifyContextBuilder", "path": "SimplifyContextBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [142, 1], "end": [180, 2], "filename": "src/simplify.rs"}, "trait": null, "trait_path": null}`

Source: `src/simplify.rs:144`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Set the [`ConfigOptions`](../operations/datafusion_common.config.ConfigOptions.md#op-0fede8afa640e38e337e0cc4) for this context.

<a id="op-e44fba504469a9dc385f344d"></a>
## with_current_time

`function` · `datafusion_expr::simplify::SimplifyContextBuilder::with_current_time` · datafusion-expr 55.1.0

```rust
fn with_current_time(self) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::simplify::SimplifyContextBuilder", "path": "SimplifyContextBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [142, 1], "end": [180, 2], "filename": "src/simplify.rs"}, "trait": null, "trait_path": null}`

Source: `src/simplify.rs:165`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Set the query execution start to the current time.

<a id="op-adc8837ddf577fbd49daabe8"></a>
## with_query_execution_start_time

`function` · `datafusion_expr::simplify::SimplifyContextBuilder::with_query_execution_start_time` · datafusion-expr 55.1.0

```rust
fn with_query_execution_start_time(self, query_execution_start_time: Option<DateTime<Utc>>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::simplify::SimplifyContextBuilder", "path": "SimplifyContextBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [142, 1], "end": [180, 2], "filename": "src/simplify.rs"}, "trait": null, "trait_path": null}`

Source: `src/simplify.rs:156`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Set the query execution start time.

<a id="op-c6d5ddf7290ae41842a2417a"></a>
## with_schema

`function` · `datafusion_expr::simplify::SimplifyContextBuilder::with_schema` · datafusion-expr 55.1.0

```rust
fn with_schema(self, schema: DFSchemaRef) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::simplify::SimplifyContextBuilder", "path": "SimplifyContextBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [142, 1], "end": [180, 2], "filename": "src/simplify.rs"}, "trait": null, "trait_path": null}`

Source: `src/simplify.rs:150`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Set the schema for this context.
