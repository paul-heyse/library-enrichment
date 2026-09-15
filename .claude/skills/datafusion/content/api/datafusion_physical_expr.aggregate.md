# `datafusion_physical_expr::aggregate`

Crate `datafusion-physical-expr` · 5 public items · structured records in [`model/datafusion_physical_expr.aggregate.json`](../model/datafusion_physical_expr.aggregate.json)

## AggregateExprBuilder

`struct` · `datafusion_physical_expr::aggregate::AggregateExprBuilder`

```rust
struct AggregateExprBuilder
```

**Derives**: Clone, Debug

**Methods** (12)

```rust
fn alias(self, alias: impl Into<String>) -> Self
fn build(self) -> Result<AggregateFunctionExpr>
fn distinct(self) -> Self
fn human_display(self, name: impl Into<String>) -> Self
fn ignore_nulls(self) -> Self
fn new(fun: Arc<AggregateUDF>, args: Vec<Arc<dyn PhysicalExpr>>) -> Self
fn order_by(self, order_bys: Vec<PhysicalSortExpr>) -> Self
fn reversed(self) -> Self
fn schema(self, schema: SchemaRef) -> Self
fn with_distinct(self, is_distinct: bool) -> Self
fn with_ignore_nulls(self, ignore_nulls: bool) -> Self
fn with_reversed(self, is_reversed: bool) -> Self
```

Builder for physical [`AggregateFunctionExpr`]

`AggregateFunctionExpr` contains the information necessary to call
an aggregate expression.

---

## AggregateFunctionExpr

`struct` · `datafusion_physical_expr::aggregate::AggregateFunctionExpr`

Also reachable as `datafusion_physical_plan::udaf::AggregateFunctionExpr`

```rust
struct AggregateFunctionExpr
```

**Derives**: Clone, Debug, PartialEq

**Methods** (24)

```rust
fn all_expressions(&self) -> AggregatePhysicalExpressions
fn create_accumulator(&self) -> Result<Box<dyn Accumulator>>
fn create_groups_accumulator(&self) -> Result<Box<dyn GroupsAccumulator>>
fn create_sliding_accumulator(&self) -> Result<Box<dyn Accumulator>>
fn default_value(&self, data_type: &DataType) -> Result<ScalarValue>
fn expressions(&self) -> Vec<Arc<dyn PhysicalExpr>>
fn field(&self) -> FieldRef
fn fun(&self) -> &AggregateUDF
fn get_minmax_desc(&self) -> Option<(FieldRef, bool)>
fn get_result_ordering(&self, aggr_func_idx: usize) -> Option<PhysicalSortExpr>
fn groups_accumulator_supported(&self) -> bool
fn human_display(&self) -> Option<&str>
fn ignore_nulls(&self) -> bool
fn is_distinct(&self) -> bool
fn is_nullable(&self) -> bool
fn is_reversed(&self) -> bool
fn name(&self) -> &str
fn order_bys(&self) -> &[PhysicalSortExpr]
fn order_sensitivity(&self) -> AggregateOrderSensitivity
fn reverse_expr(&self) -> Option<AggregateFunctionExpr>
fn set_monotonicity(&self) -> SetMonotonicity
fn state_fields(&self) -> Result<Vec<FieldRef>>
fn with_beneficial_ordering(Arc<self>, beneficial_ordering: bool) -> Result<Option<AggregateFunctionExpr>>
fn with_new_expressions(&self, args: Vec<Arc<dyn PhysicalExpr>>, order_by_exprs: Vec<Arc<dyn PhysicalExpr>>) -> Option<AggregateFunctionExpr>
```

Physical aggregate expression of a UDAF.

Instances are constructed via [`AggregateExprBuilder`].

---

## AggregatePhysicalExpressions

`struct` · `datafusion_physical_expr::aggregate::AggregatePhysicalExpressions`

```rust
struct AggregatePhysicalExpressions
```

**Fields**: `args`, `order_by_exprs`

Stores the physical expressions used inside the `AggregateExpr`.

---

## LoweredAggregate

`struct` · `datafusion_physical_expr::aggregate::LoweredAggregate`

```rust
struct LoweredAggregate
```

**Fields**: `aggregate`, `filter`, `order_bys`

**Derives**: Clone, Debug

Result of lowering a logical aggregate expression into physical aggregate
planning pieces.

---

## LoweredAggregateBuilder

`struct` · `datafusion_physical_expr::aggregate::LoweredAggregateBuilder`

```rust
struct LoweredAggregateBuilder<'a>
```

**Methods** (4)

```rust
fn build(self) -> Result<LoweredAggregate>
fn new(expr: &'a Expr, logical_input_schema: &'a DFSchema, physical_input_schema: &'a Schema, execution_props: &'a ExecutionProps, planning_ctx: &'a PhysicalPlanningContext) -> Self
fn with_human_display(self, human_display: impl Into<String>) -> Self
fn with_name(self, name: impl Into<String>) -> Self
```

Builder for converting a logical aggregate [`Expr`] into physical aggregate
planning pieces.

This builder handles the logical-to-physical work needed for aggregate
planning: unwrapping aggregate aliases, choosing the output name, preserving
user-facing display text, lowering aggregate arguments, lowering the optional
filter, and lowering aggregate `ORDER BY` expressions.

---
