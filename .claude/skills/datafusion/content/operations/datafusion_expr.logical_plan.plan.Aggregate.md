# `datafusion_expr::logical_plan::plan::Aggregate`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.logical_plan.plan.Aggregate.json).

<a id="op-ec4cb4b20e4e0b500a880004"></a>
## Aggregate

`struct` · `datafusion_expr::logical_plan::plan::Aggregate` · datafusion-expr 55.1.0

```rust
struct Aggregate
```

Source: `src/logical_plan/plan.rs:3889`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Aggregates its input based on a set of grouping and aggregate
expressions (e.g. SUM).

# Output Schema

The output schema is the group expressions followed by the aggregate
expressions in order.

For example, given the input schema `"A", "B", "C"` and the aggregate
`SUM(A) GROUP BY C+B`, the output schema will be `"C+B", "SUM(A)"` where
"C+B" and "SUM(A)" are the names of the output columns. Note that "C+B" is a
single new column

<a id="op-f998e00ebaf228c7575ec016"></a>
## INTERNAL_GROUPING_ID

`assoc_const` · `datafusion_expr::logical_plan::plan::Aggregate::INTERNAL_GROUPING_ID` · datafusion-expr 55.1.0

```rust
INTERNAL_GROUPING_ID
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::Aggregate", "path": "Aggregate"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3902, 1], "end": [4074, 2], "filename": "src/logical_plan/plan.rs"}, "trait": null, "trait_path": null}`

Source: `src/logical_plan/plan.rs:4073`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Internal column used when the aggregation is a grouping set.

This column packs two values into a single unsigned integer:

- **Low bits (positions 0 .. n-1)**: a semantic bitmask where each bit
  represents one of the `n` grouping expressions.  The least significant
  bit corresponds to the rightmost grouping expression.  A `1` bit means
  the corresponding column is replaced with `NULL` for this grouping set;
  a `0` bit means it is included.
- **High bits (positions n and above)**: a *duplicate ordinal* that
  distinguishes multiple occurrences of the same semantic grouping set
  pattern within a single query.  The ordinal is `0` for the first
  occurrence, `1` for the second, and so on.

The integer type is chosen by [`Self::grouping_id_type`](../operations/datafusion_expr.logical_plan.plan.Aggregate.md#op-91ad0391fbaba70198d2a51a) to be the
smallest `UInt8 / UInt16 / UInt32 / UInt64` that can represent both
parts.

For example, for the grouping expressions CUBE(a, b) (no duplicates),
the grouping ID column will have the following values:
    0b00: Both `a` and `b` are included
    0b01: `b` is excluded
    0b10: `a` is excluded
    0b11: Both `a` and `b` are excluded

When the same set appears twice and `n = 2`, the duplicate ordinal is
packed into bit 2:
    first occurrence:  `0b0_01` (ordinal = 0, mask = 0b01)
    second occurrence: `0b1_01` (ordinal = 1, mask = 0b01)

The GROUPING function always masks the value with `(1 << n) - 1` before
interpreting it so the ordinal bits are invisible to user-facing SQL.

<a id="op-8d7c9d141e88c8f7253182eb"></a>
## aggr_expr

`struct_field` · `datafusion_expr::logical_plan::plan::Aggregate::aggr_expr` · datafusion-expr 55.1.0

```rust
aggr_expr: Vec<Expr>
```

Source: `src/logical_plan/plan.rs:3897`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Aggregate expressions.

Note these *must* be either [`Expr::AggregateFunction`](../operations/datafusion_expr.expr.Expr.md#op-98c26e3c882c5c14cacb82a2) or [`Expr::Alias`](../operations/datafusion_expr.expr.Expr.md#op-99ef95e1ad9855e695be1ebb)

<a id="op-0f8c6622bc2480ca2b8b13c2"></a>
## clone

`function` · `datafusion_expr::logical_plan::plan::Aggregate::clone` · datafusion-expr 55.1.0

```rust
fn clone(&self) -> Aggregate
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::Aggregate", "path": "Aggregate"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3886, 17], "end": [3886, 22], "filename": "src/logical_plan/plan.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/logical_plan/plan.rs:3886`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e62012866f3f1035fcd570df"></a>
## eq

`function` · `datafusion_expr::logical_plan::plan::Aggregate::eq` · datafusion-expr 55.1.0

```rust
fn eq(&self, other: &Aggregate) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::Aggregate", "path": "Aggregate"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3886, 24], "end": [3886, 33], "filename": "src/logical_plan/plan.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/logical_plan/plan.rs:3886`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-809b798ae2ebbcb08120aea3"></a>
## fmt

`function` · `datafusion_expr::logical_plan::plan::Aggregate::fmt` · datafusion-expr 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::Aggregate", "path": "Aggregate"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3886, 10], "end": [3886, 15], "filename": "src/logical_plan/plan.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/logical_plan/plan.rs:3886`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e1df8f63695b8479f72106fe"></a>
## group_expr

`struct_field` · `datafusion_expr::logical_plan::plan::Aggregate::group_expr` · datafusion-expr 55.1.0

```rust
group_expr: Vec<Expr>
```

Source: `src/logical_plan/plan.rs:3893`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Grouping expressions

<a id="op-647b5d4c0e0b51643f9854b4"></a>
## group_expr_len

`function` · `datafusion_expr::logical_plan::plan::Aggregate::group_expr_len` · datafusion-expr 55.1.0

```rust
fn group_expr_len(&self) -> Result<usize>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::Aggregate", "path": "Aggregate"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3902, 1], "end": [4074, 2], "filename": "src/logical_plan/plan.rs"}, "trait": null, "trait_path": null}`

Source: `src/logical_plan/plan.rs:4012`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Get the length of the group by expression in the output schema
This is not simply group by expression length. Expression may be
GroupingSet, etc. In these case we need to get inner expression lengths.

<a id="op-91ad0391fbaba70198d2a51a"></a>
## grouping_id_type

`function` · `datafusion_expr::logical_plan::plan::Aggregate::grouping_id_type` · datafusion-expr 55.1.0

```rust
fn grouping_id_type(group_exprs: usize, max_ordinal: usize) -> DataType
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::Aggregate", "path": "Aggregate"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3902, 1], "end": [4074, 2], "filename": "src/logical_plan/plan.rs"}, "trait": null, "trait_path": null}`

Source: `src/logical_plan/plan.rs:4027`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Returns the data type of the grouping id.

The grouping ID packs two pieces of information into a single integer:
- The low `group_exprs` bits are the semantic bitmask (a set bit means the
  corresponding grouping expression is NULL for this grouping set).
- The bits above position `group_exprs` encode a duplicate ordinal that
  distinguishes multiple occurrences of the same grouping set pattern.

`max_ordinal` is the highest ordinal value that will appear (0 when there
are no duplicate grouping sets).  The type is chosen to be the smallest
unsigned integer that can represent both parts.

<a id="op-2dabfe6c837dc9b26f0162f4"></a>
## hash

`function` · `datafusion_expr::logical_plan::plan::Aggregate::hash` · datafusion-expr 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::Aggregate", "path": "Aggregate"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3886, 39], "end": [3886, 43], "filename": "src/logical_plan/plan.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/logical_plan/plan.rs:3886`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c9c6b15131341d663b599ddc"></a>
## input

`struct_field` · `datafusion_expr::logical_plan::plan::Aggregate::input` · datafusion-expr 55.1.0

```rust
input: std::sync::Arc<LogicalPlan>
```

Source: `src/logical_plan/plan.rs:3891`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

The incoming logical plan

<a id="op-11b4e0fdcbd0dccfca26b29d"></a>
## partial_cmp

`function` · `datafusion_expr::logical_plan::plan::Aggregate::partial_cmp` · datafusion-expr 55.1.0

```rust
fn partial_cmp(&self, other: &Self) -> Option<Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::Aggregate", "path": "Aggregate"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4077, 1], "end": [4091, 2], "filename": "src/logical_plan/plan.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/logical_plan/plan.rs:4078`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-778f40a7a2e9ec1dfd9e41b8"></a>
## schema

`struct_field` · `datafusion_expr::logical_plan::plan::Aggregate::schema` · datafusion-expr 55.1.0

```rust
schema: datafusion_common::DFSchemaRef
```

Source: `src/logical_plan/plan.rs:3899`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

The schema description of the aggregate output

<a id="op-6d187e90f76b92ce140c49d7"></a>
## try_new

`function` · `datafusion_expr::logical_plan::plan::Aggregate::try_new` · datafusion-expr 55.1.0

```rust
fn try_new(input: Arc<LogicalPlan>, group_expr: Vec<Expr>, aggr_expr: Vec<Expr>) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::Aggregate", "path": "Aggregate"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3902, 1], "end": [4074, 2], "filename": "src/logical_plan/plan.rs"}, "trait": null, "trait_path": null}`

Source: `src/logical_plan/plan.rs:3904`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Create a new aggregate operator.

<a id="op-9f33ba7686de538d93574101"></a>
## try_new_with_schema

`function` · `datafusion_expr::logical_plan::plan::Aggregate::try_new_with_schema` · datafusion-expr 55.1.0

```rust
fn try_new_with_schema(input: Arc<LogicalPlan>, group_expr: Vec<Expr>, aggr_expr: Vec<Expr>, schema: DFSchemaRef) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::Aggregate", "path": "Aggregate"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3902, 1], "end": [4074, 2], "filename": "src/logical_plan/plan.rs"}, "trait": null, "trait_path": null}`

Source: `src/logical_plan/plan.rs:3954`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Create a new aggregate operator using the provided schema to avoid the overhead of
building the schema again when the schema is already known.

This method should only be called when you are absolutely sure that the schema being
provided is correct for the aggregate. If in doubt, call [try_new](Self::try_new) instead.
