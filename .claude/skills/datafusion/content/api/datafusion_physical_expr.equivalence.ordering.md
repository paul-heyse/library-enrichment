# `datafusion_physical_expr::equivalence::ordering`

Crate `datafusion-physical-expr` · 1 public items · structured records in [`model/datafusion_physical_expr.equivalence.ordering.json`](../model/datafusion_physical_expr.equivalence.ordering.json)

## OrderingEquivalenceClass

`struct` · `datafusion_physical_expr::equivalence::ordering::OrderingEquivalenceClass`

Also reachable as `datafusion_physical_expr::equivalence::OrderingEquivalenceClass`

```rust
struct OrderingEquivalenceClass
```

**Implements**: `core::convert::From`, `core::fmt::Display`, `core::iter::traits::collect::IntoIterator`, `core::ops::deref::Deref`

**Derives**: Clone, Debug, Default, Eq, PartialEq, StructuralPartialEq

**Methods** (10)

```rust
fn add_offset(&mut self, offset: isize) -> Result<()>
fn add_orderings(&mut self, sort_exprs: impl IntoIterator<Item = impl IntoIterator<Item = PhysicalSortExpr>>)
fn clear(&mut self)
fn extend(&mut self, orderings: impl IntoIterator<Item = LexOrdering>)
fn get_options(&self, expr: &Arc<dyn PhysicalExpr>) -> Option<SortOptions>
fn is_expr_partial_const(&self, expr: &Arc<dyn PhysicalExpr>) -> bool
fn join_suffix(self, other: &Self) -> Self
fn new(orderings: impl IntoIterator<Item = impl IntoIterator<Item = PhysicalSortExpr>>) -> Self
fn output_ordering(&self) -> Option<LexOrdering>
fn with_new_schema(self, schema: &SchemaRef) -> Result<Self>
```

**via `core::convert::From`**

```rust
fn from(orderings: Vec<LexOrdering>) -> Self
fn from(eq_properties: EquivalenceProperties) -> Self
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `core::iter::traits::collect::IntoIterator`**

```rust
fn into_iter(self) -> Self::IntoIter
```

**via `core::ops::deref::Deref`**

```rust
fn deref(&self) -> &Self::Target
```

[Full member, field, variant and typed contracts](../operations/datafusion_physical_expr.equivalence.ordering.OrderingEquivalenceClass.md).


An `OrderingEquivalenceClass` keeps track of distinct alternative orderings
than can describe a table. For example, consider the following table:

```text
┌───┬───┬───┬───┐
│ a │ b │ c │ d │
├───┼───┼───┼───┤
│ 1 │ 4 │ 3 │ 1 │
│ 2 │ 3 │ 3 │ 2 │
│ 3 │ 1 │ 2 │ 2 │
│ 3 │ 2 │ 1 │ 3 │
└───┴───┴───┴───┘
```

Here, both `[a ASC, b ASC]` and `[c DESC, d ASC]` describe the table
ordering. In this case, we say that these orderings are equivalent.

An `OrderingEquivalenceClass` is a set of such equivalent orderings, which
is represented by a vector of `LexOrdering`s. The set does not store any
redundant information by enforcing the invariant that no suffix of an
ordering in the equivalence class is a prefix of another ordering in the
equivalence class. The set can be empty, which means that there are no
orderings that describe the table.

---
