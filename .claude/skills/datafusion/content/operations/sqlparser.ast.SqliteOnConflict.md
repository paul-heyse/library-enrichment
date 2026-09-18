# `sqlparser::ast::SqliteOnConflict`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.SqliteOnConflict.json).

<a id="op-ae68228c1a42af2daefc01af"></a>
## SqliteOnConflict

`enum` · `sqlparser::ast::SqliteOnConflict` · sqlparser 0.62.0

```rust
enum SqliteOnConflict
```

Source: `src/ast/mod.rs:9196`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Sqlite specific syntax

See [Sqlite documentation](https://sqlite.org/lang_conflict.html)
for more details.

<a id="op-a916ba77c7327efe738cfcd2"></a>
## Abort

`variant` · `sqlparser::ast::SqliteOnConflict::Abort` · sqlparser 0.62.0

```rust
Abort
```

Source: `src/ast/mod.rs:9200`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Use ABORT on conflict.

<a id="op-cfac32bf2404ec6ade9e2f57"></a>
## Fail

`variant` · `sqlparser::ast::SqliteOnConflict::Fail` · sqlparser 0.62.0

```rust
Fail
```

Source: `src/ast/mod.rs:9202`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Use FAIL on conflict.

<a id="op-52a8cf4526f233bf2f86c7b0"></a>
## Ignore

`variant` · `sqlparser::ast::SqliteOnConflict::Ignore` · sqlparser 0.62.0

```rust
Ignore
```

Source: `src/ast/mod.rs:9204`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Use IGNORE on conflict.

<a id="op-652e5ca104b62e67ae556fa7"></a>
## Replace

`variant` · `sqlparser::ast::SqliteOnConflict::Replace` · sqlparser 0.62.0

```rust
Replace
```

Source: `src/ast/mod.rs:9206`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Use REPLACE on conflict.

<a id="op-b1af3a35d0316eb6eb0418ae"></a>
## Rollback

`variant` · `sqlparser::ast::SqliteOnConflict::Rollback` · sqlparser 0.62.0

```rust
Rollback
```

Source: `src/ast/mod.rs:9198`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Use ROLLBACK on conflict.

<a id="op-3f408b2c2768fb496c29aa13"></a>
## clone

`function` · `sqlparser::ast::SqliteOnConflict::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> SqliteOnConflict
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::SqliteOnConflict", "path": "SqliteOnConflict"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9193, 23], "end": [9193, 28], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/mod.rs:9193`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-dc88b9371fee39c0f62cdf1b"></a>
## cmp

`function` · `sqlparser::ast::SqliteOnConflict::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &SqliteOnConflict) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::SqliteOnConflict", "path": "SqliteOnConflict"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9193, 57], "end": [9193, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/mod.rs:9193`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bd9194fb3ace6f584fe212d9"></a>
## deserialize

`function` · `sqlparser::ast::SqliteOnConflict::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::SqliteOnConflict", "path": "SqliteOnConflict"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [9194, 49], "end": [9194, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/mod.rs:9194`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0da61c243ce9cbd2232627ad"></a>
## eq

`function` · `sqlparser::ast::SqliteOnConflict::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &SqliteOnConflict) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::SqliteOnConflict", "path": "SqliteOnConflict"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9193, 30], "end": [9193, 39], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/mod.rs:9193`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c5860397e7d4c705246300eb"></a>
## fmt

`function` · `sqlparser::ast::SqliteOnConflict::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::SqliteOnConflict", "path": "SqliteOnConflict"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9209, 1], "end": [9220, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/mod.rs:9210`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f20a622a5156c0356514271a"></a>
## fmt

`function` · `sqlparser::ast::SqliteOnConflict::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::SqliteOnConflict", "path": "SqliteOnConflict"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9193, 10], "end": [9193, 15], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/mod.rs:9193`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c2477abfc37851135da197ac"></a>
## hash

`function` · `sqlparser::ast::SqliteOnConflict::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::SqliteOnConflict", "path": "SqliteOnConflict"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9193, 62], "end": [9193, 66], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/mod.rs:9193`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ac9ae93314f0daff7c027808"></a>
## partial_cmp

`function` · `sqlparser::ast::SqliteOnConflict::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &SqliteOnConflict) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::SqliteOnConflict", "path": "SqliteOnConflict"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9193, 41], "end": [9193, 51], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/mod.rs:9193`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fc3779c4fc26758a4c4dd76a"></a>
## serialize

`function` · `sqlparser::ast::SqliteOnConflict::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::SqliteOnConflict", "path": "SqliteOnConflict"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9194, 38], "end": [9194, 47], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/mod.rs:9194`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8d8642b34f22a2ba36c2aa98"></a>
## visit

`function` · `sqlparser::ast::SqliteOnConflict::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::SqliteOnConflict", "path": "SqliteOnConflict"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9195, 47], "end": [9195, 55], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/mod.rs:9195`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9407fd9e31df4f7f376ce3e7"></a>
## visit

`function` · `sqlparser::ast::SqliteOnConflict::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::SqliteOnConflict", "path": "SqliteOnConflict"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9195, 40], "end": [9195, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/mod.rs:9195`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
