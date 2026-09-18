# `datafusion_functions_nested::expr_ext`

Crate `datafusion-functions-nested` · 2 public items · structured records in [`model/datafusion_functions_nested.expr_ext.json`](../model/datafusion_functions_nested.expr_ext.json)

## IndexAccessor

`trait` · `datafusion_functions_nested::expr_ext::IndexAccessor`

```rust
trait IndexAccessor
```

**Implementors** (1)

- `datafusion_expr::expr::Expr`

**Methods** (1)

```rust
fn index(self, key: Expr) -> Expr
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions_nested.expr_ext.IndexAccessor.md).


Return access to the element field. Example `expr["name"]`

## Example Access element 2 from column "c1"

For example if column "c1" holds documents like this

```json
[10, 20, 30, 40]
```

You can access the value "30" with

```
# use datafusion_expr::{lit, col, Expr};
# use datafusion_functions_nested::expr_ext::IndexAccessor;
let expr = col("c1").index(lit(3));
assert_eq!(expr.schema_name().to_string(), "c1[Int32(3)]");
```

---

## SliceAccessor

`trait` · `datafusion_functions_nested::expr_ext::SliceAccessor`

```rust
trait SliceAccessor
```

**Implementors** (1)

- `datafusion_expr::expr::Expr`

**Methods** (1)

```rust
fn range(self, start: Expr, stop: Expr) -> Expr
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions_nested.expr_ext.SliceAccessor.md).


Return elements between `1` based `start` and `stop`, for
example `expr[1:3]`

## Example: Access element 2, 3, 4 from column "c1"

For example if column "c1" holds documents like this

```json
[10, 20, 30, 40]
```

You can access the value `[20, 30, 40]` with

```
# use datafusion_expr::{lit, col};
# use datafusion_functions_nested::expr_ext::SliceAccessor;
let expr = col("c1").range(lit(2), lit(4));
assert_eq!(expr.schema_name().to_string(), "c1[Int32(2):Int32(4)]");
```

---
