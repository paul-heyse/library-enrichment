# `sqlparser::ast::ddl::DropPolicy`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.ddl.DropPolicy.json).

<a id="op-274c4ac206ea3f4c0ed75705"></a>
## DropPolicy

`struct` · `sqlparser::ast::ddl::DropPolicy` · sqlparser 0.62.0

```rust
struct DropPolicy
```

Source: `src/ast/ddl.rs:5684`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

DROP POLICY statement.

See [PostgreSQL](https://www.postgresql.org/docs/current/sql-droppolicy.html)

<a id="op-49bc7a1d82904e5c34a7a770"></a>
## clone

`function` · `sqlparser::ast::ddl::DropPolicy::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> DropPolicy
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::DropPolicy", "path": "DropPolicy"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [5681, 17], "end": [5681, 22], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/ddl.rs:5681`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c09797ab3c52fe06e8c8345c"></a>
## cmp

`function` · `sqlparser::ast::ddl::DropPolicy::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &DropPolicy) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::DropPolicy", "path": "DropPolicy"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [5681, 51], "end": [5681, 54], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/ddl.rs:5681`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b42c7e56d6394e92231d6efe"></a>
## deserialize

`function` · `sqlparser::ast::ddl::DropPolicy::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::DropPolicy", "path": "DropPolicy"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [5682, 49], "end": [5682, 60], "filename": "src/ast/ddl.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/ddl.rs:5682`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d7ab51e5753a24b0de538fd4"></a>
## drop_behavior

`struct_field` · `sqlparser::ast::ddl::DropPolicy::drop_behavior` · sqlparser 0.62.0

```rust
drop_behavior: Option<DropBehavior>
```

Source: `src/ast/ddl.rs:5693`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional drop behavior (`CASCADE` or `RESTRICT`).

<a id="op-a337ac76267690d1ce98ed4d"></a>
## eq

`function` · `sqlparser::ast::ddl::DropPolicy::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &DropPolicy) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::DropPolicy", "path": "DropPolicy"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [5681, 24], "end": [5681, 33], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/ddl.rs:5681`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0b95f67cb50ef2d71139eb8a"></a>
## fmt

`function` · `sqlparser::ast::ddl::DropPolicy::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::DropPolicy", "path": "DropPolicy"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [5696, 1], "end": [5710, 2], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/ddl.rs:5697`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-694f0998c007cbbf298dc266"></a>
## fmt

`function` · `sqlparser::ast::ddl::DropPolicy::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::DropPolicy", "path": "DropPolicy"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [5681, 10], "end": [5681, 15], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/ddl.rs:5681`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3636e13c550da14e1d8f74ec"></a>
## hash

`function` · `sqlparser::ast::ddl::DropPolicy::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::DropPolicy", "path": "DropPolicy"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [5681, 56], "end": [5681, 60], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/ddl.rs:5681`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2a2400fe1d7a31ecbdd90005"></a>
## if_exists

`struct_field` · `sqlparser::ast::ddl::DropPolicy::if_exists` · sqlparser 0.62.0

```rust
if_exists: bool
```

Source: `src/ast/ddl.rs:5686`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`true` when `IF EXISTS` was present.

<a id="op-b25bfe7d1224b7977ba2487e"></a>
## name

`struct_field` · `sqlparser::ast::ddl::DropPolicy::name` · sqlparser 0.62.0

```rust
name: ast::Ident
```

Source: `src/ast/ddl.rs:5688`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Name of the policy to drop.

<a id="op-0701d391d65c2ac93135a278"></a>
## partial_cmp

`function` · `sqlparser::ast::ddl::DropPolicy::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &DropPolicy) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::DropPolicy", "path": "DropPolicy"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [5681, 35], "end": [5681, 45], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/ddl.rs:5681`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-86e3e34d30a0ad491b74e48d"></a>
## serialize

`function` · `sqlparser::ast::ddl::DropPolicy::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::DropPolicy", "path": "DropPolicy"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [5682, 38], "end": [5682, 47], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/ddl.rs:5682`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7c625e35d8fb4b5b4c41eeab"></a>
## table_name

`struct_field` · `sqlparser::ast::ddl::DropPolicy::table_name` · sqlparser 0.62.0

```rust
table_name: ast::ObjectName
```

Source: `src/ast/ddl.rs:5691`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Name of the table the policy applies to.

<a id="op-8af231a471a26a1c31e9437c"></a>
## visit

`function` · `sqlparser::ast::ddl::DropPolicy::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::DropPolicy", "path": "DropPolicy"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [5683, 47], "end": [5683, 55], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/ddl.rs:5683`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f454eea1c674b68341cc8dfb"></a>
## visit

`function` · `sqlparser::ast::ddl::DropPolicy::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::DropPolicy", "path": "DropPolicy"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [5683, 40], "end": [5683, 45], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/ddl.rs:5683`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
