# `datafusion_physical_expr::simplifier::not`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_expr.simplifier.not.json).

<a id="op-bef13bd7f9acc77859f549e4"></a>
## not

`module` · `datafusion_physical_expr::simplifier::not` · datafusion-physical-expr 55.1.0

```rust
mod not
```

Source: `src/simplifier/not.rs:18`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Simplify NOT expressions in physical expressions

This module provides optimizations for NOT expressions such as:
- Double negation elimination: NOT(NOT(expr)) -> expr
- NOT with binary comparisons: NOT(a = b) -> a != b
- NOT with IN expressions: NOT(a IN (list)) -> a NOT IN (list)
- De Morgan's laws: NOT(A AND B) -> NOT A OR NOT B
- Constant folding: NOT(TRUE) -> FALSE, NOT(FALSE) -> TRUE

This function is designed to work with TreeNodeRewriter's f_up traversal,
which means children are already simplified when this function is called.
The TreeNodeRewriter will automatically call this function repeatedly until
no more transformations are possible.
