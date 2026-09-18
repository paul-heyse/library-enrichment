# `sqlparser::ast::ExtractSyntax`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.ExtractSyntax.json).

<a id="op-8c8ac02299cec49514c7889b"></a>
## ExtractSyntax

`enum` · `sqlparser::ast::ExtractSyntax` · sqlparser 0.62.0

```rust
enum ExtractSyntax
```

Source: `src/ast/mod.rs:799`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`EXTRACT` syntax variants.

In Snowflake dialect, the `EXTRACT` expression can support either the `from` syntax
or the comma syntax.

See <https://docs.snowflake.com/en/sql-reference/functions/extract>

<a id="op-4c538d0dc5adf8d808bce039"></a>
## Comma

`variant` · `sqlparser::ast::ExtractSyntax::Comma` · sqlparser 0.62.0

```rust
Comma
```

Source: `src/ast/mod.rs:803`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`EXTRACT( <date_or_time_part> , <date_or_timestamp_expr> )`

<a id="op-933686df1037e966505825a2"></a>
## From

`variant` · `sqlparser::ast::ExtractSyntax::From` · sqlparser 0.62.0

```rust
From
```

Source: `src/ast/mod.rs:801`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`EXTRACT( <date_or_time_part> FROM <date_or_time_expr> )`

<a id="op-127dbb642e6bd557c6946b60"></a>
## clone

`function` · `sqlparser::ast::ExtractSyntax::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> ExtractSyntax
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ExtractSyntax", "path": "ExtractSyntax"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [796, 17], "end": [796, 22], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/mod.rs:796`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fc5dbba3cdd25c7c27b52bc4"></a>
## cmp

`function` · `sqlparser::ast::ExtractSyntax::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &ExtractSyntax) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ExtractSyntax", "path": "ExtractSyntax"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [796, 51], "end": [796, 54], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/mod.rs:796`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-914e74598862710e25a5c2e6"></a>
## deserialize

`function` · `sqlparser::ast::ExtractSyntax::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ExtractSyntax", "path": "ExtractSyntax"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [797, 49], "end": [797, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/mod.rs:797`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-70b0098f8c8e324186e6532a"></a>
## eq

`function` · `sqlparser::ast::ExtractSyntax::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &ExtractSyntax) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ExtractSyntax", "path": "ExtractSyntax"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [796, 24], "end": [796, 33], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/mod.rs:796`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a8b3a78755990b75a457e95f"></a>
## fmt

`function` · `sqlparser::ast::ExtractSyntax::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ExtractSyntax", "path": "ExtractSyntax"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [796, 10], "end": [796, 15], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/mod.rs:796`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-14222547ecc4864dc94533fc"></a>
## hash

`function` · `sqlparser::ast::ExtractSyntax::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ExtractSyntax", "path": "ExtractSyntax"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [796, 56], "end": [796, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/mod.rs:796`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0d2b0fc1fec6116c57b3e865"></a>
## partial_cmp

`function` · `sqlparser::ast::ExtractSyntax::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &ExtractSyntax) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ExtractSyntax", "path": "ExtractSyntax"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [796, 35], "end": [796, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/mod.rs:796`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-634a918981e895ef87742c66"></a>
## serialize

`function` · `sqlparser::ast::ExtractSyntax::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ExtractSyntax", "path": "ExtractSyntax"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [797, 38], "end": [797, 47], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/mod.rs:797`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-119f99b846b6c1b2b23317ca"></a>
## visit

`function` · `sqlparser::ast::ExtractSyntax::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ExtractSyntax", "path": "ExtractSyntax"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [798, 47], "end": [798, 55], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/mod.rs:798`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b296b555a1aea0051aab220e"></a>
## visit

`function` · `sqlparser::ast::ExtractSyntax::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ExtractSyntax", "path": "ExtractSyntax"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [798, 40], "end": [798, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/mod.rs:798`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
