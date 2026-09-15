# `datafusion_physical_expr::scalar_function`

Crate `datafusion-physical-expr` · 1 public items · structured records in [`model/datafusion_physical_expr.scalar_function.json`](../model/datafusion_physical_expr.scalar_function.json)

## ScalarFunctionExpr

`struct` · `datafusion_physical_expr::scalar_function::ScalarFunctionExpr`

Also reachable as `datafusion::physical_expr::ScalarFunctionExpr`, `datafusion_physical_expr::ScalarFunctionExpr`

```rust
struct ScalarFunctionExpr
```

**Implements**: `core::fmt::Display`, `datafusion_physical_expr_common::physical_expr::PhysicalExpr`

**Derives**: Debug, Eq, Hash, PartialEq

**Methods** (10)

```rust
fn args(&self) -> &[Arc<dyn PhysicalExpr>]
fn config_options(&self) -> &ConfigOptions
fn fun(&self) -> &ScalarUDF
fn name(&self) -> &str
fn new(name: &str, fun: Arc<ScalarUDF>, args: Vec<Arc<dyn PhysicalExpr>>, return_field: FieldRef, config_options: Arc<ConfigOptions>) -> Self
fn nullable(&self) -> bool
fn return_type(&self) -> &DataType
fn try_downcast_func<T>(expr: &dyn PhysicalExpr) -> Option<&ScalarFunctionExpr> where T: ScalarUDFImpl
fn try_new(fun: Arc<ScalarUDF>, args: Vec<Arc<dyn PhysicalExpr>>, schema: &Schema, config_options: Arc<ConfigOptions>) -> Result<Self>
fn with_nullable(self, nullable: bool) -> Self
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result
```

**via `datafusion_physical_expr_common::physical_expr::PhysicalExpr`**

```rust
fn children(&self) -> Vec<&Arc<dyn PhysicalExpr>>
fn data_type(&self, _input_schema: &Schema) -> Result<DataType>
fn evaluate(&self, batch: &RecordBatch) -> Result<ColumnarValue>
fn evaluate_bounds(&self, children: &[&Interval]) -> Result<Interval>
fn fmt_sql(&self, f: &mut Formatter<'_>) -> fmt::Result
fn get_properties(&self, children: &[ExprProperties]) -> Result<ExprProperties>
fn is_volatile_node(&self) -> bool
fn nullable(&self, _input_schema: &Schema) -> Result<bool>
fn placement(&self) -> ExpressionPlacement
fn propagate_constraints(&self, interval: &Interval, children: &[&Interval]) -> Result<Option<Vec<Interval>>>
fn return_field(&self, _input_schema: &Schema) -> Result<FieldRef>
fn with_new_children(Arc<self>, children: Vec<Arc<dyn PhysicalExpr>>) -> Result<Arc<dyn PhysicalExpr>>
```

Physical expression of a scalar function

---
