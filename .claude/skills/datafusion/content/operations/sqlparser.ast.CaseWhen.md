# `sqlparser::ast::CaseWhen`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.CaseWhen.json).

<a id="op-dc021df8ab0b3f6ec247b971"></a>
## CaseWhen

`struct` · `sqlparser::ast::CaseWhen` · sqlparser 0.62.0

```rust
struct CaseWhen
```

Source: `src/ast/mod.rs:829`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

A WHEN clause in a CASE expression containing both
the condition and its corresponding result

<a id="op-20866235b2f03d52f4914420"></a>
## clone

`function` · `sqlparser::ast::CaseWhen::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> CaseWhen
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CaseWhen", "path": "CaseWhen"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [826, 17], "end": [826, 22], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/mod.rs:826`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2a244286e8bab441b4019c7c"></a>
## cmp

`function` · `sqlparser::ast::CaseWhen::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &CaseWhen) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CaseWhen", "path": "CaseWhen"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [826, 51], "end": [826, 54], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/mod.rs:826`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-02d9f9da4618589595382686"></a>
## condition

`struct_field` · `sqlparser::ast::CaseWhen::condition` · sqlparser 0.62.0

```rust
condition: Expr
```

Source: `src/ast/mod.rs:831`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The `WHEN` condition expression.

<a id="op-d3cec9741bdc91a2c86faaa2"></a>
## deserialize

`function` · `sqlparser::ast::CaseWhen::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CaseWhen", "path": "CaseWhen"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [827, 49], "end": [827, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/mod.rs:827`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0cddf9fae1939acddca5b7ba"></a>
## eq

`function` · `sqlparser::ast::CaseWhen::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &CaseWhen) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CaseWhen", "path": "CaseWhen"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [826, 24], "end": [826, 33], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/mod.rs:826`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-48a5a0ab53317e953f30876d"></a>
## fmt

`function` · `sqlparser::ast::CaseWhen::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CaseWhen", "path": "CaseWhen"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [836, 1], "end": [845, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/mod.rs:837`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fe66d3aeff01a50d4a163279"></a>
## fmt

`function` · `sqlparser::ast::CaseWhen::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CaseWhen", "path": "CaseWhen"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [826, 10], "end": [826, 15], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/mod.rs:826`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-05ae671ee8290a4a73099ecf"></a>
## hash

`function` · `sqlparser::ast::CaseWhen::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CaseWhen", "path": "CaseWhen"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [826, 56], "end": [826, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/mod.rs:826`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c5a3f6c547cd4712775119c7"></a>
## partial_cmp

`function` · `sqlparser::ast::CaseWhen::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &CaseWhen) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CaseWhen", "path": "CaseWhen"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [826, 35], "end": [826, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/mod.rs:826`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f67c1356c151936fc57b8440"></a>
## result

`struct_field` · `sqlparser::ast::CaseWhen::result` · sqlparser 0.62.0

```rust
result: Expr
```

Source: `src/ast/mod.rs:833`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The expression returned when `condition` matches.

<a id="op-21cb4b2f8c2a9449cdb510f3"></a>
## serialize

`function` · `sqlparser::ast::CaseWhen::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CaseWhen", "path": "CaseWhen"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [827, 38], "end": [827, 47], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/mod.rs:827`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0fd41fe7a39ec4995933b6ac"></a>
## visit

`function` · `sqlparser::ast::CaseWhen::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CaseWhen", "path": "CaseWhen"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [828, 47], "end": [828, 55], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/mod.rs:828`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b6a46d0f572c630e08ca996e"></a>
## visit

`function` · `sqlparser::ast::CaseWhen::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CaseWhen", "path": "CaseWhen"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [828, 40], "end": [828, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/mod.rs:828`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
