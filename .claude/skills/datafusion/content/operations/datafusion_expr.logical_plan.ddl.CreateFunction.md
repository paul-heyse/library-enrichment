# `datafusion_expr::logical_plan::ddl::CreateFunction`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.logical_plan.ddl.CreateFunction.json).

<a id="op-3668c6565f5901a6967c489f"></a>
## CreateFunction

`struct` · `datafusion_expr::logical_plan::ddl::CreateFunction` · datafusion-expr 55.1.0

```rust
struct CreateFunction
```

Source: `src/logical_plan/ddl.rs:643`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Arguments passed to the `CREATE FUNCTION` statement

These statements are turned into executable functions using [`FunctionFactory`]

# Notes

This structure purposely mirrors the structure in sqlparser's
[`sqlparser::ast::Statement::CreateFunction`](../operations/sqlparser.ast.Statement.md#op-93a71ee268befe396d8e0d06), but does not use it directly
to avoid a dependency on sqlparser in the core crate.


[`FunctionFactory`]: https://docs.rs/datafusion/latest/datafusion/execution/context/trait.FunctionFactory.html

<a id="op-1a7d44d3753f0d05b95c35ac"></a>
## args

`struct_field` · `datafusion_expr::logical_plan::ddl::CreateFunction::args` · datafusion-expr 55.1.0

```rust
args: Option<Vec<OperateFunctionArg>>
```

Source: `src/logical_plan/ddl.rs:647`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-47d2c2efee2c19bac1d0eefb"></a>
## clone

`function` · `datafusion_expr::logical_plan::ddl::CreateFunction::clone` · datafusion-expr 55.1.0

```rust
fn clone(&self) -> CreateFunction
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::ddl::CreateFunction", "path": "CreateFunction"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [642, 10], "end": [642, 15], "filename": "src/logical_plan/ddl.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/logical_plan/ddl.rs:642`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5f7ff5b03890b3f64a5fc6c7"></a>
## eq

`function` · `datafusion_expr::logical_plan::ddl::CreateFunction::eq` · datafusion-expr 55.1.0

```rust
fn eq(&self, other: &CreateFunction) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::ddl::CreateFunction", "path": "CreateFunction"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [642, 17], "end": [642, 26], "filename": "src/logical_plan/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/logical_plan/ddl.rs:642`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-77e37a12a0df2ec337c2802b"></a>
## fmt

`function` · `datafusion_expr::logical_plan::ddl::CreateFunction::fmt` · datafusion-expr 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::ddl::CreateFunction", "path": "CreateFunction"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [642, 38], "end": [642, 43], "filename": "src/logical_plan/ddl.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/logical_plan/ddl.rs:642`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-733fa066441f6a43e5f7c856"></a>
## hash

`function` · `datafusion_expr::logical_plan::ddl::CreateFunction::hash` · datafusion-expr 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::ddl::CreateFunction", "path": "CreateFunction"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [642, 32], "end": [642, 36], "filename": "src/logical_plan/ddl.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/logical_plan/ddl.rs:642`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fe6ac9853e956d3a8bae117e"></a>
## name

`struct_field` · `datafusion_expr::logical_plan::ddl::CreateFunction::name` · datafusion-expr 55.1.0

```rust
name: String
```

Source: `src/logical_plan/ddl.rs:646`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-234c2926d95d228922d1674c"></a>
## or_replace

`struct_field` · `datafusion_expr::logical_plan::ddl::CreateFunction::or_replace` · datafusion-expr 55.1.0

```rust
or_replace: bool
```

Source: `src/logical_plan/ddl.rs:644`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-88fc6089b3bfd01517a797ba"></a>
## params

`struct_field` · `datafusion_expr::logical_plan::ddl::CreateFunction::params` · datafusion-expr 55.1.0

```rust
params: CreateFunctionBody
```

Source: `src/logical_plan/ddl.rs:649`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6250a5153cd2e548bb6ba07c"></a>
## partial_cmp

`function` · `datafusion_expr::logical_plan::ddl::CreateFunction::partial_cmp` · datafusion-expr 55.1.0

```rust
fn partial_cmp(&self, other: &Self) -> Option<Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::ddl::CreateFunction", "path": "CreateFunction"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [655, 1], "end": [687, 2], "filename": "src/logical_plan/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/logical_plan/ddl.rs:656`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9eab65868bb49f1f221747f5"></a>
## return_type

`struct_field` · `datafusion_expr::logical_plan::ddl::CreateFunction::return_type` · datafusion-expr 55.1.0

```rust
return_type: Option<arrow::datatypes::DataType>
```

Source: `src/logical_plan/ddl.rs:648`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cdf1bd7782337e8c92080f5b"></a>
## schema

`struct_field` · `datafusion_expr::logical_plan::ddl::CreateFunction::schema` · datafusion-expr 55.1.0

```rust
schema: datafusion_common::DFSchemaRef
```

Source: `src/logical_plan/ddl.rs:651`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Dummy schema

<a id="op-35df1e89ea764b87da3930b3"></a>
## temporary

`struct_field` · `datafusion_expr::logical_plan::ddl::CreateFunction::temporary` · datafusion-expr 55.1.0

```rust
temporary: bool
```

Source: `src/logical_plan/ddl.rs:645`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
