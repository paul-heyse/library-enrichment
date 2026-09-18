# `sqlparser::ast::ddl::CreateDomain`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.ddl.CreateDomain.json).

<a id="op-47dc1e99abfdadb1a7dc4d61"></a>
## CreateDomain

`struct` · `sqlparser::ast::ddl::CreateDomain` · sqlparser 0.62.0

```rust
struct CreateDomain
```

Source: `src/ast/ddl.rs:3509`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

```sql
CREATE DOMAIN name [ AS ] data_type
        [ COLLATE collation ]
        [ DEFAULT expression ]
        [ domain_constraint [ ... ] ]

    where domain_constraint is:

    [ CONSTRAINT constraint_name ]
    { NOT NULL | NULL | CHECK (expression) }
```
See [PostgreSQL](https://www.postgresql.org/docs/current/sql-createdomain.html)

<a id="op-1f12aea2b60f14714de35875"></a>
## clone

`function` · `sqlparser::ast::ddl::CreateDomain::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> CreateDomain
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::CreateDomain", "path": "CreateDomain"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3494, 17], "end": [3494, 22], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/ddl.rs:3494`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b3d4b54e01c3c7f5ffa26447"></a>
## cmp

`function` · `sqlparser::ast::ddl::CreateDomain::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &CreateDomain) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::CreateDomain", "path": "CreateDomain"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3494, 51], "end": [3494, 54], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/ddl.rs:3494`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fa9bea567ec0c446ffc0b723"></a>
## collation

`struct_field` · `sqlparser::ast::ddl::CreateDomain::collation` · sqlparser 0.62.0

```rust
collation: Option<ast::Ident>
```

Source: `src/ast/ddl.rs:3515`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The collation of the domain.

<a id="op-e62fc83fcce425308a55ff57"></a>
## constraints

`struct_field` · `sqlparser::ast::ddl::CreateDomain::constraints` · sqlparser 0.62.0

```rust
constraints: Vec<ast::table_constraints::TableConstraint>
```

Source: `src/ast/ddl.rs:3519`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The constraints of the domain.

<a id="op-4009f84b2a90fe1fa9e8b9a0"></a>
## data_type

`struct_field` · `sqlparser::ast::ddl::CreateDomain::data_type` · sqlparser 0.62.0

```rust
data_type: ast::DataType
```

Source: `src/ast/ddl.rs:3513`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The data type of the domain.

<a id="op-1d025b3cf0df4288bb4ca754"></a>
## default

`struct_field` · `sqlparser::ast::ddl::CreateDomain::default` · sqlparser 0.62.0

```rust
default: Option<ast::Expr>
```

Source: `src/ast/ddl.rs:3517`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The default value of the domain.

<a id="op-0f98b7257d0f975cafc4bbba"></a>
## deserialize

`function` · `sqlparser::ast::ddl::CreateDomain::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::CreateDomain", "path": "CreateDomain"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [3495, 49], "end": [3495, 60], "filename": "src/ast/ddl.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/ddl.rs:3495`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2ce3fde9e26ab1768a1475eb"></a>
## eq

`function` · `sqlparser::ast::ddl::CreateDomain::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &CreateDomain) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::CreateDomain", "path": "CreateDomain"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3494, 24], "end": [3494, 33], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/ddl.rs:3494`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bb7d93b0c73ec8fde4a9d191"></a>
## fmt

`function` · `sqlparser::ast::ddl::CreateDomain::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::CreateDomain", "path": "CreateDomain"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3522, 1], "end": [3541, 2], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/ddl.rs:3523`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d0cc0f2fa2425230da63e9af"></a>
## fmt

`function` · `sqlparser::ast::ddl::CreateDomain::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::CreateDomain", "path": "CreateDomain"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3494, 10], "end": [3494, 15], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/ddl.rs:3494`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-36b0ba5c677d6fb8cbc08be9"></a>
## hash

`function` · `sqlparser::ast::ddl::CreateDomain::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::CreateDomain", "path": "CreateDomain"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3494, 56], "end": [3494, 60], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/ddl.rs:3494`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ee38e5ee2b2d8f799aa2d321"></a>
## name

`struct_field` · `sqlparser::ast::ddl::CreateDomain::name` · sqlparser 0.62.0

```rust
name: ast::ObjectName
```

Source: `src/ast/ddl.rs:3511`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The name of the domain to be created.

<a id="op-d97b43df00ef8a396b15a8b4"></a>
## partial_cmp

`function` · `sqlparser::ast::ddl::CreateDomain::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &CreateDomain) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::CreateDomain", "path": "CreateDomain"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3494, 35], "end": [3494, 45], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/ddl.rs:3494`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ae6598a38d4dcd66bd4226c9"></a>
## serialize

`function` · `sqlparser::ast::ddl::CreateDomain::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::CreateDomain", "path": "CreateDomain"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3495, 38], "end": [3495, 47], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/ddl.rs:3495`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-34695f206a4581c027210523"></a>
## visit

`function` · `sqlparser::ast::ddl::CreateDomain::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::CreateDomain", "path": "CreateDomain"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3496, 40], "end": [3496, 45], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/ddl.rs:3496`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b92855129485cb23bab1dfe5"></a>
## visit

`function` · `sqlparser::ast::ddl::CreateDomain::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::CreateDomain", "path": "CreateDomain"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3496, 47], "end": [3496, 55], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/ddl.rs:3496`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
