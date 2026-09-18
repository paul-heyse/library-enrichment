# `datafusion_optimizer::simplify_expressions::simplify_literal`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_optimizer.simplify_expressions.simplify_literal.json).

<a id="op-d1f47e99eef98f7f5fc58136"></a>
## simplify_literal

`module` · `datafusion_optimizer::simplify_expressions::simplify_literal` · datafusion-optimizer 55.1.0

```rust
mod simplify_literal
```

Source: `src/simplify_expressions/simplify_literal.rs:18`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

Parses and simplifies an expression to a literal of a given type.

This module provides functionality to parse and simplify static expressions
used in SQL constructs like `FROM TABLE SAMPLE (10 + 50 * 2)`. If they are required
in a planning (not an execution) phase, they need to be reduced to literals of a given type.
