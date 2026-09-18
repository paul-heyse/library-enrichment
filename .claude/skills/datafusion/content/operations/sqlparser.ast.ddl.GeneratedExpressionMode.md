# `sqlparser::ast::ddl::GeneratedExpressionMode`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.ddl.GeneratedExpressionMode.json).

<a id="op-80aa6d3948fd4a8ee46606b1"></a>
## GeneratedExpressionMode

`enum` · `sqlparser::ast::ddl::GeneratedExpressionMode` · sqlparser 0.62.0

```rust
enum GeneratedExpressionMode
```

Source: `src/ast/ddl.rs:2176`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`GeneratedExpressionMode`s are modifiers that follow an expression in a `generated`.
No modifier is typically the same as Virtual.

<a id="op-031d93c5498647378c39fa90"></a>
## Stored

`variant` · `sqlparser::ast::ddl::GeneratedExpressionMode::Stored` · sqlparser 0.62.0

```rust
Stored
```

Source: `src/ast/ddl.rs:2180`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`STORED` generated expression

<a id="op-0c4c7437c18cff32beca85ab"></a>
## Virtual

`variant` · `sqlparser::ast::ddl::GeneratedExpressionMode::Virtual` · sqlparser 0.62.0

```rust
Virtual
```

Source: `src/ast/ddl.rs:2178`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`VIRTUAL` generated expression

<a id="op-57d4209351bd751aecf92d1a"></a>
## clone

`function` · `sqlparser::ast::ddl::GeneratedExpressionMode::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> GeneratedExpressionMode
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::GeneratedExpressionMode", "path": "GeneratedExpressionMode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2173, 17], "end": [2173, 22], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/ddl.rs:2173`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-80a322b4ff10ff797bd0d780"></a>
## cmp

`function` · `sqlparser::ast::ddl::GeneratedExpressionMode::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &GeneratedExpressionMode) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::GeneratedExpressionMode", "path": "GeneratedExpressionMode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2173, 57], "end": [2173, 60], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/ddl.rs:2173`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-910e037ebf22402cccea6304"></a>
## deserialize

`function` · `sqlparser::ast::ddl::GeneratedExpressionMode::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::GeneratedExpressionMode", "path": "GeneratedExpressionMode"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [2174, 49], "end": [2174, 60], "filename": "src/ast/ddl.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/ddl.rs:2174`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-92dc724957845441d91e4e80"></a>
## eq

`function` · `sqlparser::ast::ddl::GeneratedExpressionMode::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &GeneratedExpressionMode) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::GeneratedExpressionMode", "path": "GeneratedExpressionMode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2173, 30], "end": [2173, 39], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/ddl.rs:2173`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0f9116705bd428177dbfeabc"></a>
## fmt

`function` · `sqlparser::ast::ddl::GeneratedExpressionMode::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::GeneratedExpressionMode", "path": "GeneratedExpressionMode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2173, 10], "end": [2173, 15], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/ddl.rs:2173`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e1eda1fe8722d3bcc2fde196"></a>
## hash

`function` · `sqlparser::ast::ddl::GeneratedExpressionMode::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::GeneratedExpressionMode", "path": "GeneratedExpressionMode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2173, 62], "end": [2173, 66], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/ddl.rs:2173`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9732e49e9a6d2bfa405efcde"></a>
## partial_cmp

`function` · `sqlparser::ast::ddl::GeneratedExpressionMode::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &GeneratedExpressionMode) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::GeneratedExpressionMode", "path": "GeneratedExpressionMode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2173, 41], "end": [2173, 51], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/ddl.rs:2173`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3299b9ee45c08cd438da9684"></a>
## serialize

`function` · `sqlparser::ast::ddl::GeneratedExpressionMode::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::GeneratedExpressionMode", "path": "GeneratedExpressionMode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2174, 38], "end": [2174, 47], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/ddl.rs:2174`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9528fbaae587e83c975dc4ed"></a>
## visit

`function` · `sqlparser::ast::ddl::GeneratedExpressionMode::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::GeneratedExpressionMode", "path": "GeneratedExpressionMode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2175, 47], "end": [2175, 55], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/ddl.rs:2175`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cea8496baab9b3a2669158a9"></a>
## visit

`function` · `sqlparser::ast::ddl::GeneratedExpressionMode::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::GeneratedExpressionMode", "path": "GeneratedExpressionMode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2175, 40], "end": [2175, 45], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/ddl.rs:2175`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
