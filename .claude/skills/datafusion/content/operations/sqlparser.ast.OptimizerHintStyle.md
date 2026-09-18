# `sqlparser::ast::OptimizerHintStyle`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.OptimizerHintStyle.json).

<a id="op-d4fc5bc273e5bf609f60283b"></a>
## OptimizerHintStyle

`enum` · `sqlparser::ast::OptimizerHintStyle` · sqlparser 0.62.0

```rust
enum OptimizerHintStyle
```

Source: `src/ast/mod.rs:12004`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The commentary style of an [optimizer hint](OptimizerHint)

<a id="op-110d9e40a5cf527cdadb8837"></a>
## MultiLine

`variant` · `sqlparser::ast::OptimizerHintStyle::MultiLine` · sqlparser 0.62.0

```rust
MultiLine
```

Source: `src/ast/mod.rs:12013`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

A hint corresponding to a multi line comment,
e.g. `/*+ LEADING(v.e v.d t) */`

<a id="op-47ae9896b19d51f6cf47381c"></a>
## SingleLine

`variant` · `sqlparser::ast::OptimizerHintStyle::SingleLine` · sqlparser 0.62.0

```rust
SingleLine
```

Source: `src/ast/mod.rs:12007`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

A hint corresponding to a single line comment,
e.g. `--+ LEADING(v.e v.d t)`

<a id="op-f51399d70ff44ad691489251"></a>
## clone

`function` · `sqlparser::ast::OptimizerHintStyle::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> OptimizerHintStyle
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::OptimizerHintStyle", "path": "OptimizerHintStyle"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [12001, 17], "end": [12001, 22], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/mod.rs:12001`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d1057b6cd9baed203ceed5c0"></a>
## cmp

`function` · `sqlparser::ast::OptimizerHintStyle::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &OptimizerHintStyle) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::OptimizerHintStyle", "path": "OptimizerHintStyle"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [12001, 51], "end": [12001, 54], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/mod.rs:12001`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-07f4c9c73125e8eed936ae2e"></a>
## deserialize

`function` · `sqlparser::ast::OptimizerHintStyle::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::OptimizerHintStyle", "path": "OptimizerHintStyle"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [12002, 49], "end": [12002, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/mod.rs:12002`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0e460994e26ce5fab64f3188"></a>
## eq

`function` · `sqlparser::ast::OptimizerHintStyle::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &OptimizerHintStyle) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::OptimizerHintStyle", "path": "OptimizerHintStyle"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [12001, 24], "end": [12001, 33], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/mod.rs:12001`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c098e7ad2fff5437a8be22b3"></a>
## fmt

`function` · `sqlparser::ast::OptimizerHintStyle::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::OptimizerHintStyle", "path": "OptimizerHintStyle"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [12001, 10], "end": [12001, 15], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/mod.rs:12001`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-90ffe64308de4fbb22138bf7"></a>
## hash

`function` · `sqlparser::ast::OptimizerHintStyle::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::OptimizerHintStyle", "path": "OptimizerHintStyle"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [12001, 56], "end": [12001, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/mod.rs:12001`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9f490e147a83a943ebc5a6ba"></a>
## partial_cmp

`function` · `sqlparser::ast::OptimizerHintStyle::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &OptimizerHintStyle) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::OptimizerHintStyle", "path": "OptimizerHintStyle"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [12001, 35], "end": [12001, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/mod.rs:12001`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-dbf93cb0a5e0c98b6dd011fa"></a>
## serialize

`function` · `sqlparser::ast::OptimizerHintStyle::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::OptimizerHintStyle", "path": "OptimizerHintStyle"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [12002, 38], "end": [12002, 47], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/mod.rs:12002`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b338ad9927c4237cab943527"></a>
## visit

`function` · `sqlparser::ast::OptimizerHintStyle::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::OptimizerHintStyle", "path": "OptimizerHintStyle"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [12003, 40], "end": [12003, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/mod.rs:12003`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-dbceef20b05f137578e9ed59"></a>
## visit

`function` · `sqlparser::ast::OptimizerHintStyle::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::OptimizerHintStyle", "path": "OptimizerHintStyle"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [12003, 47], "end": [12003, 55], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/mod.rs:12003`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
