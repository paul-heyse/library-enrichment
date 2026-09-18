# `datafusion_optimizer::analyzer::type_coercion`

Crate `datafusion-optimizer` · 3 public items · structured records in [`model/datafusion_optimizer.analyzer.type_coercion.json`](../model/datafusion_optimizer.analyzer.type_coercion.json)

## coerce_union_schema

`function` · `datafusion_optimizer::analyzer::type_coercion::coerce_union_schema`

```rust
fn coerce_union_schema(inputs: &[std::sync::Arc<datafusion_expr::LogicalPlan>]) -> datafusion_common::Result<datafusion_common::DFSchema>
```

[Full member, field, variant and typed contracts](../operations/datafusion_optimizer.analyzer.type_coercion.coerce_union_schema.md).


Get a common schema that is compatible with all inputs of UNION.

This method presumes that the wildcard expansion is unneeded, or has already
been applied.

## Schema and Field Handling in Union Coercion

**Processing order**: The function starts with the base schema (first input) and then
processes remaining inputs sequentially, with later inputs taking precedence in merging.

**Schema-level metadata merging**: Later schemas take precedence for duplicate keys.

**Field-level metadata merging**: Later fields take precedence for duplicate metadata keys.

**Type coercion precedence**: The coerced type is determined by iteratively applying
`type_union_coercion()` between the accumulated type and each new input's type. The
result depends on type coercion rules, not input order.

**Nullability merging**: Nullability is accumulated using logical OR (`||`).
Once any input field is nullable, the result field becomes nullable permanently.
Later inputs can make a field nullable but cannot make it non-nullable.

**Field precedence**: Field names come from the first (base) schema, but the field properties
(nullability and field-level metadata) have later schemas taking precedence.

**Example**:
```sql
SELECT a, b FROM table1  -- a: Int32, metadata {"source": "t1"}, nullable=false
UNION
SELECT a, b FROM table2  -- a: Int64, metadata {"source": "t2"}, nullable=true
UNION
SELECT a, b FROM table3  -- a: Int32, metadata {"encoding": "utf8"}, nullable=false
-- Result:
-- a: Int64 (from type coercion), nullable=true (from table2),
-- metadata: {"source": "t2", "encoding": "utf8"} (later inputs take precedence)
```

**Precedence Summary**:
- **Datatypes**: Determined by `type_union_coercion()` rules, not input order
- **Nullability**: Later inputs can add nullability but cannot remove it (logical OR)
- **Metadata**: Later inputs take precedence for same keys (HashMap::extend semantics)

---

## TypeCoercion

`struct` · `datafusion_optimizer::analyzer::type_coercion::TypeCoercion`

```rust
struct TypeCoercion
```

**Implements**: `datafusion_optimizer::analyzer::AnalyzerRule`

**Derives**: Debug, Default

**Methods** (1)

```rust
fn new() -> Self
```

**via `datafusion_optimizer::analyzer::AnalyzerRule`**

```rust
fn analyze(&self, plan: LogicalPlan, config: &ConfigOptions) -> Result<LogicalPlan>
fn name(&self) -> &str
```

[Full member, field, variant and typed contracts](../operations/datafusion_optimizer.analyzer.type_coercion.TypeCoercion.md).


Performs type coercion by determining the schema
and performing the expression rewrites.

---

## TypeCoercionRewriter

`struct` · `datafusion_optimizer::analyzer::type_coercion::TypeCoercionRewriter`

```rust
struct TypeCoercionRewriter<'a>
```

**Implements**: `datafusion_common::tree_node::TreeNodeRewriter`

**Methods** (4)

```rust
fn coerce_join(&mut self, join: Join) -> Result<LogicalPlan>
fn coerce_plan(&mut self, plan: LogicalPlan) -> Result<LogicalPlan>
fn coerce_union(union_plan: Union) -> Result<LogicalPlan>
fn new(schema: &'a DFSchema) -> Self
```

**via `datafusion_common::tree_node::TreeNodeRewriter`**

```rust
fn f_up(&mut self, expr: Expr) -> Result<Transformed<Expr>>
```

[Full member, field, variant and typed contracts](../operations/datafusion_optimizer.analyzer.type_coercion.TypeCoercionRewriter.md).


Rewrite expressions to apply type coercion.

---
