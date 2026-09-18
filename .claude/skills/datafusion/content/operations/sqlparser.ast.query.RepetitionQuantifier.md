# `sqlparser::ast::query::RepetitionQuantifier`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.query.RepetitionQuantifier.json).

<a id="op-a640264f92297d6374bd8660"></a>
## RepetitionQuantifier

`enum` · `sqlparser::ast::query::RepetitionQuantifier` · sqlparser 0.62.0

```rust
enum RepetitionQuantifier
```

Source: `src/ast/query.rs:2168`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Determines the minimum and maximum allowed occurrences of a pattern in a
`MATCH_RECOGNIZE` operation.

<a id="op-aa7b8a92650c2bcaefebc0e3"></a>
## AtLeast

`variant` · `sqlparser::ast::query::RepetitionQuantifier::AtLeast` · sqlparser 0.62.0

```rust
AtLeast
```

Source: `src/ast/query.rs:2178`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`{n,}`

<a id="op-4c1c9d8d1cb23ee22f813555"></a>
## AtMost

`variant` · `sqlparser::ast::query::RepetitionQuantifier::AtMost` · sqlparser 0.62.0

```rust
AtMost
```

Source: `src/ast/query.rs:2180`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`{,n}`

<a id="op-808d446acc0b32b41ba6ccc0"></a>
## AtMostOne

`variant` · `sqlparser::ast::query::RepetitionQuantifier::AtMostOne` · sqlparser 0.62.0

```rust
AtMostOne
```

Source: `src/ast/query.rs:2174`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`?`

<a id="op-5c734baa6ebd1882604705da"></a>
## Exactly

`variant` · `sqlparser::ast::query::RepetitionQuantifier::Exactly` · sqlparser 0.62.0

```rust
Exactly
```

Source: `src/ast/query.rs:2176`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`{n}`

<a id="op-a95ec5b508f5a219905086c6"></a>
## OneOrMore

`variant` · `sqlparser::ast::query::RepetitionQuantifier::OneOrMore` · sqlparser 0.62.0

```rust
OneOrMore
```

Source: `src/ast/query.rs:2172`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`+`

<a id="op-e5867815a2911599adb1963c"></a>
## Range

`variant` · `sqlparser::ast::query::RepetitionQuantifier::Range` · sqlparser 0.62.0

```rust
Range
```

Source: `src/ast/query.rs:2182`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`{n,m}

<a id="op-51d5c8d168c62ae166dab974"></a>
## ZeroOrMore

`variant` · `sqlparser::ast::query::RepetitionQuantifier::ZeroOrMore` · sqlparser 0.62.0

```rust
ZeroOrMore
```

Source: `src/ast/query.rs:2170`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`*`

<a id="op-a7d2da67e8dea8dde497c769"></a>
## clone

`function` · `sqlparser::ast::query::RepetitionQuantifier::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> RepetitionQuantifier
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::RepetitionQuantifier", "path": "RepetitionQuantifier"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2165, 17], "end": [2165, 22], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/query.rs:2165`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-63049534e975e8184faad3c0"></a>
## cmp

`function` · `sqlparser::ast::query::RepetitionQuantifier::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &RepetitionQuantifier) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::RepetitionQuantifier", "path": "RepetitionQuantifier"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2165, 51], "end": [2165, 54], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/query.rs:2165`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e07048ae7a6a44140e63af96"></a>
## deserialize

`function` · `sqlparser::ast::query::RepetitionQuantifier::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::RepetitionQuantifier", "path": "RepetitionQuantifier"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [2166, 49], "end": [2166, 60], "filename": "src/ast/query.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/query.rs:2166`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3a76d2a80455cd1144e71ba2"></a>
## eq

`function` · `sqlparser::ast::query::RepetitionQuantifier::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &RepetitionQuantifier) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::RepetitionQuantifier", "path": "RepetitionQuantifier"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2165, 24], "end": [2165, 33], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/query.rs:2165`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4c2d55695b11b15bca6723a5"></a>
## fmt

`function` · `sqlparser::ast::query::RepetitionQuantifier::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::RepetitionQuantifier", "path": "RepetitionQuantifier"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2185, 1], "end": [2198, 2], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/query.rs:2186`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8e38d2d8496a3b3ce6815e47"></a>
## fmt

`function` · `sqlparser::ast::query::RepetitionQuantifier::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::RepetitionQuantifier", "path": "RepetitionQuantifier"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2165, 10], "end": [2165, 15], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/query.rs:2165`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4503383b2d321dfc0572bd2b"></a>
## hash

`function` · `sqlparser::ast::query::RepetitionQuantifier::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::RepetitionQuantifier", "path": "RepetitionQuantifier"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2165, 56], "end": [2165, 60], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/query.rs:2165`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-66d0138d82e1595f36de8120"></a>
## partial_cmp

`function` · `sqlparser::ast::query::RepetitionQuantifier::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &RepetitionQuantifier) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::RepetitionQuantifier", "path": "RepetitionQuantifier"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2165, 35], "end": [2165, 45], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/query.rs:2165`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d4afd6d8b94c95fc979067e4"></a>
## serialize

`function` · `sqlparser::ast::query::RepetitionQuantifier::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::RepetitionQuantifier", "path": "RepetitionQuantifier"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2166, 38], "end": [2166, 47], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/query.rs:2166`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-08c59ae250cd2c52d82a1036"></a>
## visit

`function` · `sqlparser::ast::query::RepetitionQuantifier::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::RepetitionQuantifier", "path": "RepetitionQuantifier"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2167, 47], "end": [2167, 55], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/query.rs:2167`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4bd58d17d92cf189f7f93bd2"></a>
## visit

`function` · `sqlparser::ast::query::RepetitionQuantifier::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::RepetitionQuantifier", "path": "RepetitionQuantifier"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2167, 40], "end": [2167, 45], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/query.rs:2167`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
