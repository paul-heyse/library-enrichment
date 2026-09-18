# `sqlparser::ast::Method`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.Method.json).

<a id="op-a20f490413c453056bf1fa98"></a>
## Method

`struct` · `sqlparser::ast::Method` · sqlparser 0.62.0

```rust
struct Method
```

Source: `src/ast/mod.rs:8263`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

A method call

<a id="op-9168e370c17f27ceb183a583"></a>
## clone

`function` · `sqlparser::ast::Method::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> Method
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Method", "path": "Method"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8260, 17], "end": [8260, 22], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/mod.rs:8260`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c5c4d5ca1a5fe8f86bfbb5d6"></a>
## cmp

`function` · `sqlparser::ast::Method::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &Method) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Method", "path": "Method"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8260, 51], "end": [8260, 54], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/mod.rs:8260`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-05615bd3cc7176b9bbf6d7a4"></a>
## deserialize

`function` · `sqlparser::ast::Method::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Method", "path": "Method"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [8261, 49], "end": [8261, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/mod.rs:8261`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7c3c80e2cfde2fafc56b2e7f"></a>
## eq

`function` · `sqlparser::ast::Method::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &Method) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Method", "path": "Method"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8260, 24], "end": [8260, 33], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/mod.rs:8260`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9677816afdfe3e88621ecf7a"></a>
## expr

`struct_field` · `sqlparser::ast::Method::expr` · sqlparser 0.62.0

```rust
expr: Box<Expr>
```

Source: `src/ast/mod.rs:8265`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The expression on which the method is invoked.

<a id="op-202f9b6e7b5ad42efc58f7fa"></a>
## fmt

`function` · `sqlparser::ast::Method::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Method", "path": "Method"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8271, 1], "end": [8280, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/mod.rs:8272`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-eda9aa47d38f261953697266"></a>
## fmt

`function` · `sqlparser::ast::Method::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Method", "path": "Method"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8260, 10], "end": [8260, 15], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/mod.rs:8260`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d948187e9ef5336db148393e"></a>
## hash

`function` · `sqlparser::ast::Method::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Method", "path": "Method"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8260, 56], "end": [8260, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/mod.rs:8260`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-91b253d814c1ce5a84d4c410"></a>
## method_chain

`struct_field` · `sqlparser::ast::Method::method_chain` · sqlparser 0.62.0

```rust
method_chain: Vec<Function>
```

Source: `src/ast/mod.rs:8268`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The sequence of chained method calls.

<a id="op-170aaed31e98863a4f3cabdf"></a>
## partial_cmp

`function` · `sqlparser::ast::Method::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &Method) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Method", "path": "Method"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8260, 35], "end": [8260, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/mod.rs:8260`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-759eff434dabaa5eb95d2c0e"></a>
## serialize

`function` · `sqlparser::ast::Method::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Method", "path": "Method"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8261, 38], "end": [8261, 47], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/mod.rs:8261`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-03ae542d987b2871d9d553c4"></a>
## visit

`function` · `sqlparser::ast::Method::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Method", "path": "Method"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8262, 47], "end": [8262, 55], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/mod.rs:8262`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-795a09b2d0f4166299eca9cf"></a>
## visit

`function` · `sqlparser::ast::Method::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Method", "path": "Method"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8262, 40], "end": [8262, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/mod.rs:8262`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
