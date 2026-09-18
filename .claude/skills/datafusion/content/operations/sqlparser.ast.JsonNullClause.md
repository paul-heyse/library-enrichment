# `sqlparser::ast::JsonNullClause`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.JsonNullClause.json).

<a id="op-c6b39beee26e4cf08574fa00"></a>
## JsonNullClause

`enum` · `sqlparser::ast::JsonNullClause` · sqlparser 0.62.0

```rust
enum JsonNullClause
```

Source: `src/ast/mod.rs:10942`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

MSSQL's json null clause

```plaintext
<json_null_clause> ::=
      NULL ON NULL
    | ABSENT ON NULL
```

<https://learn.microsoft.com/en-us/sql/t-sql/functions/json-object-transact-sql?view=sql-server-ver16#json_null_clause>

<a id="op-97ae28bd411fd56adb67f878"></a>
## AbsentOnNull

`variant` · `sqlparser::ast::JsonNullClause::AbsentOnNull` · sqlparser 0.62.0

```rust
AbsentOnNull
```

Source: `src/ast/mod.rs:10946`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`ABSENT ON NULL` behavior for JSON functions.

<a id="op-ffeee635c92d253171a3ef73"></a>
## NullOnNull

`variant` · `sqlparser::ast::JsonNullClause::NullOnNull` · sqlparser 0.62.0

```rust
NullOnNull
```

Source: `src/ast/mod.rs:10944`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`NULL ON NULL` behavior for JSON functions.

<a id="op-9fac1743bf84610901cdb90b"></a>
## clone

`function` · `sqlparser::ast::JsonNullClause::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> JsonNullClause
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::JsonNullClause", "path": "JsonNullClause"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10939, 17], "end": [10939, 22], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/mod.rs:10939`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2727ab99686b3bcb53c1ccfa"></a>
## cmp

`function` · `sqlparser::ast::JsonNullClause::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &JsonNullClause) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::JsonNullClause", "path": "JsonNullClause"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10939, 51], "end": [10939, 54], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/mod.rs:10939`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-39b2310b96e20a533780c5de"></a>
## deserialize

`function` · `sqlparser::ast::JsonNullClause::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::JsonNullClause", "path": "JsonNullClause"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [10940, 49], "end": [10940, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/mod.rs:10940`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-48c7cf7a55a33350d7b5778f"></a>
## eq

`function` · `sqlparser::ast::JsonNullClause::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &JsonNullClause) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::JsonNullClause", "path": "JsonNullClause"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10939, 24], "end": [10939, 33], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/mod.rs:10939`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-858a8a0794c5290c1566a925"></a>
## fmt

`function` · `sqlparser::ast::JsonNullClause::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::JsonNullClause", "path": "JsonNullClause"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10949, 1], "end": [10956, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/mod.rs:10950`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b5780384f73c08f89a9a7bcb"></a>
## fmt

`function` · `sqlparser::ast::JsonNullClause::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::JsonNullClause", "path": "JsonNullClause"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10939, 10], "end": [10939, 15], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/mod.rs:10939`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0ef9eea2ba16bda4f2b3f419"></a>
## hash

`function` · `sqlparser::ast::JsonNullClause::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::JsonNullClause", "path": "JsonNullClause"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10939, 56], "end": [10939, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/mod.rs:10939`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-635a97269f82c8371e2dbf3a"></a>
## partial_cmp

`function` · `sqlparser::ast::JsonNullClause::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &JsonNullClause) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::JsonNullClause", "path": "JsonNullClause"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10939, 35], "end": [10939, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/mod.rs:10939`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cfdb73b3133a899b5f1042b1"></a>
## serialize

`function` · `sqlparser::ast::JsonNullClause::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::JsonNullClause", "path": "JsonNullClause"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10940, 38], "end": [10940, 47], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/mod.rs:10940`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9cd75ae087089a13e56ca4be"></a>
## visit

`function` · `sqlparser::ast::JsonNullClause::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::JsonNullClause", "path": "JsonNullClause"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10941, 40], "end": [10941, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/mod.rs:10941`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fb0bff7a52f4e40656f457e1"></a>
## visit

`function` · `sqlparser::ast::JsonNullClause::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::JsonNullClause", "path": "JsonNullClause"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10941, 47], "end": [10941, 55], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/mod.rs:10941`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
