# `datafusion_physical_expr::equivalence::properties::EquivalenceProperties`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_expr.equivalence.properties.EquivalenceProperties.json).

<a id="op-eeda1c3d472a48e6007359b5"></a>
## EquivalenceProperties

`struct` · `datafusion_physical_expr::equivalence::properties::EquivalenceProperties` · datafusion-physical-expr 55.1.0

```rust
struct EquivalenceProperties
```

Source: `src/equivalence/properties/mod.rs:136`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

`EquivalenceProperties` stores information about the output of a plan node
that can be used to optimize the plan. Currently, it keeps track of:
- Sort expressions (orderings),
- Equivalent expressions; i.e. expressions known to have the same value.
- Constants expressions; i.e. expressions known to contain a single constant
  value.

Please see the [Using Ordering for Better Plans] blog for more details.

[Using Ordering for Better Plans]: https://datafusion.apache.org/blog/2025/03/11/ordering-analysis/

# Example equivalent sort expressions

Consider table below:

```text
┌-------┐
| a | b |
|---|---|
| 1 | 9 |
| 2 | 8 |
| 3 | 7 |
| 5 | 5 |
└---┴---┘
```

In this case, both `a ASC` and `b DESC` can describe the table ordering.
`EquivalenceProperties` tracks these different valid sort expressions and
treat `a ASC` and `b DESC` on an equal footing. For example, if the query
specifies the output sorted by EITHER `a ASC` or `b DESC`, the sort can be
avoided.

# Example equivalent expressions

Similarly, consider the table below:

```text
┌-------┐
| a | b |
|---|---|
| 1 | 1 |
| 2 | 2 |
| 3 | 3 |
| 5 | 5 |
└---┴---┘
```

In this case,  columns `a` and `b` always have the same value. With this
information, Datafusion can optimize various operations. For example, if
the partition requirement is `Hash(a)` and output partitioning is
`Hash(b)`, then DataFusion avoids repartitioning the data as the existing
partitioning satisfies the requirement.

# Code Example
```
# use std::sync::Arc;
# use arrow::datatypes::{Schema, Field, DataType, SchemaRef};
# use datafusion_physical_expr::{ConstExpr, EquivalenceProperties};
# use datafusion_physical_expr::expressions::col;
use datafusion_physical_expr_common::sort_expr::{LexOrdering, PhysicalSortExpr};
# let schema: SchemaRef = Arc::new(Schema::new(vec![
#   Field::new("a", DataType::Int32, false),
#   Field::new("b", DataType::Int32, false),
#   Field::new("c", DataType::Int32, false),
# ]));
# let col_a = col("a", &schema).unwrap();
# let col_b = col("b", &schema).unwrap();
# let col_c = col("c", &schema).unwrap();
// This object represents data that is sorted by a ASC, c DESC
// with a single constant value of b
let mut eq_properties = EquivalenceProperties::new(schema);
eq_properties.add_constants(vec![ConstExpr::from(col_b)]);
eq_properties.add_ordering([
    PhysicalSortExpr::new_default(col_a).asc(),
    PhysicalSortExpr::new_default(col_c).desc(),
]);

assert_eq!(
    eq_properties.to_string(),
    "order: [[a@0 ASC, c@2 DESC]], eq: [{members: [b@1], constant: (heterogeneous)}]"
);
```

<a id="op-29625c1aedfa75a91a736ba4"></a>
## add_constants

`function` · `datafusion_physical_expr::equivalence::properties::EquivalenceProperties::add_constants` · datafusion-physical-expr 55.1.0

```rust
fn add_constants(&mut self, constants: impl IntoIterator<Item = ConstExpr>) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::equivalence::properties::EquivalenceProperties", "path": "EquivalenceProperties"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [197, 1], "end": [1369, 2], "filename": "src/equivalence/properties/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/equivalence/properties/mod.rs:423`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Track/register physical expressions with constant values.

<a id="op-8ac806682d63bfd7e367e3cd"></a>
## add_equal_conditions

`function` · `datafusion_physical_expr::equivalence::properties::EquivalenceProperties::add_equal_conditions` · datafusion-physical-expr 55.1.0

```rust
fn add_equal_conditions(&mut self, left: Arc<dyn PhysicalExpr>, right: Arc<dyn PhysicalExpr>) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::equivalence::properties::EquivalenceProperties", "path": "EquivalenceProperties"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [197, 1], "end": [1369, 2], "filename": "src/equivalence/properties/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/equivalence/properties/mod.rs:409`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Adds a new equality condition into the existing equivalence group.
If the given equality defines a new equivalence class, adds this new
equivalence class to the equivalence group.

<a id="op-4e932ab8a8e07008d47b40cc"></a>
## add_equivalence_group

`function` · `datafusion_physical_expr::equivalence::properties::EquivalenceProperties::add_equivalence_group` · datafusion-physical-expr 55.1.0

```rust
fn add_equivalence_group(&mut self, other_eq_group: EquivalenceGroup) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::equivalence::properties::EquivalenceProperties", "path": "EquivalenceProperties"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [197, 1], "end": [1369, 2], "filename": "src/equivalence/properties/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/equivalence/properties/mod.rs:383`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Incorporates the given equivalence group to into the existing
equivalence group within.

<a id="op-bb3609b1b0475e284ee68318"></a>
## add_ordering

`function` · `datafusion_physical_expr::equivalence::properties::EquivalenceProperties::add_ordering` · datafusion-physical-expr 55.1.0

```rust
fn add_ordering(&mut self, ordering: impl IntoIterator<Item = PhysicalSortExpr>)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::equivalence::properties::EquivalenceProperties", "path": "EquivalenceProperties"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [197, 1], "end": [1369, 2], "filename": "src/equivalence/properties/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/equivalence/properties/mod.rs:361`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Adds a single ordering to the existing ordering equivalence class.

<a id="op-dfcaffd6d92e6bb287051436"></a>
## add_orderings

`function` · `datafusion_physical_expr::equivalence::properties::EquivalenceProperties::add_orderings` · datafusion-physical-expr 55.1.0

```rust
fn add_orderings(&mut self, orderings: impl IntoIterator<Item = impl IntoIterator<Item = PhysicalSortExpr>>)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::equivalence::properties::EquivalenceProperties", "path": "EquivalenceProperties"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [197, 1], "end": [1369, 2], "filename": "src/equivalence/properties/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/equivalence/properties/mod.rs:339`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Adds new orderings into the existing ordering equivalence class.

<a id="op-788406d31ed07fd863a7b77a"></a>
## clear_orderings

`function` · `datafusion_physical_expr::equivalence::properties::EquivalenceProperties::clear_orderings` · datafusion-physical-expr 55.1.0

```rust
fn clear_orderings(&mut self)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::equivalence::properties::EquivalenceProperties", "path": "EquivalenceProperties"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [197, 1], "end": [1369, 2], "filename": "src/equivalence/properties/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/equivalence/properties/mod.rs:319`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Clears (empties) the ordering equivalence class within this object.
Call this method when existing orderings are invalidated.

<a id="op-17a7d1d3b38a8da73f93038e"></a>
## clear_per_partition_constants

`function` · `datafusion_physical_expr::equivalence::properties::EquivalenceProperties::clear_per_partition_constants` · datafusion-physical-expr 55.1.0

```rust
fn clear_per_partition_constants(&mut self)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::equivalence::properties::EquivalenceProperties", "path": "EquivalenceProperties"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [197, 1], "end": [1369, 2], "filename": "src/equivalence/properties/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/equivalence/properties/mod.rs:326`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Removes constant expressions that may change across partitions.
This method should be used when merging data from different partitions.

<a id="op-851b5454d691c7473d6aedc1"></a>
## clone

`function` · `datafusion_physical_expr::equivalence::properties::EquivalenceProperties::clone` · datafusion-physical-expr 55.1.0

```rust
fn clone(&self) -> EquivalenceProperties
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::equivalence::properties::EquivalenceProperties", "path": "EquivalenceProperties"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [135, 10], "end": [135, 15], "filename": "src/equivalence/properties/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/equivalence/properties/mod.rs:135`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ad8723c89dd98b1c642da063"></a>
## constants

`function` · `datafusion_physical_expr::equivalence::properties::EquivalenceProperties::constants` · datafusion-physical-expr 55.1.0

```rust
fn constants(&self) -> Vec<ConstExpr>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::equivalence::properties::EquivalenceProperties", "path": "EquivalenceProperties"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [197, 1], "end": [1369, 2], "filename": "src/equivalence/properties/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/equivalence/properties/mod.rs:290`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Returns all the known constants expressions.

<a id="op-1590c691d055b9bdcedf91cb"></a>
## constraints

`function` · `datafusion_physical_expr::equivalence::properties::EquivalenceProperties::constraints` · datafusion-physical-expr 55.1.0

```rust
fn constraints(&self) -> &Constraints
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::equivalence::properties::EquivalenceProperties", "path": "EquivalenceProperties"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [197, 1], "end": [1369, 2], "filename": "src/equivalence/properties/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/equivalence/properties/mod.rs:285`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Returns a reference to the constraints within.

<a id="op-df79021337c56a835cad469f"></a>
## eq_group

`function` · `datafusion_physical_expr::equivalence::properties::EquivalenceProperties::eq_group` · datafusion-physical-expr 55.1.0

```rust
fn eq_group(&self) -> &EquivalenceGroup
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::equivalence::properties::EquivalenceProperties", "path": "EquivalenceProperties"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [197, 1], "end": [1369, 2], "filename": "src/equivalence/properties/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/equivalence/properties/mod.rs:280`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Returns a reference to the equivalence group within.

<a id="op-a7057ac019ee5d0aa5120f17"></a>
## extend

`function` · `datafusion_physical_expr::equivalence::properties::EquivalenceProperties::extend` · datafusion-physical-expr 55.1.0

```rust
fn extend(self, other: Self) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::equivalence::properties::EquivalenceProperties", "path": "EquivalenceProperties"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [197, 1], "end": [1369, 2], "filename": "src/equivalence/properties/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/equivalence/properties/mod.rs:310`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Extends this `EquivalenceProperties` with the `other` object.

<a id="op-d5ecf4371ae6bd436e3cb322"></a>
## extract_common_sort_prefix

`function` · `datafusion_physical_expr::equivalence::properties::EquivalenceProperties::extract_common_sort_prefix` · datafusion-physical-expr 55.1.0

```rust
fn extract_common_sort_prefix(&self, ordering: LexOrdering) -> Result<(Vec<PhysicalSortExpr>, bool)>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::equivalence::properties::EquivalenceProperties", "path": "EquivalenceProperties"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [197, 1], "end": [1369, 2], "filename": "src/equivalence/properties/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/equivalence/properties/mod.rs:729`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Determines the longest normal prefix of `ordering` satisfied by the
existing ordering. Returns that prefix as a new `LexOrdering`, and a
boolean indicating whether all the sort expressions are satisfied.

<a id="op-7afd3ad70522618d1e2dcde9"></a>
## find_longest_permutation

`function` · `datafusion_physical_expr::equivalence::properties::EquivalenceProperties::find_longest_permutation` · datafusion-physical-expr 55.1.0

```rust
fn find_longest_permutation(&self, exprs: &[Arc<dyn PhysicalExpr>]) -> Result<(Vec<PhysicalSortExpr>, Vec<usize>)>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::equivalence::properties::EquivalenceProperties", "path": "EquivalenceProperties"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [197, 1], "end": [1369, 2], "filename": "src/equivalence/properties/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/equivalence/properties/mod.rs:1197`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Returns the longest (potentially partial) permutation satisfying the
existing ordering. For example, if we have the equivalent orderings
`[a ASC, b ASC]` and `[c DESC]`, with `exprs` containing `[c, b, a, d]`,
then this function returns `([a ASC, b ASC, c DESC], [2, 1, 0])`.
This means that the specification `[a ASC, b ASC, c DESC]` is satisfied
by the existing ordering, and `[a, b, c]` resides at indices: `2, 1, 0`
inside the argument `exprs` (respectively). For the mathematical
definition of "partial permutation", see:

<https://en.wikipedia.org/wiki/Permutation#k-permutations_of_n>

<a id="op-63269516cf895c9c7473f1e3"></a>
## fmt

`function` · `datafusion_physical_expr::equivalence::properties::EquivalenceProperties::fmt` · datafusion-physical-expr 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::equivalence::properties::EquivalenceProperties", "path": "EquivalenceProperties"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [135, 17], "end": [135, 22], "filename": "src/equivalence/properties/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/equivalence/properties/mod.rs:135`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-dbcb4c56eeb1984ee396b8f0"></a>
## fmt

`function` · `datafusion_physical_expr::equivalence::properties::EquivalenceProperties::fmt` · datafusion-physical-expr 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::equivalence::properties::EquivalenceProperties", "path": "EquivalenceProperties"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1383, 1], "end": [1399, 2], "filename": "src/equivalence/properties/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/equivalence/properties/mod.rs:1384`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ec7b614c983ab062907a40ae"></a>
## get_expr_properties

`function` · `datafusion_physical_expr::equivalence::properties::EquivalenceProperties::get_expr_properties` · datafusion-physical-expr 55.1.0

```rust
fn get_expr_properties(&self, expr: Arc<dyn PhysicalExpr>) -> ExprProperties
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::equivalence::properties::EquivalenceProperties", "path": "EquivalenceProperties"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [197, 1], "end": [1369, 2], "filename": "src/equivalence/properties/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/equivalence/properties/mod.rs:1295`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Retrieves the properties for a given physical expression.

This function constructs an [`ExprProperties`](../operations/datafusion_expr_common.sort_properties.ExprProperties.md#op-51897a0df15cf12898939b7d) object for the given
expression, which encapsulates information about the expression's
properties, including its [`SortProperties`](../operations/datafusion_expr_common.sort_properties.SortProperties.md#op-a357ca132b9df8290c59f5bf) and [`Interval`](../operations/datafusion_expr_common.interval_arithmetic.Interval.md#op-0e8ed6898e67ba9321c8f53e).

# Parameters

- `expr`: An `Arc<dyn PhysicalExpr>` representing the physical expression
  for which ordering information is sought.

# Returns

Returns an [`ExprProperties`](../operations/datafusion_expr_common.sort_properties.ExprProperties.md#op-51897a0df15cf12898939b7d) object containing the ordering and range
information for the given expression.

<a id="op-5caf2349db299247afb56233"></a>
## is_expr_constant

`function` · `datafusion_physical_expr::equivalence::properties::EquivalenceProperties::is_expr_constant` · datafusion-physical-expr 55.1.0

```rust
fn is_expr_constant(&self, expr: &Arc<dyn PhysicalExpr>) -> Option<AcrossPartitions>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::equivalence::properties::EquivalenceProperties", "path": "EquivalenceProperties"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [197, 1], "end": [1369, 2], "filename": "src/equivalence/properties/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/equivalence/properties/mod.rs:1273`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

This function determines whether the provided expression is constant
based on the known constants. For example, if columns `a` and `b` are
constant, then expressions `a`, `b` and `a + b` will all return `true`
whereas expression `c` will return `false`.

# Parameters

- `expr`: A reference to a `Arc<dyn PhysicalExpr>` representing the
  expression to be checked.

# Returns

Returns a `Some` value if the expression is constant according to
equivalence group, and `None` otherwise. The `Some` variant contains
an `AcrossPartitions` value indicating whether the expression is
constant across partitions, and its actual value (if available).

<a id="op-b2a4db0580796cf9d3ad547a"></a>
## new

`function` · `datafusion_physical_expr::equivalence::properties::EquivalenceProperties::new` · datafusion-physical-expr 55.1.0

```rust
fn new(schema: SchemaRef) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::equivalence::properties::EquivalenceProperties", "path": "EquivalenceProperties"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [197, 1], "end": [1369, 2], "filename": "src/equivalence/properties/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/equivalence/properties/mod.rs:226`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Creates an empty `EquivalenceProperties` object.

<a id="op-2a80b266e24baf3366e4cdef"></a>
## new_with_orderings

`function` · `datafusion_physical_expr::equivalence::properties::EquivalenceProperties::new_with_orderings` · datafusion-physical-expr 55.1.0

```rust
fn new_with_orderings(schema: SchemaRef, orderings: impl IntoIterator<Item = impl IntoIterator<Item = PhysicalSortExpr>>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::equivalence::properties::EquivalenceProperties", "path": "EquivalenceProperties"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [197, 1], "end": [1369, 2], "filename": "src/equivalence/properties/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/equivalence/properties/mod.rs:248`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Creates a new `EquivalenceProperties` object with the given orderings.

<a id="op-47874fbf0c272d76c82d5383"></a>
## normalize_sort_exprs

`function` · `datafusion_physical_expr::equivalence::properties::EquivalenceProperties::normalize_sort_exprs` · datafusion-physical-expr 55.1.0

```rust
fn normalize_sort_exprs(&self, sort_exprs: impl IntoIterator<Item = PhysicalSortExpr>) -> Option<LexOrdering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::equivalence::properties::EquivalenceProperties", "path": "EquivalenceProperties"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [197, 1], "end": [1369, 2], "filename": "src/equivalence/properties/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/equivalence/properties/mod.rs:545`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Normalizes the given sort expressions (i.e. `sort_exprs`) using the
equivalence group within. Returns a `LexOrdering` instance if the
expressions define a proper lexicographical ordering. For more details,
see [`EquivalenceGroup::normalize_sort_exprs`](../operations/datafusion_physical_expr.equivalence.class.EquivalenceGroup.md#op-bd759896b3c2a5466d04ab77).

<a id="op-5c0838ee822bc97bfb45b78c"></a>
## normalize_sort_requirements

`function` · `datafusion_physical_expr::equivalence::properties::EquivalenceProperties::normalize_sort_requirements` · datafusion-physical-expr 55.1.0

```rust
fn normalize_sort_requirements(&self, sort_reqs: impl IntoIterator<Item = PhysicalSortRequirement>) -> Option<LexRequirement>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::equivalence::properties::EquivalenceProperties", "path": "EquivalenceProperties"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [197, 1], "end": [1369, 2], "filename": "src/equivalence/properties/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/equivalence/properties/mod.rs:556`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Normalizes the given sort requirements (i.e. `sort_reqs`) using the
equivalence group within. Returns a `LexRequirement` instance if the
expressions define a proper lexicographical requirement. For more
details, see [`EquivalenceGroup::normalize_sort_exprs`](../operations/datafusion_physical_expr.equivalence.class.EquivalenceGroup.md#op-bd759896b3c2a5466d04ab77).

<a id="op-473cc336c7f43bc1bcd693e5"></a>
## normalized_oeq_class

`function` · `datafusion_physical_expr::equivalence::properties::EquivalenceProperties::normalized_oeq_class` · datafusion-physical-expr 55.1.0

```rust
fn normalized_oeq_class(&self) -> OrderingEquivalenceClass
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::equivalence::properties::EquivalenceProperties", "path": "EquivalenceProperties"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [197, 1], "end": [1369, 2], "filename": "src/equivalence/properties/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/equivalence/properties/mod.rs:397`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Returns the ordering equivalence class within in normal form.
Normalization standardizes expressions according to the equivalence
group within, and removes constants/duplicates.

<a id="op-b182c316d5e52ac88aae109a"></a>
## oeq_class

`function` · `datafusion_physical_expr::equivalence::properties::EquivalenceProperties::oeq_class` · datafusion-physical-expr 55.1.0

```rust
fn oeq_class(&self) -> &OrderingEquivalenceClass
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::equivalence::properties::EquivalenceProperties", "path": "EquivalenceProperties"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [197, 1], "end": [1369, 2], "filename": "src/equivalence/properties/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/equivalence/properties/mod.rs:275`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Returns a reference to the ordering equivalence class within.

<a id="op-a7e115e118d8b85068f0a1ee"></a>
## ordering_satisfy

`function` · `datafusion_physical_expr::equivalence::properties::EquivalenceProperties::ordering_satisfy` · datafusion-physical-expr 55.1.0

```rust
fn ordering_satisfy(&self, given: impl IntoIterator<Item = PhysicalSortExpr>) -> Result<bool>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::equivalence::properties::EquivalenceProperties", "path": "EquivalenceProperties"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [197, 1], "end": [1369, 2], "filename": "src/equivalence/properties/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/equivalence/properties/mod.rs:566`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Iteratively checks whether the given ordering is satisfied by any of
the existing orderings. See [`Self::ordering_satisfy_requirement`](../operations/datafusion_physical_expr.equivalence.properties.EquivalenceProperties.md#op-4499009ac7107a41b6f5af76) for
more details and examples.

<a id="op-4499009ac7107a41b6f5af76"></a>
## ordering_satisfy_requirement

`function` · `datafusion_physical_expr::equivalence::properties::EquivalenceProperties::ordering_satisfy_requirement` · datafusion-physical-expr 55.1.0

```rust
fn ordering_satisfy_requirement(&self, given: impl IntoIterator<Item = PhysicalSortRequirement>) -> Result<bool>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::equivalence::properties::EquivalenceProperties", "path": "EquivalenceProperties"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [197, 1], "end": [1369, 2], "filename": "src/equivalence/properties/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/equivalence/properties/mod.rs:604`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Iteratively checks whether the given sort requirement is satisfied by
any of the existing orderings.

### Example Scenarios

In these scenarios, assume that all expressions share the same sort
properties.

#### Case 1: Sort Requirement `[a, c]`

**Existing orderings:** `[[a, b, c], [a, d]]`, **constants:** `[]`
1. The function first checks the leading requirement `a`, which is
   satisfied by `[a, b, c].first()`.
2. `a` is added as a constant for the next iteration.
3. Normal orderings become `[[b, c], [d]]`.
4. The function fails for `c` in the second iteration, as neither
   `[b, c]` nor `[d]` satisfies `c`.

#### Case 2: Sort Requirement `[a, d]`

**Existing orderings:** `[[a, b, c], [a, d]]`, **constants:** `[]`
1. The function first checks the leading requirement `a`, which is
   satisfied by `[a, b, c].first()`.
2. `a` is added as a constant for the next iteration.
3. Normal orderings become `[[b, c], [d]]`.
4. The function returns `true` as `[d]` satisfies `d`.

<a id="op-718b40888cffa8127199c070"></a>
## output_ordering

`function` · `datafusion_physical_expr::equivalence::properties::EquivalenceProperties::output_ordering` · datafusion-physical-expr 55.1.0

```rust
fn output_ordering(&self) -> Option<LexOrdering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::equivalence::properties::EquivalenceProperties", "path": "EquivalenceProperties"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [197, 1], "end": [1369, 2], "filename": "src/equivalence/properties/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/equivalence/properties/mod.rs:304`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Returns the output ordering of the properties.

<a id="op-2c842fa92daa73f2acfd7315"></a>
## project

`function` · `datafusion_physical_expr::equivalence::properties::EquivalenceProperties::project` · datafusion-physical-expr 55.1.0

```rust
fn project(&self, mapping: &ProjectionMapping, output_schema: SchemaRef) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::equivalence::properties::EquivalenceProperties", "path": "EquivalenceProperties"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [197, 1], "end": [1369, 2], "filename": "src/equivalence/properties/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/equivalence/properties/mod.rs:1170`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Projects the equivalences within according to `mapping` and
`output_schema`.

<a id="op-71b33a5aa4df88734d9ca146"></a>
## project_expr

`function` · `datafusion_physical_expr::equivalence::properties::EquivalenceProperties::project_expr` · datafusion-physical-expr 55.1.0

```rust
fn project_expr(&self, expr: &Arc<dyn PhysicalExpr>, mapping: &ProjectionMapping) -> Option<Arc<dyn PhysicalExpr>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::equivalence::properties::EquivalenceProperties", "path": "EquivalenceProperties"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [197, 1], "end": [1369, 2], "filename": "src/equivalence/properties/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/equivalence/properties/mod.rs:920`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Projects argument `expr` according to the projection described by
`mapping`, taking equivalences into account.

For example, assume that columns `a` and `c` are always equal, and that
the projection described by `mapping` encodes the following:

```text
a -> a1
b -> b1
```

Then, this function projects `a + b` to `Some(a1 + b1)`, `c + b` to
`Some(a1 + b1)` and `d` to `None`, meaning that it is not projectable.

<a id="op-76fe9e941142309eaee4ec62"></a>
## project_expressions

`function` · `datafusion_physical_expr::equivalence::properties::EquivalenceProperties::project_expressions` · datafusion-physical-expr 55.1.0

```rust
fn project_expressions<'a>(&'a self, expressions: impl IntoIterator<Item = &'a Arc<dyn PhysicalExpr>> + 'a, mapping: &'a ProjectionMapping) -> impl Iterator<Item = Option<Arc<dyn PhysicalExpr>>> + 'a
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::equivalence::properties::EquivalenceProperties", "path": "EquivalenceProperties"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [197, 1], "end": [1369, 2], "filename": "src/equivalence/properties/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/equivalence/properties/mod.rs:932`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Projects the given `expressions` according to the projection described
by `mapping`, taking equivalences into account. This function is similar
to [`Self::project_expr`](../operations/datafusion_physical_expr.equivalence.properties.EquivalenceProperties.md#op-71b33a5aa4df88734d9ca146), but projects multiple expressions at once
more efficiently than calling `project_expr` for each expression.

<a id="op-bdf7c92b957674880fd50253"></a>
## reorder

`function` · `datafusion_physical_expr::equivalence::properties::EquivalenceProperties::reorder` · datafusion-physical-expr 55.1.0

```rust
fn reorder(&mut self, ordering: impl IntoIterator<Item = PhysicalSortExpr>) -> Result<bool>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::equivalence::properties::EquivalenceProperties", "path": "EquivalenceProperties"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [197, 1], "end": [1369, 2], "filename": "src/equivalence/properties/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/equivalence/properties/mod.rs:522`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Updates the ordering equivalence class within assuming that the table
is re-sorted according to the argument `ordering`, and returns whether
this operation resulted in any change. Note that equivalence classes
(and constants) do not change as they are unaffected by a re-sort. If
the given ordering is already satisfied, the function does nothing.

<a id="op-1bdc3d634e1f7ca7703a6d51"></a>
## requirements_compatible

`function` · `datafusion_physical_expr::equivalence::properties::EquivalenceProperties::requirements_compatible` · datafusion-physical-expr 55.1.0

```rust
fn requirements_compatible(&self, given: LexRequirement, reference: LexRequirement) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::equivalence::properties::EquivalenceProperties", "path": "EquivalenceProperties"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [197, 1], "end": [1369, 2], "filename": "src/equivalence/properties/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/equivalence/properties/mod.rs:840`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Checks whether the `given` sort requirements are equal or more specific
than the `reference` sort requirements.

<a id="op-221a64153bb6c66bfcb6d893"></a>
## schema

`function` · `datafusion_physical_expr::equivalence::properties::EquivalenceProperties::schema` · datafusion-physical-expr 55.1.0

```rust
fn schema(&self) -> &SchemaRef
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::equivalence::properties::EquivalenceProperties", "path": "EquivalenceProperties"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [197, 1], "end": [1369, 2], "filename": "src/equivalence/properties/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/equivalence/properties/mod.rs:270`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Returns the associated schema.

<a id="op-ff0698e89c1612776b659c1b"></a>
## set_constraints

`function` · `datafusion_physical_expr::equivalence::properties::EquivalenceProperties::set_constraints` · datafusion-physical-expr 55.1.0

```rust
fn set_constraints(&mut self, constraints: Constraints)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::equivalence::properties::EquivalenceProperties", "path": "EquivalenceProperties"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [197, 1], "end": [1369, 2], "filename": "src/equivalence/properties/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/equivalence/properties/mod.rs:237`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Adds constraints to the properties.

<a id="op-06ad68f95e94e096fab3853d"></a>
## with_constraints

`function` · `datafusion_physical_expr::equivalence::properties::EquivalenceProperties::with_constraints` · datafusion-physical-expr 55.1.0

```rust
fn with_constraints(self, constraints: Constraints) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::equivalence::properties::EquivalenceProperties", "path": "EquivalenceProperties"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [197, 1], "end": [1369, 2], "filename": "src/equivalence/properties/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/equivalence/properties/mod.rs:242`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Adds constraints to the properties.

<a id="op-6c53ffc791d37ff2c24d211f"></a>
## with_new_schema

`function` · `datafusion_physical_expr::equivalence::properties::EquivalenceProperties::with_new_schema` · datafusion-physical-expr 55.1.0

```rust
fn with_new_schema(self, schema: SchemaRef) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::equivalence::properties::EquivalenceProperties", "path": "EquivalenceProperties"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [197, 1], "end": [1369, 2], "filename": "src/equivalence/properties/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/equivalence/properties/mod.rs:1305`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Transforms this `EquivalenceProperties` by mapping columns in the
original schema to columns in the new schema by index.
