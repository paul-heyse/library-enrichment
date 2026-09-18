# `sqlparser::ast::operator::UnaryOperator`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.operator.UnaryOperator.json).

<a id="op-4f119d8eeb815f43c257da04"></a>
## UnaryOperator

`enum` · `sqlparser::ast::operator::UnaryOperator` · sqlparser 0.62.0

```rust
enum UnaryOperator
```

Source: `src/ast/operator.rs:35`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Unary operators

<a id="op-936e2e4fc509003606d0e3f1"></a>
## AtDashAt

`variant` · `sqlparser::ast::operator::UnaryOperator::AtDashAt` · sqlparser 0.62.0

```rust
AtDashAt
```

Source: `src/ast/operator.rs:38`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`@-@` Length or circumference (PostgreSQL/Redshift geometric operator)
see <https://www.postgresql.org/docs/9.5/functions-geometry.html>

<a id="op-89d33ed92ee9d3c0b70e143c"></a>
## BangNot

`variant` · `sqlparser::ast::operator::UnaryOperator::BangNot` · sqlparser 0.62.0

```rust
BangNot
```

Source: `src/ast/operator.rs:40`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Unary logical not operator: e.g. `! false` (Hive-specific)

<a id="op-ce65ca9927c47b3a3dcbbb51"></a>
## BitwiseNot

`variant` · `sqlparser::ast::operator::UnaryOperator::BitwiseNot` · sqlparser 0.62.0

```rust
BitwiseNot
```

Source: `src/ast/operator.rs:42`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Bitwise Not, e.g. `~9`

<a id="op-f57978b46235b859c3dbfb57"></a>
## DoubleAt

`variant` · `sqlparser::ast::operator::UnaryOperator::DoubleAt` · sqlparser 0.62.0

```rust
DoubleAt
```

Source: `src/ast/operator.rs:45`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`@@` Center (PostgreSQL/Redshift geometric operator)
see <https://www.postgresql.org/docs/9.5/functions-geometry.html>

<a id="op-1a86b308d52800b9e4a9d78e"></a>
## Hash

`variant` · `sqlparser::ast::operator::UnaryOperator::Hash` · sqlparser 0.62.0

```rust
Hash
```

Source: `src/ast/operator.rs:48`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`#` Number of points in path or polygon (PostgreSQL/Redshift geometric operator)
see <https://www.postgresql.org/docs/9.5/functions-geometry.html>

<a id="op-5dd44c171b8852f1e448c1ba"></a>
## Minus

`variant` · `sqlparser::ast::operator::UnaryOperator::Minus` · sqlparser 0.62.0

```rust
Minus
```

Source: `src/ast/operator.rs:52`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Minus, e.g. `-9`

<a id="op-a105e8c0aa0502ea278e23ba"></a>
## Not

`variant` · `sqlparser::ast::operator::UnaryOperator::Not` · sqlparser 0.62.0

```rust
Not
```

Source: `src/ast/operator.rs:54`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Not, e.g. `NOT(true)`

<a id="op-e599545c2a88896421b6b784"></a>
## PGAbs

`variant` · `sqlparser::ast::operator::UnaryOperator::PGAbs` · sqlparser 0.62.0

```rust
PGAbs
```

Source: `src/ast/operator.rs:56`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Absolute value, e.g. `@ -9` (PostgreSQL-specific)

<a id="op-6e12170384ee3fd8c7f38271"></a>
## PGCubeRoot

`variant` · `sqlparser::ast::operator::UnaryOperator::PGCubeRoot` · sqlparser 0.62.0

```rust
PGCubeRoot
```

Source: `src/ast/operator.rs:58`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Cube root, e.g. `||/27` (PostgreSQL-specific)

<a id="op-ec8b03eff226ed1de2b90e9f"></a>
## PGPostfixFactorial

`variant` · `sqlparser::ast::operator::UnaryOperator::PGPostfixFactorial` · sqlparser 0.62.0

```rust
PGPostfixFactorial
```

Source: `src/ast/operator.rs:60`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Factorial, e.g. `9!` (PostgreSQL-specific)

<a id="op-e96ffdf262e171b8376eea1c"></a>
## PGPrefixFactorial

`variant` · `sqlparser::ast::operator::UnaryOperator::PGPrefixFactorial` · sqlparser 0.62.0

```rust
PGPrefixFactorial
```

Source: `src/ast/operator.rs:62`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Factorial, e.g. `!!9` (PostgreSQL-specific)

<a id="op-47470ae53e34ce5b30184ec3"></a>
## PGSquareRoot

`variant` · `sqlparser::ast::operator::UnaryOperator::PGSquareRoot` · sqlparser 0.62.0

```rust
PGSquareRoot
```

Source: `src/ast/operator.rs:64`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Square root, e.g. `|/9` (PostgreSQL-specific)

<a id="op-668b32c9d0366343ce2e9711"></a>
## Plus

`variant` · `sqlparser::ast::operator::UnaryOperator::Plus` · sqlparser 0.62.0

```rust
Plus
```

Source: `src/ast/operator.rs:50`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Plus, e.g. `+9`

<a id="op-f1d37bb46282dc1828f57d76"></a>
## QuestionDash

`variant` · `sqlparser::ast::operator::UnaryOperator::QuestionDash` · sqlparser 0.62.0

```rust
QuestionDash
```

Source: `src/ast/operator.rs:67`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`?-` Is horizontal? (PostgreSQL/Redshift geometric operator)
see <https://www.postgresql.org/docs/9.5/functions-geometry.html>

<a id="op-82be8c5fc6b0b5e1075ef505"></a>
## QuestionPipe

`variant` · `sqlparser::ast::operator::UnaryOperator::QuestionPipe` · sqlparser 0.62.0

```rust
QuestionPipe
```

Source: `src/ast/operator.rs:70`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`?|` Is vertical? (PostgreSQL/Redshift geometric operator)
see <https://www.postgresql.org/docs/9.5/functions-geometry.html>

<a id="op-8b2c9571829d7e154ad17b7a"></a>
## clone

`function` · `sqlparser::ast::operator::UnaryOperator::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> UnaryOperator
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::operator::UnaryOperator", "path": "UnaryOperator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [32, 23], "end": [32, 28], "filename": "src/ast/operator.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/operator.rs:32`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-84004f36fb5fab15b56a7d93"></a>
## cmp

`function` · `sqlparser::ast::operator::UnaryOperator::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &UnaryOperator) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::operator::UnaryOperator", "path": "UnaryOperator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [32, 57], "end": [32, 60], "filename": "src/ast/operator.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/operator.rs:32`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6d8951187b917d568d9d098c"></a>
## deserialize

`function` · `sqlparser::ast::operator::UnaryOperator::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::operator::UnaryOperator", "path": "UnaryOperator"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [33, 49], "end": [33, 60], "filename": "src/ast/operator.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/operator.rs:33`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7d3c72653aa7cf375e3cdf24"></a>
## eq

`function` · `sqlparser::ast::operator::UnaryOperator::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &UnaryOperator) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::operator::UnaryOperator", "path": "UnaryOperator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [32, 30], "end": [32, 39], "filename": "src/ast/operator.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/operator.rs:32`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-20f1bd077e0fa10aa0e52496"></a>
## fmt

`function` · `sqlparser::ast::operator::UnaryOperator::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::operator::UnaryOperator", "path": "UnaryOperator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [32, 10], "end": [32, 15], "filename": "src/ast/operator.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/operator.rs:32`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3f33ccaf650c13860781bcd8"></a>
## fmt

`function` · `sqlparser::ast::operator::UnaryOperator::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::operator::UnaryOperator", "path": "UnaryOperator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [73, 1], "end": [93, 2], "filename": "src/ast/operator.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/operator.rs:74`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4c47aa70610d2e1943b72d12"></a>
## hash

`function` · `sqlparser::ast::operator::UnaryOperator::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::operator::UnaryOperator", "path": "UnaryOperator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [32, 62], "end": [32, 66], "filename": "src/ast/operator.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/operator.rs:32`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6d14dd7f7ead4d315e307c2a"></a>
## partial_cmp

`function` · `sqlparser::ast::operator::UnaryOperator::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &UnaryOperator) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::operator::UnaryOperator", "path": "UnaryOperator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [32, 41], "end": [32, 51], "filename": "src/ast/operator.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/operator.rs:32`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1d9c77e32191c1ed16d476b1"></a>
## serialize

`function` · `sqlparser::ast::operator::UnaryOperator::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::operator::UnaryOperator", "path": "UnaryOperator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [33, 38], "end": [33, 47], "filename": "src/ast/operator.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/operator.rs:33`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8120215c69e60f2eea177e39"></a>
## visit

`function` · `sqlparser::ast::operator::UnaryOperator::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::operator::UnaryOperator", "path": "UnaryOperator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [34, 40], "end": [34, 45], "filename": "src/ast/operator.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/operator.rs:34`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d7de849be243d853fa14edf2"></a>
## visit

`function` · `sqlparser::ast::operator::UnaryOperator::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::operator::UnaryOperator", "path": "UnaryOperator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [34, 47], "end": [34, 55], "filename": "src/ast/operator.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/operator.rs:34`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
