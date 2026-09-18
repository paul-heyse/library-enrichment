# `datafusion_physical_expr::utils::guarantee`

Crate `datafusion-physical-expr` · 2 public items · structured records in [`model/datafusion_physical_expr.utils.guarantee.json`](../model/datafusion_physical_expr.utils.guarantee.json)

## Guarantee

`enum` · `datafusion_physical_expr::utils::guarantee::Guarantee`

Also reachable as `datafusion_physical_expr::utils::Guarantee`

```rust
enum Guarantee
```

**Variants**: `In`, `NotIn`

**Derives**: Clone, Copy, Debug, Eq, Hash, PartialEq, StructuralPartialEq

[Full member, field, variant and typed contracts](../operations/datafusion_physical_expr.utils.guarantee.Guarantee.md).


What is guaranteed about the values for a [`LiteralGuarantee`]?

---

## LiteralGuarantee

`struct` · `datafusion_physical_expr::utils::guarantee::LiteralGuarantee`

Also reachable as `datafusion_physical_expr::utils::LiteralGuarantee`

```rust
struct LiteralGuarantee
```

**Fields**: `column`, `guarantee`, `literals`

**Implements**: `core::fmt::Display`

**Derives**: Clone, Debug, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn analyze(expr: &Arc<dyn PhysicalExpr>) -> Vec<LiteralGuarantee>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result
```

[Full member, field, variant and typed contracts](../operations/datafusion_physical_expr.utils.guarantee.LiteralGuarantee.md).


Represents a guarantee that must be true for a boolean expression to
evaluate to `true`.

The guarantee takes the form of a column and a set of literal (constant)
[`ScalarValue`]s. For the expression to evaluate to `true`, the column *must
satisfy* the guarantee(s).

To satisfy the guarantee, depending on [`Guarantee`], the values in the
column must either:

1. be ONLY one of that set
2. NOT be ANY of that set

# Uses `LiteralGuarantee`s

`LiteralGuarantee`s can be used to simplify filter expressions and skip data
files (e.g. row groups in parquet files) by proving expressions can not
possibly evaluate to `true`. For example, if we have a guarantee that `a`
must be in (`1`) for a filter to evaluate to `true`, then we can skip any
partition where we know that `a` never has the value of `1`.

**Important**: If a `LiteralGuarantee` is not satisfied, the relevant
expression is *guaranteed* to evaluate to `false` or `null`. **However**,
the opposite does not hold. Even if all `LiteralGuarantee`s are satisfied,
that does **not** guarantee that the predicate will actually evaluate to
`true`: it may still evaluate to `true`, `false` or `null`.

# Creating `LiteralGuarantee`s

Use [`LiteralGuarantee::analyze`] to extract literal guarantees from a
filter predicate.

# Details
A guarantee can be one of two forms:

1. The column must be one the values for the predicate to be `true`. If the
   column takes on any other value, the predicate can not evaluate to `true`.
   For example,
   `(a = 1)`, `(a = 1 OR a = 2)` or `a IN (1, 2, 3)`

2. The column must NOT be one of the values for the predicate to be `true`.
   If the column can ONLY take one of these values, the predicate can not
   evaluate to `true`. For example,
   `(a != 1)`, `(a != 1 AND a != 2)` or `a NOT IN (1, 2, 3)`

---
