# `sqlparser::ast::CreateServerStatement`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.CreateServerStatement.json).

<a id="op-572f1238bc49ab302c67d23b"></a>
## CreateServerStatement

`struct` · `sqlparser::ast::CreateServerStatement` · sqlparser 0.62.0

```rust
struct CreateServerStatement
```

Source: `src/ast/mod.rs:8947`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

A `CREATE SERVER` statement.

[PostgreSQL Documentation](https://www.postgresql.org/docs/current/sql-createserver.html)

<a id="op-b85efc58e2e846da3bf3dc4c"></a>
## clone

`function` · `sqlparser::ast::CreateServerStatement::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> CreateServerStatement
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CreateServerStatement", "path": "CreateServerStatement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8944, 17], "end": [8944, 22], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/mod.rs:8944`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-02fd695cee4fb6441abd6046"></a>
## cmp

`function` · `sqlparser::ast::CreateServerStatement::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &CreateServerStatement) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CreateServerStatement", "path": "CreateServerStatement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8944, 51], "end": [8944, 54], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/mod.rs:8944`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6e5e1b11aa4a906682c04d8f"></a>
## deserialize

`function` · `sqlparser::ast::CreateServerStatement::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CreateServerStatement", "path": "CreateServerStatement"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [8945, 49], "end": [8945, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/mod.rs:8945`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c35b9be0d7bc9a0b0bbb6b2e"></a>
## eq

`function` · `sqlparser::ast::CreateServerStatement::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &CreateServerStatement) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CreateServerStatement", "path": "CreateServerStatement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8944, 24], "end": [8944, 33], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/mod.rs:8944`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1c3c419256368c2337c30d09"></a>
## fmt

`function` · `sqlparser::ast::CreateServerStatement::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CreateServerStatement", "path": "CreateServerStatement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8962, 1], "end": [8995, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/mod.rs:8963`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-70c3b81caf666d157ab8d1bb"></a>
## fmt

`function` · `sqlparser::ast::CreateServerStatement::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CreateServerStatement", "path": "CreateServerStatement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8944, 10], "end": [8944, 15], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/mod.rs:8944`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6effcd227b579d194345d600"></a>
## foreign_data_wrapper

`struct_field` · `sqlparser::ast::CreateServerStatement::foreign_data_wrapper` · sqlparser 0.62.0

```rust
foreign_data_wrapper: ObjectName
```

Source: `src/ast/mod.rs:8957`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Foreign-data wrapper object name.

<a id="op-e9a90af8a00b3fc8e5b1fb1e"></a>
## hash

`function` · `sqlparser::ast::CreateServerStatement::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CreateServerStatement", "path": "CreateServerStatement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8944, 56], "end": [8944, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/mod.rs:8944`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d47f2f66ae6cebd9fa4cdc74"></a>
## if_not_exists

`struct_field` · `sqlparser::ast::CreateServerStatement::if_not_exists` · sqlparser 0.62.0

```rust
if_not_exists: bool
```

Source: `src/ast/mod.rs:8951`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Whether `IF NOT EXISTS` was specified.

<a id="op-62d17310963c40df0bf762a2"></a>
## name

`struct_field` · `sqlparser::ast::CreateServerStatement::name` · sqlparser 0.62.0

```rust
name: ObjectName
```

Source: `src/ast/mod.rs:8949`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The server name.

<a id="op-bf72f37bba436ef05c155779"></a>
## options

`struct_field` · `sqlparser::ast::CreateServerStatement::options` · sqlparser 0.62.0

```rust
options: Option<Vec<CreateServerOption>>
```

Source: `src/ast/mod.rs:8959`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional list of server options.

<a id="op-6648b70ce7d0c6974cdd7671"></a>
## partial_cmp

`function` · `sqlparser::ast::CreateServerStatement::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &CreateServerStatement) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CreateServerStatement", "path": "CreateServerStatement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8944, 35], "end": [8944, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/mod.rs:8944`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a7b94d2ac766c8322b37e6b8"></a>
## serialize

`function` · `sqlparser::ast::CreateServerStatement::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CreateServerStatement", "path": "CreateServerStatement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8945, 38], "end": [8945, 47], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/mod.rs:8945`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c0d846a2abe27fedffbc8738"></a>
## server_type

`struct_field` · `sqlparser::ast::CreateServerStatement::server_type` · sqlparser 0.62.0

```rust
server_type: Option<Ident>
```

Source: `src/ast/mod.rs:8953`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional server type identifier.

<a id="op-e9596105999fb94569d07c2b"></a>
## version

`struct_field` · `sqlparser::ast::CreateServerStatement::version` · sqlparser 0.62.0

```rust
version: Option<Ident>
```

Source: `src/ast/mod.rs:8955`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional server version identifier.

<a id="op-c30a5750a1ceb84983b6d6f4"></a>
## visit

`function` · `sqlparser::ast::CreateServerStatement::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CreateServerStatement", "path": "CreateServerStatement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8946, 40], "end": [8946, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/mod.rs:8946`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d706cc3d87f57e1c6214b138"></a>
## visit

`function` · `sqlparser::ast::CreateServerStatement::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CreateServerStatement", "path": "CreateServerStatement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8946, 47], "end": [8946, 55], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/mod.rs:8946`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
