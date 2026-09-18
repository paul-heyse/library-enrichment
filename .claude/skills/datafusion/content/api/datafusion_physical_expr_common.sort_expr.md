# `datafusion_physical_expr_common::sort_expr`

Crate `datafusion-physical-expr-common` · 12 public items · structured records in [`model/datafusion_physical_expr_common.sort_expr.json`](../model/datafusion_physical_expr_common.sort_expr.json)

## OrderingRequirements

`enum` · `datafusion_physical_expr_common::sort_expr::OrderingRequirements`

Also reachable as `datafusion::physical_expr::OrderingRequirements`, `datafusion_physical_expr::OrderingRequirements`

```rust
enum OrderingRequirements
```

**Variants**: `Hard`, `Soft`

**Implements**: `core::convert::From`, `core::ops::deref::Deref`, `core::ops::deref::DerefMut`

**Derives**: Clone, Debug, PartialEq, StructuralPartialEq

**Methods** (7)

```rust
fn add_alternative(&mut self, requirement: LexRequirement)
fn first(&self) -> &LexRequirement
fn into_alternatives(self) -> (Vec<LexRequirement>, bool)
fn into_single(self) -> LexRequirement
fn new(requirement: LexRequirement) -> Self
fn new_alternatives(alternatives: impl IntoIterator<Item = LexRequirement>, soft: bool) -> Option<Self>
fn new_soft(requirement: LexRequirement) -> Self
```

**via `core::convert::From`**

```rust
fn from(requirement: LexRequirement) -> Self
fn from(ordering: LexOrdering) -> Self
```

**via `core::ops::deref::Deref`**

```rust
fn deref(&self) -> &Self::Target
```

**via `core::ops::deref::DerefMut`**

```rust
fn deref_mut(&mut self) -> &mut Self::Target
```

[Full member, field, variant and typed contracts](../operations/datafusion_physical_expr_common.sort_expr.OrderingRequirements.md).


Represents a plan's input ordering requirements. Vector elements represent
alternative ordering requirements in the order of preference. The list of
alternatives can be either hard or soft, depending on whether the operator
can work without an input ordering.

# Invariants

The following always hold true for a `OrderingRequirements`:

1. It is non-degenerate, meaning it contains at least one ordering. The
   absence of an input ordering requirement is represented by a `None` value
   in `ExecutionPlan` APIs, which return an `Option<OrderingRequirements>`.

---

## format_physical_sort_requirement_list

`function` · `datafusion_physical_expr_common::sort_expr::format_physical_sort_requirement_list`

```rust
fn format_physical_sort_requirement_list(exprs: &[PhysicalSortRequirement]) -> impl Display + '_
```

[Full member, field, variant and typed contracts](../operations/datafusion_physical_expr_common.sort_expr.format_physical_sort_requirement_list.md).


Writes a list of [`PhysicalSortRequirement`]s to a `std::fmt::Formatter`.

Example output: `[a + 1, b]`

---

## is_reversed_sort_options

`function` · `datafusion_physical_expr_common::sort_expr::is_reversed_sort_options`

```rust
fn is_reversed_sort_options(lhs: &arrow::compute::kernels::sort::SortOptions, rhs: &arrow::compute::kernels::sort::SortOptions) -> bool
```

[Full member, field, variant and typed contracts](../operations/datafusion_physical_expr_common.sort_expr.is_reversed_sort_options.md).


Check if two SortOptions represent reversed orderings.

Returns `true` if both `descending` and `nulls_first` are opposite.

# Example
```
use arrow::compute::SortOptions;
# use datafusion_physical_expr_common::sort_expr::is_reversed_sort_options;

let asc_nulls_last = SortOptions {
    descending: false,
    nulls_first: false,
};
let desc_nulls_first = SortOptions {
    descending: true,
    nulls_first: true,
};

assert!(is_reversed_sort_options(&asc_nulls_last, &desc_nulls_first));
assert!(is_reversed_sort_options(&desc_nulls_first, &asc_nulls_last));
```

---

## optional_ordering_try_from_proto

`function` · `datafusion_physical_expr_common::sort_expr::optional_ordering_try_from_proto`

```rust
fn optional_ordering_try_from_proto(nodes: &[datafusion_proto_models::protobuf::PhysicalSortExprNode], ctx: &physical_expr::proto_decode::PhysicalExprDecodeCtx<'_>) -> datafusion_common::Result<Option<LexOrdering>>
```

[Full member, field, variant and typed contracts](../operations/datafusion_physical_expr_common.sort_expr.optional_ordering_try_from_proto.md).


Counterpart of [`optional_ordering_try_to_proto`]: an empty list decodes
as `None`.

---

## optional_ordering_try_to_proto

`function` · `datafusion_physical_expr_common::sort_expr::optional_ordering_try_to_proto`

```rust
fn optional_ordering_try_to_proto(ordering: Option<&LexOrdering>, ctx: &physical_expr::proto_encode::PhysicalExprEncodeCtx<'_>) -> datafusion_common::Result<Vec<datafusion_proto_models::protobuf::PhysicalSortExprNode>>
```

[Full member, field, variant and typed contracts](../operations/datafusion_physical_expr_common.sort_expr.optional_ordering_try_to_proto.md).


Serialize an optional [`LexOrdering`], encoding `None` as an empty list.

---

## options_compatible

`function` · `datafusion_physical_expr_common::sort_expr::options_compatible`

```rust
fn options_compatible(options_lhs: &arrow::compute::kernels::sort::SortOptions, options_rhs: &arrow::compute::kernels::sort::SortOptions, nullable: bool) -> bool
```

[Full member, field, variant and typed contracts](../operations/datafusion_physical_expr_common.sort_expr.options_compatible.md).


Returns whether the given two [`SortOptions`] are compatible. Here,
compatibility means that they are either exactly equal, or they differ only
in whether NULL values come in first/last, which is immaterial because the
column in question is not nullable (specified by the `nullable` parameter).

---

## sort_exprs_try_from_proto

`function` · `datafusion_physical_expr_common::sort_expr::sort_exprs_try_from_proto`

```rust
fn sort_exprs_try_from_proto(nodes: &[datafusion_proto_models::protobuf::PhysicalSortExprNode], ctx: &physical_expr::proto_decode::PhysicalExprDecodeCtx<'_>) -> datafusion_common::Result<Vec<PhysicalSortExpr>>
```

[Full member, field, variant and typed contracts](../operations/datafusion_physical_expr_common.sort_expr.sort_exprs_try_from_proto.md).


Reconstruct a sequence of sort expressions from the flat
[`PhysicalSortExprNode`] list, the counterpart of
[`sort_exprs_try_to_proto`].

Returns the expressions rather than a [`LexOrdering`] or a
[`LexRequirement`], because callers differ in what an empty list means:
`LexOrdering::new` / `LexRequirement::new` return `None` for it, which is
"no ordering declared" for a scan and an error for an operator that requires
one. Callers with the former convention can use
[`optional_ordering_try_from_proto`] instead.

[`PhysicalSortExprNode`]: datafusion_proto_models::protobuf::PhysicalSortExprNode

---

## sort_exprs_try_to_proto

`function` · `datafusion_physical_expr_common::sort_expr::sort_exprs_try_to_proto`

```rust
fn sort_exprs_try_to_proto<E: std::borrow::Borrow<PhysicalSortExpr>>(exprs: impl IntoIterator<Item = E>, ctx: &physical_expr::proto_encode::PhysicalExprEncodeCtx<'_>) -> datafusion_common::Result<Vec<datafusion_proto_models::protobuf::PhysicalSortExprNode>>
```

[Full member, field, variant and typed contracts](../operations/datafusion_physical_expr_common.sort_expr.sort_exprs_try_to_proto.md).


Serialize a sequence of sort expressions into the flat
[`PhysicalSortExprNode`] list the wire format uses for an ordering.

Accepts anything that yields [`PhysicalSortExpr`]s by value or by reference,
so a [`LexOrdering`], a `&[PhysicalSortExpr]`, or a [`LexRequirement`]
mapped through [`PhysicalSortExpr::from`] all work:

```ignore
let nodes = sort_exprs_try_to_proto(ordering.iter(), ctx)?;
let nodes = sort_exprs_try_to_proto(
    requirement.iter().map(|req| PhysicalSortExpr::from(req.clone())),
    ctx,
)?;
```

The `PhysicalSortExprNodeCollection` message some plans use is just this
list in a wrapper, so those callers wrap the result themselves rather than
this function guessing which shape they mean.

[`PhysicalSortExprNode`]: datafusion_proto_models::protobuf::PhysicalSortExprNode

---

## LexOrdering

`struct` · `datafusion_physical_expr_common::sort_expr::LexOrdering`

Also reachable as `datafusion::physical_expr::LexOrdering`, `datafusion_physical_expr::LexOrdering`

```rust
struct LexOrdering
```

**Implements**: `core::convert::From`, `core::fmt::Display`, `core::iter::traits::collect::IntoIterator`, `core::ops::deref::Deref`

**Derives**: Clone, Debug, Eq, PartialEq, PartialOrd

**Methods** (8)

```rust
fn capacity(&self) -> usize
fn extend(&mut self, sort_exprs: impl IntoIterator<Item = PhysicalSortExpr>)
fn first(&self) -> &PhysicalSortExpr
fn get_sort_options(&self, expr: &dyn PhysicalExpr) -> Option<SortOptions>
fn is_reverse(&self, other: &LexOrdering) -> bool
fn new(exprs: impl IntoIterator<Item = PhysicalSortExpr>) -> Option<Self>
fn push(&mut self, sort_expr: PhysicalSortExpr)
fn truncate(&mut self, len: usize) -> bool
```

**via `core::convert::From`**

```rust
fn from(value: [PhysicalSortExpr; N]) -> Self
fn from(value: LexRequirement) -> Self
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result
```

**via `core::iter::traits::collect::IntoIterator`**

```rust
fn into_iter(self) -> Self::IntoIter
```

**via `core::ops::deref::Deref`**

```rust
fn deref(&self) -> &Self::Target
```

[Full member, field, variant and typed contracts](../operations/datafusion_physical_expr_common.sort_expr.LexOrdering.md).


This object represents a lexicographical ordering and contains a vector
of `PhysicalSortExpr` objects.

For example, a `vec![a ASC, b DESC]` represents a lexicographical ordering
that first sorts by column `a` in ascending order, then by column `b` in
descending order.

# Invariants

The following always hold true for a `LexOrdering`:

1. It is non-degenerate, meaning it contains at least one element.
2. It is duplicate-free, meaning it does not contain multiple entries for
   the same column.

---

## LexRequirement

`struct` · `datafusion_physical_expr_common::sort_expr::LexRequirement`

Also reachable as `datafusion::physical_expr::LexRequirement`, `datafusion_physical_expr::LexRequirement`

```rust
struct LexRequirement
```

**Implements**: `core::convert::From`, `core::iter::traits::collect::IntoIterator`, `core::ops::deref::Deref`

**Derives**: Clone, Debug, PartialEq, StructuralPartialEq

**Methods** (2)

```rust
fn first(&self) -> &PhysicalSortRequirement
fn new(reqs: impl IntoIterator<Item = PhysicalSortRequirement>) -> Option<Self>
```

**via `core::convert::From`**

```rust
fn from(value: [PhysicalSortRequirement; N]) -> Self
fn from(value: LexOrdering) -> Self
```

**via `core::iter::traits::collect::IntoIterator`**

```rust
fn into_iter(self) -> Self::IntoIter
```

**via `core::ops::deref::Deref`**

```rust
fn deref(&self) -> &Self::Target
```

[Full member, field, variant and typed contracts](../operations/datafusion_physical_expr_common.sort_expr.LexRequirement.md).


This object represents a lexicographical ordering requirement and contains
a vector of `PhysicalSortRequirement` objects.

For example, a `vec![a Some(ASC), b None]` represents a lexicographical
requirement that firsts imposes an ordering by column `a` in ascending
order, then by column `b` in *any* (ascending or descending) order. The
ordering is non-degenerate, meaning it contains at least one element, and
it is duplicate-free, meaning it does not contain multiple entries for the
same column.

Note that a `LexRequirement` need not enforce the uniqueness of its sort
expressions after construction like a `LexOrdering` does, because it provides
no mutation methods. If such methods become necessary, we will need to
enforce uniqueness like the latter object.

---

## PhysicalSortExpr

`struct` · `datafusion_physical_expr_common::sort_expr::PhysicalSortExpr`

Also reachable as `datafusion::physical_expr::PhysicalSortExpr`, `datafusion_physical_expr::PhysicalSortExpr`, `datafusion_physical_expr::expressions::PhysicalSortExpr`, `datafusion_physical_plan::execution_plan::expressions::PhysicalSortExpr`, `datafusion_physical_plan::expressions::PhysicalSortExpr`

```rust
struct PhysicalSortExpr
```

**Fields**: `expr`, `options`

**Implements**: `core::convert::From`, `core::fmt::Display`

**Derives**: Clone, Debug, Eq, Hash, PartialEq

**Methods** (13)

```rust
fn asc(self) -> Self
fn desc(self) -> Self
fn evaluate_to_sort_column(&self, batch: &RecordBatch) -> Result<SortColumn>
fn fmt_sql(&self, f: &mut Formatter<'_>) -> fmt::Result
fn new(expr: Arc<dyn PhysicalExpr>, options: SortOptions) -> Self
fn new_default(expr: Arc<dyn PhysicalExpr>) -> Self
fn nulls_first(self) -> Self
fn nulls_last(self) -> Self
fn reverse(&self) -> Self
fn satisfy(&self, requirement: &PhysicalSortRequirement, schema: &Schema) -> bool
fn satisfy_expr(&self, sort_expr: &Self, schema: &Schema) -> bool
fn try_from_proto(node: &datafusion_proto_models::protobuf::PhysicalSortExprNode, ctx: &physical_expr::proto_decode::PhysicalExprDecodeCtx<'_>) -> Result<Self>
fn try_to_proto(&self, ctx: &physical_expr::proto_encode::PhysicalExprEncodeCtx<'_>) -> Result<datafusion_proto_models::protobuf::PhysicalSortExprNode>
```

**via `core::convert::From`**

```rust
fn from(value: PhysicalSortRequirement) -> Self
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result
```

[Full member, field, variant and typed contracts](../operations/datafusion_physical_expr_common.sort_expr.PhysicalSortExpr.md).


Represents Sort operation for a column in a RecordBatch

Example:
```
# use std::any::Any;
# use std::collections::HashMap;
# use std::fmt::{Display, Formatter};
# use std::hash::Hasher;
# use std::sync::Arc;
# use arrow::array::RecordBatch;
# use datafusion_common::Result;
# use arrow::compute::SortOptions;
# use arrow::datatypes::{DataType, Field, FieldRef, Schema};
# use datafusion_expr_common::columnar_value::ColumnarValue;
# use datafusion_physical_expr_common::physical_expr::PhysicalExpr;
# use datafusion_physical_expr_common::sort_expr::PhysicalSortExpr;
# // this crate doesn't have a physical expression implementation
# // so make a really simple one
# #[derive(Clone, Debug, PartialEq, Eq, Hash)]
# struct MyPhysicalExpr;
# impl PhysicalExpr for MyPhysicalExpr {
#  fn data_type(&self, input_schema: &Schema) -> Result<DataType> {todo!()}
#  fn nullable(&self, input_schema: &Schema) -> Result<bool> {todo!() }
#  fn evaluate(&self, batch: &RecordBatch) -> Result<ColumnarValue> {todo!() }
#  fn return_field(&self, input_schema: &Schema) -> Result<FieldRef> { unimplemented!() }
#  fn children(&self) -> Vec<&Arc<dyn PhysicalExpr>> {todo!()}
#  fn with_new_children(self: Arc<Self>, children: Vec<Arc<dyn PhysicalExpr>>) -> Result<Arc<dyn PhysicalExpr>> {todo!()}
# fn fmt_sql(&self, f: &mut Formatter<'_>) -> std::fmt::Result { todo!() }
# }
# impl Display for MyPhysicalExpr {
#    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result { write!(f, "a") }
# }
# fn col(name: &str) -> Arc<dyn PhysicalExpr> { Arc::new(MyPhysicalExpr) }
// Sort by a ASC
let options = SortOptions::default();
let sort_expr = PhysicalSortExpr::new(col("a"), options);
assert_eq!(sort_expr.to_string(), "a ASC");

// Sort by a DESC NULLS LAST
let sort_expr = PhysicalSortExpr::new_default(col("a"))
  .desc()
  .nulls_last();
assert_eq!(sort_expr.to_string(), "a DESC NULLS LAST");
```

---

## PhysicalSortRequirement

`struct` · `datafusion_physical_expr_common::sort_expr::PhysicalSortRequirement`

Also reachable as `datafusion::physical_expr::PhysicalSortRequirement`, `datafusion_physical_expr::PhysicalSortRequirement`

```rust
struct PhysicalSortRequirement
```

**Fields**: `expr`, `options`

**Implements**: `core::convert::From`, `core::fmt::Display`

**Derives**: Clone, Debug, PartialEq

**Methods** (2)

```rust
fn compatible(&self, other: &Self) -> bool
fn new(expr: Arc<dyn PhysicalExpr>, options: Option<SortOptions>) -> Self
```

**via `core::convert::From`**

```rust
fn from(value: PhysicalSortExpr) -> Self
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result
```

[Full member, field, variant and typed contracts](../operations/datafusion_physical_expr_common.sort_expr.PhysicalSortRequirement.md).


Represents sort requirement associated with a plan

If the requirement includes [`SortOptions`] then both the
expression *and* the sort options must match.

If the requirement does not include [`SortOptions`]) then only the
expressions must match.

# Examples

With sort options (`A`, `DESC NULLS FIRST`):
* `ORDER BY A DESC NULLS FIRST` matches
* `ORDER BY A ASC  NULLS FIRST` does not match (`ASC` vs `DESC`)
* `ORDER BY B DESC NULLS FIRST` does not match (different expr)

Without sort options (`A`, None):
* `ORDER BY A DESC NULLS FIRST` matches
* `ORDER BY A ASC  NULLS FIRST` matches (`ASC` and `NULL` options ignored)
* `ORDER BY B DESC NULLS FIRST` does not match  (different expr)

---
