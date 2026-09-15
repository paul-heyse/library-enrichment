# `datafusion_physical_expr::equivalence::properties`

Crate `datafusion-physical-expr` · 1 public items · structured records in [`model/datafusion_physical_expr.equivalence.properties.json`](../model/datafusion_physical_expr.equivalence.properties.json)

## EquivalenceProperties

`struct` · `datafusion_physical_expr::equivalence::properties::EquivalenceProperties`

Also reachable as `datafusion::physical_expr::EquivalenceProperties`, `datafusion_physical_expr::EquivalenceProperties`, `datafusion_physical_expr::equivalence::EquivalenceProperties`

```rust
struct EquivalenceProperties
```

**Implements**: `core::fmt::Display`

**Derives**: Clone, Debug

**Methods** (33)

```rust
fn add_constants(&mut self, constants: impl IntoIterator<Item = ConstExpr>) -> Result<()>
fn add_equal_conditions(&mut self, left: Arc<dyn PhysicalExpr>, right: Arc<dyn PhysicalExpr>) -> Result<()>
fn add_equivalence_group(&mut self, other_eq_group: EquivalenceGroup) -> Result<()>
fn add_ordering(&mut self, ordering: impl IntoIterator<Item = PhysicalSortExpr>)
fn add_orderings(&mut self, orderings: impl IntoIterator<Item = impl IntoIterator<Item = PhysicalSortExpr>>)
fn clear_orderings(&mut self)
fn clear_per_partition_constants(&mut self)
fn constants(&self) -> Vec<ConstExpr>
fn constraints(&self) -> &Constraints
fn eq_group(&self) -> &EquivalenceGroup
fn extend(self, other: Self) -> Result<Self>
fn extract_common_sort_prefix(&self, ordering: LexOrdering) -> Result<(Vec<PhysicalSortExpr>, bool)>
fn find_longest_permutation(&self, exprs: &[Arc<dyn PhysicalExpr>]) -> Result<(Vec<PhysicalSortExpr>, Vec<usize>)>
fn get_expr_properties(&self, expr: Arc<dyn PhysicalExpr>) -> ExprProperties
fn is_expr_constant(&self, expr: &Arc<dyn PhysicalExpr>) -> Option<AcrossPartitions>
fn new(schema: SchemaRef) -> Self
fn new_with_orderings(schema: SchemaRef, orderings: impl IntoIterator<Item = impl IntoIterator<Item = PhysicalSortExpr>>) -> Self
fn normalize_sort_exprs(&self, sort_exprs: impl IntoIterator<Item = PhysicalSortExpr>) -> Option<LexOrdering>
fn normalize_sort_requirements(&self, sort_reqs: impl IntoIterator<Item = PhysicalSortRequirement>) -> Option<LexRequirement>
fn normalized_oeq_class(&self) -> OrderingEquivalenceClass
fn oeq_class(&self) -> &OrderingEquivalenceClass
fn ordering_satisfy(&self, given: impl IntoIterator<Item = PhysicalSortExpr>) -> Result<bool>
fn ordering_satisfy_requirement(&self, given: impl IntoIterator<Item = PhysicalSortRequirement>) -> Result<bool>
fn output_ordering(&self) -> Option<LexOrdering>
fn project(&self, mapping: &ProjectionMapping, output_schema: SchemaRef) -> Self
fn project_expr(&self, expr: &Arc<dyn PhysicalExpr>, mapping: &ProjectionMapping) -> Option<Arc<dyn PhysicalExpr>>
fn project_expressions<'a>(&'a self, expressions: impl IntoIterator<Item = &'a Arc<dyn PhysicalExpr>> + 'a, mapping: &'a ProjectionMapping) -> impl Iterator<Item = Option<Arc<dyn PhysicalExpr>>> + 'a
fn reorder(&mut self, ordering: impl IntoIterator<Item = PhysicalSortExpr>) -> Result<bool>
fn requirements_compatible(&self, given: LexRequirement, reference: LexRequirement) -> bool
fn schema(&self) -> &SchemaRef
fn set_constraints(&mut self, constraints: Constraints)
fn with_constraints(self, constraints: Constraints) -> Self
fn with_new_schema(self, schema: SchemaRef) -> Result<Self>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

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

---
