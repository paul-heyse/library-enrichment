# `datafusion::execution::context::SQLOptions`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion.execution.context.SQLOptions.json).

<a id="op-45fe1731f80c6b691c6957b5"></a>
## SQLOptions

`struct` · `datafusion::execution::context::SQLOptions` · datafusion 55.1.0

```rust
struct SQLOptions
```

Source: `src/execution/context/mod.rs:2280`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Describes which SQL statements can be run.

See [`SessionContext::sql_with_options`](../operations/datafusion.execution.context.SessionContext.md#op-7e09781b035fd8617b201286) for more details.

<a id="op-916496c391d72484208b19d6"></a>
## clone

`function` · `datafusion::execution::context::SQLOptions::clone` · datafusion 55.1.0

```rust
fn clone(&self) -> SQLOptions
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::context::SQLOptions", "path": "SQLOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2279, 10], "end": [2279, 15], "filename": "src/execution/context/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/execution/context/mod.rs:2279`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f1e36590aa074e690b72c132"></a>
## default

`function` · `datafusion::execution::context::SQLOptions::default` · datafusion 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::context::SQLOptions", "path": "SQLOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2289, 1], "end": [2297, 2], "filename": "src/execution/context/mod.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/execution/context/mod.rs:2290`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b20d72411ffa17fed08975b8"></a>
## fmt

`function` · `datafusion::execution::context::SQLOptions::fmt` · datafusion 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::context::SQLOptions", "path": "SQLOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2279, 17], "end": [2279, 22], "filename": "src/execution/context/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/execution/context/mod.rs:2279`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b15578e4b620a1cc23673c33"></a>
## new

`function` · `datafusion::execution::context::SQLOptions::new` · datafusion 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::context::SQLOptions", "path": "SQLOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2299, 1], "end": [2329, 2], "filename": "src/execution/context/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution/context/mod.rs:2301`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Create a new `SQLOptions` with default values

<a id="op-93354441f7a2ec957b5995ae"></a>
## verify_plan

`function` · `datafusion::execution::context::SQLOptions::verify_plan` · datafusion 55.1.0

```rust
fn verify_plan(&self, plan: &LogicalPlan) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::context::SQLOptions", "path": "SQLOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2299, 1], "end": [2329, 2], "filename": "src/execution/context/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution/context/mod.rs:2325`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Return an error if the [`LogicalPlan`](../operations/datafusion_expr.logical_plan.plan.LogicalPlan.md#op-2f2092c4f87ff1cc0b33c3da) has any nodes that are
incompatible with this [`SQLOptions`](../operations/datafusion.execution.context.SQLOptions.md#op-45fe1731f80c6b691c6957b5).

<a id="op-5c77143c41df9d330632208e"></a>
## with_allow_ddl

`function` · `datafusion::execution::context::SQLOptions::with_allow_ddl` · datafusion 55.1.0

```rust
fn with_allow_ddl(self, allow: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::context::SQLOptions", "path": "SQLOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2299, 1], "end": [2329, 2], "filename": "src/execution/context/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution/context/mod.rs:2306`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Should DDL data definition commands  (e.g. `CREATE TABLE`) be run? Defaults to `true`.

<a id="op-efcdcbdc557df7b67848cb27"></a>
## with_allow_dml

`function` · `datafusion::execution::context::SQLOptions::with_allow_dml` · datafusion 55.1.0

```rust
fn with_allow_dml(self, allow: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::context::SQLOptions", "path": "SQLOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2299, 1], "end": [2329, 2], "filename": "src/execution/context/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution/context/mod.rs:2312`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Should DML data modification commands (e.g. `INSERT` and `COPY`) be run? Defaults to `true`

<a id="op-032c5064d4319632828c990c"></a>
## with_allow_statements

`function` · `datafusion::execution::context::SQLOptions::with_allow_statements` · datafusion 55.1.0

```rust
fn with_allow_statements(self, allow: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::context::SQLOptions", "path": "SQLOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2299, 1], "end": [2329, 2], "filename": "src/execution/context/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution/context/mod.rs:2318`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Should Statements such as (e.g. `SET VARIABLE and `BEGIN TRANSACTION` ...`) be run?. Defaults to `true`
