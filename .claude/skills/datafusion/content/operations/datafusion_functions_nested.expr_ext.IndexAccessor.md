# `datafusion_functions_nested::expr_ext::IndexAccessor`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions_nested.expr_ext.IndexAccessor.json).

<a id="op-4568f92df6fadee4108f2c97"></a>
## IndexAccessor

`trait` · `datafusion_functions_nested::expr_ext::IndexAccessor` · datafusion-functions-nested 55.1.0

```rust
trait IndexAccessor
```

Source: `src/expr_ext.rs:42`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-nested/55.1.0/json).

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

<a id="op-51545be108764110d649aeed"></a>
## index

`function` · `datafusion_functions_nested::expr_ext::IndexAccessor::index` · datafusion-functions-nested 55.1.0

```rust
fn index(self, key: Expr) -> Expr
```

Source: `src/expr_ext.rs:43`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-nested/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
