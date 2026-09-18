# `sqlparser::ast::WindowFrameBound`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.WindowFrameBound.json).

<a id="op-1a9b3713be5a017d0d99fea3"></a>
## WindowFrameBound

`enum` · `sqlparser::ast::WindowFrameBound` · sqlparser 0.62.0

```rust
enum WindowFrameBound
```

Source: `src/ast/mod.rs:2411`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Specifies [WindowFrame](../operations/sqlparser.ast.WindowFrame.md#op-454ad5a072bd35c144616b74)'s `start_bound` and `end_bound`

<a id="op-11787b498d39d5bbdae647d1"></a>
## CurrentRow

`variant` · `sqlparser::ast::WindowFrameBound::CurrentRow` · sqlparser 0.62.0

```rust
CurrentRow
```

Source: `src/ast/mod.rs:2413`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`CURRENT ROW`

<a id="op-8bee24ce3f998d782d616fae"></a>
## Following

`variant` · `sqlparser::ast::WindowFrameBound::Following` · sqlparser 0.62.0

```rust
Following
```

Source: `src/ast/mod.rs:2417`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`<N> FOLLOWING` or `UNBOUNDED FOLLOWING`.

<a id="op-32d7f14b46473ee02373676f"></a>
## Preceding

`variant` · `sqlparser::ast::WindowFrameBound::Preceding` · sqlparser 0.62.0

```rust
Preceding
```

Source: `src/ast/mod.rs:2415`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`<N> PRECEDING` or `UNBOUNDED PRECEDING`

<a id="op-e4883e4ea7c1b91d1adac265"></a>
## clone

`function` · `sqlparser::ast::WindowFrameBound::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> WindowFrameBound
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::WindowFrameBound", "path": "WindowFrameBound"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2408, 17], "end": [2408, 22], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/mod.rs:2408`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c3652ee995c996fd7e15b2ad"></a>
## cmp

`function` · `sqlparser::ast::WindowFrameBound::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &WindowFrameBound) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::WindowFrameBound", "path": "WindowFrameBound"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2408, 51], "end": [2408, 54], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/mod.rs:2408`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ddb48c1e8800353972e79eb5"></a>
## deserialize

`function` · `sqlparser::ast::WindowFrameBound::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::WindowFrameBound", "path": "WindowFrameBound"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [2409, 49], "end": [2409, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/mod.rs:2409`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fc746e55f8e6e053b3b09108"></a>
## eq

`function` · `sqlparser::ast::WindowFrameBound::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &WindowFrameBound) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::WindowFrameBound", "path": "WindowFrameBound"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2408, 24], "end": [2408, 33], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/mod.rs:2408`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-09de029dfaf6dd1bbc86261d"></a>
## fmt

`function` · `sqlparser::ast::WindowFrameBound::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::WindowFrameBound", "path": "WindowFrameBound"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2408, 10], "end": [2408, 15], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/mod.rs:2408`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bd19d835b2ba260b9a670d15"></a>
## fmt

`function` · `sqlparser::ast::WindowFrameBound::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::WindowFrameBound", "path": "WindowFrameBound"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2420, 1], "end": [2430, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/mod.rs:2421`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f867163f5a7cf757c4677611"></a>
## hash

`function` · `sqlparser::ast::WindowFrameBound::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::WindowFrameBound", "path": "WindowFrameBound"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2408, 56], "end": [2408, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/mod.rs:2408`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b6145c7663232ea1f21e671d"></a>
## partial_cmp

`function` · `sqlparser::ast::WindowFrameBound::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &WindowFrameBound) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::WindowFrameBound", "path": "WindowFrameBound"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2408, 35], "end": [2408, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/mod.rs:2408`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2b71f2fd4dfb084a13ac9a8d"></a>
## serialize

`function` · `sqlparser::ast::WindowFrameBound::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::WindowFrameBound", "path": "WindowFrameBound"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2409, 38], "end": [2409, 47], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/mod.rs:2409`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-356d561c35a5810613e10ebf"></a>
## visit

`function` · `sqlparser::ast::WindowFrameBound::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::WindowFrameBound", "path": "WindowFrameBound"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2410, 40], "end": [2410, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/mod.rs:2410`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-57d56133c64b536a939974b3"></a>
## visit

`function` · `sqlparser::ast::WindowFrameBound::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::WindowFrameBound", "path": "WindowFrameBound"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2410, 47], "end": [2410, 55], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/mod.rs:2410`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
