# `sqlparser::ast::SessionParamStatsTopic`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.SessionParamStatsTopic.json).

<a id="op-dbc2f40fa167e5be361ee663"></a>
## SessionParamStatsTopic

`enum` · `sqlparser::ast::SessionParamStatsTopic` · sqlparser 0.62.0

```rust
enum SessionParamStatsTopic
```

Source: `src/ast/mod.rs:11178`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Topics available for session statistics configuration.

<a id="op-86c51ee2150d98c720643e32"></a>
## IO

`variant` · `sqlparser::ast::SessionParamStatsTopic::IO` · sqlparser 0.62.0

```rust
IO
```

Source: `src/ast/mod.rs:11180`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Input/output statistics.

<a id="op-b967eb744ed4b1a3a9478e67"></a>
## Profile

`variant` · `sqlparser::ast::SessionParamStatsTopic::Profile` · sqlparser 0.62.0

```rust
Profile
```

Source: `src/ast/mod.rs:11182`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Profile statistics.

<a id="op-9fe91bc88e8aa365b4d5ad4b"></a>
## Time

`variant` · `sqlparser::ast::SessionParamStatsTopic::Time` · sqlparser 0.62.0

```rust
Time
```

Source: `src/ast/mod.rs:11184`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Time statistics.

<a id="op-a5e3421aa88cfacfd64c3d20"></a>
## Xml

`variant` · `sqlparser::ast::SessionParamStatsTopic::Xml` · sqlparser 0.62.0

```rust
Xml
```

Source: `src/ast/mod.rs:11186`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

XML-related statistics.

<a id="op-98ed7f93c6c6a3d69e98056c"></a>
## clone

`function` · `sqlparser::ast::SessionParamStatsTopic::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> SessionParamStatsTopic
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::SessionParamStatsTopic", "path": "SessionParamStatsTopic"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11174, 17], "end": [11174, 22], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/mod.rs:11174`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9580b8f0d76fb3b8721f9496"></a>
## cmp

`function` · `sqlparser::ast::SessionParamStatsTopic::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &SessionParamStatsTopic) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::SessionParamStatsTopic", "path": "SessionParamStatsTopic"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11174, 51], "end": [11174, 54], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/mod.rs:11174`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-aa0255da113f92b767e358bb"></a>
## deserialize

`function` · `sqlparser::ast::SessionParamStatsTopic::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::SessionParamStatsTopic", "path": "SessionParamStatsTopic"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [11175, 49], "end": [11175, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/mod.rs:11175`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-252d0f6ea567d7bdd24b4438"></a>
## eq

`function` · `sqlparser::ast::SessionParamStatsTopic::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &SessionParamStatsTopic) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::SessionParamStatsTopic", "path": "SessionParamStatsTopic"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11174, 24], "end": [11174, 33], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/mod.rs:11174`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-631f7ce7ae633926ad9387fb"></a>
## fmt

`function` · `sqlparser::ast::SessionParamStatsTopic::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::SessionParamStatsTopic", "path": "SessionParamStatsTopic"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11174, 10], "end": [11174, 15], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/mod.rs:11174`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c191614cfbea9a807aed3d37"></a>
## fmt

`function` · `sqlparser::ast::SessionParamStatsTopic::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::SessionParamStatsTopic", "path": "SessionParamStatsTopic"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11189, 1], "end": [11198, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/mod.rs:11190`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fa8af9397a610f98ddf14341"></a>
## hash

`function` · `sqlparser::ast::SessionParamStatsTopic::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::SessionParamStatsTopic", "path": "SessionParamStatsTopic"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11174, 56], "end": [11174, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/mod.rs:11174`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-207eab57853cf25e40ccdc58"></a>
## partial_cmp

`function` · `sqlparser::ast::SessionParamStatsTopic::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &SessionParamStatsTopic) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::SessionParamStatsTopic", "path": "SessionParamStatsTopic"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11174, 35], "end": [11174, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/mod.rs:11174`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f74f10db68d99cdcd1bbf7de"></a>
## serialize

`function` · `sqlparser::ast::SessionParamStatsTopic::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::SessionParamStatsTopic", "path": "SessionParamStatsTopic"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11175, 38], "end": [11175, 47], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/mod.rs:11175`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5b3a1c13402fdc942e79bfb2"></a>
## visit

`function` · `sqlparser::ast::SessionParamStatsTopic::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::SessionParamStatsTopic", "path": "SessionParamStatsTopic"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11176, 40], "end": [11176, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/mod.rs:11176`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-74aaa1b0afb81a7d608a4ef8"></a>
## visit

`function` · `sqlparser::ast::SessionParamStatsTopic::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::SessionParamStatsTopic", "path": "SessionParamStatsTopic"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11176, 47], "end": [11176, 55], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/mod.rs:11176`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
