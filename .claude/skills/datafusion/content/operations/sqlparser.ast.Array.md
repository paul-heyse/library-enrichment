# `sqlparser::ast::Array`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.Array.json).

<a id="op-b3392a43db92e330c4d16b13"></a>
## Array

`struct` · `sqlparser::ast::Array` · sqlparser 0.62.0

```rust
struct Array
```

Source: `src/ast/mod.rs:471`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Represents an Array Expression, either
`ARRAY[..]`, or `[..]`

<a id="op-1850e43ad85dc920c585dc0c"></a>
## clone

`function` · `sqlparser::ast::Array::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> Array
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Array", "path": "Array"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [468, 17], "end": [468, 22], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/mod.rs:468`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8a1238a2954c8b3ceab5392d"></a>
## cmp

`function` · `sqlparser::ast::Array::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &Array) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Array", "path": "Array"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [468, 51], "end": [468, 54], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/mod.rs:468`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-11869bf1dd8bab65d61739a7"></a>
## deserialize

`function` · `sqlparser::ast::Array::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Array", "path": "Array"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [469, 49], "end": [469, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/mod.rs:469`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d4305389130ed938f4d2572b"></a>
## elem

`struct_field` · `sqlparser::ast::Array::elem` · sqlparser 0.62.0

```rust
elem: Vec<Expr>
```

Source: `src/ast/mod.rs:473`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The list of expressions between brackets

<a id="op-1850a2fc5dd71937836c33f0"></a>
## eq

`function` · `sqlparser::ast::Array::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &Array) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Array", "path": "Array"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [468, 24], "end": [468, 33], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/mod.rs:468`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-106d3c3eaa388262bd39551b"></a>
## fmt

`function` · `sqlparser::ast::Array::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Array", "path": "Array"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [479, 1], "end": [488, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/mod.rs:480`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-379c7d7eff0112e40eed9a82"></a>
## fmt

`function` · `sqlparser::ast::Array::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Array", "path": "Array"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [468, 10], "end": [468, 15], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/mod.rs:468`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d9d288e17d3ec41e7476d3fd"></a>
## hash

`function` · `sqlparser::ast::Array::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Array", "path": "Array"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [468, 56], "end": [468, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/mod.rs:468`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-133e464c327db34fc7903298"></a>
## named

`struct_field` · `sqlparser::ast::Array::named` · sqlparser 0.62.0

```rust
named: bool
```

Source: `src/ast/mod.rs:476`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`true` for  `ARRAY[..]`, `false` for `[..]`

<a id="op-ae2f399e4ff1ae0cf9224179"></a>
## partial_cmp

`function` · `sqlparser::ast::Array::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &Array) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Array", "path": "Array"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [468, 35], "end": [468, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/mod.rs:468`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d66009793983c00b5730f5e4"></a>
## serialize

`function` · `sqlparser::ast::Array::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Array", "path": "Array"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [469, 38], "end": [469, 47], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/mod.rs:469`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-27cd7b2caebf5f91423ac057"></a>
## span

`function` · `sqlparser::ast::Array::span` · sqlparser 0.62.0

```rust
fn span(&self) -> Span
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Array", "path": "super::Array"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1720, 1], "end": [1729, 2], "filename": "src/ast/spans.rs"}, "trait": {"args": null, "id": "sqlparser::ast::spans::Spanned", "path": "Spanned"}, "trait_path": "sqlparser::ast::spans::Spanned"}`

Source: `src/ast/spans.rs:1721`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-63c1520d1dd98958df177d6d"></a>
## visit

`function` · `sqlparser::ast::Array::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Array", "path": "Array"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [470, 40], "end": [470, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/mod.rs:470`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-67105c930eda64619239dc50"></a>
## visit

`function` · `sqlparser::ast::Array::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Array", "path": "Array"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [470, 47], "end": [470, 55], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/mod.rs:470`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
