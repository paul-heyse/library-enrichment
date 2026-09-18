# `datafusion_sql::unparser::Unparser`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_sql.unparser.Unparser.json).

<a id="op-7e732a2e0e80279190139cfb"></a>
## Unparser

`struct` · `datafusion_sql::unparser::Unparser` · datafusion-sql 55.1.0

```rust
struct Unparser<'a>
```

Source: `src/unparser/mod.rs:57`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

Convert a DataFusion [`Expr`] to [`sqlparser::ast::Expr`](../operations/sqlparser.ast.Expr.md#op-60bb4348f2010610c575bb63)

See [`expr_to_sql`](../operations/datafusion_sql.unparser.expr.expr_to_sql.md#op-e296114a3665696472053bea) for background. `Unparser` allows greater control of
the conversion, but with a more complicated API.

To get more human-readable output, see [`Self::with_pretty`](../operations/datafusion_sql.unparser.Unparser.md#op-ccece758017cce7ddf2ce3a0)

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

<a id="op-15735b7fbe7ee05bc398263f"></a>
## col_to_sql

`function` · `datafusion_sql::unparser::Unparser::col_to_sql` · datafusion-sql 55.1.0

```rust
fn col_to_sql(&self, col: &Column) -> Result<ast::Expr>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}], "constraints": []}}, "id": "datafusion_sql::unparser::Unparser", "path": "super::Unparser"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [95, 1], "end": [1954, 2], "filename": "src/unparser/expr.rs"}, "trait": null, "trait_path": null}`

Source: `src/unparser/expr.rs:894`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-20b88f6c62651d34d332633a"></a>
## default

`function` · `datafusion_sql::unparser::Unparser::default` · datafusion-sql 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}], "constraints": []}}, "id": "datafusion_sql::unparser::Unparser", "path": "Unparser"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [133, 1], "end": [141, 2], "filename": "src/unparser/mod.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/unparser/mod.rs:134`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f830e7fd4e84e93cc089c99b"></a>
## expr_to_sql

`function` · `datafusion_sql::unparser::Unparser::expr_to_sql` · datafusion-sql 55.1.0

```rust
fn expr_to_sql(&self, expr: &Expr) -> Result<ast::Expr>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}], "constraints": []}}, "id": "datafusion_sql::unparser::Unparser", "path": "super::Unparser"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [95, 1], "end": [1954, 2], "filename": "src/unparser/expr.rs"}, "trait": null, "trait_path": null}`

Source: `src/unparser/expr.rs:96`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2d74fb9535ba5fb58a9db988"></a>
## new

`function` · `datafusion_sql::unparser::Unparser::new` · datafusion-sql 55.1.0

```rust
fn new(dialect: &'a dyn Dialect) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion_sql::unparser::Unparser", "path": "Unparser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [63, 1], "end": [131, 2], "filename": "src/unparser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/unparser/mod.rs:64`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9bcf08a51de1e7d8377c2170"></a>
## plan_to_sql

`function` · `datafusion_sql::unparser::Unparser::plan_to_sql` · datafusion-sql 55.1.0

```rust
fn plan_to_sql(&self, plan: &LogicalPlan) -> Result<ast::Statement>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}], "constraints": []}}, "id": "datafusion_sql::unparser::Unparser", "path": "super::Unparser"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [167, 1], "end": [2609, 2], "filename": "src/unparser/plan.rs"}, "trait": null, "trait_path": null}`

Source: `src/unparser/plan.rs:168`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f1a5516d2e30f5a3ec465381"></a>
## scalar_function_to_sql

`function` · `datafusion_sql::unparser::Unparser::scalar_function_to_sql` · datafusion-sql 55.1.0

```rust
fn scalar_function_to_sql(&self, func_name: &str, args: &[Expr]) -> Result<ast::Expr>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}], "constraints": []}}, "id": "datafusion_sql::unparser::Unparser", "path": "super::Unparser"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [95, 1], "end": [1954, 2], "filename": "src/unparser/expr.rs"}, "trait": null, "trait_path": null}`

Source: `src/unparser/expr.rs:666`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-13b795871ab07863d50c4d56"></a>
## sort_to_sql

`function` · `datafusion_sql::unparser::Unparser::sort_to_sql` · datafusion-sql 55.1.0

```rust
fn sort_to_sql(&self, sort: &Sort) -> Result<ast::OrderByExpr>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}], "constraints": []}}, "id": "datafusion_sql::unparser::Unparser", "path": "super::Unparser"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [95, 1], "end": [1954, 2], "filename": "src/unparser/expr.rs"}, "trait": null, "trait_path": null}`

Source: `src/unparser/expr.rs:862`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-521f171853bc3d4cc2014bd8"></a>
## with_extension_unparsers

`function` · `datafusion_sql::unparser::Unparser::with_extension_unparsers` · datafusion-sql 55.1.0

```rust
fn with_extension_unparsers(self, extension_unparsers: Vec<Arc<dyn UserDefinedLogicalNodeUnparser>>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion_sql::unparser::Unparser", "path": "Unparser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [63, 1], "end": [131, 2], "filename": "src/unparser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/unparser/mod.rs:124`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

Add a custom unparser for user defined logical nodes

DataFusion allows user to define custom logical nodes. This method allows to add custom child unparsers for these nodes.
Implementation of [`UserDefinedLogicalNodeUnparser`](../operations/datafusion_sql.unparser.extension_unparser.UserDefinedLogicalNodeUnparser.md#op-176cebcf1171735c57540ca8) can be added to the root unparser to handle custom logical nodes.

The child unparsers are called iteratively.
There are two methods in [`Unparser`](../operations/datafusion_sql.unparser.Unparser.md#op-7e732a2e0e80279190139cfb) will be called:
- `extension_to_statement`: This method is called when the custom logical node is a custom statement.
  If multiple child unparsers return a non-None value, the last unparsing result will be returned.
- `extension_to_sql`: This method is called when the custom logical node is part of a statement.
  If multiple child unparsers are registered for the same custom logical node, all of them will be called in order.

<a id="op-ccece758017cce7ddf2ce3a0"></a>
## with_pretty

`function` · `datafusion_sql::unparser::Unparser::with_pretty` · datafusion-sql 55.1.0

```rust
fn with_pretty(self, pretty: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion_sql::unparser::Unparser", "path": "Unparser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [63, 1], "end": [131, 2], "filename": "src/unparser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/unparser/mod.rs:108`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

Create pretty SQL output, better suited for human consumption

See example on the struct level documentation

# Pretty Output

By default, `Unparser` generates SQL text that will parse back to the
same parsed [`Expr`], which is useful for creating machine readable
expressions to send to other systems. However, the resulting expressions are
not always nice to read for humans.

For example

```sql
((a + 4) > 5)
```

This method removes parenthesis using to the precedence rules of
DataFusion. If the output is reparsed, the resulting [`Expr`] produces
same value as the original in DataFusion, but with a potentially
different order of operations.

Note that this setting may create invalid SQL for other SQL query
engines with different precedence rules

# Example
```
use datafusion_expr::{col, lit};
use datafusion_sql::unparser::Unparser;
let expr = col("a").gt(lit(4)).and(col("b").lt(lit(5))); // form an expression `a > 4 AND b < 5`
let unparser = Unparser::default().with_pretty(true);
let sql = unparser.expr_to_sql(&expr).unwrap();
assert_eq!(sql.to_string(), "a > 4 AND b < 5"); // note lack of parenthesis
```

[`Expr`]: datafusion_expr::Expr
