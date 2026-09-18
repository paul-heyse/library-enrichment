# `sqlparser::ast::ddl::DropTrigger`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.ddl.DropTrigger.json).

<a id="op-9596571eea72374765c7d35c"></a>
## DropTrigger

`struct` · `sqlparser::ast::ddl::DropTrigger` · sqlparser 0.62.0

```rust
struct DropTrigger
```

Source: `src/ast/ddl.rs:4142`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

DROP TRIGGER

```sql
DROP TRIGGER [ IF EXISTS ] name ON table_name [ CASCADE | RESTRICT ]
```


<a id="op-a1537b4cf3b7aff3d899ed58"></a>
## clone

`function` · `sqlparser::ast::ddl::DropTrigger::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> DropTrigger
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::DropTrigger", "path": "DropTrigger"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4133, 17], "end": [4133, 22], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/ddl.rs:4133`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-74134902979891261c4b684c"></a>
## cmp

`function` · `sqlparser::ast::ddl::DropTrigger::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &DropTrigger) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::DropTrigger", "path": "DropTrigger"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4133, 51], "end": [4133, 54], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/ddl.rs:4133`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ca44d7051534d27e80cb35d8"></a>
## deserialize

`function` · `sqlparser::ast::ddl::DropTrigger::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::DropTrigger", "path": "DropTrigger"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [4134, 49], "end": [4134, 60], "filename": "src/ast/ddl.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/ddl.rs:4134`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e28cd6705b1ad524d2444bb3"></a>
## eq

`function` · `sqlparser::ast::ddl::DropTrigger::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &DropTrigger) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::DropTrigger", "path": "DropTrigger"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4133, 24], "end": [4133, 33], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/ddl.rs:4133`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0c99c0aad32235ea6f89da49"></a>
## fmt

`function` · `sqlparser::ast::ddl::DropTrigger::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::DropTrigger", "path": "DropTrigger"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4153, 1], "end": [4174, 2], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/ddl.rs:4154`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4178d1f3fee122653fb929e0"></a>
## fmt

`function` · `sqlparser::ast::ddl::DropTrigger::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::DropTrigger", "path": "DropTrigger"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4133, 10], "end": [4133, 15], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/ddl.rs:4133`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-962f3ad56618a04cdeac0915"></a>
## hash

`function` · `sqlparser::ast::ddl::DropTrigger::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::DropTrigger", "path": "DropTrigger"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4133, 56], "end": [4133, 60], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/ddl.rs:4133`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0ba55ba5dede8cb34222b549"></a>
## if_exists

`struct_field` · `sqlparser::ast::ddl::DropTrigger::if_exists` · sqlparser 0.62.0

```rust
if_exists: bool
```

Source: `src/ast/ddl.rs:4144`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Whether to include the `IF EXISTS` clause.

<a id="op-1b4f2d89f17b5453be865a02"></a>
## option

`struct_field` · `sqlparser::ast::ddl::DropTrigger::option` · sqlparser 0.62.0

```rust
option: Option<ReferentialAction>
```

Source: `src/ast/ddl.rs:4150`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`CASCADE` or `RESTRICT`

<a id="op-64baadfeff2e5c0d81a3d8d7"></a>
## partial_cmp

`function` · `sqlparser::ast::ddl::DropTrigger::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &DropTrigger) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::DropTrigger", "path": "DropTrigger"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4133, 35], "end": [4133, 45], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/ddl.rs:4133`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a98801d70e7f6869502220f7"></a>
## serialize

`function` · `sqlparser::ast::ddl::DropTrigger::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::DropTrigger", "path": "DropTrigger"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4134, 38], "end": [4134, 47], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/ddl.rs:4134`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-84aa60e9d8bb0bf362bb4b24"></a>
## table_name

`struct_field` · `sqlparser::ast::ddl::DropTrigger::table_name` · sqlparser 0.62.0

```rust
table_name: Option<ast::ObjectName>
```

Source: `src/ast/ddl.rs:4148`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The name of the table from which the trigger is to be dropped.

<a id="op-ea2a372e9f8219ade6296bbb"></a>
## trigger_name

`struct_field` · `sqlparser::ast::ddl::DropTrigger::trigger_name` · sqlparser 0.62.0

```rust
trigger_name: ast::ObjectName
```

Source: `src/ast/ddl.rs:4146`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The name of the trigger to be dropped.

<a id="op-08edee672c4f80a49adf713e"></a>
## visit

`function` · `sqlparser::ast::ddl::DropTrigger::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::DropTrigger", "path": "DropTrigger"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4135, 40], "end": [4135, 45], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/ddl.rs:4135`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f40b2b1797a62944e751f0a0"></a>
## visit

`function` · `sqlparser::ast::ddl::DropTrigger::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::DropTrigger", "path": "DropTrigger"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4135, 47], "end": [4135, 55], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/ddl.rs:4135`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
