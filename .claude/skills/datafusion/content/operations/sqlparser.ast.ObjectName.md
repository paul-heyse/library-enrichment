# `sqlparser::ast::ObjectName`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.ObjectName.json).

<a id="op-2e2c3eb4ae55b1ee1de29b6a"></a>
## ObjectName

`struct` · `sqlparser::ast::ObjectName` · sqlparser 0.62.0

```rust
struct ObjectName
```

Source: `src/ast/mod.rs:395`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

A name of a table, view, custom type, etc., possibly multi-part, i.e. db.schema.obj

<a id="op-2ce3220225aee02ee1200e60"></a>
## 0

`struct_field` · `sqlparser::ast::ObjectName::0` · sqlparser 0.62.0

```rust
0: Vec<ObjectNamePart>
```

Source: `src/ast/mod.rs:395`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e0b9442dd2e0cd47d3c0f832"></a>
## clone

`function` · `sqlparser::ast::ObjectName::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> ObjectName
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ObjectName", "path": "ObjectName"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [392, 17], "end": [392, 22], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/mod.rs:392`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e4df0542fbba835efbd556aa"></a>
## cmp

`function` · `sqlparser::ast::ObjectName::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &ObjectName) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ObjectName", "path": "ObjectName"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [392, 51], "end": [392, 54], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/mod.rs:392`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e5b0591d1d3a6be6dd7761da"></a>
## deserialize

`function` · `sqlparser::ast::ObjectName::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ObjectName", "path": "ObjectName"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [393, 49], "end": [393, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/mod.rs:393`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8cb2cb2183511d1db0daeb6a"></a>
## eq

`function` · `sqlparser::ast::ObjectName::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &ObjectName) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ObjectName", "path": "ObjectName"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [392, 24], "end": [392, 33], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/mod.rs:392`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2252b1c61b814b44e71c0c15"></a>
## fmt

`function` · `sqlparser::ast::ObjectName::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ObjectName", "path": "ObjectName"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [409, 1], "end": [413, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/mod.rs:410`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-df72f6f951125b184abb53d6"></a>
## fmt

`function` · `sqlparser::ast::ObjectName::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ObjectName", "path": "ObjectName"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [392, 10], "end": [392, 15], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/mod.rs:392`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-19012f42f43354dd8882d6db"></a>
## from

`function` · `sqlparser::ast::ObjectName::from` · sqlparser 0.62.0

```rust
fn from(ident: Ident) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ObjectName", "path": "ObjectName"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [403, 1], "end": [407, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "sqlparser::ast::Ident", "path": "Ident"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/ast/mod.rs:404`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ff6e900d12524308c997774d"></a>
## from

`function` · `sqlparser::ast::ObjectName::from` · sqlparser 0.62.0

```rust
fn from(idents: Vec<Ident>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ObjectName", "path": "ObjectName"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [397, 1], "end": [401, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "sqlparser::ast::Ident", "path": "Ident"}}}], "constraints": []}}, "id": "alloc::vec::Vec", "path": "Vec"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/ast/mod.rs:398`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-221538d413bb1e1ed8aecb6e"></a>
## hash

`function` · `sqlparser::ast::ObjectName::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ObjectName", "path": "ObjectName"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [392, 56], "end": [392, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/mod.rs:392`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a5026d41f1ce9cda24027c52"></a>
## partial_cmp

`function` · `sqlparser::ast::ObjectName::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &ObjectName) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ObjectName", "path": "ObjectName"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [392, 35], "end": [392, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/mod.rs:392`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9429abbfa0da50cae8f153ad"></a>
## serialize

`function` · `sqlparser::ast::ObjectName::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ObjectName", "path": "ObjectName"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [393, 38], "end": [393, 47], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/mod.rs:393`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d9a932350b7ca6e5da091009"></a>
## span

`function` · `sqlparser::ast::ObjectName::span` · sqlparser 0.62.0

```rust
fn span(&self) -> Span
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ObjectName", "path": "super::ObjectName"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1700, 1], "end": [1706, 2], "filename": "src/ast/spans.rs"}, "trait": {"args": null, "id": "sqlparser::ast::spans::Spanned", "path": "Spanned"}, "trait_path": "sqlparser::ast::spans::Spanned"}`

Source: `src/ast/spans.rs:1701`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-443d656e3729198eada0ce1d"></a>
## visit

`function` · `sqlparser::ast::ObjectName::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ObjectName", "path": "ObjectName"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [394, 40], "end": [394, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/mod.rs:394`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9c09cf9bb7e27b065043f0d2"></a>
## visit

`function` · `sqlparser::ast::ObjectName::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ObjectName", "path": "ObjectName"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [394, 47], "end": [394, 55], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/mod.rs:394`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
