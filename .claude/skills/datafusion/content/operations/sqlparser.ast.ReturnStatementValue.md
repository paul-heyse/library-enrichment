# `sqlparser::ast::ReturnStatementValue`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.ReturnStatementValue.json).

<a id="op-f1001b9f462a19e9c3a5dd86"></a>
## ReturnStatementValue

`enum` · `sqlparser::ast::ReturnStatementValue` · sqlparser 0.62.0

```rust
enum ReturnStatementValue
```

Source: `src/ast/mod.rs:11364`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Variants of a `RETURN` statement

<a id="op-b3878fdfb739ec2844865a5c"></a>
## Expr

`variant` · `sqlparser::ast::ReturnStatementValue::Expr` · sqlparser 0.62.0

```rust
Expr
```

Source: `src/ast/mod.rs:11366`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Return an expression from a function or trigger.

<a id="op-5ec6b7f66f42404f8314b2b0"></a>
## clone

`function` · `sqlparser::ast::ReturnStatementValue::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> ReturnStatementValue
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ReturnStatementValue", "path": "ReturnStatementValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11361, 17], "end": [11361, 22], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/mod.rs:11361`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e11233482199f6d905ea0bcd"></a>
## cmp

`function` · `sqlparser::ast::ReturnStatementValue::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &ReturnStatementValue) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ReturnStatementValue", "path": "ReturnStatementValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11361, 51], "end": [11361, 54], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/mod.rs:11361`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-51d1eb444fa3d29d5199e022"></a>
## deserialize

`function` · `sqlparser::ast::ReturnStatementValue::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ReturnStatementValue", "path": "ReturnStatementValue"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [11362, 49], "end": [11362, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/mod.rs:11362`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1b01f5c2d1a7593866b29165"></a>
## eq

`function` · `sqlparser::ast::ReturnStatementValue::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &ReturnStatementValue) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ReturnStatementValue", "path": "ReturnStatementValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11361, 24], "end": [11361, 33], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/mod.rs:11361`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3d42349652131dc4ab8e59ca"></a>
## fmt

`function` · `sqlparser::ast::ReturnStatementValue::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ReturnStatementValue", "path": "ReturnStatementValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11361, 10], "end": [11361, 15], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/mod.rs:11361`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a8b2c48ff05ed647e7d879f0"></a>
## hash

`function` · `sqlparser::ast::ReturnStatementValue::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ReturnStatementValue", "path": "ReturnStatementValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11361, 56], "end": [11361, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/mod.rs:11361`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bacce79dff14975a2df5fa81"></a>
## partial_cmp

`function` · `sqlparser::ast::ReturnStatementValue::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &ReturnStatementValue) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ReturnStatementValue", "path": "ReturnStatementValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11361, 35], "end": [11361, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/mod.rs:11361`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ace0f7e659e6e5784ad920ec"></a>
## serialize

`function` · `sqlparser::ast::ReturnStatementValue::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ReturnStatementValue", "path": "ReturnStatementValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11362, 38], "end": [11362, 47], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/mod.rs:11362`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-22129a67ac3af7d6bb0abb4d"></a>
## visit

`function` · `sqlparser::ast::ReturnStatementValue::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ReturnStatementValue", "path": "ReturnStatementValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11363, 47], "end": [11363, 55], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/mod.rs:11363`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-32f882d9f8d919ba54aa035c"></a>
## visit

`function` · `sqlparser::ast::ReturnStatementValue::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ReturnStatementValue", "path": "ReturnStatementValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11363, 40], "end": [11363, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/mod.rs:11363`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
