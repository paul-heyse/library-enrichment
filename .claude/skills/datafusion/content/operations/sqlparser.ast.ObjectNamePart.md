# `sqlparser::ast::ObjectNamePart`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.ObjectNamePart.json).

<a id="op-46ecddf301ac68e0b0c0665d"></a>
## ObjectNamePart

`enum` · `sqlparser::ast::ObjectNamePart` · sqlparser 0.62.0

```rust
enum ObjectNamePart
```

Source: `src/ast/mod.rs:419`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

A single part of an ObjectName

<a id="op-0eae77719ac035a8666b814e"></a>
## Function

`variant` · `sqlparser::ast::ObjectNamePart::Function` · sqlparser 0.62.0

```rust
Function
```

Source: `src/ast/mod.rs:423`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

A function that returns an identifier (dialect-specific).

<a id="op-48afb9cb83425ca7b76de98e"></a>
## Identifier

`variant` · `sqlparser::ast::ObjectNamePart::Identifier` · sqlparser 0.62.0

```rust
Identifier
```

Source: `src/ast/mod.rs:421`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

A single identifier part, e.g. `schema` or `table`.

<a id="op-16668622c1bc4fcecdd5053b"></a>
## as_ident

`function` · `sqlparser::ast::ObjectNamePart::as_ident` · sqlparser 0.62.0

```rust
fn as_ident(&self) -> Option<&Ident>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ObjectNamePart", "path": "ObjectNamePart"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [426, 1], "end": [434, 2], "filename": "src/ast/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/ast/mod.rs:428`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Return the identifier if this is an `Identifier` variant.

<a id="op-3ca06fef5d51b8f3b83de2bd"></a>
## clone

`function` · `sqlparser::ast::ObjectNamePart::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> ObjectNamePart
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ObjectNamePart", "path": "ObjectNamePart"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [416, 17], "end": [416, 22], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/mod.rs:416`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-25bdea7a0e193f4815de88c7"></a>
## cmp

`function` · `sqlparser::ast::ObjectNamePart::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &ObjectNamePart) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ObjectNamePart", "path": "ObjectNamePart"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [416, 51], "end": [416, 54], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/mod.rs:416`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-46dd7e565b36863b41c428ec"></a>
## deserialize

`function` · `sqlparser::ast::ObjectNamePart::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ObjectNamePart", "path": "ObjectNamePart"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [417, 49], "end": [417, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/mod.rs:417`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-59dfbdf186c79b5971b5e3d6"></a>
## eq

`function` · `sqlparser::ast::ObjectNamePart::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &ObjectNamePart) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ObjectNamePart", "path": "ObjectNamePart"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [416, 24], "end": [416, 33], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/mod.rs:416`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0a85ed2727ce222ab86ac955"></a>
## fmt

`function` · `sqlparser::ast::ObjectNamePart::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ObjectNamePart", "path": "ObjectNamePart"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [416, 10], "end": [416, 15], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/mod.rs:416`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1001bbe01a5c89b2a5f5d8f7"></a>
## fmt

`function` · `sqlparser::ast::ObjectNamePart::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ObjectNamePart", "path": "ObjectNamePart"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [436, 1], "end": [443, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/mod.rs:437`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ffaecb54793bf513ad7a912f"></a>
## hash

`function` · `sqlparser::ast::ObjectNamePart::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ObjectNamePart", "path": "ObjectNamePart"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [416, 56], "end": [416, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/mod.rs:416`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c9a2a7ff77a295fb9b665512"></a>
## partial_cmp

`function` · `sqlparser::ast::ObjectNamePart::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &ObjectNamePart) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ObjectNamePart", "path": "ObjectNamePart"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [416, 35], "end": [416, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/mod.rs:416`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-81fefe02b97b947a1d7518aa"></a>
## serialize

`function` · `sqlparser::ast::ObjectNamePart::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ObjectNamePart", "path": "ObjectNamePart"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [417, 38], "end": [417, 47], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/mod.rs:417`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-60c4c962e172088aa7947202"></a>
## span

`function` · `sqlparser::ast::ObjectNamePart::span` · sqlparser 0.62.0

```rust
fn span(&self) -> Span
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ObjectNamePart", "path": "super::ObjectNamePart"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1708, 1], "end": [1718, 2], "filename": "src/ast/spans.rs"}, "trait": {"args": null, "id": "sqlparser::ast::spans::Spanned", "path": "Spanned"}, "trait_path": "sqlparser::ast::spans::Spanned"}`

Source: `src/ast/spans.rs:1709`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-619e44b0011dec8ed4f4f11b"></a>
## visit

`function` · `sqlparser::ast::ObjectNamePart::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ObjectNamePart", "path": "ObjectNamePart"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [418, 47], "end": [418, 55], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/mod.rs:418`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d62631290382930a14caa28e"></a>
## visit

`function` · `sqlparser::ast::ObjectNamePart::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ObjectNamePart", "path": "ObjectNamePart"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [418, 40], "end": [418, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/mod.rs:418`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
