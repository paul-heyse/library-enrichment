# `datafusion_sql::unparser`

Crate `datafusion-sql` · 1 public items · structured records in [`model/datafusion_sql.unparser.json`](../model/datafusion_sql.unparser.json)

## Unparser

`struct` · `datafusion_sql::unparser::Unparser`

```rust
struct Unparser<'a>
```

**Derives**: Default

**Methods** (8)

```rust
fn col_to_sql(&self, col: &Column) -> Result<ast::Expr>
fn expr_to_sql(&self, expr: &Expr) -> Result<ast::Expr>
fn new(dialect: &'a dyn Dialect) -> Self
fn plan_to_sql(&self, plan: &LogicalPlan) -> Result<ast::Statement>
fn scalar_function_to_sql(&self, func_name: &str, args: &[Expr]) -> Result<ast::Expr>
fn sort_to_sql(&self, sort: &Sort) -> Result<ast::OrderByExpr>
fn with_extension_unparsers(self, extension_unparsers: Vec<Arc<dyn UserDefinedLogicalNodeUnparser>>) -> Self
fn with_pretty(self, pretty: bool) -> Self
```

[Full member, field, variant and typed contracts](../operations/datafusion_sql.unparser.Unparser.md).


Convert a DataFusion [`Expr`] to [`sqlparser::ast::Expr`]

See [`expr_to_sql`] for background. `Unparser` allows greater control of
the conversion, but with a more complicated API.

To get more human-readable output, see [`Self::with_pretty`]

# Example
```
use datafusion_expr::{col, lit};
use datafusion_sql::unparser::Unparser;
let expr = col("a").gt(lit(4)); // form an expression `a > 4`
let unparser = Unparser::default();
let sql = unparser.expr_to_sql(&expr).unwrap();// convert to AST
// use the Display impl to convert to SQL text
assert_eq!(sql.to_string(), "(a > 4)");
// now convert to pretty sql
let unparser = unparser.with_pretty(true);
let sql = unparser.expr_to_sql(&expr).unwrap();
assert_eq!(sql.to_string(), "a > 4"); // note lack of parenthesis
```

[`Expr`]: datafusion_expr::Expr

---
