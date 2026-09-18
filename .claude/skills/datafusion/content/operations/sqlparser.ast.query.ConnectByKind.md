# `sqlparser::ast::query::ConnectByKind`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.query.ConnectByKind.json).

<a id="op-344e1c3a4410f887e7f2da38"></a>
## ConnectByKind

`enum` · `sqlparser::ast::query::ConnectByKind` · sqlparser 0.62.0

```rust
enum ConnectByKind
```

Source: `src/ast/query.rs:1234`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Joins a table to itself to process hierarchical data in the table.

See <https://docs.snowflake.com/en/sql-reference/constructs/connect-by>.
See <https://docs.oracle.com/en/database/oracle/oracle-database/21/sqlrf/Hierarchical-Queries.html>

<a id="op-bfbf8b23fe8694969b7ac408"></a>
## ConnectBy

`variant` · `sqlparser::ast::query::ConnectByKind::ConnectBy` · sqlparser 0.62.0

```rust
ConnectBy
```

Source: `src/ast/query.rs:1236`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

CONNECT BY

<a id="op-45dadbc1dfde21cb9b46fda5"></a>
## StartWith

`variant` · `sqlparser::ast::query::ConnectByKind::StartWith` · sqlparser 0.62.0

```rust
StartWith
```

Source: `src/ast/query.rs:1253`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

START WITH

Optional on [Oracle](https://docs.oracle.com/en/database/oracle/oracle-database/21/sqlrf/Hierarchical-Queries.html#GUID-0118DF1D-B9A9-41EB-8556-C6E7D6A5A84E)
when comming _after_ the `CONNECT BY`.

<a id="op-9c680bd248fba164eba842e9"></a>
## clone

`function` · `sqlparser::ast::query::ConnectByKind::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> ConnectByKind
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::ConnectByKind", "path": "ConnectByKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1231, 17], "end": [1231, 22], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/query.rs:1231`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-abe4a87653ced6ba39158e28"></a>
## cmp

`function` · `sqlparser::ast::query::ConnectByKind::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &ConnectByKind) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::ConnectByKind", "path": "ConnectByKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1231, 51], "end": [1231, 54], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/query.rs:1231`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1e2f86eb0964c77cbe228e1f"></a>
## deserialize

`function` · `sqlparser::ast::query::ConnectByKind::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::ConnectByKind", "path": "ConnectByKind"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1232, 49], "end": [1232, 60], "filename": "src/ast/query.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/query.rs:1232`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b64b6606fe178f5bf3da06cb"></a>
## eq

`function` · `sqlparser::ast::query::ConnectByKind::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &ConnectByKind) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::ConnectByKind", "path": "ConnectByKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1231, 24], "end": [1231, 33], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/query.rs:1231`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9efe990574281ea9a8798f0d"></a>
## fmt

`function` · `sqlparser::ast::query::ConnectByKind::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::ConnectByKind", "path": "ConnectByKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1231, 10], "end": [1231, 15], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/query.rs:1231`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-eec214dfbf9bbc66d3a4e17d"></a>
## fmt

`function` · `sqlparser::ast::query::ConnectByKind::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::ConnectByKind", "path": "ConnectByKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1262, 1], "end": [1285, 2], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/query.rs:1263`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-49f61f734f95968e4cb1fc33"></a>
## hash

`function` · `sqlparser::ast::query::ConnectByKind::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::ConnectByKind", "path": "ConnectByKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1231, 56], "end": [1231, 60], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/query.rs:1231`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1479cf23cf7ddb29e7a90ce3"></a>
## partial_cmp

`function` · `sqlparser::ast::query::ConnectByKind::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &ConnectByKind) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::ConnectByKind", "path": "ConnectByKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1231, 35], "end": [1231, 45], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/query.rs:1231`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0b6a224b333143ec48d61f55"></a>
## serialize

`function` · `sqlparser::ast::query::ConnectByKind::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::ConnectByKind", "path": "ConnectByKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1232, 38], "end": [1232, 47], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/query.rs:1232`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ca17496dbc8de279e6d47151"></a>
## span

`function` · `sqlparser::ast::query::ConnectByKind::span` · sqlparser 0.62.0

```rust
fn span(&self) -> Span
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::ConnectByKind", "path": "super::ConnectByKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2335, 1], "end": [2352, 2], "filename": "src/ast/spans.rs"}, "trait": {"args": null, "id": "sqlparser::ast::spans::Spanned", "path": "Spanned"}, "trait_path": "sqlparser::ast::spans::Spanned"}`

Source: `src/ast/spans.rs:2336`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0a8f35b92ddf133a2283f1b8"></a>
## visit

`function` · `sqlparser::ast::query::ConnectByKind::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::ConnectByKind", "path": "ConnectByKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1233, 47], "end": [1233, 55], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/query.rs:1233`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ae2186a806460c52b0e4c0a9"></a>
## visit

`function` · `sqlparser::ast::query::ConnectByKind::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::ConnectByKind", "path": "ConnectByKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1233, 40], "end": [1233, 45], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/query.rs:1233`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
