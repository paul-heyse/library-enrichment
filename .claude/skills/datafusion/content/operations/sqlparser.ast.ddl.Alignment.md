# `sqlparser::ast::ddl::Alignment`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.ddl.Alignment.json).

<a id="op-e7c6813bc14d6000ce8295e0"></a>
## Alignment

`enum` · `sqlparser::ast::ddl::Alignment` · sqlparser 0.62.0

```rust
enum Alignment
```

Source: `src/ast/ddl.rs:2500`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Alignment specification for PostgreSQL user-defined base types.

Specifies the storage alignment requirement for values of the data type.
The allowed values equate to alignment on 1, 2, 4, or 8 byte boundaries.
Note that variable-length types must have an alignment of at least 4, since
they necessarily contain an int4 as their first component.

# PostgreSQL Documentation
See: <https://www.postgresql.org/docs/current/sql-createtype.html>

# Examples
```sql
CREATE TYPE mytype (
    INPUT = in_func,
    OUTPUT = out_func,
    ALIGNMENT = int4  -- 4-byte alignment
);
```

<a id="op-78850badb46330d4c3666376"></a>
## Char

`variant` · `sqlparser::ast::ddl::Alignment::Char` · sqlparser 0.62.0

```rust
Char
```

Source: `src/ast/ddl.rs:2502`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Single-byte alignment: `ALIGNMENT = char`

<a id="op-864e7026796a956ee45e8bce"></a>
## Double

`variant` · `sqlparser::ast::ddl::Alignment::Double` · sqlparser 0.62.0

```rust
Double
```

Source: `src/ast/ddl.rs:2508`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

8-byte alignment: `ALIGNMENT = double`

<a id="op-c5e96c8b5d03fdb0144c95e4"></a>
## Int2

`variant` · `sqlparser::ast::ddl::Alignment::Int2` · sqlparser 0.62.0

```rust
Int2
```

Source: `src/ast/ddl.rs:2504`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

2-byte alignment: `ALIGNMENT = int2`

<a id="op-3fcbc2ed92b255fa4deef585"></a>
## Int4

`variant` · `sqlparser::ast::ddl::Alignment::Int4` · sqlparser 0.62.0

```rust
Int4
```

Source: `src/ast/ddl.rs:2506`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

4-byte alignment: `ALIGNMENT = int4`

<a id="op-ea07948af5c9409c55c4f89b"></a>
## clone

`function` · `sqlparser::ast::ddl::Alignment::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> Alignment
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::Alignment", "path": "Alignment"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2497, 23], "end": [2497, 28], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/ddl.rs:2497`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b3c2089d6113c39fd3bf9d5a"></a>
## cmp

`function` · `sqlparser::ast::ddl::Alignment::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &Alignment) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::Alignment", "path": "Alignment"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2497, 57], "end": [2497, 60], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/ddl.rs:2497`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0b51d843dbbb2cf39ace14bc"></a>
## deserialize

`function` · `sqlparser::ast::ddl::Alignment::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::Alignment", "path": "Alignment"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [2498, 49], "end": [2498, 60], "filename": "src/ast/ddl.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/ddl.rs:2498`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-080299572018efda170a0335"></a>
## eq

`function` · `sqlparser::ast::ddl::Alignment::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &Alignment) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::Alignment", "path": "Alignment"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2497, 30], "end": [2497, 39], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/ddl.rs:2497`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ccb6bb5fd8976c67f2aafae3"></a>
## fmt

`function` · `sqlparser::ast::ddl::Alignment::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::Alignment", "path": "Alignment"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2497, 10], "end": [2497, 15], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/ddl.rs:2497`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ee4cb2f6fd0daea29cb46f90"></a>
## fmt

`function` · `sqlparser::ast::ddl::Alignment::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::Alignment", "path": "Alignment"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2511, 1], "end": [2520, 2], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/ddl.rs:2512`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-399f8ce802ba5cf82c1de7ad"></a>
## hash

`function` · `sqlparser::ast::ddl::Alignment::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::Alignment", "path": "Alignment"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2497, 62], "end": [2497, 66], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/ddl.rs:2497`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-05ff55614e2bf98b0c0124b4"></a>
## partial_cmp

`function` · `sqlparser::ast::ddl::Alignment::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &Alignment) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::Alignment", "path": "Alignment"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2497, 41], "end": [2497, 51], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/ddl.rs:2497`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-13b880fbe7b665238351ec5b"></a>
## serialize

`function` · `sqlparser::ast::ddl::Alignment::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::Alignment", "path": "Alignment"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2498, 38], "end": [2498, 47], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/ddl.rs:2498`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-50cfb41ae1427c5b926e5390"></a>
## visit

`function` · `sqlparser::ast::ddl::Alignment::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::Alignment", "path": "Alignment"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2499, 47], "end": [2499, 55], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/ddl.rs:2499`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a803a29ec5d077c619adad4d"></a>
## visit

`function` · `sqlparser::ast::ddl::Alignment::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::Alignment", "path": "Alignment"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2499, 40], "end": [2499, 45], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/ddl.rs:2499`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
