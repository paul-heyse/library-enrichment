# `sqlparser::ast::ddl::UserDefinedTypeRangeOption`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.ddl.UserDefinedTypeRangeOption.json).

<a id="op-6d74c68e0d3403e01c10d3ec"></a>
## UserDefinedTypeRangeOption

`enum` · `sqlparser::ast::ddl::UserDefinedTypeRangeOption` · sqlparser 0.62.0

```rust
enum UserDefinedTypeRangeOption
```

Source: `src/ast/ddl.rs:2586`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Options for PostgreSQL `CREATE TYPE ... AS RANGE` statement.

Range types are data types representing a range of values of some element type
(called the range's subtype). These options configure the behavior of the range type.

# PostgreSQL Documentation
See: <https://www.postgresql.org/docs/current/sql-createtype.html>

# Examples
```sql
CREATE TYPE int4range AS RANGE (
    SUBTYPE = int4,
    SUBTYPE_OPCLASS = int4_ops,
    CANONICAL = int4range_canonical,
    SUBTYPE_DIFF = int4range_subdiff
);
```

<a id="op-bce281a308370298ac9f220f"></a>
## Canonical

`variant` · `sqlparser::ast::ddl::UserDefinedTypeRangeOption::Canonical` · sqlparser 0.62.0

```rust
Canonical
```

Source: `src/ast/ddl.rs:2594`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Function to convert range values to canonical form: `CANONICAL = canonical_function`

<a id="op-c9b8cf19230920049f92ca0a"></a>
## Collation

`variant` · `sqlparser::ast::ddl::UserDefinedTypeRangeOption::Collation` · sqlparser 0.62.0

```rust
Collation
```

Source: `src/ast/ddl.rs:2592`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Collation to use for ordering the subtype: `COLLATION = collation`

<a id="op-f8d1bf660737a0169fc4d93d"></a>
## MultirangeTypeName

`variant` · `sqlparser::ast::ddl::UserDefinedTypeRangeOption::MultirangeTypeName` · sqlparser 0.62.0

```rust
MultirangeTypeName
```

Source: `src/ast/ddl.rs:2598`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Name of the corresponding multirange type: `MULTIRANGE_TYPE_NAME = multirange_type_name`

<a id="op-721be0d0a5b475f1e6e44721"></a>
## Subtype

`variant` · `sqlparser::ast::ddl::UserDefinedTypeRangeOption::Subtype` · sqlparser 0.62.0

```rust
Subtype
```

Source: `src/ast/ddl.rs:2588`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The element type that the range type will represent: `SUBTYPE = subtype`

<a id="op-bc5a8b337e987548a889ca91"></a>
## SubtypeDiff

`variant` · `sqlparser::ast::ddl::UserDefinedTypeRangeOption::SubtypeDiff` · sqlparser 0.62.0

```rust
SubtypeDiff
```

Source: `src/ast/ddl.rs:2596`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Function to compute the difference between two subtype values: `SUBTYPE_DIFF = subtype_diff_function`

<a id="op-ea6688df55c680b820f412b2"></a>
## SubtypeOpClass

`variant` · `sqlparser::ast::ddl::UserDefinedTypeRangeOption::SubtypeOpClass` · sqlparser 0.62.0

```rust
SubtypeOpClass
```

Source: `src/ast/ddl.rs:2590`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The operator class for the subtype: `SUBTYPE_OPCLASS = subtype_operator_class`

<a id="op-2b8589fb448808025a308ddc"></a>
## clone

`function` · `sqlparser::ast::ddl::UserDefinedTypeRangeOption::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> UserDefinedTypeRangeOption
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::UserDefinedTypeRangeOption", "path": "UserDefinedTypeRangeOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2583, 17], "end": [2583, 22], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/ddl.rs:2583`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-84e160576891ea8e9a339178"></a>
## cmp

`function` · `sqlparser::ast::ddl::UserDefinedTypeRangeOption::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &UserDefinedTypeRangeOption) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::UserDefinedTypeRangeOption", "path": "UserDefinedTypeRangeOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2583, 51], "end": [2583, 54], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/ddl.rs:2583`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6659e839effc2a0baa3eab47"></a>
## deserialize

`function` · `sqlparser::ast::ddl::UserDefinedTypeRangeOption::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::UserDefinedTypeRangeOption", "path": "UserDefinedTypeRangeOption"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [2584, 49], "end": [2584, 60], "filename": "src/ast/ddl.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/ddl.rs:2584`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-56c527d4dc7402a4bcc406a0"></a>
## eq

`function` · `sqlparser::ast::ddl::UserDefinedTypeRangeOption::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &UserDefinedTypeRangeOption) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::UserDefinedTypeRangeOption", "path": "UserDefinedTypeRangeOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2583, 24], "end": [2583, 33], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/ddl.rs:2583`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bf18673ea4eae5fceef1ccb8"></a>
## fmt

`function` · `sqlparser::ast::ddl::UserDefinedTypeRangeOption::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::UserDefinedTypeRangeOption", "path": "UserDefinedTypeRangeOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2583, 10], "end": [2583, 15], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/ddl.rs:2583`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c9439018e2c42b6549655c81"></a>
## fmt

`function` · `sqlparser::ast::ddl::UserDefinedTypeRangeOption::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::UserDefinedTypeRangeOption", "path": "UserDefinedTypeRangeOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2601, 1], "end": [2616, 2], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/ddl.rs:2602`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e7ef9e58dbb7519db87fafb0"></a>
## hash

`function` · `sqlparser::ast::ddl::UserDefinedTypeRangeOption::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::UserDefinedTypeRangeOption", "path": "UserDefinedTypeRangeOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2583, 56], "end": [2583, 60], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/ddl.rs:2583`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-69d971b9bde603dbd430cec0"></a>
## partial_cmp

`function` · `sqlparser::ast::ddl::UserDefinedTypeRangeOption::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &UserDefinedTypeRangeOption) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::UserDefinedTypeRangeOption", "path": "UserDefinedTypeRangeOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2583, 35], "end": [2583, 45], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/ddl.rs:2583`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d06789313cfd78d4c4834582"></a>
## serialize

`function` · `sqlparser::ast::ddl::UserDefinedTypeRangeOption::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::UserDefinedTypeRangeOption", "path": "UserDefinedTypeRangeOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2584, 38], "end": [2584, 47], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/ddl.rs:2584`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-43d1e01c5f30a3d7f0b8689e"></a>
## visit

`function` · `sqlparser::ast::ddl::UserDefinedTypeRangeOption::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::UserDefinedTypeRangeOption", "path": "UserDefinedTypeRangeOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2585, 40], "end": [2585, 45], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/ddl.rs:2585`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-83092cb1f03118b2875cc9e5"></a>
## visit

`function` · `sqlparser::ast::ddl::UserDefinedTypeRangeOption::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::UserDefinedTypeRangeOption", "path": "UserDefinedTypeRangeOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2585, 47], "end": [2585, 55], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/ddl.rs:2585`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
