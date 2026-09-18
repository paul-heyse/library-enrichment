# `sqlparser::ast::ShowStatementFilterPosition`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.ShowStatementFilterPosition.json).

<a id="op-52b5145384592414d3bb2ddc"></a>
## ShowStatementFilterPosition

`enum` · `sqlparser::ast::ShowStatementFilterPosition` · sqlparser 0.62.0

```rust
enum ShowStatementFilterPosition
```

Source: `src/ast/mod.rs:10829`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Where a `SHOW` filter appears relative to the main clause.

<a id="op-55f618ba0914de67ff086cd7"></a>
## Infix

`variant` · `sqlparser::ast::ShowStatementFilterPosition::Infix` · sqlparser 0.62.0

```rust
Infix
```

Source: `src/ast/mod.rs:10831`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Put the filter in an infix position (e.g. `SHOW COLUMNS LIKE '%name%' IN TABLE tbl`).

<a id="op-88f130430a4f4dae05124ec5"></a>
## Suffix

`variant` · `sqlparser::ast::ShowStatementFilterPosition::Suffix` · sqlparser 0.62.0

```rust
Suffix
```

Source: `src/ast/mod.rs:10833`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Put the filter in a suffix position (e.g. `SHOW COLUMNS IN tbl LIKE '%name%'`).

<a id="op-bcdb8dd826782fcd79647878"></a>
## clone

`function` · `sqlparser::ast::ShowStatementFilterPosition::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> ShowStatementFilterPosition
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ShowStatementFilterPosition", "path": "ShowStatementFilterPosition"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10825, 17], "end": [10825, 22], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/mod.rs:10825`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-694624b8a89bb7dfec533dd4"></a>
## cmp

`function` · `sqlparser::ast::ShowStatementFilterPosition::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &ShowStatementFilterPosition) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ShowStatementFilterPosition", "path": "ShowStatementFilterPosition"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10825, 51], "end": [10825, 54], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/mod.rs:10825`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7c354045db634a31eb9a8d17"></a>
## deserialize

`function` · `sqlparser::ast::ShowStatementFilterPosition::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ShowStatementFilterPosition", "path": "ShowStatementFilterPosition"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [10826, 49], "end": [10826, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/mod.rs:10826`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d5f5c7332517a4bff60a7577"></a>
## eq

`function` · `sqlparser::ast::ShowStatementFilterPosition::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &ShowStatementFilterPosition) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ShowStatementFilterPosition", "path": "ShowStatementFilterPosition"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10825, 24], "end": [10825, 33], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/mod.rs:10825`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ce4002308928aefad55f36b9"></a>
## fmt

`function` · `sqlparser::ast::ShowStatementFilterPosition::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ShowStatementFilterPosition", "path": "ShowStatementFilterPosition"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10825, 10], "end": [10825, 15], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/mod.rs:10825`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1d71bc4ffdeba7e7c252e5d8"></a>
## hash

`function` · `sqlparser::ast::ShowStatementFilterPosition::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ShowStatementFilterPosition", "path": "ShowStatementFilterPosition"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10825, 56], "end": [10825, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/mod.rs:10825`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5456af34c6d08f45ae088ffe"></a>
## partial_cmp

`function` · `sqlparser::ast::ShowStatementFilterPosition::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &ShowStatementFilterPosition) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ShowStatementFilterPosition", "path": "ShowStatementFilterPosition"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10825, 35], "end": [10825, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/mod.rs:10825`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1e16ca8c9405ee6fbe371da0"></a>
## serialize

`function` · `sqlparser::ast::ShowStatementFilterPosition::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ShowStatementFilterPosition", "path": "ShowStatementFilterPosition"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10826, 38], "end": [10826, 47], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/mod.rs:10826`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4fbaefc87ef38a9f491ec53c"></a>
## visit

`function` · `sqlparser::ast::ShowStatementFilterPosition::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ShowStatementFilterPosition", "path": "ShowStatementFilterPosition"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10827, 47], "end": [10827, 55], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/mod.rs:10827`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f3ee7e590098b6778ad8a893"></a>
## visit

`function` · `sqlparser::ast::ShowStatementFilterPosition::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ShowStatementFilterPosition", "path": "ShowStatementFilterPosition"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10827, 40], "end": [10827, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/mod.rs:10827`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
