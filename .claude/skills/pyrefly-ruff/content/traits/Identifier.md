# Identifier

`ruff_python_ast::identifier::Identifier`

```rust
trait Identifier
```

Prose: [`api/ruff_python_ast.identifier.md`](../api/ruff_python_ast.identifier.md#identifier) · records: [`model/ruff_python_ast.identifier.json`](../model/ruff_python_ast.identifier.json)

## Required

Every implementation must supply these.

```rust
fn identifier(&self) -> TextRange
```

## Implementors (6)

Read one before writing your own.

- `ruff_python_ast::generated::Stmt`
- `ruff_python_ast::generated::StmtClassDef`
- `ruff_python_ast::generated::StmtFunctionDef`
- `ruff_python_ast::nodes::Alias`
- `ruff_python_ast::nodes::Parameter`
- `ruff_python_ast::nodes::ParameterWithDefault`
