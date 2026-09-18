# `sqlparser::ast::FunctionArg`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.FunctionArg.json).

<a id="op-3454b34e4ba34a97b7d33446"></a>
## FunctionArg

`enum` · `sqlparser::ast::FunctionArg` · sqlparser 0.62.0

```rust
enum FunctionArg
```

Source: `src/ast/mod.rs:7917`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Forms of function arguments (named, expression-named, or positional).

<a id="op-555527c00d23ee6568405478"></a>
## ExprNamed

`variant` · `sqlparser::ast::FunctionArg::ExprNamed` · sqlparser 0.62.0

```rust
ExprNamed
```

Source: `src/ast/mod.rs:7932`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`name` is arbitrary expression

Enabled when `Dialect::supports_named_fn_args_with_expr_name` returns 'true'

<a id="op-c30246fc7a1f8c4a7504ba07"></a>
## Named

`variant` · `sqlparser::ast::FunctionArg::Named` · sqlparser 0.62.0

```rust
Named
```

Source: `src/ast/mod.rs:7921`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`name` is identifier

Enabled when `Dialect::supports_named_fn_args_with_expr_name` returns 'false'

<a id="op-625291ceab0ef63ae7624cf2"></a>
## Unnamed

`variant` · `sqlparser::ast::FunctionArg::Unnamed` · sqlparser 0.62.0

```rust
Unnamed
```

Source: `src/ast/mod.rs:7941`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

An unnamed argument (positional), given by expression or wildcard.

<a id="op-16d6b80d3d816bae53291e1b"></a>
## clone

`function` · `sqlparser::ast::FunctionArg::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> FunctionArg
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FunctionArg", "path": "FunctionArg"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [7913, 17], "end": [7913, 22], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/mod.rs:7913`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-eec70bb10472c5ee983eb419"></a>
## cmp

`function` · `sqlparser::ast::FunctionArg::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &FunctionArg) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FunctionArg", "path": "FunctionArg"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [7913, 51], "end": [7913, 54], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/mod.rs:7913`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2f640f9872a5f7970b5f5bc5"></a>
## deserialize

`function` · `sqlparser::ast::FunctionArg::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FunctionArg", "path": "FunctionArg"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [7914, 49], "end": [7914, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/mod.rs:7914`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9ac367dd20e26f09758fb787"></a>
## eq

`function` · `sqlparser::ast::FunctionArg::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &FunctionArg) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FunctionArg", "path": "FunctionArg"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [7913, 24], "end": [7913, 33], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/mod.rs:7913`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-05b31b99487112ad2b72f9e1"></a>
## fmt

`function` · `sqlparser::ast::FunctionArg::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FunctionArg", "path": "FunctionArg"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [7913, 10], "end": [7913, 15], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/mod.rs:7913`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3565de2b22ef565338b7ad06"></a>
## fmt

`function` · `sqlparser::ast::FunctionArg::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FunctionArg", "path": "FunctionArg"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [7944, 1], "end": [7960, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/mod.rs:7945`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-965cd2e21387aae0b7fad504"></a>
## hash

`function` · `sqlparser::ast::FunctionArg::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FunctionArg", "path": "FunctionArg"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [7913, 56], "end": [7913, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/mod.rs:7913`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1c219bfa813e2baa9d3e8ab9"></a>
## partial_cmp

`function` · `sqlparser::ast::FunctionArg::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &FunctionArg) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FunctionArg", "path": "FunctionArg"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [7913, 35], "end": [7913, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/mod.rs:7913`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-29d2a6df85efc1cf0bba66f8"></a>
## serialize

`function` · `sqlparser::ast::FunctionArg::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FunctionArg", "path": "FunctionArg"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [7914, 38], "end": [7914, 47], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/mod.rs:7914`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-01968dae41f71ab0b5fac92b"></a>
## span

`function` · `sqlparser::ast::FunctionArg::span` · sqlparser 0.62.0

```rust
fn span(&self) -> Span
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FunctionArg", "path": "super::FunctionArg"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2147, 1], "end": [2163, 2], "filename": "src/ast/spans.rs"}, "trait": {"args": null, "id": "sqlparser::ast::spans::Spanned", "path": "Spanned"}, "trait_path": "sqlparser::ast::spans::Spanned"}`

Source: `src/ast/spans.rs:2148`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3e501627b446151f600af5b2"></a>
## visit

`function` · `sqlparser::ast::FunctionArg::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FunctionArg", "path": "FunctionArg"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [7915, 40], "end": [7915, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/mod.rs:7915`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7d4f71b6383ab69d4dd3d48d"></a>
## visit

`function` · `sqlparser::ast::FunctionArg::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FunctionArg", "path": "FunctionArg"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [7915, 47], "end": [7915, 55], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/mod.rs:7915`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
