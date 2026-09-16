# The AST and traversal

`ruff_python_ast` is generated code, and its scale is the point: `AnyNodeRef` has 610 methods and `Expr` 211. There are three traversal styles -- `Visitor` (owned), `SourceOrderVisitor` (in source order, which is what you want for anything position-sensitive) and `Transformer` (rewriting). Choosing the wrong one produces subtly wrong ordering rather than an error.

## Entry points

| Type | Kind | Methods | Prose | Records |
|---|---|---:|---|---|
| `ruff_python_ast::generated::Stmt` | enum | 156 | [prose](../api/ruff_python_ast.generated.md#stmt) | [records](../model/ruff_python_ast.generated.json) |
| `ruff_python_ast::generated::Expr` | enum | 211 | [prose](../api/ruff_python_ast.generated.md#expr) | [records](../model/ruff_python_ast.generated.json) |
| `ruff_python_ast::generated::AnyNodeRef` | enum | 610 | [prose](../api/ruff_python_ast.generated.md#anynoderef) | [records](../model/ruff_python_ast.generated.json) |
| `ruff_python_ast::visitor::Visitor` | trait | 30 | [prose](../api/ruff_python_ast.visitor.md#visitor) | [records](../model/ruff_python_ast.visitor.json) |
| `ruff_python_ast::visitor::source_order::SourceOrderVisitor` | trait | 35 | [prose](../api/ruff_python_ast.visitor.source_order.md#sourceordervisitor) | [records](../model/ruff_python_ast.visitor.source_order.json) |
| `ruff_python_ast::visitor::transformer::Transformer` | trait | 30 | [prose](../api/ruff_python_ast.visitor.transformer.md#transformer) | [records](../model/ruff_python_ast.visitor.transformer.json) |

## Decision rules

- Position-sensitive work? `SourceOrderVisitor`.
- Rewriting? `Transformer`.
- Just looking for a shape? Consider ast-grep instead of a Rust visitor.

## Anti-patterns

- Matching on `AnyNodeRef` when a narrower enum would be exhaustive.
- Assuming visitor order is source order -- only one of the three guarantees it.

## Agent checklist

- Trivia (comments, whitespace) is not in the AST; it comes from `ruff_python_trivia`.
