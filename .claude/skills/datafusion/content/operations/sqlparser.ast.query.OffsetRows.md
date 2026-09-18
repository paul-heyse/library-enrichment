# `sqlparser::ast::query::OffsetRows`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.query.OffsetRows.json).

<a id="op-0ba5cf34d10498fb3dd7dfed"></a>
## OffsetRows

`enum` · `sqlparser::ast::query::OffsetRows` · sqlparser 0.62.0

```rust
enum OffsetRows
```

Source: `src/ast/query.rs:3124`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Stores the keyword after `OFFSET <number>`

<a id="op-82f1337aadb191fd45d072e4"></a>
## None

`variant` · `sqlparser::ast::query::OffsetRows::None` · sqlparser 0.62.0

```rust
None
```

Source: `src/ast/query.rs:3126`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Omitting `ROW`/`ROWS` entirely (non-standard MySQL quirk).

<a id="op-52674a0f92bcd2f2475225b1"></a>
## Row

`variant` · `sqlparser::ast::query::OffsetRows::Row` · sqlparser 0.62.0

```rust
Row
```

Source: `src/ast/query.rs:3128`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`ROW` keyword present.

<a id="op-bd4373ebb0038c54de3ae1b2"></a>
## Rows

`variant` · `sqlparser::ast::query::OffsetRows::Rows` · sqlparser 0.62.0

```rust
Rows
```

Source: `src/ast/query.rs:3130`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`ROWS` keyword present.

<a id="op-d277482029b329fc9211466b"></a>
## clone

`function` · `sqlparser::ast::query::OffsetRows::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> OffsetRows
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::OffsetRows", "path": "OffsetRows"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3121, 23], "end": [3121, 28], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/query.rs:3121`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b0a620effa382b55669ea3ee"></a>
## cmp

`function` · `sqlparser::ast::query::OffsetRows::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &OffsetRows) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::OffsetRows", "path": "OffsetRows"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3121, 57], "end": [3121, 60], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/query.rs:3121`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f3a015def3dc4af6edf2c59d"></a>
## deserialize

`function` · `sqlparser::ast::query::OffsetRows::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::OffsetRows", "path": "OffsetRows"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [3122, 49], "end": [3122, 60], "filename": "src/ast/query.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/query.rs:3122`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6ec26279c43dd01f2d0f5f81"></a>
## eq

`function` · `sqlparser::ast::query::OffsetRows::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &OffsetRows) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::OffsetRows", "path": "OffsetRows"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3121, 30], "end": [3121, 39], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/query.rs:3121`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-46de7e3440613d94f041a548"></a>
## fmt

`function` · `sqlparser::ast::query::OffsetRows::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::OffsetRows", "path": "OffsetRows"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3133, 1], "end": [3141, 2], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/query.rs:3134`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cc0832de759085dec3c6d559"></a>
## fmt

`function` · `sqlparser::ast::query::OffsetRows::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::OffsetRows", "path": "OffsetRows"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3121, 10], "end": [3121, 15], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/query.rs:3121`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-be72baac6ef9e74683a8df40"></a>
## hash

`function` · `sqlparser::ast::query::OffsetRows::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::OffsetRows", "path": "OffsetRows"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3121, 62], "end": [3121, 66], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/query.rs:3121`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6a025b93c79eb8b2f9e48c13"></a>
## partial_cmp

`function` · `sqlparser::ast::query::OffsetRows::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &OffsetRows) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::OffsetRows", "path": "OffsetRows"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3121, 41], "end": [3121, 51], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/query.rs:3121`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f862a82a330cbdcc23f8bb83"></a>
## serialize

`function` · `sqlparser::ast::query::OffsetRows::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::OffsetRows", "path": "OffsetRows"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3122, 38], "end": [3122, 47], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/query.rs:3122`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-862f87aeaba42a79662d84a7"></a>
## visit

`function` · `sqlparser::ast::query::OffsetRows::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::OffsetRows", "path": "OffsetRows"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3123, 47], "end": [3123, 55], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/query.rs:3123`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-99ba5430f43643532f6a860e"></a>
## visit

`function` · `sqlparser::ast::query::OffsetRows::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::OffsetRows", "path": "OffsetRows"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3123, 40], "end": [3123, 45], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/query.rs:3123`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
