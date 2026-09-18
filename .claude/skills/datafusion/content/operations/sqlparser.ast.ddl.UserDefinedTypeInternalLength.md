# `sqlparser::ast::ddl::UserDefinedTypeInternalLength`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.ddl.UserDefinedTypeInternalLength.json).

<a id="op-e66f658c3aecfe4ef5ad80ae"></a>
## UserDefinedTypeInternalLength

`enum` · `sqlparser::ast::ddl::UserDefinedTypeInternalLength` · sqlparser 0.62.0

```rust
enum UserDefinedTypeInternalLength
```

Source: `src/ast/ddl.rs:2463`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Internal length specification for PostgreSQL user-defined base types.

Specifies the internal length in bytes of the new type's internal representation.
The default assumption is that it is variable-length.

# PostgreSQL Documentation
See: <https://www.postgresql.org/docs/current/sql-createtype.html>

# Examples
```sql
CREATE TYPE mytype (
    INPUT = in_func,
    OUTPUT = out_func,
    INTERNALLENGTH = 16  -- Fixed 16-byte length
);

CREATE TYPE mytype2 (
    INPUT = in_func,
    OUTPUT = out_func,
    INTERNALLENGTH = VARIABLE  -- Variable length
);
```

<a id="op-9a0d31cd59c340f835cb82e8"></a>
## Fixed

`variant` · `sqlparser::ast::ddl::UserDefinedTypeInternalLength::Fixed` · sqlparser 0.62.0

```rust
Fixed
```

Source: `src/ast/ddl.rs:2465`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Fixed internal length: `INTERNALLENGTH = <number>`

<a id="op-7463eb98c72ae66c4180d7a5"></a>
## Variable

`variant` · `sqlparser::ast::ddl::UserDefinedTypeInternalLength::Variable` · sqlparser 0.62.0

```rust
Variable
```

Source: `src/ast/ddl.rs:2467`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Variable internal length: `INTERNALLENGTH = VARIABLE`

<a id="op-2666777825ff76afa0b4e962"></a>
## clone

`function` · `sqlparser::ast::ddl::UserDefinedTypeInternalLength::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> UserDefinedTypeInternalLength
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::UserDefinedTypeInternalLength", "path": "UserDefinedTypeInternalLength"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2460, 23], "end": [2460, 28], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/ddl.rs:2460`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-78349beaa605bb5bcbc44ded"></a>
## cmp

`function` · `sqlparser::ast::ddl::UserDefinedTypeInternalLength::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &UserDefinedTypeInternalLength) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::UserDefinedTypeInternalLength", "path": "UserDefinedTypeInternalLength"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2460, 57], "end": [2460, 60], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/ddl.rs:2460`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f12076811994af98aa0a1ce9"></a>
## deserialize

`function` · `sqlparser::ast::ddl::UserDefinedTypeInternalLength::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::UserDefinedTypeInternalLength", "path": "UserDefinedTypeInternalLength"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [2461, 49], "end": [2461, 60], "filename": "src/ast/ddl.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/ddl.rs:2461`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-11f9c980620bdb8f5f33a698"></a>
## eq

`function` · `sqlparser::ast::ddl::UserDefinedTypeInternalLength::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &UserDefinedTypeInternalLength) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::UserDefinedTypeInternalLength", "path": "UserDefinedTypeInternalLength"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2460, 30], "end": [2460, 39], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/ddl.rs:2460`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-210e1c0b66f8dc8c45266ebd"></a>
## fmt

`function` · `sqlparser::ast::ddl::UserDefinedTypeInternalLength::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::UserDefinedTypeInternalLength", "path": "UserDefinedTypeInternalLength"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2460, 10], "end": [2460, 15], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/ddl.rs:2460`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c46094d9f7eb9aac00e94db9"></a>
## fmt

`function` · `sqlparser::ast::ddl::UserDefinedTypeInternalLength::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::UserDefinedTypeInternalLength", "path": "UserDefinedTypeInternalLength"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2470, 1], "end": [2477, 2], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/ddl.rs:2471`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-53a3c95f4e7b3160886e148e"></a>
## hash

`function` · `sqlparser::ast::ddl::UserDefinedTypeInternalLength::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::UserDefinedTypeInternalLength", "path": "UserDefinedTypeInternalLength"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2460, 62], "end": [2460, 66], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/ddl.rs:2460`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a79fa5acdd30dbfcb4414c7d"></a>
## partial_cmp

`function` · `sqlparser::ast::ddl::UserDefinedTypeInternalLength::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &UserDefinedTypeInternalLength) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::UserDefinedTypeInternalLength", "path": "UserDefinedTypeInternalLength"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2460, 41], "end": [2460, 51], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/ddl.rs:2460`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-999706411a29aca508496e93"></a>
## serialize

`function` · `sqlparser::ast::ddl::UserDefinedTypeInternalLength::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::UserDefinedTypeInternalLength", "path": "UserDefinedTypeInternalLength"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2461, 38], "end": [2461, 47], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/ddl.rs:2461`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-30c2bb79763af207f16ea2b3"></a>
## visit

`function` · `sqlparser::ast::ddl::UserDefinedTypeInternalLength::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::UserDefinedTypeInternalLength", "path": "UserDefinedTypeInternalLength"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2462, 40], "end": [2462, 45], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/ddl.rs:2462`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-dd73776904ba9e718a65e766"></a>
## visit

`function` · `sqlparser::ast::ddl::UserDefinedTypeInternalLength::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::UserDefinedTypeInternalLength", "path": "UserDefinedTypeInternalLength"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2462, 47], "end": [2462, 55], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/ddl.rs:2462`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
