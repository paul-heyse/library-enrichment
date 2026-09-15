# `datafusion_functions::core::expr_ext`

Crate `datafusion-functions` · 1 public items · structured records in [`model/datafusion_functions.core.expr_ext.json`](../model/datafusion_functions.core.expr_ext.json)

## FieldAccessor

`trait` · `datafusion_functions::core::expr_ext::FieldAccessor`

```rust
trait FieldAccessor
```

**Implementors** (1)

- `datafusion_expr::expr::Expr`

**Methods** (1)

```rust
fn field(self, name: impl Literal) -> Expr
```

Return access to the named field. Example `expr["name"]`

## Access field "my_field" from column "c1"

For example if column "c1" holds documents like this

```json
{
  "my_field": 123.34,
  "other_field": "Boston",
}
```

You can access column "my_field" with

```
# use datafusion_expr::{col};
# use datafusion_functions::core::expr_ext::FieldAccessor;
let expr = col("c1").field("my_field");
assert_eq!(expr.schema_name().to_string(), "c1[my_field]");
```

---
