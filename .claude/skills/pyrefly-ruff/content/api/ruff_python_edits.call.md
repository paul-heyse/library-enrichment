# `ruff_python_edits::call`

Crate `ruff_python_edits` · 1 public items · structured records in [`model/ruff_python_edits.call.json`](../model/ruff_python_edits.call.json)

## unwrapped_call_argument

`function` · `ruff_python_edits::call::unwrapped_call_argument`

Also reachable as `ruff_python_edits::unwrapped_call_argument`

```rust
fn unwrapped_call_argument(call: &ruff_python_ast::ExprCall, argument: &ruff_python_ast::Expr, parent: Option<ruff_python_ast::AnyNodeRef<'_>>, tokens: &ruff_python_ast::token::Tokens, source: &str) -> String
```

Returns source code that replaces a call with one of its arguments.

Preserve the argument's optional parentheses, including any comments inside them. Otherwise, add
parentheses when needed for grouping, line continuation, or separation from adjacent tokens.
Compound expressions can remain unparenthesized in expression statements, assignment or return
values, and call arguments. In other contexts, including when `parent` is unknown, group them
conservatively.

The parent can be the enclosing expression or statement, or an `Arguments` or `Keyword` node.

Callers are responsible for determining whether removing the call is valid and whether discarding
its other arguments or comments affects the applicability of a fix.

---
