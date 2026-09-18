# `sqlparser::ast::ddl::CreateConnector`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.ddl.CreateConnector.json).

<a id="op-fb9ce52832910f997bf8c948"></a>
## CreateConnector

`struct` · `sqlparser::ast::ddl::CreateConnector` · sqlparser 0.62.0

```rust
struct CreateConnector
```

Source: `src/ast/ddl.rs:3739`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

```sql
CREATE CONNECTOR [IF NOT EXISTS] connector_name
[TYPE datasource_type]
[URL datasource_url]
[COMMENT connector_comment]
[WITH DCPROPERTIES(property_name=property_value, ...)]
```

[Hive](https://cwiki.apache.org/confluence/pages/viewpage.action?pageId=27362034#LanguageManualDDL-CreateDataConnectorCreateConnector)

<a id="op-9bd6448cf3131cef1475afd0"></a>
## clone

`function` · `sqlparser::ast::ddl::CreateConnector::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> CreateConnector
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::CreateConnector", "path": "CreateConnector"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3736, 17], "end": [3736, 22], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/ddl.rs:3736`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a0708dbc9cb6d9e2f226ccb8"></a>
## cmp

`function` · `sqlparser::ast::ddl::CreateConnector::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &CreateConnector) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::CreateConnector", "path": "CreateConnector"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3736, 51], "end": [3736, 54], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/ddl.rs:3736`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6fb72bdeaecb885d760213c1"></a>
## comment

`struct_field` · `sqlparser::ast::ddl::CreateConnector::comment` · sqlparser 0.62.0

```rust
comment: Option<ast::CommentDef>
```

Source: `src/ast/ddl.rs:3749`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The comment for the connector.

<a id="op-7e6948372a419074c8e82fa3"></a>
## connector_type

`struct_field` · `sqlparser::ast::ddl::CreateConnector::connector_type` · sqlparser 0.62.0

```rust
connector_type: Option<String>
```

Source: `src/ast/ddl.rs:3745`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The type of the connector.

<a id="op-99558ee2efa1b7c571879e24"></a>
## deserialize

`function` · `sqlparser::ast::ddl::CreateConnector::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::CreateConnector", "path": "CreateConnector"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [3737, 49], "end": [3737, 60], "filename": "src/ast/ddl.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/ddl.rs:3737`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7dd9b01d85cde112b0732843"></a>
## eq

`function` · `sqlparser::ast::ddl::CreateConnector::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &CreateConnector) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::CreateConnector", "path": "CreateConnector"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3736, 24], "end": [3736, 33], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/ddl.rs:3736`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8ed930ad4a3e763b5772b1b0"></a>
## fmt

`function` · `sqlparser::ast::ddl::CreateConnector::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::CreateConnector", "path": "CreateConnector"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3736, 10], "end": [3736, 15], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/ddl.rs:3736`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f857940c9e34c68439eefb5d"></a>
## fmt

`function` · `sqlparser::ast::ddl::CreateConnector::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::CreateConnector", "path": "CreateConnector"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3754, 1], "end": [3789, 2], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/ddl.rs:3755`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-577bc6212cc993ca2288689f"></a>
## hash

`function` · `sqlparser::ast::ddl::CreateConnector::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::CreateConnector", "path": "CreateConnector"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3736, 56], "end": [3736, 60], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/ddl.rs:3736`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-71864c35310cda1df42785aa"></a>
## if_not_exists

`struct_field` · `sqlparser::ast::ddl::CreateConnector::if_not_exists` · sqlparser 0.62.0

```rust
if_not_exists: bool
```

Source: `src/ast/ddl.rs:3743`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Whether `IF NOT EXISTS` was specified.

<a id="op-2e6c65c1758d1571ebd70e98"></a>
## name

`struct_field` · `sqlparser::ast::ddl::CreateConnector::name` · sqlparser 0.62.0

```rust
name: ast::Ident
```

Source: `src/ast/ddl.rs:3741`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The name of the connector to be created.

<a id="op-1689c17f53ec8b24096d25f2"></a>
## partial_cmp

`function` · `sqlparser::ast::ddl::CreateConnector::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &CreateConnector) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::CreateConnector", "path": "CreateConnector"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3736, 35], "end": [3736, 45], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/ddl.rs:3736`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-353711edfb33c4986e85daa7"></a>
## serialize

`function` · `sqlparser::ast::ddl::CreateConnector::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::CreateConnector", "path": "CreateConnector"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3737, 38], "end": [3737, 47], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/ddl.rs:3737`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ba6eba3edb94ed69882161c7"></a>
## url

`struct_field` · `sqlparser::ast::ddl::CreateConnector::url` · sqlparser 0.62.0

```rust
url: Option<String>
```

Source: `src/ast/ddl.rs:3747`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The URL of the connector.

<a id="op-5a804814d231c27f43cda7dc"></a>
## visit

`function` · `sqlparser::ast::ddl::CreateConnector::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::CreateConnector", "path": "CreateConnector"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3738, 47], "end": [3738, 55], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/ddl.rs:3738`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ecdc16aa4b332354490f6ea9"></a>
## visit

`function` · `sqlparser::ast::ddl::CreateConnector::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::CreateConnector", "path": "CreateConnector"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3738, 40], "end": [3738, 45], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/ddl.rs:3738`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0e1867a72b89407c4288de97"></a>
## with_dcproperties

`struct_field` · `sqlparser::ast::ddl::CreateConnector::with_dcproperties` · sqlparser 0.62.0

```rust
with_dcproperties: Option<Vec<ast::SqlOption>>
```

Source: `src/ast/ddl.rs:3751`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The DC properties for the connector.
