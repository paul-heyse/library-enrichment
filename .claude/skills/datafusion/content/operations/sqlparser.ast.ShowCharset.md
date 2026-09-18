# `sqlparser::ast::ShowCharset`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.ShowCharset.json).

<a id="op-5259ebd1bde5474f1eab8707"></a>
## ShowCharset

`struct` · `sqlparser::ast::ShowCharset` · sqlparser 0.62.0

```rust
struct ShowCharset
```

Source: `src/ast/mod.rs:10896`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

A Show Charset statement

<a id="op-efa12cfb85f9acc53a39c356"></a>
## clone

`function` · `sqlparser::ast::ShowCharset::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> ShowCharset
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ShowCharset", "path": "ShowCharset"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10893, 17], "end": [10893, 22], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/mod.rs:10893`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1260d8b6a3c44ef50b501540"></a>
## cmp

`function` · `sqlparser::ast::ShowCharset::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &ShowCharset) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ShowCharset", "path": "ShowCharset"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10893, 51], "end": [10893, 54], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/mod.rs:10893`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7b8176e86c4d703c2b808c27"></a>
## deserialize

`function` · `sqlparser::ast::ShowCharset::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ShowCharset", "path": "ShowCharset"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [10894, 49], "end": [10894, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/mod.rs:10894`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-074819027fef40b068b04169"></a>
## eq

`function` · `sqlparser::ast::ShowCharset::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &ShowCharset) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ShowCharset", "path": "ShowCharset"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10893, 24], "end": [10893, 33], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/mod.rs:10893`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5c5952029afcfa60efcac6c3"></a>
## filter

`struct_field` · `sqlparser::ast::ShowCharset::filter` · sqlparser 0.62.0

```rust
filter: Option<ShowStatementFilter>
```

Source: `src/ast/mod.rs:10901`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional `LIKE`/`WHERE`-style filter for the statement.

<a id="op-8a5e4e9cea50d7864f7abc52"></a>
## fmt

`function` · `sqlparser::ast::ShowCharset::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ShowCharset", "path": "ShowCharset"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10893, 10], "end": [10893, 15], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/mod.rs:10893`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fde495b3ec1c5162c1b1877a"></a>
## fmt

`function` · `sqlparser::ast::ShowCharset::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ShowCharset", "path": "ShowCharset"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10904, 1], "end": [10917, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/mod.rs:10905`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b9e0dccc4443a9b1f02d8eeb"></a>
## hash

`function` · `sqlparser::ast::ShowCharset::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ShowCharset", "path": "ShowCharset"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10893, 56], "end": [10893, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/mod.rs:10893`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-129605780b115a55b149ca51"></a>
## is_shorthand

`struct_field` · `sqlparser::ast::ShowCharset::is_shorthand` · sqlparser 0.62.0

```rust
is_shorthand: bool
```

Source: `src/ast/mod.rs:10899`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The statement can be written as `SHOW CHARSET` or `SHOW CHARACTER SET`
true means CHARSET was used and false means CHARACTER SET was used

<a id="op-f8a86911117b97247291055f"></a>
## partial_cmp

`function` · `sqlparser::ast::ShowCharset::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &ShowCharset) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ShowCharset", "path": "ShowCharset"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10893, 35], "end": [10893, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/mod.rs:10893`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-45522b6d819eb6b959ce9e40"></a>
## serialize

`function` · `sqlparser::ast::ShowCharset::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ShowCharset", "path": "ShowCharset"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10894, 38], "end": [10894, 47], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/mod.rs:10894`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4069a513297261f34da15e94"></a>
## visit

`function` · `sqlparser::ast::ShowCharset::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ShowCharset", "path": "ShowCharset"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10895, 47], "end": [10895, 55], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/mod.rs:10895`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5125eb085882e0dbef2a6ee7"></a>
## visit

`function` · `sqlparser::ast::ShowCharset::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ShowCharset", "path": "ShowCharset"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10895, 40], "end": [10895, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/mod.rs:10895`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
