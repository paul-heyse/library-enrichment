# `sqlparser::dialect::Precedence`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.dialect.Precedence.json).

<a id="op-d4389f30a52150b967962a36"></a>
## Precedence

`enum` · `sqlparser::dialect::Precedence` · sqlparser 0.62.0

```rust
enum Precedence
```

Source: `src/dialect/mod.rs:1794`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Operators for which precedence must be defined.

Higher number -> higher precedence.
See expression parsing for how these values are used.

<a id="op-ca8d4e0bc787014d09a16bcc"></a>
## Ampersand

`variant` · `sqlparser::dialect::Precedence::Ampersand` · sqlparser 0.62.0

```rust
Ampersand
```

Source: `src/dialect/mod.rs:1808`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Bitwise `AND` operator (`&`).

<a id="op-9dfa233e0cf30587282beb67"></a>
## And

`variant` · `sqlparser::dialect::Precedence::And` · sqlparser 0.62.0

```rust
And
```

Source: `src/dialect/mod.rs:1828`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Logical `AND`.

<a id="op-5e5a214a0981a4ea57310c2b"></a>
## AtTz

`variant` · `sqlparser::dialect::Precedence::AtTz` · sqlparser 0.62.0

```rust
AtTz
```

Source: `src/dialect/mod.rs:1800`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Timezone operator (e.g. `AT TIME ZONE`).

<a id="op-a3856897463b36776d463723"></a>
## Between

`variant` · `sqlparser::dialect::Precedence::Between` · sqlparser 0.62.0

```rust
Between
```

Source: `src/dialect/mod.rs:1816`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`BETWEEN` operator.

<a id="op-1bb73733e3e3e487cfe4e29b"></a>
## Caret

`variant` · `sqlparser::dialect::Precedence::Caret` · sqlparser 0.62.0

```rust
Caret
```

Source: `src/dialect/mod.rs:1810`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Bitwise `CARET` (^) for some dialects.

<a id="op-ba1a8db7dfbb884fe1f64048"></a>
## Colon

`variant` · `sqlparser::dialect::Precedence::Colon` · sqlparser 0.62.0

```rust
Colon
```

Source: `src/dialect/mod.rs:1814`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`:` operator for json/variant access.

<a id="op-853709d769307e435bda9ba9"></a>
## DoubleColon

`variant` · `sqlparser::dialect::Precedence::DoubleColon` · sqlparser 0.62.0

```rust
DoubleColon
```

Source: `src/dialect/mod.rs:1798`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Postgres style type cast `::`.

<a id="op-068a1fdf9eb57df6547eef10"></a>
## Eq

`variant` · `sqlparser::dialect::Precedence::Eq` · sqlparser 0.62.0

```rust
Eq
```

Source: `src/dialect/mod.rs:1818`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Equality operator (`=`).

<a id="op-f0d7c85f44e93027eb23f39c"></a>
## Is

`variant` · `sqlparser::dialect::Precedence::Is` · sqlparser 0.62.0

```rust
Is
```

Source: `src/dialect/mod.rs:1822`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`IS` operator (e.g. `IS NULL`).

<a id="op-43249c38057b2875ce4ac621"></a>
## Like

`variant` · `sqlparser::dialect::Precedence::Like` · sqlparser 0.62.0

```rust
Like
```

Source: `src/dialect/mod.rs:1820`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Pattern matching (`LIKE`).

<a id="op-d7d8fbbb5476182199074916"></a>
## MulDivModOp

`variant` · `sqlparser::dialect::Precedence::MulDivModOp` · sqlparser 0.62.0

```rust
MulDivModOp
```

Source: `src/dialect/mod.rs:1802`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Multiplication / Division / Modulo operators (`*`, `/`, `%`).

<a id="op-4827d1b88c7824446d054ab9"></a>
## Or

`variant` · `sqlparser::dialect::Precedence::Or` · sqlparser 0.62.0

```rust
Or
```

Source: `src/dialect/mod.rs:1830`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Logical `OR` (lowest precedence).

<a id="op-6ed00d2681de4580bbe4e018"></a>
## Period

`variant` · `sqlparser::dialect::Precedence::Period` · sqlparser 0.62.0

```rust
Period
```

Source: `src/dialect/mod.rs:1796`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Member access operator `.` (highest precedence).

<a id="op-ecb654603d93a69bb51da5c9"></a>
## PgOther

`variant` · `sqlparser::dialect::Precedence::PgOther` · sqlparser 0.62.0

```rust
PgOther
```

Source: `src/dialect/mod.rs:1824`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Other Postgres-specific operators.

<a id="op-181eb5e6da8d1e8ff5cd3348"></a>
## Pipe

`variant` · `sqlparser::dialect::Precedence::Pipe` · sqlparser 0.62.0

```rust
Pipe
```

Source: `src/dialect/mod.rs:1812`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Bitwise `OR` / pipe operator (`|`).

<a id="op-cad9413d680656ded84503cf"></a>
## PlusMinus

`variant` · `sqlparser::dialect::Precedence::PlusMinus` · sqlparser 0.62.0

```rust
PlusMinus
```

Source: `src/dialect/mod.rs:1804`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Addition / Subtraction (`+`, `-`).

<a id="op-f8952007fcf458217a7f96e1"></a>
## UnaryNot

`variant` · `sqlparser::dialect::Precedence::UnaryNot` · sqlparser 0.62.0

```rust
UnaryNot
```

Source: `src/dialect/mod.rs:1826`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Unary `NOT`.

<a id="op-692127f89c1f612445f7473f"></a>
## Xor

`variant` · `sqlparser::dialect::Precedence::Xor` · sqlparser 0.62.0

```rust
Xor
```

Source: `src/dialect/mod.rs:1806`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Bitwise `XOR` operator (`^`).

<a id="op-ddc87d5856c95aaf084db67e"></a>
## clone

`function` · `sqlparser::dialect::Precedence::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> Precedence
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::Precedence", "path": "Precedence"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1793, 17], "end": [1793, 22], "filename": "src/dialect/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/dialect/mod.rs:1793`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-49491fa891d7eabf60a7f5fd"></a>
## fmt

`function` · `sqlparser::dialect::Precedence::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::Precedence", "path": "Precedence"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1793, 10], "end": [1793, 15], "filename": "src/dialect/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/dialect/mod.rs:1793`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
