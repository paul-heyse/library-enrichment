# `sqlparser::ast::MacroArg`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.MacroArg.json).

<a id="op-cad8fdbcfe12ff645c655e3e"></a>
## MacroArg

`struct` · `sqlparser::ast::MacroArg` · sqlparser 0.62.0

```rust
struct MacroArg
```

Source: `src/ast/mod.rs:10239`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`NAME = <EXPR>` arguments for DuckDB macros

See [Create Macro - DuckDB](https://duckdb.org/docs/sql/statements/create_macro)
for more details

<a id="op-8eff17754759b68b7769e9f7"></a>
## clone

`function` · `sqlparser::ast::MacroArg::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> MacroArg
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::MacroArg", "path": "MacroArg"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10236, 17], "end": [10236, 22], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/mod.rs:10236`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8c1dce50c185f3f3f6b44f49"></a>
## cmp

`function` · `sqlparser::ast::MacroArg::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &MacroArg) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::MacroArg", "path": "MacroArg"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10236, 51], "end": [10236, 54], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/mod.rs:10236`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5bc647d5fa338e0d1a1ebd1c"></a>
## default_expr

`struct_field` · `sqlparser::ast::MacroArg::default_expr` · sqlparser 0.62.0

```rust
default_expr: Option<Expr>
```

Source: `src/ast/mod.rs:10243`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional default expression for the argument.

<a id="op-38d80549b9ab28fa88f42897"></a>
## deserialize

`function` · `sqlparser::ast::MacroArg::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::MacroArg", "path": "MacroArg"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [10237, 49], "end": [10237, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/mod.rs:10237`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-52c384c650066b20ca104118"></a>
## eq

`function` · `sqlparser::ast::MacroArg::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &MacroArg) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::MacroArg", "path": "MacroArg"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10236, 24], "end": [10236, 33], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/mod.rs:10236`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-873a4b202dce15780e52f333"></a>
## fmt

`function` · `sqlparser::ast::MacroArg::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::MacroArg", "path": "MacroArg"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10236, 10], "end": [10236, 15], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/mod.rs:10236`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c1dbcb4562ac7bd6141302d8"></a>
## fmt

`function` · `sqlparser::ast::MacroArg::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::MacroArg", "path": "MacroArg"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10256, 1], "end": [10264, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/mod.rs:10257`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-04e0d0ac417c91f20ac137c3"></a>
## hash

`function` · `sqlparser::ast::MacroArg::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::MacroArg", "path": "MacroArg"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10236, 56], "end": [10236, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/mod.rs:10236`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6a132afe6024dd46c8a91660"></a>
## name

`struct_field` · `sqlparser::ast::MacroArg::name` · sqlparser 0.62.0

```rust
name: Ident
```

Source: `src/ast/mod.rs:10241`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The argument name.

<a id="op-b7bbcd99de09aa5af63084b7"></a>
## new

`function` · `sqlparser::ast::MacroArg::new` · sqlparser 0.62.0

```rust
fn new(name: &str) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::MacroArg", "path": "MacroArg"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10246, 1], "end": [10254, 2], "filename": "src/ast/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/ast/mod.rs:10248`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Returns an argument with name.

<a id="op-84882ae06d9d8e6125579c76"></a>
## partial_cmp

`function` · `sqlparser::ast::MacroArg::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &MacroArg) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::MacroArg", "path": "MacroArg"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10236, 35], "end": [10236, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/mod.rs:10236`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c447361ee04fe8a73e0d51e9"></a>
## serialize

`function` · `sqlparser::ast::MacroArg::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::MacroArg", "path": "MacroArg"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10237, 38], "end": [10237, 47], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/mod.rs:10237`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1ca5cd65562ddc0a61e2482c"></a>
## visit

`function` · `sqlparser::ast::MacroArg::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::MacroArg", "path": "MacroArg"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10238, 40], "end": [10238, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/mod.rs:10238`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-99c8a2424e3fca126f516f1c"></a>
## visit

`function` · `sqlparser::ast::MacroArg::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::MacroArg", "path": "MacroArg"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10238, 47], "end": [10238, 55], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/mod.rs:10238`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
