# `sqlparser::ast::ThrowStatement`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.ThrowStatement.json).

<a id="op-16716312ea068876d87933d7"></a>
## ThrowStatement

`struct` · `sqlparser::ast::ThrowStatement` · sqlparser 0.62.0

```rust
struct ThrowStatement
```

Source: `src/ast/mod.rs:2931`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

A MSSQL `THROW` statement.

```sql
THROW [ error_number, message, state ]
```

[MSSQL](https://learn.microsoft.com/en-us/sql/t-sql/language-elements/throw-transact-sql)

<a id="op-c1513180707a623ea8d6a471"></a>
## clone

`function` · `sqlparser::ast::ThrowStatement::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> ThrowStatement
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ThrowStatement", "path": "ThrowStatement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2928, 17], "end": [2928, 22], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/mod.rs:2928`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-837661f3b8442b0f051282fb"></a>
## cmp

`function` · `sqlparser::ast::ThrowStatement::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &ThrowStatement) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ThrowStatement", "path": "ThrowStatement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2928, 51], "end": [2928, 54], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/mod.rs:2928`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4d0b4b64af82a384f69e9656"></a>
## deserialize

`function` · `sqlparser::ast::ThrowStatement::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ThrowStatement", "path": "ThrowStatement"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [2929, 49], "end": [2929, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/mod.rs:2929`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9705fae7e9c757ba40eca8bc"></a>
## eq

`function` · `sqlparser::ast::ThrowStatement::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &ThrowStatement) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ThrowStatement", "path": "ThrowStatement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2928, 24], "end": [2928, 33], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/mod.rs:2928`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b5dbb1bd4147013cb9e9ff52"></a>
## error_number

`struct_field` · `sqlparser::ast::ThrowStatement::error_number` · sqlparser 0.62.0

```rust
error_number: Option<Box<Expr>>
```

Source: `src/ast/mod.rs:2933`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Error number expression.

<a id="op-a952a9ba4c622c15b1a5be35"></a>
## fmt

`function` · `sqlparser::ast::ThrowStatement::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ThrowStatement", "path": "ThrowStatement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2940, 1], "end": [2954, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/mod.rs:2941`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c78002f9cfb6667e9504af92"></a>
## fmt

`function` · `sqlparser::ast::ThrowStatement::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ThrowStatement", "path": "ThrowStatement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2928, 10], "end": [2928, 15], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/mod.rs:2928`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f92baeb9a88204858120fc7a"></a>
## hash

`function` · `sqlparser::ast::ThrowStatement::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ThrowStatement", "path": "ThrowStatement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2928, 56], "end": [2928, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/mod.rs:2928`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-86bdcff020cf1a054eb00920"></a>
## message

`struct_field` · `sqlparser::ast::ThrowStatement::message` · sqlparser 0.62.0

```rust
message: Option<Box<Expr>>
```

Source: `src/ast/mod.rs:2935`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Error message expression.

<a id="op-e1e98ff2823cc306a944687d"></a>
## partial_cmp

`function` · `sqlparser::ast::ThrowStatement::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &ThrowStatement) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ThrowStatement", "path": "ThrowStatement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2928, 35], "end": [2928, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/mod.rs:2928`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f2c6745e7ee3b87213eb4e44"></a>
## serialize

`function` · `sqlparser::ast::ThrowStatement::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ThrowStatement", "path": "ThrowStatement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2929, 38], "end": [2929, 47], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/mod.rs:2929`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9abf01e52f85a18b5b3148c7"></a>
## state

`struct_field` · `sqlparser::ast::ThrowStatement::state` · sqlparser 0.62.0

```rust
state: Option<Box<Expr>>
```

Source: `src/ast/mod.rs:2937`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

State expression.

<a id="op-50c3a23e1b5fdd33ba64155c"></a>
## visit

`function` · `sqlparser::ast::ThrowStatement::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ThrowStatement", "path": "ThrowStatement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2930, 47], "end": [2930, 55], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/mod.rs:2930`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fefdd2750fd8b51e80688258"></a>
## visit

`function` · `sqlparser::ast::ThrowStatement::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ThrowStatement", "path": "ThrowStatement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2930, 40], "end": [2930, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/mod.rs:2930`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
