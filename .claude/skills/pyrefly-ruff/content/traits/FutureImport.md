# FutureImport

`ruff_python_semantic::imports::FutureImport`

```rust
trait FutureImport
```

Also reachable as `ruff_python_semantic::FutureImport`

Prose: [`api/ruff_python_semantic.imports.md`](../api/ruff_python_semantic.imports.md#futureimport) · records: [`model/ruff_python_semantic.imports.json`](../model/ruff_python_semantic.imports.json)

## Required

Every implementation must supply these.

```rust
fn is_future_import(&self) -> bool
```

## Implementors (3)

Read one before writing your own.

- `ruff_python_semantic::imports::MemberNameImport`
- `ruff_python_semantic::imports::ModuleNameImport`
- `ruff_python_semantic::imports::NameImport`
