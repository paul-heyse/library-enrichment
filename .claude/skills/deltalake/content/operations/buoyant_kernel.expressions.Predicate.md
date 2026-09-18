# `buoyant_kernel::expressions::Predicate`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.expressions.Predicate.json).

<a id="op-0c0ee21b4ebfcd851e64ceb4"></a>
## Predicate

`enum` · `buoyant_kernel::expressions::Predicate` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
enum Predicate
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/mod.rs#L425).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/mod.rs:425`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

A SQL predicate.

These predicates do not track or validate data types, other than the type
of literals. It is up to the predicate evaluator to validate the
predicate against a schema and add appropriate casts as required.

<a id="op-3b5ad5796d0bd14bb2e7acbf"></a>
## Binary

`variant` · `buoyant_kernel::expressions::Predicate::Binary` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
Binary
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/mod.rs#L438).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/mod.rs:438`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

A binary operation.

<a id="op-8ed19ee54dbf644e251d45e5"></a>
## BooleanExpression

`variant` · `buoyant_kernel::expressions::Predicate::BooleanExpression` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
BooleanExpression
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/mod.rs#L427).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/mod.rs:427`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

A boolean-valued expression, useful for e.g. `AND(<boolean_col1>, <boolean_col2>)`.

<a id="op-c6c6e24d7c6123b4347616a8"></a>
## Junction

`variant` · `buoyant_kernel::expressions::Predicate::Junction` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
Junction
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/mod.rs#L440).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/mod.rs:440`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

A junction operation (AND/OR).

<a id="op-1e98d189910f98d1378713b1"></a>
## Not

`variant` · `buoyant_kernel::expressions::Predicate::Not` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
Not
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/mod.rs#L434).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/mod.rs:434`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Boolean inversion (true <-> false)

NOTE: NOT is not a normal unary predicate, because it requires a predicate as input (not an
expression), and is never directly evaluated. Instead, observing that all predicates are
invertible, NOT is always pushed down into its child predicate, inverting it. For example,
`NOT (a < b)` pushes down and inverts `<` to `>=`, producing `a >= b`.

<a id="op-5a8c5aaf105d4ca2a5666ce8"></a>
## Opaque

`variant` · `buoyant_kernel::expressions::Predicate::Opaque` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
Opaque
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/mod.rs#L445).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/mod.rs:445`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

A predicate that the engine defines and implements. Kernel interacts with the predicate
only through methods provided by the [`OpaquePredicateOp`](../operations/buoyant_kernel.expressions.OpaquePredicateOp.md#op-0af44540a36e1b39fab1b889) trait.

<a id="op-80fd7757cd74cfeac8a4310b"></a>
## Unary

`variant` · `buoyant_kernel::expressions::Predicate::Unary` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
Unary
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/mod.rs#L436).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/mod.rs:436`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

A unary operation.

<a id="op-cd7b989c2c1d268e136f3ac1"></a>
## Unknown

`variant` · `buoyant_kernel::expressions::Predicate::Unknown` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
Unknown
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/mod.rs#L454).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/mod.rs:454`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

An unknown predicate (i.e. one that neither kernel nor engine attempts to evaluate). For
data skipping purposes, kernel treats unknown predicates as if they were literal NULL
values (which may disable skipping if it "poisons" the predicate), but engines MUST NOT
attempt to interpret them as NULL when evaluating query filters because it could
produce incorrect results. For example, converting `WHERE <fancy-udf-invocation>` to
`WHERE NULL` is equivalent to `WHERE FALSE` and would filter out all rows -- almost
certainly NOT what the query author intended. Use `Predicate::Opaque` for predicates
kernel doesn't understand but which engine can still evaluate.

<a id="op-2830cc526f34f0dc015bb6b4"></a>
## and

`function` · `buoyant_kernel::expressions::Predicate::and` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn and(a: impl Into<Self>, b: impl Into<Self>) -> Self
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/mod.rs#L833).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::expressions::Predicate", "path": "Predicate"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [751, 1], "end": [902, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/mod.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/mod.rs:833`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Create a new predicate `self AND other`

<a id="op-5bb4be8dc11c578f96c7300e"></a>
## and_from

`function` · `buoyant_kernel::expressions::Predicate::and_from` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn and_from(preds: impl IntoIterator<Item = Self>) -> Self
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/mod.rs#L844).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::expressions::Predicate", "path": "Predicate"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [751, 1], "end": [902, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/mod.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/mod.rs:844`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Creates a new predicate AND(preds...). See [`Self::junction`](../operations/buoyant_kernel.expressions.Predicate.md#op-d6ca7fb490f33837909edbfb) for normalization of
empty and single-element inputs.

<a id="op-af9bcf3e34394446f2adddb8"></a>
## arrow_opaque

`function` · `buoyant_kernel::expressions::Predicate::arrow_opaque` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn arrow_opaque<T: ArrowOpaquePredicateOp>(op: T, exprs: impl IntoIterator<Item = Expression>) -> Predicate
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/engine/arrow_expression/opaque.rs#L115).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::expressions::Predicate", "path": "crate::expressions::Predicate"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [114, 1], "end": [121, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/engine/arrow_expression/opaque.rs"}, "trait": {"args": null, "id": "buoyant_kernel::engine::arrow_expression::opaque::ArrowOpaquePredicate", "path": "ArrowOpaquePredicate"}, "trait_path": "buoyant_kernel::engine::arrow_expression::opaque::ArrowOpaquePredicate"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/engine/arrow_expression/opaque.rs:115`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ae28f09760deb38f460d91c8"></a>
## binary

`function` · `buoyant_kernel::expressions::Predicate::binary` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn binary(op: BinaryPredicateOp, lhs: impl Into<Expression>, rhs: impl Into<Expression>) -> Self
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/mod.rs#L861).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::expressions::Predicate", "path": "Predicate"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [751, 1], "end": [902, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/mod.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/mod.rs:861`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Creates a new binary predicate lhs OP rhs

<a id="op-57095769caa4db08e0348326"></a>
## clone

`function` · `buoyant_kernel::expressions::Predicate::clone` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn clone(&self) -> Predicate
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/mod.rs#L424).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::expressions::Predicate", "path": "Predicate"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [424, 17], "end": [424, 22], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/mod.rs:424`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-87c6e2fa9d80609cd410c20b"></a>
## column

`function` · `buoyant_kernel::expressions::Predicate::column` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn column(field_names: impl CollectInto<ColumnName>) -> Predicate
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/mod.rs#L760).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::expressions::Predicate", "path": "Predicate"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [751, 1], "end": [902, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/mod.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/mod.rs:760`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Creates a new boolean column reference. See also [`Expression::column`](../operations/buoyant_kernel.expressions.Expression.md#op-411fb272c1cb374bc1e85f91).

<a id="op-c95317deaee005c72b3ff13f"></a>
## deserialize

`function` · `buoyant_kernel::expressions::Predicate::deserialize` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/mod.rs#L424).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::expressions::Predicate", "path": "Predicate"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [424, 46], "end": [424, 57], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/mod.rs:424`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e4ce5f45c2045906d3fb8b05"></a>
## distinct

`function` · `buoyant_kernel::expressions::Predicate::distinct` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn distinct(a: impl Into<Expression>, b: impl Into<Expression>) -> Self
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/mod.rs#L828).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::expressions::Predicate", "path": "Predicate"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [751, 1], "end": [902, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/mod.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/mod.rs:828`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Create a new predicate `DISTINCT(self, other)`

<a id="op-8bb6723164dbc0e7d7fa6124"></a>
## eq

`function` · `buoyant_kernel::expressions::Predicate::eq` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn eq(a: impl Into<Expression>, b: impl Into<Expression>) -> Self
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/mod.rs#L798).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::expressions::Predicate", "path": "Predicate"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [751, 1], "end": [902, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/mod.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/mod.rs:798`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Create a new predicate `self == other`

<a id="op-dd175434bcc295379ff1f4f0"></a>
## eq

`function` · `buoyant_kernel::expressions::Predicate::eq` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn eq(&self, other: &Predicate) -> bool
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/mod.rs#L424).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::expressions::Predicate", "path": "Predicate"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [424, 24], "end": [424, 33], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/mod.rs:424`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-840d7fc28e1e837fdc5b23ef"></a>
## fmt

`function` · `buoyant_kernel::expressions::Predicate::fmt` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/mod.rs#L424).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::expressions::Predicate", "path": "Predicate"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [424, 10], "end": [424, 15], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/mod.rs:424`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b2abd24caf5fec9d65bfb6cb"></a>
## fmt

`function` · `buoyant_kernel::expressions::Predicate::fmt` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/mod.rs#L1033).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::expressions::Predicate", "path": "Predicate"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1032, 1], "end": [1060, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/mod.rs:1033`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b8675a31cbc75883b7f3e05b"></a>
## from

`function` · `buoyant_kernel::expressions::Predicate::from` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn from(value: ColumnName) -> Self
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/mod.rs#L1081).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::expressions::Predicate", "path": "Predicate"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1080, 1], "end": [1084, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "buoyant_kernel::expressions::column_names::ColumnName", "path": "ColumnName"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/mod.rs:1081`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6645a5a7a02e5a745524e229"></a>
## from_expr

`function` · `buoyant_kernel::expressions::Predicate::from_expr` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn from_expr(expr: impl Into<Expression>) -> Self
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/mod.rs#L775).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::expressions::Predicate", "path": "Predicate"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [751, 1], "end": [902, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/mod.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/mod.rs:775`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Converts a boolean-valued expression into a predicate

<a id="op-5072095f4dfb45f84b0963fc"></a>
## ge

`function` · `buoyant_kernel::expressions::Predicate::ge` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn ge(a: impl Into<Expression>, b: impl Into<Expression>) -> Self
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/mod.rs#L818).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::expressions::Predicate", "path": "Predicate"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [751, 1], "end": [902, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/mod.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/mod.rs:818`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Create a new predicate `self >= other`

<a id="op-627bc99fac9d96c2db108163"></a>
## gt

`function` · `buoyant_kernel::expressions::Predicate::gt` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn gt(a: impl Into<Expression>, b: impl Into<Expression>) -> Self
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/mod.rs#L823).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::expressions::Predicate", "path": "Predicate"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [751, 1], "end": [902, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/mod.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/mod.rs:823`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Create a new predicate `self > other`

<a id="op-fe573af954e331e5ecef4151"></a>
## is_not_null

`function` · `buoyant_kernel::expressions::Predicate::is_not_null` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn is_not_null(expr: impl Into<Expression>) -> Predicate
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/mod.rs#L793).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::expressions::Predicate", "path": "Predicate"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [751, 1], "end": [902, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/mod.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/mod.rs:793`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Create a new predicate `self IS NOT NULL`

<a id="op-86a2e18e8f4acd40df655348"></a>
## is_null

`function` · `buoyant_kernel::expressions::Predicate::is_null` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn is_null(expr: impl Into<Expression>) -> Predicate
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/mod.rs#L788).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::expressions::Predicate", "path": "Predicate"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [751, 1], "end": [902, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/mod.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/mod.rs:788`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Create a new predicate `self IS NULL`

<a id="op-d6ca7fb490f33837909edbfb"></a>
## junction

`function` · `buoyant_kernel::expressions::Predicate::junction` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn junction(op: JunctionPredicateOp, preds: impl IntoIterator<Item = Self>) -> Self
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/mod.rs#L880).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::expressions::Predicate", "path": "Predicate"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [751, 1], "end": [902, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/mod.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/mod.rs:880`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Creates a new junction predicate OP(preds...). Normalizes degenerate cases:

- Empty junction returns the identity element (the value that has no effect when combined
  with other predicates under the same operator):
  - `AND()` -> `true`, because `true AND p` == `p` for any predicate `p`.
  - `OR()` -> `false`, because `false OR p` == `p` for any predicate `p`.
- Single-element junction unwraps the element: `AND(p)` / `OR(p)` -> `p`.

<a id="op-6cec101dd1bb55b2c3a88465"></a>
## le

`function` · `buoyant_kernel::expressions::Predicate::le` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn le(a: impl Into<Expression>, b: impl Into<Expression>) -> Self
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/mod.rs#L808).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::expressions::Predicate", "path": "Predicate"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [751, 1], "end": [902, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/mod.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/mod.rs:808`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Create a new predicate `self <= other`

<a id="op-b907740291abc91acc6a8791"></a>
## literal

`function` · `buoyant_kernel::expressions::Predicate::literal` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
const fn literal(value: bool) -> Self
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/mod.rs#L765).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::expressions::Predicate", "path": "Predicate"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [751, 1], "end": [902, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/mod.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/mod.rs:765`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Create a new literal boolean value

<a id="op-82970eec4eb9361196b25630"></a>
## lt

`function` · `buoyant_kernel::expressions::Predicate::lt` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn lt(a: impl Into<Expression>, b: impl Into<Expression>) -> Self
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/mod.rs#L813).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::expressions::Predicate", "path": "Predicate"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [751, 1], "end": [902, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/mod.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/mod.rs:813`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Create a new predicate `self < other`

<a id="op-bad35da9e0272ee4541fea9f"></a>
## ne

`function` · `buoyant_kernel::expressions::Predicate::ne` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn ne(a: impl Into<Expression>, b: impl Into<Expression>) -> Self
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/mod.rs#L803).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::expressions::Predicate", "path": "Predicate"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [751, 1], "end": [902, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/mod.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/mod.rs:803`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Create a new predicate `self != other`

<a id="op-6f47208e5384a0870865133e"></a>
## not

`function` · `buoyant_kernel::expressions::Predicate::not` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn not(pred: impl Into<Self>) -> Self
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/mod.rs#L783).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::expressions::Predicate", "path": "Predicate"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [751, 1], "end": [902, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/mod.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/mod.rs:783`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Logical NOT (boolean inversion)

<a id="op-3dd984763826611e5eecddc5"></a>
## null_literal

`function` · `buoyant_kernel::expressions::Predicate::null_literal` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
const fn null_literal() -> Self
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/mod.rs#L770).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::expressions::Predicate", "path": "Predicate"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [751, 1], "end": [902, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/mod.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/mod.rs:770`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Creates a NULL literal boolean value

<a id="op-386d6810ec10b93244da1c09"></a>
## opaque

`function` · `buoyant_kernel::expressions::Predicate::opaque` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn opaque(op: impl OpaquePredicateOp, exprs: impl IntoIterator<Item = Expression>) -> Self
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/mod.rs#L894).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::expressions::Predicate", "path": "Predicate"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [751, 1], "end": [902, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/mod.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/mod.rs:894`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Creates a new opaque predicate

<a id="op-e3fef9f0b44a37b27dbe0058"></a>
## or

`function` · `buoyant_kernel::expressions::Predicate::or` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn or(a: impl Into<Self>, b: impl Into<Self>) -> Self
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/mod.rs#L838).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::expressions::Predicate", "path": "Predicate"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [751, 1], "end": [902, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/mod.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/mod.rs:838`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Create a new predicate `self OR other`

<a id="op-73f0fe24ac08d229df07b067"></a>
## or_from

`function` · `buoyant_kernel::expressions::Predicate::or_from` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn or_from(preds: impl IntoIterator<Item = Self>) -> Self
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/mod.rs#L850).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::expressions::Predicate", "path": "Predicate"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [751, 1], "end": [902, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/mod.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/mod.rs:850`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Creates a new predicate OR(preds...). See [`Self::junction`](../operations/buoyant_kernel.expressions.Predicate.md#op-d6ca7fb490f33837909edbfb) for normalization of
empty and single-element inputs.

<a id="op-632bae1926385248678d8439"></a>
## references

`function` · `buoyant_kernel::expressions::Predicate::references` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn references(&self) -> HashSet<&ColumnName>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/mod.rs#L753).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::expressions::Predicate", "path": "Predicate"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [751, 1], "end": [902, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/mod.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/mod.rs:753`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Returns a set of columns referenced by this predicate.

<a id="op-e90fd5e1386ac10ec4dce9b9"></a>
## serialize

`function` · `buoyant_kernel::expressions::Predicate::serialize` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/mod.rs#L424).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::expressions::Predicate", "path": "Predicate"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [424, 35], "end": [424, 44], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/mod.rs:424`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-512dc98c9e4a90b741aad886"></a>
## unary

`function` · `buoyant_kernel::expressions::Predicate::unary` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn unary(op: UnaryPredicateOp, expr: impl Into<Expression>) -> Self
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/mod.rs#L855).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::expressions::Predicate", "path": "Predicate"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [751, 1], "end": [902, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/mod.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/mod.rs:855`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Creates a new unary predicate OP expr

<a id="op-e230826b83ea491064c04bbc"></a>
## unknown

`function` · `buoyant_kernel::expressions::Predicate::unknown` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn unknown(name: impl Into<String>) -> Self
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/mod.rs#L899).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::expressions::Predicate", "path": "Predicate"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [751, 1], "end": [902, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/mod.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/mod.rs:899`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Creates a new unknown predicate
