# `sqlparser::ast::MemberOf`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.MemberOf.json).

<a id="op-fd87b5d62967fb89d7e0ca36"></a>
## MemberOf

`struct` · `sqlparser::ast::MemberOf` · sqlparser 0.62.0

```rust
struct MemberOf
```

Source: `src/ast/mod.rs:11416`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Checks membership of a value in a JSON array

Syntax:
```sql
<value> MEMBER OF(<array>)
```
[MySQL](https://dev.mysql.com/doc/refman/8.4/en/json-search-functions.html#operator_member-of)

<a id="op-dcff87e8dc55215f5967f99c"></a>
## array

`struct_field` · `sqlparser::ast::MemberOf::array` · sqlparser 0.62.0

```rust
array: Box<Expr>
```

Source: `src/ast/mod.rs:11420`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The JSON array expression to check against.

<a id="op-15a37c334e35ba915c84f260"></a>
## clone

`function` · `sqlparser::ast::MemberOf::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> MemberOf
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::MemberOf", "path": "MemberOf"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11413, 17], "end": [11413, 22], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/mod.rs:11413`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-21170a613236bb788a33e591"></a>
## cmp

`function` · `sqlparser::ast::MemberOf::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &MemberOf) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::MemberOf", "path": "MemberOf"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11413, 51], "end": [11413, 54], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/mod.rs:11413`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-115b5d36edf28cc2e8a8c717"></a>
## deserialize

`function` · `sqlparser::ast::MemberOf::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::MemberOf", "path": "MemberOf"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [11414, 49], "end": [11414, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/mod.rs:11414`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-45608d14a4c8f001d5fdb63d"></a>
## eq

`function` · `sqlparser::ast::MemberOf::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &MemberOf) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::MemberOf", "path": "MemberOf"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11413, 24], "end": [11413, 33], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/mod.rs:11413`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-36857733e5b98e256006a4a9"></a>
## fmt

`function` · `sqlparser::ast::MemberOf::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::MemberOf", "path": "MemberOf"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11423, 1], "end": [11427, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/mod.rs:11424`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4bf7b22d45f34de0ec40271b"></a>
## fmt

`function` · `sqlparser::ast::MemberOf::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::MemberOf", "path": "MemberOf"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11413, 10], "end": [11413, 15], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/mod.rs:11413`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-33c0c523f6c5fcce8896356d"></a>
## hash

`function` · `sqlparser::ast::MemberOf::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::MemberOf", "path": "MemberOf"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11413, 56], "end": [11413, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/mod.rs:11413`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-66feb6eace737e1f5de36c5c"></a>
## partial_cmp

`function` · `sqlparser::ast::MemberOf::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &MemberOf) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::MemberOf", "path": "MemberOf"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11413, 35], "end": [11413, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/mod.rs:11413`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-edc27bbd72c00b4d2bcc9b5b"></a>
## serialize

`function` · `sqlparser::ast::MemberOf::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::MemberOf", "path": "MemberOf"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11414, 38], "end": [11414, 47], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/mod.rs:11414`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-836d18e33d17e61c9f1b8e7f"></a>
## value

`struct_field` · `sqlparser::ast::MemberOf::value` · sqlparser 0.62.0

```rust
value: Box<Expr>
```

Source: `src/ast/mod.rs:11418`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The value to check for membership.

<a id="op-4ed9b0932e9f336e0b258343"></a>
## visit

`function` · `sqlparser::ast::MemberOf::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::MemberOf", "path": "MemberOf"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11415, 40], "end": [11415, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/mod.rs:11415`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d4a9694bb5f8150559b42e7f"></a>
## visit

`function` · `sqlparser::ast::MemberOf::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::MemberOf", "path": "MemberOf"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11415, 47], "end": [11415, 55], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/mod.rs:11415`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
