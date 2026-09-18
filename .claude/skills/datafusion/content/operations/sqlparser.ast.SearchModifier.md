# `sqlparser::ast::SearchModifier`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.SearchModifier.json).

<a id="op-c2a8241a38329462814d26e8"></a>
## SearchModifier

`enum` · `sqlparser::ast::SearchModifier` · sqlparser 0.62.0

```rust
enum SearchModifier
```

Source: `src/ast/mod.rs:10324`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Fulltext search modifiers ([1]).

[1]: https://dev.mysql.com/doc/refman/8.0/en/fulltext-search.html#function_match

<a id="op-5be32c22977e541200fa2bd0"></a>
## InBooleanMode

`variant` · `sqlparser::ast::SearchModifier::InBooleanMode` · sqlparser 0.62.0

```rust
InBooleanMode
```

Source: `src/ast/mod.rs:10330`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`IN BOOLEAN MODE`.

<a id="op-a06ab8033090e9286a91f157"></a>
## InNaturalLanguageMode

`variant` · `sqlparser::ast::SearchModifier::InNaturalLanguageMode` · sqlparser 0.62.0

```rust
InNaturalLanguageMode
```

Source: `src/ast/mod.rs:10326`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`IN NATURAL LANGUAGE MODE`.

<a id="op-41f35cdd7320d8176cf99e2a"></a>
## InNaturalLanguageModeWithQueryExpansion

`variant` · `sqlparser::ast::SearchModifier::InNaturalLanguageModeWithQueryExpansion` · sqlparser 0.62.0

```rust
InNaturalLanguageModeWithQueryExpansion
```

Source: `src/ast/mod.rs:10328`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`IN NATURAL LANGUAGE MODE WITH QUERY EXPANSION`.

<a id="op-34f0f7889f92d967af736a94"></a>
## WithQueryExpansion

`variant` · `sqlparser::ast::SearchModifier::WithQueryExpansion` · sqlparser 0.62.0

```rust
WithQueryExpansion
```

Source: `src/ast/mod.rs:10332`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`WITH QUERY EXPANSION`.

<a id="op-f03f84de09ef0e1f32d0e845"></a>
## clone

`function` · `sqlparser::ast::SearchModifier::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> SearchModifier
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::SearchModifier", "path": "SearchModifier"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10321, 17], "end": [10321, 22], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/mod.rs:10321`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-41a7e611850612d2bc92efda"></a>
## cmp

`function` · `sqlparser::ast::SearchModifier::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &SearchModifier) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::SearchModifier", "path": "SearchModifier"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10321, 51], "end": [10321, 54], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/mod.rs:10321`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b9b7152b8948e43fa3930bb1"></a>
## deserialize

`function` · `sqlparser::ast::SearchModifier::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::SearchModifier", "path": "SearchModifier"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [10322, 49], "end": [10322, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/mod.rs:10322`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d888486ef2bbba515a4984a6"></a>
## eq

`function` · `sqlparser::ast::SearchModifier::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &SearchModifier) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::SearchModifier", "path": "SearchModifier"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10321, 24], "end": [10321, 33], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/mod.rs:10321`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-13e377b8bc6d8e2acc7f4b26"></a>
## fmt

`function` · `sqlparser::ast::SearchModifier::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::SearchModifier", "path": "SearchModifier"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10321, 10], "end": [10321, 15], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/mod.rs:10321`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-dd78b2c2c27ce968380b0b2f"></a>
## fmt

`function` · `sqlparser::ast::SearchModifier::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::SearchModifier", "path": "SearchModifier"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10335, 1], "end": [10354, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/mod.rs:10336`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-788128355d9a36e654b5b082"></a>
## hash

`function` · `sqlparser::ast::SearchModifier::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::SearchModifier", "path": "SearchModifier"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10321, 56], "end": [10321, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/mod.rs:10321`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-472a9d11a2a6b0763ea3038a"></a>
## partial_cmp

`function` · `sqlparser::ast::SearchModifier::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &SearchModifier) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::SearchModifier", "path": "SearchModifier"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10321, 35], "end": [10321, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/mod.rs:10321`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b49453b12c4ff9bf4a3cab4d"></a>
## serialize

`function` · `sqlparser::ast::SearchModifier::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::SearchModifier", "path": "SearchModifier"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10322, 38], "end": [10322, 47], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/mod.rs:10322`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2226bba22b952cd8054ccfca"></a>
## visit

`function` · `sqlparser::ast::SearchModifier::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::SearchModifier", "path": "SearchModifier"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10323, 40], "end": [10323, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/mod.rs:10323`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5374baea82ba3ccd6935e76a"></a>
## visit

`function` · `sqlparser::ast::SearchModifier::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::SearchModifier", "path": "SearchModifier"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10323, 47], "end": [10323, 55], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/mod.rs:10323`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
