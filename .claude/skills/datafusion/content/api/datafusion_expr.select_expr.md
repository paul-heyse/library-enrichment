# `datafusion_expr::select_expr`

Crate `datafusion-expr` · 1 public items · structured records in [`model/datafusion_expr.select_expr.json`](../model/datafusion_expr.select_expr.json)

## SelectExpr

`enum` · `datafusion_expr::select_expr::SelectExpr`

```rust
enum SelectExpr
```

**Variants**: `Wildcard`, `QualifiedWildcard`, `Expression`

**Implements**: `core::convert::From`, `core::fmt::Display`

**Derives**: Clone, Debug

**via `core::convert::From`**

```rust
fn from(value: Column) -> Self
fn from(expr: Expr) -> Self
fn from(value: (Option<&'a TableReference>, &'a FieldRef)) -> Self
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Represents a SELECT expression in a SQL query.

`SelectExpr` supports three types of expressions commonly found in the SELECT clause:

* Wildcard (`*`) - Selects all columns
* Qualified wildcard (`table.*`) - Selects all columns from a specific table
* Regular expression - Any other expression like columns, functions, literals etc.

This enum is typically used when you need to handle wildcards. After expanding `*` in the query,
you can use `Expr` for all other expressions.

# Examples

```
use datafusion_expr::col;
use datafusion_expr::expr::WildcardOptions;
use datafusion_expr::select_expr::SelectExpr;

// SELECT *
let wildcard = SelectExpr::Wildcard(WildcardOptions::default());

// SELECT mytable.*
let qualified =
    SelectExpr::QualifiedWildcard("mytable".into(), WildcardOptions::default());

// SELECT col1
let expr = SelectExpr::Expression(col("col1").into());
```

---
