# `ruff_python_formatter::expression::parentheses`

Crate `ruff_python_formatter` · 1 public items · structured records in [`model/ruff_python_formatter.expression.parentheses.json`](../model/ruff_python_formatter.expression.parentheses.json)

## Parentheses

`enum` · `ruff_python_formatter::expression::parentheses::Parentheses`

```rust
enum Parentheses
```

**Variants**: `Preserve`, `Always`, `Never`

Whether it is necessary to add parentheses around an expression.
This is different from [`Parenthesize`] in that it is the resolved representation: It takes into account
whether there are parentheses in the source code or not.

---
