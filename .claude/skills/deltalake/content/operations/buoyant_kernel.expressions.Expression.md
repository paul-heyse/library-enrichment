# `buoyant_kernel::expressions::Expression`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.expressions.Expression.json).

<a id="op-b6e8a7405239f2ae57fc5e51"></a>
## Expression

`enum` · `buoyant_kernel::expressions::Expression` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
enum Expression
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/mod.rs#L377).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/mod.rs:377`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

A SQL expression.

These expressions do not track or validate data types, other than the type
of literals. It is up to the expression evaluator to validate the
expression against a schema and add appropriate casts as required.

<a id="op-25e4f2b75305ede859d1d490"></a>
## Binary

`variant` · `buoyant_kernel::expressions::Expression::Binary` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
Binary
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/mod.rs#L395).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/mod.rs:395`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

An expression that takes two expressions as input.

<a id="op-5c6f2251feb782a0ea3b0a8f"></a>
## Column

`variant` · `buoyant_kernel::expressions::Expression::Column` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
Column
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/mod.rs#L381).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/mod.rs:381`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

A column reference by name.

<a id="op-27c1daaf577668ddbb85bbe0"></a>
## Literal

`variant` · `buoyant_kernel::expressions::Expression::Literal` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
Literal
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/mod.rs#L379).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/mod.rs:379`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

A literal value.

<a id="op-24f49714b29d7c708849d5c2"></a>
## MapToStruct

`variant` · `buoyant_kernel::expressions::Expression::MapToStruct` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
MapToStruct
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/mod.rs#L416).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/mod.rs:416`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Extract keys from a `Map<String, String>` and parse values into a typed struct. See
[`MapToStructExpression`](../operations/buoyant_kernel.expressions.MapToStructExpression.md#op-0343b75651d1ad5e7379f876) for how values are parsed.

<a id="op-f421627945fc34e1eca00b07"></a>
## Opaque

`variant` · `buoyant_kernel::expressions::Expression::Opaque` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
Opaque
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/mod.rs#L402).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/mod.rs:402`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

An expression that the engine defines and implements. Kernel interacts with the expression
only through methods provided by the [`OpaqueExpressionOp`](../operations/buoyant_kernel.expressions.OpaqueExpressionOp.md#op-353b5fa46cb006e99cf79ad2) trait.

<a id="op-5239ec22cb41e01bc1da4d22"></a>
## Output

`assoc_type` · `buoyant_kernel::expressions::Expression::Output` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
type Output = Expression
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/mod.rs#L1103).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::expressions::Expression", "path": "Expression"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "buoyant_kernel::expressions::Expression", "path": "Expression"}}}], "constraints": []}}, "id": "core::convert::Into", "path": "Into"}}}], "default": null, "is_synthetic": false}}, "name": "R"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1102, 1], "end": [1108, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "R"}}], "constraints": []}}, "id": "core::ops::arith::Mul", "path": "Mul"}, "trait_path": "core::ops::arith::Mul"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/mod.rs:1103`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9c6793299004b6040d6fb40b"></a>
## Output

`assoc_type` · `buoyant_kernel::expressions::Expression::Output` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
type Output = Expression
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/mod.rs#L1087).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::expressions::Expression", "path": "Expression"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "buoyant_kernel::expressions::Expression", "path": "Expression"}}}], "constraints": []}}, "id": "core::convert::Into", "path": "Into"}}}], "default": null, "is_synthetic": false}}, "name": "R"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1086, 1], "end": [1092, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "R"}}], "constraints": []}}, "id": "core::ops::arith::Add", "path": "Add"}, "trait_path": "core::ops::arith::Add"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/mod.rs:1087`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-efd8ba9a26705a9ce103a3b2"></a>
## Output

`assoc_type` · `buoyant_kernel::expressions::Expression::Output` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
type Output = Expression
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/mod.rs#L1095).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::expressions::Expression", "path": "Expression"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "buoyant_kernel::expressions::Expression", "path": "Expression"}}}], "constraints": []}}, "id": "core::convert::Into", "path": "Into"}}}], "default": null, "is_synthetic": false}}, "name": "R"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1094, 1], "end": [1100, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "R"}}], "constraints": []}}, "id": "core::ops::arith::Sub", "path": "Sub"}, "trait_path": "core::ops::arith::Sub"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/mod.rs:1095`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f5ef2b0e1e6c7311ba9b8663"></a>
## Output

`assoc_type` · `buoyant_kernel::expressions::Expression::Output` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
type Output = Expression
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/mod.rs#L1111).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::expressions::Expression", "path": "Expression"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "buoyant_kernel::expressions::Expression", "path": "Expression"}}}], "constraints": []}}, "id": "core::convert::Into", "path": "Into"}}}], "default": null, "is_synthetic": false}}, "name": "R"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1110, 1], "end": [1116, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "R"}}], "constraints": []}}, "id": "core::ops::arith::Div", "path": "Div"}, "trait_path": "core::ops::arith::Div"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/mod.rs:1111`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ff955e1644d2c5adb9b7ad27"></a>
## ParseJson

`variant` · `buoyant_kernel::expressions::Expression::ParseJson` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
ParseJson
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/mod.rs#L413).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/mod.rs:413`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Parse a JSON string expression into a struct with the given schema.

<a id="op-31381e25b63bcaf9f83a21ad"></a>
## Predicate

`variant` · `buoyant_kernel::expressions::Expression::Predicate` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
Predicate
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/mod.rs#L383).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/mod.rs:383`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

A predicate treated as a boolean expression

<a id="op-fce5e2fce01a147b0fe0025d"></a>
## Struct

`variant` · `buoyant_kernel::expressions::Expression::Struct` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
Struct
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/mod.rs#L387).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/mod.rs:387`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

A struct computed from a Vec of expressions.
The optional nullability predicate, if provided and evaluates to false/null, makes the
entire struct null.

<a id="op-f672b2960ea3237f7b15cc88"></a>
## StructPatch

`variant` · `buoyant_kernel::expressions::Expression::StructPatch` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
StructPatch
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/mod.rs#L391).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/mod.rs:391`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

A sparse patch of a struct. More efficient than `Struct` for wide schemas
where only a few fields change, achieving O(changes) instead of O(schema_width) complexity.

<a id="op-71c4c304a65039d96b8c4d53"></a>
## Unary

`variant` · `buoyant_kernel::expressions::Expression::Unary` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
Unary
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/mod.rs#L393).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/mod.rs:393`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

An expression that takes one expression as input.

<a id="op-dc07956d37c78e012f25f69d"></a>
## Unknown

`variant` · `buoyant_kernel::expressions::Expression::Unknown` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
Unknown
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/mod.rs#L411).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/mod.rs:411`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

An unknown expression (i.e. one that neither kernel nor engine attempts to evaluate). For
data skipping purposes, kernel treats unknown expressions as if they were literal NULL
values (which may disable skipping if it "poisons" the predicate), but engines MUST NOT
attempt to interpret them as NULL when evaluating query filters because it could produce
incorrect results. For example, converting `WHERE <fancy-udf-invocation> IS NULL` to `WHERE
<unknown> IS NULL` to `WHERE NULL IS NULL` is equivalent to `WHERE TRUE` and would include
all rows -- almost certainly NOT what the query author intended. Use `Expression::Opaque`
for expressions kernel doesn't understand but which engine can still evaluate.

<a id="op-170ae4455ffd05b950f64309"></a>
## Variadic

`variant` · `buoyant_kernel::expressions::Expression::Variadic` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
Variadic
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/mod.rs#L397).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/mod.rs:397`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

An expression that takes a variable number of expressions as input.

<a id="op-29f021ab6f24673ce0735f3a"></a>
## add

`function` · `buoyant_kernel::expressions::Expression::add` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn add(self, rhs: R) -> Self::Output
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/mod.rs#L1089).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::expressions::Expression", "path": "Expression"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "buoyant_kernel::expressions::Expression", "path": "Expression"}}}], "constraints": []}}, "id": "core::convert::Into", "path": "Into"}}}], "default": null, "is_synthetic": false}}, "name": "R"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1086, 1], "end": [1092, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "R"}}], "constraints": []}}, "id": "core::ops::arith::Add", "path": "Add"}, "trait_path": "core::ops::arith::Add"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/mod.rs:1089`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b9b175b4216013ca1ff898e9"></a>
## array

`function` · `buoyant_kernel::expressions::Expression::array` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn array(exprs: impl IntoIterator<Item = impl Into<Expression>>) -> Self
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/mod.rs#L719).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::expressions::Expression", "path": "Expression"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [571, 1], "end": [749, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/mod.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/mod.rs:719`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Creates a new Array constructor expression. See [`VariadicExpressionOp::Array`](../operations/buoyant_kernel.expressions.VariadicExpressionOp.md#op-41ff2df0bd693d9adf29f055).

<a id="op-e6b1361d8113aa8b7b95ee3f"></a>
## arrow_opaque

`function` · `buoyant_kernel::expressions::Expression::arrow_opaque` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn arrow_opaque(op: impl ArrowOpaqueExpressionOp, exprs: impl IntoIterator<Item = Expression>) -> Expression
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/engine/arrow_expression/opaque.rs#L97).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::expressions::Expression", "path": "crate::expressions::Expression"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [96, 1], "end": [103, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/engine/arrow_expression/opaque.rs"}, "trait": {"args": null, "id": "buoyant_kernel::engine::arrow_expression::opaque::ArrowOpaqueExpression", "path": "ArrowOpaqueExpression"}, "trait_path": "buoyant_kernel::engine::arrow_expression::opaque::ArrowOpaqueExpression"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/engine/arrow_expression/opaque.rs:97`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-27999f5e4373578b2f153529"></a>
## binary

`function` · `buoyant_kernel::expressions::Expression::binary` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn binary(op: BinaryExpressionOp, lhs: impl Into<Expression>, rhs: impl Into<Expression>) -> Self
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/mod.rs#L694).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::expressions::Expression", "path": "Expression"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [571, 1], "end": [749, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/mod.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/mod.rs:694`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Creates a new binary expression lhs OP rhs

<a id="op-b1edb49871f2bb0843077098"></a>
## clone

`function` · `buoyant_kernel::expressions::Expression::clone` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn clone(&self) -> Expression
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/mod.rs#L376).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::expressions::Expression", "path": "Expression"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [376, 17], "end": [376, 22], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/mod.rs:376`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3a9634db3a99161a74442e91"></a>
## coalesce

`function` · `buoyant_kernel::expressions::Expression::coalesce` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn coalesce(exprs: impl IntoIterator<Item = impl Into<Expression>>) -> Self
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/mod.rs#L714).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::expressions::Expression", "path": "Expression"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [571, 1], "end": [749, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/mod.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/mod.rs:714`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Creates a new COALESCE expression that returns the first non-null value.

COALESCE evaluates expressions in order and returns the first non-null result.
If all expressions evaluate to null, the result is null.

<a id="op-411fb272c1cb374bc1e85f91"></a>
## column

`function` · `buoyant_kernel::expressions::Expression::column` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn column(field_names: impl CollectInto<ColumnName>) -> Expression
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/mod.rs#L580).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::expressions::Expression", "path": "Expression"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [571, 1], "end": [749, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/mod.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/mod.rs:580`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Create a new column name expression from input satisfying `FromIterator for ColumnName`.

<a id="op-88e0ffad5b196ffffd2cb0de"></a>
## deserialize

`function` · `buoyant_kernel::expressions::Expression::deserialize` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/mod.rs#L376).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::expressions::Expression", "path": "Expression"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [376, 46], "end": [376, 57], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/mod.rs:376`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-48d5157a07d30950162bfb36"></a>
## distinct

`function` · `buoyant_kernel::expressions::Expression::distinct` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn distinct(self, other: impl Into<Self>) -> Predicate
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/mod.rs#L684).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::expressions::Expression", "path": "Expression"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [571, 1], "end": [749, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/mod.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/mod.rs:684`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Create a new predicate `DISTINCT(self, other)`

<a id="op-8b9b1f6216f4ad933eaa78bc"></a>
## div

`function` · `buoyant_kernel::expressions::Expression::div` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn div(self, rhs: R) -> Self
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/mod.rs#L1113).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::expressions::Expression", "path": "Expression"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "buoyant_kernel::expressions::Expression", "path": "Expression"}}}], "constraints": []}}, "id": "core::convert::Into", "path": "Into"}}}], "default": null, "is_synthetic": false}}, "name": "R"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1110, 1], "end": [1116, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "R"}}], "constraints": []}}, "id": "core::ops::arith::Div", "path": "Div"}, "trait_path": "core::ops::arith::Div"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/mod.rs:1113`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-32732f1a9ae95599910d9e1a"></a>
## eq

`function` · `buoyant_kernel::expressions::Expression::eq` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn eq(self, other: impl Into<Self>) -> Predicate
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/mod.rs#L654).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::expressions::Expression", "path": "Expression"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [571, 1], "end": [749, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/mod.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/mod.rs:654`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Create a new predicate `self == other`

<a id="op-93b94e6c1917ad771006e6e7"></a>
## eq

`function` · `buoyant_kernel::expressions::Expression::eq` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn eq(&self, other: &Expression) -> bool
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/mod.rs#L376).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::expressions::Expression", "path": "Expression"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [376, 24], "end": [376, 33], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/mod.rs:376`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-99d19d36b5c0a10b72dfa33d"></a>
## fmt

`function` · `buoyant_kernel::expressions::Expression::fmt` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/mod.rs#L973).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::expressions::Expression", "path": "Expression"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [972, 1], "end": [1030, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/mod.rs:973`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-da85551aa5b97091c11e1aa3"></a>
## fmt

`function` · `buoyant_kernel::expressions::Expression::fmt` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/mod.rs#L376).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::expressions::Expression", "path": "Expression"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [376, 10], "end": [376, 15], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/mod.rs:376`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0c2ba6f0f881993f64ada64a"></a>
## from

`function` · `buoyant_kernel::expressions::Expression::from` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn from(value: Scalar) -> Self
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/mod.rs#L1063).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::expressions::Expression", "path": "Expression"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1062, 1], "end": [1066, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "buoyant_kernel::expressions::scalars::Scalar", "path": "Scalar"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/mod.rs:1063`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-12f02d2e4d993a9ac2c8f172"></a>
## from

`function` · `buoyant_kernel::expressions::Expression::from` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn from(value: Predicate) -> Self
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/mod.rs#L1075).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::expressions::Expression", "path": "Expression"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1074, 1], "end": [1078, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "buoyant_kernel::expressions::Predicate", "path": "Predicate"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/mod.rs:1075`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-83059d579bc772d526ed18be"></a>
## from

`function` · `buoyant_kernel::expressions::Expression::from` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn from(value: ColumnName) -> Self
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/mod.rs#L1069).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::expressions::Expression", "path": "Expression"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1068, 1], "end": [1072, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "buoyant_kernel::expressions::column_names::ColumnName", "path": "ColumnName"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/mod.rs:1069`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e721a1c6e948e3675e17c241"></a>
## from_pred

`function` · `buoyant_kernel::expressions::Expression::from_pred` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn from_pred(value: Predicate) -> Self
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/mod.rs#L595).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::expressions::Expression", "path": "Expression"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [571, 1], "end": [749, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/mod.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/mod.rs:595`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Wraps a predicate as a boolean-valued expression

<a id="op-48f7412a1a89edabc66b6778"></a>
## ge

`function` · `buoyant_kernel::expressions::Expression::ge` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn ge(self, other: impl Into<Self>) -> Predicate
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/mod.rs#L674).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::expressions::Expression", "path": "Expression"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [571, 1], "end": [749, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/mod.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/mod.rs:674`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Create a new predicate `self >= other`

<a id="op-abcc973261a370100fcefdf2"></a>
## gt

`function` · `buoyant_kernel::expressions::Expression::gt` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn gt(self, other: impl Into<Self>) -> Predicate
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/mod.rs#L679).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::expressions::Expression", "path": "Expression"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [571, 1], "end": [749, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/mod.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/mod.rs:679`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Create a new predicate `self > other`

<a id="op-a7bb1a3bbf4b1eca949ed670"></a>
## is_not_null

`function` · `buoyant_kernel::expressions::Expression::is_not_null` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn is_not_null(self) -> Predicate
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/mod.rs#L649).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::expressions::Expression", "path": "Expression"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [571, 1], "end": [749, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/mod.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/mod.rs:649`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Create a new predicate `self IS NOT NULL`

<a id="op-526b6e7698bb4bf55bd9ac63"></a>
## is_null

`function` · `buoyant_kernel::expressions::Expression::is_null` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn is_null(self) -> Predicate
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/mod.rs#L644).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::expressions::Expression", "path": "Expression"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [571, 1], "end": [749, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/mod.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/mod.rs:644`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Create a new predicate `self IS NULL`

<a id="op-f368c676bc31c2305da19e4b"></a>
## le

`function` · `buoyant_kernel::expressions::Expression::le` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn le(self, other: impl Into<Self>) -> Predicate
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/mod.rs#L664).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::expressions::Expression", "path": "Expression"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [571, 1], "end": [749, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/mod.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/mod.rs:664`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Create a new predicate `self <= other`

<a id="op-3f92c8cb2152d1236538d451"></a>
## literal

`function` · `buoyant_kernel::expressions::Expression::literal` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn literal(value: impl Into<Scalar>) -> Self
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/mod.rs#L585).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::expressions::Expression", "path": "Expression"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [571, 1], "end": [749, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/mod.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/mod.rs:585`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Create a new expression for a literal value

<a id="op-9b9c1a97efa97c05720f98c0"></a>
## lt

`function` · `buoyant_kernel::expressions::Expression::lt` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn lt(self, other: impl Into<Self>) -> Predicate
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/mod.rs#L669).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::expressions::Expression", "path": "Expression"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [571, 1], "end": [749, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/mod.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/mod.rs:669`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Create a new predicate `self < other`

<a id="op-5cfd563cc10d970f32e304b8"></a>
## map_to_struct

`function` · `buoyant_kernel::expressions::Expression::map_to_struct` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn map_to_struct(map_expr: impl Into<Expression>) -> Self
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/mod.rs#L746).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::expressions::Expression", "path": "Expression"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [571, 1], "end": [749, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/mod.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/mod.rs:746`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Extracts keys from a `Map<String, String>` and parses values into a typed struct. The output
struct schema is determined by the evaluator's `result_type`. An empty-string value is the
exception (aligning with Spark): it casts to itself for string, to empty bytes for binary,
and to null for every other type. See [`MapToStructExpression`](../operations/buoyant_kernel.expressions.MapToStructExpression.md#op-0343b75651d1ad5e7379f876) for the full contract.

<a id="op-319cdd59812dcf890a681a11"></a>
## mul

`function` · `buoyant_kernel::expressions::Expression::mul` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn mul(self, rhs: R) -> Self
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/mod.rs#L1105).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::expressions::Expression", "path": "Expression"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "buoyant_kernel::expressions::Expression", "path": "Expression"}}}], "constraints": []}}, "id": "core::convert::Into", "path": "Into"}}}], "default": null, "is_synthetic": false}}, "name": "R"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1102, 1], "end": [1108, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "R"}}], "constraints": []}}, "id": "core::ops::arith::Mul", "path": "Mul"}, "trait_path": "core::ops::arith::Mul"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/mod.rs:1105`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c35398c405bad877d8da944a"></a>
## ne

`function` · `buoyant_kernel::expressions::Expression::ne` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn ne(self, other: impl Into<Self>) -> Predicate
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/mod.rs#L659).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::expressions::Expression", "path": "Expression"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [571, 1], "end": [749, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/mod.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/mod.rs:659`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Create a new predicate `self != other`

<a id="op-311335dd8168df2281a125f7"></a>
## null_literal

`function` · `buoyant_kernel::expressions::Expression::null_literal` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
const fn null_literal(data_type: DataType) -> Self
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/mod.rs#L590).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::expressions::Expression", "path": "Expression"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [571, 1], "end": [749, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/mod.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/mod.rs:590`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Creates a NULL literal expression

<a id="op-fafc306d0bf13887c9564f57"></a>
## opaque

`function` · `buoyant_kernel::expressions::Expression::opaque` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn opaque(op: impl OpaqueExpressionOp, exprs: impl IntoIterator<Item = Expression>) -> Self
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/mod.rs#L724).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::expressions::Expression", "path": "Expression"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [571, 1], "end": [749, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/mod.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/mod.rs:724`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Creates a new opaque expression

<a id="op-0b63741faf5b625b5727726b"></a>
## parse_json

`function` · `buoyant_kernel::expressions::Expression::parse_json` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn parse_json(json_expr: impl Into<Expression>, output_schema: SchemaRef) -> Self
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/mod.rs#L738).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::expressions::Expression", "path": "Expression"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [571, 1], "end": [749, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/mod.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/mod.rs:738`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Creates a new ParseJson expression that parses a JSON string column into a struct.
This is the inverse of `ToJson` - it converts a JSON-encoded string into a struct.

<a id="op-bf1500efa49601571020e8e7"></a>
## references

`function` · `buoyant_kernel::expressions::Expression::references` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn references(&self) -> HashSet<&ColumnName>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/mod.rs#L573).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::expressions::Expression", "path": "Expression"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [571, 1], "end": [749, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/mod.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/mod.rs:573`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Returns a set of columns referenced by this expression.

<a id="op-e28b05a28d721b2495da59f7"></a>
## serialize

`function` · `buoyant_kernel::expressions::Expression::serialize` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/mod.rs#L376).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::expressions::Expression", "path": "Expression"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [376, 35], "end": [376, 44], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/mod.rs:376`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-62af65763a113c849a3a1381"></a>
## struct_from

`function` · `buoyant_kernel::expressions::Expression::struct_from` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn struct_from(exprs: impl IntoIterator<Item = impl Into<Arc<Self>>>) -> Self
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/mod.rs#L608).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::expressions::Expression", "path": "Expression"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [571, 1], "end": [749, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/mod.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/mod.rs:608`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Create a new struct expression.

The field names and types are supplied by the caller at evaluation time via the
`result_type` parameter of the expression evaluator. Use this when the schema is
always available from external context (e.g. the expression is the top-level output
of [`crate::ExpressionEvaluator`](../operations/buoyant_kernel.ExpressionEvaluator.md#op-02eea996b6bdb19503c4315a)).

<a id="op-bebdae31f4482e2b21c37d89"></a>
## struct_patch

`function` · `buoyant_kernel::expressions::Expression::struct_patch` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn struct_patch<P>(patch: P) -> DeltaResult<Self> where P: TryInto<ExpressionStructPatch>, Error: From<P::Error>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/mod.rs#L635).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::expressions::Expression", "path": "Expression"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [571, 1], "end": [749, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/mod.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/mod.rs:635`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Creates a new struct patch expression from a raw patch or patch builder.

Returns an expression that applies the supplied sparse patch to an input struct. Passing a
raw [`ExpressionStructPatch`](../operations/buoyant_kernel.struct_patch.ExpressionStructPatch.md#op-1f79369df99deab3abc687ed) is infallible; passing an [`ExpressionStructPatchBuilder`](../operations/buoyant_kernel.expressions.ExpressionStructPatchBuilder.md#op-c292c4f3a2cbf09638cc5e18)
validates and lowers the recorded operations before constructing the expression.

# Errors

Returns an error if the supplied patch builder contains conflicting operations.

<a id="op-072e156f23e0c71a07885e14"></a>
## struct_with_nullability_from

`function` · `buoyant_kernel::expressions::Expression::struct_with_nullability_from` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn struct_with_nullability_from(exprs: impl IntoIterator<Item = impl Into<Arc<Self>>>, nullability_predicate: impl Into<Arc<Self>>) -> Self
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/mod.rs#L616).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::expressions::Expression", "path": "Expression"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [571, 1], "end": [749, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/mod.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/mod.rs:616`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Create a new struct expression with a nullability predicate.

When the predicate evaluates to false or null for a row, the entire struct is null
for that row.

<a id="op-44961a80a3f7a35bc57a4e8d"></a>
## sub

`function` · `buoyant_kernel::expressions::Expression::sub` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn sub(self, rhs: R) -> Self
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/mod.rs#L1097).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::expressions::Expression", "path": "Expression"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "buoyant_kernel::expressions::Expression", "path": "Expression"}}}], "constraints": []}}, "id": "core::convert::Into", "path": "Into"}}}], "default": null, "is_synthetic": false}}, "name": "R"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1094, 1], "end": [1100, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "R"}}], "constraints": []}}, "id": "core::ops::arith::Sub", "path": "Sub"}, "trait_path": "core::ops::arith::Sub"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/mod.rs:1097`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b533d25753e994ab3c65c130"></a>
## unary

`function` · `buoyant_kernel::expressions::Expression::unary` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn unary(op: UnaryExpressionOp, expr: impl Into<Expression>) -> Self
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/mod.rs#L689).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::expressions::Expression", "path": "Expression"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [571, 1], "end": [749, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/mod.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/mod.rs:689`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Creates a new unary expression

<a id="op-26103d0b01185f1cad10ab5e"></a>
## unknown

`function` · `buoyant_kernel::expressions::Expression::unknown` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn unknown(name: impl Into<String>) -> Self
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/mod.rs#L732).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::expressions::Expression", "path": "Expression"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [571, 1], "end": [749, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/mod.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/mod.rs:732`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Creates a new unknown expression

<a id="op-d342865ab826a1baa38cf7ba"></a>
## variadic

`function` · `buoyant_kernel::expressions::Expression::variadic` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn variadic(op: VariadicExpressionOp, exprs: impl IntoIterator<Item = impl Into<Expression>>) -> Self
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/mod.rs#L703).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::expressions::Expression", "path": "Expression"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [571, 1], "end": [749, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/mod.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/mod.rs:703`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Creates a new variadic expression
