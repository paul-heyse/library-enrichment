# `sqlparser::ast::MySQLColumnPosition`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.MySQLColumnPosition.json).

<a id="op-6aec2e7b0dadf1a4f8ffde50"></a>
## MySQLColumnPosition

`enum` · `sqlparser::ast::MySQLColumnPosition` · sqlparser 0.62.0

```rust
enum MySQLColumnPosition
```

Source: `src/ast/mod.rs:10450`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

MySQL `ALTER TABLE` only  [FIRST | AFTER column_name]
MySQL `ALTER TABLE` column position specifier: `FIRST` or `AFTER <column>`.

<a id="op-11023924e8244d6507dcee34"></a>
## After

`variant` · `sqlparser::ast::MySQLColumnPosition::After` · sqlparser 0.62.0

```rust
After
```

Source: `src/ast/mod.rs:10454`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Place the column after the specified identifier.

<a id="op-22eb7854f12d6f5730843e1e"></a>
## First

`variant` · `sqlparser::ast::MySQLColumnPosition::First` · sqlparser 0.62.0

```rust
First
```

Source: `src/ast/mod.rs:10452`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Place the column first in the table.

<a id="op-1b778ffdc579ef29c141963a"></a>
## clone

`function` · `sqlparser::ast::MySQLColumnPosition::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> MySQLColumnPosition
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::MySQLColumnPosition", "path": "MySQLColumnPosition"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10446, 17], "end": [10446, 22], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/mod.rs:10446`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-188ad9b6bc42b0b57c2f8c9e"></a>
## cmp

`function` · `sqlparser::ast::MySQLColumnPosition::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &MySQLColumnPosition) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::MySQLColumnPosition", "path": "MySQLColumnPosition"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10446, 51], "end": [10446, 54], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/mod.rs:10446`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0b1a622b5db05b87f55b9db5"></a>
## deserialize

`function` · `sqlparser::ast::MySQLColumnPosition::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::MySQLColumnPosition", "path": "MySQLColumnPosition"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [10447, 49], "end": [10447, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/mod.rs:10447`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-da73cad3701a895dd17d0d6b"></a>
## eq

`function` · `sqlparser::ast::MySQLColumnPosition::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &MySQLColumnPosition) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::MySQLColumnPosition", "path": "MySQLColumnPosition"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10446, 24], "end": [10446, 33], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/mod.rs:10446`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1f65e93629682495d758a016"></a>
## fmt

`function` · `sqlparser::ast::MySQLColumnPosition::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::MySQLColumnPosition", "path": "MySQLColumnPosition"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10446, 10], "end": [10446, 15], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/mod.rs:10446`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-36e16aae20d397b78c20ae38"></a>
## fmt

`function` · `sqlparser::ast::MySQLColumnPosition::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::MySQLColumnPosition", "path": "MySQLColumnPosition"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10457, 1], "end": [10467, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/mod.rs:10458`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0cf7b2df222931ade4c55ee2"></a>
## hash

`function` · `sqlparser::ast::MySQLColumnPosition::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::MySQLColumnPosition", "path": "MySQLColumnPosition"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10446, 56], "end": [10446, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/mod.rs:10446`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-51e531d6e5df92285dfac0f5"></a>
## partial_cmp

`function` · `sqlparser::ast::MySQLColumnPosition::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &MySQLColumnPosition) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::MySQLColumnPosition", "path": "MySQLColumnPosition"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10446, 35], "end": [10446, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/mod.rs:10446`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fd5442187c58246abd5d474b"></a>
## serialize

`function` · `sqlparser::ast::MySQLColumnPosition::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::MySQLColumnPosition", "path": "MySQLColumnPosition"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10447, 38], "end": [10447, 47], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/mod.rs:10447`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-81f44af00c68eaf878ddff4a"></a>
## visit

`function` · `sqlparser::ast::MySQLColumnPosition::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::MySQLColumnPosition", "path": "MySQLColumnPosition"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10448, 47], "end": [10448, 55], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/mod.rs:10448`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bac19925fe8bbb28e7e56325"></a>
## visit

`function` · `sqlparser::ast::MySQLColumnPosition::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::MySQLColumnPosition", "path": "MySQLColumnPosition"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10448, 40], "end": [10448, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/mod.rs:10448`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
