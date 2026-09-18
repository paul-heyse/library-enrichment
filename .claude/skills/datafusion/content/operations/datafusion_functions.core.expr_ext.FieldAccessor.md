# `datafusion_functions::core::expr_ext::FieldAccessor`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions.core.expr_ext.FieldAccessor.json).

<a id="op-eedc47aabc23a6260a69a951"></a>
## FieldAccessor

`trait` · `datafusion_functions::core::expr_ext::FieldAccessor` · datafusion-functions 55.1.0

```rust
trait FieldAccessor
```

Source: `src/core/expr_ext.rs:45`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

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

<a id="op-d1016fe1fbf7e3868c8e023f"></a>
## field

`function` · `datafusion_functions::core::expr_ext::FieldAccessor::field` · datafusion-functions 55.1.0

```rust
fn field(self, name: impl Literal) -> Expr
```

Source: `src/core/expr_ext.rs:46`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
