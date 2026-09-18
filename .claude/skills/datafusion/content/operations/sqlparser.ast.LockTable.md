# `sqlparser::ast::LockTable`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.LockTable.json).

<a id="op-5f779ccdd1106f5095b4f5ed"></a>
## LockTable

`struct` · `sqlparser::ast::LockTable` · sqlparser 0.62.0

```rust
struct LockTable
```

Source: `src/ast/mod.rs:10360`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Represents a `LOCK TABLE` clause with optional alias and lock type.

<a id="op-76e4853ab2c2470fdcb2c5c2"></a>
## alias

`struct_field` · `sqlparser::ast::LockTable::alias` · sqlparser 0.62.0

```rust
alias: Option<Ident>
```

Source: `src/ast/mod.rs:10364`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional alias for the table.

<a id="op-9266246085675fea1f391794"></a>
## clone

`function` · `sqlparser::ast::LockTable::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> LockTable
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::LockTable", "path": "LockTable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10357, 17], "end": [10357, 22], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/mod.rs:10357`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6391e10ac75c3db2472ae063"></a>
## cmp

`function` · `sqlparser::ast::LockTable::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &LockTable) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::LockTable", "path": "LockTable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10357, 51], "end": [10357, 54], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/mod.rs:10357`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0dc30ef49ea9a663065f530b"></a>
## deserialize

`function` · `sqlparser::ast::LockTable::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::LockTable", "path": "LockTable"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [10358, 49], "end": [10358, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/mod.rs:10358`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-793015968d12fb4304843576"></a>
## eq

`function` · `sqlparser::ast::LockTable::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &LockTable) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::LockTable", "path": "LockTable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10357, 24], "end": [10357, 33], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/mod.rs:10357`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-11256905d3176d7d6c0f0be9"></a>
## fmt

`function` · `sqlparser::ast::LockTable::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::LockTable", "path": "LockTable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10369, 1], "end": [10384, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/mod.rs:10370`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-36f0b2fec7533c29824f3b78"></a>
## fmt

`function` · `sqlparser::ast::LockTable::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::LockTable", "path": "LockTable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10357, 10], "end": [10357, 15], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/mod.rs:10357`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d32de6d7f720abdbfe4b8b1a"></a>
## hash

`function` · `sqlparser::ast::LockTable::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::LockTable", "path": "LockTable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10357, 56], "end": [10357, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/mod.rs:10357`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9d8384c8b2e3382b43475724"></a>
## lock_type

`struct_field` · `sqlparser::ast::LockTable::lock_type` · sqlparser 0.62.0

```rust
lock_type: LockTableType
```

Source: `src/ast/mod.rs:10366`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The type of lock to apply to the table.

<a id="op-9e3e5c08e8efc223e35954b1"></a>
## partial_cmp

`function` · `sqlparser::ast::LockTable::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &LockTable) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::LockTable", "path": "LockTable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10357, 35], "end": [10357, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/mod.rs:10357`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-06878b5d8cd194002c23487f"></a>
## serialize

`function` · `sqlparser::ast::LockTable::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::LockTable", "path": "LockTable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10358, 38], "end": [10358, 47], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/mod.rs:10358`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7132a5d1fc0dec5b8363b81f"></a>
## table

`struct_field` · `sqlparser::ast::LockTable::table` · sqlparser 0.62.0

```rust
table: Ident
```

Source: `src/ast/mod.rs:10362`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The table identifier to lock.

<a id="op-3f5147a5d0e5e259c5d096fc"></a>
## visit

`function` · `sqlparser::ast::LockTable::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::LockTable", "path": "LockTable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10359, 47], "end": [10359, 55], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/mod.rs:10359`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f17629ab3222e31e1659405f"></a>
## visit

`function` · `sqlparser::ast::LockTable::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::LockTable", "path": "LockTable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10359, 40], "end": [10359, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/mod.rs:10359`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
