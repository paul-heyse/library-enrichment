# `sqlparser::ast::WindowSpec`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.WindowSpec.json).

<a id="op-78e6e24034c6fc5825b1b7a8"></a>
## WindowSpec

`struct` · `sqlparser::ast::WindowSpec` · sqlparser 0.62.0

```rust
struct WindowSpec
```

Source: `src/ast/mod.rs:2265`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

A window specification (i.e. `OVER ([window_name] PARTITION BY .. ORDER BY .. etc.)`)

<a id="op-a1211671caba7f1cb8887501"></a>
## clone

`function` · `sqlparser::ast::WindowSpec::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> WindowSpec
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::WindowSpec", "path": "WindowSpec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2262, 17], "end": [2262, 22], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/mod.rs:2262`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e1a1ab6efd28ac560fce28e6"></a>
## cmp

`function` · `sqlparser::ast::WindowSpec::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &WindowSpec) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::WindowSpec", "path": "WindowSpec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2262, 51], "end": [2262, 54], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/mod.rs:2262`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6a2266dc0f4e9d79781140a6"></a>
## deserialize

`function` · `sqlparser::ast::WindowSpec::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::WindowSpec", "path": "WindowSpec"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [2263, 49], "end": [2263, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/mod.rs:2263`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3a0338f7400687f96b5912b3"></a>
## eq

`function` · `sqlparser::ast::WindowSpec::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &WindowSpec) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::WindowSpec", "path": "WindowSpec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2262, 24], "end": [2262, 33], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/mod.rs:2262`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-be05706e635730b9597e2e3d"></a>
## fmt

`function` · `sqlparser::ast::WindowSpec::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::WindowSpec", "path": "WindowSpec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2262, 10], "end": [2262, 15], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/mod.rs:2262`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c00cb84b7e03320384bc4a03"></a>
## fmt

`function` · `sqlparser::ast::WindowSpec::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::WindowSpec", "path": "WindowSpec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2282, 1], "end": [2326, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/mod.rs:2283`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d7b5cdaadf8fceb0f0755fc0"></a>
## hash

`function` · `sqlparser::ast::WindowSpec::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::WindowSpec", "path": "WindowSpec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2262, 56], "end": [2262, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/mod.rs:2262`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-03c0da86edd5bde786abc8a6"></a>
## order_by

`struct_field` · `sqlparser::ast::WindowSpec::order_by` · sqlparser 0.62.0

```rust
order_by: Vec<OrderByExpr>
```

Source: `src/ast/mod.rs:2277`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`OVER (ORDER BY ...)`

<a id="op-94b833ff36276db851d83b1e"></a>
## partial_cmp

`function` · `sqlparser::ast::WindowSpec::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &WindowSpec) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::WindowSpec", "path": "WindowSpec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2262, 35], "end": [2262, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/mod.rs:2262`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-67f660d37c6e83e268ea04e8"></a>
## partition_by

`struct_field` · `sqlparser::ast::WindowSpec::partition_by` · sqlparser 0.62.0

```rust
partition_by: Vec<Expr>
```

Source: `src/ast/mod.rs:2275`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`OVER (PARTITION BY ...)`

<a id="op-477b71f94922bf1dc96bc356"></a>
## serialize

`function` · `sqlparser::ast::WindowSpec::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::WindowSpec", "path": "WindowSpec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2263, 38], "end": [2263, 47], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/mod.rs:2263`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9b63702ce45b4d9d3afc06cf"></a>
## visit

`function` · `sqlparser::ast::WindowSpec::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::WindowSpec", "path": "WindowSpec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2264, 47], "end": [2264, 55], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/mod.rs:2264`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ee4f60e02c29e31f9d181f2d"></a>
## visit

`function` · `sqlparser::ast::WindowSpec::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::WindowSpec", "path": "WindowSpec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2264, 40], "end": [2264, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/mod.rs:2264`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d385d95a44bb57d38906f269"></a>
## window_frame

`struct_field` · `sqlparser::ast::WindowSpec::window_frame` · sqlparser 0.62.0

```rust
window_frame: Option<WindowFrame>
```

Source: `src/ast/mod.rs:2279`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`OVER (window frame)`

<a id="op-6e6b0d1203379f9529a614b1"></a>
## window_name

`struct_field` · `sqlparser::ast::WindowSpec::window_name` · sqlparser 0.62.0

```rust
window_name: Option<Ident>
```

Source: `src/ast/mod.rs:2273`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional window name.

You can find it at least in [MySQL][1], [BigQuery][2], [PostgreSQL][3]

[1]: https://dev.mysql.com/doc/refman/8.0/en/window-functions-named-windows.html
[2]: https://cloud.google.com/bigquery/docs/reference/standard-sql/window-function-calls
[3]: https://www.postgresql.org/docs/current/sql-expressions.html#SYNTAX-WINDOW-FUNCTIONS
