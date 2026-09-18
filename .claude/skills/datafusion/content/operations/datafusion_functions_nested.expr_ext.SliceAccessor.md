# `datafusion_functions_nested::expr_ext::SliceAccessor`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions_nested.expr_ext.SliceAccessor.json).

<a id="op-0d6a0560df79795ab221e03a"></a>
## SliceAccessor

`trait` · `datafusion_functions_nested::expr_ext::SliceAccessor` · datafusion-functions-nested 55.1.0

```rust
trait SliceAccessor
```

Source: `src/expr_ext.rs:71`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-nested/55.1.0/json).

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

<a id="op-223dc505b30d80a78c32bfb7"></a>
## range

`function` · `datafusion_functions_nested::expr_ext::SliceAccessor::range` · datafusion-functions-nested 55.1.0

```rust
fn range(self, start: Expr, stop: Expr) -> Expr
```

Source: `src/expr_ext.rs:72`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-nested/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
