# `sqlparser::ast::ddl::UserDefinedTypeRepresentation`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.ddl.UserDefinedTypeRepresentation.json).

<a id="op-6de72aa14ae51c8c48022b4b"></a>
## UserDefinedTypeRepresentation

`enum` · `sqlparser::ast::ddl::UserDefinedTypeRepresentation` · sqlparser 0.62.0

```rust
enum UserDefinedTypeRepresentation
```

Source: `src/ast/ddl.rs:2364`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

SQL user defined type definition

<a id="op-483bb5355ce7dab47d36f688"></a>
## Composite

`variant` · `sqlparser::ast::ddl::UserDefinedTypeRepresentation::Composite` · sqlparser 0.62.0

```rust
Composite
```

Source: `src/ast/ddl.rs:2366`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Composite type: `CREATE TYPE name AS (attributes)`

<a id="op-53d55765b3b2a6c7176f0530"></a>
## Enum

`variant` · `sqlparser::ast::ddl::UserDefinedTypeRepresentation::Enum` · sqlparser 0.62.0

```rust
Enum
```

Source: `src/ast/ddl.rs:2374`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Enum type: `CREATE TYPE name AS ENUM (labels)`

Note: this is PostgreSQL-specific. See <https://www.postgresql.org/docs/current/sql-createtype.html>
Enum type: `CREATE TYPE name AS ENUM (labels)`

<a id="op-1585f25d363f0ffb2d11307b"></a>
## Range

`variant` · `sqlparser::ast::ddl::UserDefinedTypeRepresentation::Range` · sqlparser 0.62.0

```rust
Range
```

Source: `src/ast/ddl.rs:2381`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Range type: `CREATE TYPE name AS RANGE (options)`

Note: this is PostgreSQL-specific. See <https://www.postgresql.org/docs/current/sql-createtype.html>

<a id="op-f15b9efd4576ffce67602296"></a>
## SqlDefinition

`variant` · `sqlparser::ast::ddl::UserDefinedTypeRepresentation::SqlDefinition` · sqlparser 0.62.0

```rust
SqlDefinition
```

Source: `src/ast/ddl.rs:2390`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Base type (SQL definition): `CREATE TYPE name (options)`

Note the lack of `AS` keyword

Note: this is PostgreSQL-specific. See <https://www.postgresql.org/docs/current/sql-createtype.html>

<a id="op-ffbbd06dc4cc1cf0a7e83a72"></a>
## clone

`function` · `sqlparser::ast::ddl::UserDefinedTypeRepresentation::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> UserDefinedTypeRepresentation
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::UserDefinedTypeRepresentation", "path": "UserDefinedTypeRepresentation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2361, 17], "end": [2361, 22], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/ddl.rs:2361`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-76a62f53c6d4ee90b7fbbe81"></a>
## cmp

`function` · `sqlparser::ast::ddl::UserDefinedTypeRepresentation::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &UserDefinedTypeRepresentation) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::UserDefinedTypeRepresentation", "path": "UserDefinedTypeRepresentation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2361, 51], "end": [2361, 54], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/ddl.rs:2361`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6b504c13bd6b1ef2356db430"></a>
## deserialize

`function` · `sqlparser::ast::ddl::UserDefinedTypeRepresentation::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::UserDefinedTypeRepresentation", "path": "UserDefinedTypeRepresentation"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [2362, 49], "end": [2362, 60], "filename": "src/ast/ddl.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/ddl.rs:2362`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2bec726fae62fc4b081f6b33"></a>
## eq

`function` · `sqlparser::ast::ddl::UserDefinedTypeRepresentation::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &UserDefinedTypeRepresentation) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::UserDefinedTypeRepresentation", "path": "UserDefinedTypeRepresentation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2361, 24], "end": [2361, 33], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/ddl.rs:2361`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-52009a11506040463eb2839e"></a>
## fmt

`function` · `sqlparser::ast::ddl::UserDefinedTypeRepresentation::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::UserDefinedTypeRepresentation", "path": "UserDefinedTypeRepresentation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2396, 1], "end": [2413, 2], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/ddl.rs:2397`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7fe725047ee9aef78eb922ec"></a>
## fmt

`function` · `sqlparser::ast::ddl::UserDefinedTypeRepresentation::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::UserDefinedTypeRepresentation", "path": "UserDefinedTypeRepresentation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2361, 10], "end": [2361, 15], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/ddl.rs:2361`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-55f5f2ec0956a271a57039dd"></a>
## hash

`function` · `sqlparser::ast::ddl::UserDefinedTypeRepresentation::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::UserDefinedTypeRepresentation", "path": "UserDefinedTypeRepresentation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2361, 56], "end": [2361, 60], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/ddl.rs:2361`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-610e1323075d3ca63ee6d570"></a>
## partial_cmp

`function` · `sqlparser::ast::ddl::UserDefinedTypeRepresentation::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &UserDefinedTypeRepresentation) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::UserDefinedTypeRepresentation", "path": "UserDefinedTypeRepresentation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2361, 35], "end": [2361, 45], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/ddl.rs:2361`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-165dce2a73740ec7ddeea646"></a>
## serialize

`function` · `sqlparser::ast::ddl::UserDefinedTypeRepresentation::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::UserDefinedTypeRepresentation", "path": "UserDefinedTypeRepresentation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2362, 38], "end": [2362, 47], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/ddl.rs:2362`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3afa5f26defc4cd9c75e9edd"></a>
## visit

`function` · `sqlparser::ast::ddl::UserDefinedTypeRepresentation::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::UserDefinedTypeRepresentation", "path": "UserDefinedTypeRepresentation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2363, 47], "end": [2363, 55], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/ddl.rs:2363`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-eb4c1c2386d7d6e045dc9741"></a>
## visit

`function` · `sqlparser::ast::ddl::UserDefinedTypeRepresentation::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::UserDefinedTypeRepresentation", "path": "UserDefinedTypeRepresentation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2363, 40], "end": [2363, 45], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/ddl.rs:2363`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
