# `datafusion_expr::simplify`

Crate `datafusion-expr` · 3 public items · structured records in [`model/datafusion_expr.simplify.json`](../model/datafusion_expr.simplify.json)

## ExprSimplifyResult

`enum` · `datafusion_expr::simplify::ExprSimplifyResult`

```rust
enum ExprSimplifyResult
```

**Variants**: `Simplified`, `Original`

**Derives**: Debug

Was the expression simplified?

---

## SimplifyContext

`struct` · `datafusion_expr::simplify::SimplifyContext`

Also reachable as `datafusion_optimizer::simplify_expressions::SimplifyContext`

```rust
struct SimplifyContext
```

**Derives**: Clone, Debug, Default

**Methods** (11)

```rust
fn builder() -> SimplifyContextBuilder
fn config_options(&self) -> &Arc<ConfigOptions>
fn get_data_type(&self, expr: &Expr) -> Result<DataType>
fn is_boolean_type(&self, expr: &Expr) -> Result<bool>
fn nullable(&self, expr: &Expr) -> Result<bool>
fn query_execution_start_time(&self) -> Option<DateTime<Utc>>
fn schema(&self) -> &DFSchemaRef
fn with_config_options(self, config_options: Arc<ConfigOptions>) -> Self
fn with_current_time(self) -> Self
fn with_query_execution_start_time(self, query_execution_start_time: Option<DateTime<Utc>>) -> Self
fn with_schema(self, schema: DFSchemaRef) -> Self
```

Provides simplification information based on schema, query execution time,
and configuration options.

# Example
See the `simplify_demo` in the [`expr_api` example]

[`expr_api` example]: https://github.com/apache/datafusion/blob/main/datafusion-examples/examples/query_planning/expr_api.rs

---

## SimplifyContextBuilder

`struct` · `datafusion_expr::simplify::SimplifyContextBuilder`

```rust
struct SimplifyContextBuilder
```

**Derives**: Debug, Default

**Methods** (5)

```rust
fn build(self) -> SimplifyContext
fn with_config_options(self, config_options: Arc<ConfigOptions>) -> Self
fn with_current_time(self) -> Self
fn with_query_execution_start_time(self, query_execution_start_time: Option<DateTime<Utc>>) -> Self
fn with_schema(self, schema: DFSchemaRef) -> Self
```

Builder for [`SimplifyContext`].

---
