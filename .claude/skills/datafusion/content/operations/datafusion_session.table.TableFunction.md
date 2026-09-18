# `datafusion_session::table::TableFunction`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_session.table.TableFunction.json).

<a id="op-5f9bc702571844223c6d4a37"></a>
## TableFunction

`struct` · `datafusion_session::table::TableFunction` · datafusion-session 55.1.0

```rust
struct TableFunction
```

Source: `src/table.rs:622`. [Exact documentation build](https://docs.rs/crate/datafusion-session/55.1.0/json).

A table that uses a function to generate data

<a id="op-e5ba117f357b158f5b992f9a"></a>
## clone

`function` · `datafusion_session::table::TableFunction::clone` · datafusion-session 55.1.0

```rust
fn clone(&self) -> TableFunction
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_session::table::TableFunction", "path": "TableFunction"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [621, 10], "end": [621, 15], "filename": "src/table.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/table.rs:621`. [Exact documentation build](https://docs.rs/crate/datafusion-session/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d768ea0195a3a8763bec4327"></a>
## create_table_provider

`function` · `datafusion_session::table::TableFunction::create_table_provider` · datafusion-session 55.1.0

```rust
fn create_table_provider(&self, args: &[Expr]) -> Result<Arc<dyn TableProvider>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_session::table::TableFunction", "path": "TableFunction"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [629, 1], "end": [662, 2], "filename": "src/table.rs"}, "trait": null, "trait_path": null}`

Source: `src/table.rs:650`. [Exact documentation build](https://docs.rs/crate/datafusion-session/55.1.0/json).

Get the function implementation and generate a table

<a id="op-92ba53be470c4204018cf6aa"></a>
## create_table_provider_with_args

`function` · `datafusion_session::table::TableFunction::create_table_provider_with_args` · datafusion-session 55.1.0

```rust
fn create_table_provider_with_args(&self, args: TableFunctionArgs<'_, '_>) -> Result<Arc<dyn TableProvider>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_session::table::TableFunction", "path": "TableFunction"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [629, 1], "end": [662, 2], "filename": "src/table.rs"}, "trait": null, "trait_path": null}`

Source: `src/table.rs:656`. [Exact documentation build](https://docs.rs/crate/datafusion-session/55.1.0/json).

Get the function implementation and generate a table

<a id="op-905a8d6ee6e7b7f0ab4c092d"></a>
## fmt

`function` · `datafusion_session::table::TableFunction::fmt` · datafusion-session 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_session::table::TableFunction", "path": "TableFunction"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [621, 17], "end": [621, 22], "filename": "src/table.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/table.rs:621`. [Exact documentation build](https://docs.rs/crate/datafusion-session/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a3dc9d65e7a3e9251eca9c75"></a>
## function

`function` · `datafusion_session::table::TableFunction::function` · datafusion-session 55.1.0

```rust
fn function(&self) -> &Arc<dyn TableFunctionImpl>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_session::table::TableFunction", "path": "TableFunction"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [629, 1], "end": [662, 2], "filename": "src/table.rs"}, "trait": null, "trait_path": null}`

Source: `src/table.rs:641`. [Exact documentation build](https://docs.rs/crate/datafusion-session/55.1.0/json).

Get the implementation of the table function

<a id="op-04c490773fac8335227e595c"></a>
## name

`function` · `datafusion_session::table::TableFunction::name` · datafusion-session 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_session::table::TableFunction", "path": "TableFunction"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [629, 1], "end": [662, 2], "filename": "src/table.rs"}, "trait": null, "trait_path": null}`

Source: `src/table.rs:636`. [Exact documentation build](https://docs.rs/crate/datafusion-session/55.1.0/json).

Get the name of the table function

<a id="op-9761819cea1f5a7385a39bda"></a>
## new

`function` · `datafusion_session::table::TableFunction::new` · datafusion-session 55.1.0

```rust
fn new(name: String, fun: Arc<dyn TableFunctionImpl>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_session::table::TableFunction", "path": "TableFunction"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [629, 1], "end": [662, 2], "filename": "src/table.rs"}, "trait": null, "trait_path": null}`

Source: `src/table.rs:631`. [Exact documentation build](https://docs.rs/crate/datafusion-session/55.1.0/json).

Create a new table function
