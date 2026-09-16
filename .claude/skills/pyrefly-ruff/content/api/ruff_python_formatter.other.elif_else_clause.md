# `ruff_python_formatter::other::elif_else_clause`

Crate `ruff_python_formatter` · 1 public items · structured records in [`model/ruff_python_formatter.other.elif_else_clause.json`](../model/ruff_python_formatter.other.elif_else_clause.json)

## FormatElifElseClause

`struct` · `ruff_python_formatter::other::elif_else_clause::FormatElifElseClause`

```rust
struct FormatElifElseClause
```

Note that this implementation misses the leading newlines before the leading comments because
it does not have access to the last node of the previous branch. The `StmtIf` therefore doesn't
call this but `format_elif_else_clause` directly.

---
