# `datafusion_expr_common::sort_properties`

Crate `datafusion-expr-common` · 2 public items · structured records in [`model/datafusion_expr_common.sort_properties.json`](../model/datafusion_expr_common.sort_properties.json)

## SortProperties

`enum` · `datafusion_expr_common::sort_properties::SortProperties`

Also reachable as `datafusion_expr::sort_properties::SortProperties`

```rust
enum SortProperties
```

**Variants**: `Ordered`, `Unordered`, `Singleton`

**Implements**: `core::convert::From`, `core::ops::arith::Neg`

**Derives**: Clone, Copy, Debug, Default, PartialEq, StructuralPartialEq

**Methods** (4)

```rust
fn add(&self, rhs: &Self) -> Self
fn and_or(&self, rhs: &Self) -> Self
fn gt_or_gteq(&self, rhs: &Self) -> Self
fn sub(&self, rhs: &Self) -> Self
```

**via `core::ops::arith::Neg`**

```rust
fn neg(self) -> Self::Output
```

[Full member, field, variant and typed contracts](../operations/datafusion_expr_common.sort_properties.SortProperties.md).


To propagate [`SortOptions`] across the `PhysicalExpr`, it is insufficient
to simply use `Option<SortOptions>`: There must be a differentiation between
unordered columns and literal values, since literals may not break the ordering
when they are used as a child of some binary expression when the other child has
some ordering. On the other hand, unordered columns cannot maintain ordering when
they take part in such operations.

Example: ((a_ordered + b_unordered) + c_ordered) expression cannot end up with
sorted data; however the ((a_ordered + 999) + c_ordered) expression can. Therefore,
we need two different variants for literals and unordered columns as literals are
often more ordering-friendly under most mathematical operations.

---

## ExprProperties

`struct` · `datafusion_expr_common::sort_properties::ExprProperties`

Also reachable as `datafusion_expr::sort_properties::ExprProperties`

```rust
struct ExprProperties
```

**Fields**: `sort_properties`, `range`, `preserves_lex_ordering`, `strictly_order_preserving`

**Implements**: `core::convert::TryFrom`

**Derives**: Clone, Debug

**Methods** (5)

```rust
fn new_unknown() -> Self
fn with_order(self, order: SortProperties) -> Self
fn with_preserves_lex_ordering(self, preserves_lex_ordering: bool) -> Self
fn with_range(self, range: Interval) -> Self
fn with_strictly_order_preserving(self, strictly_order_preserving: bool) -> Self
```

[Full member, field, variant and typed contracts](../operations/datafusion_expr_common.sort_properties.ExprProperties.md).


Represents the properties of a `PhysicalExpr`, including its sorting,
range, and whether it preserves lexicographical ordering.

---
