# Imported

`ruff_python_semantic::binding::Imported`

```rust
trait Imported<'a>
```

Also reachable as `ruff_python_semantic::Imported`

Prose: [`api/ruff_python_semantic.binding.md`](../api/ruff_python_semantic.binding.md#imported) · records: [`model/ruff_python_semantic.binding.json`](../model/ruff_python_semantic.binding.json)

## Required

Every implementation must supply these.

```rust
fn member_name(&self) -> Cow<'a, str>
fn module_name(&self) -> &[&'a str]
fn qualified_name(&self) -> &QualifiedName<'a>
fn source_name(&self) -> &[&'a str]
```

## Implementors (4)

Read one before writing your own.

- `ruff_python_semantic::binding::AnyImport`
- `ruff_python_semantic::binding::FromImport`
- `ruff_python_semantic::binding::Import`
- `ruff_python_semantic::binding::SubmoduleImport`

## Documentation

A trait for imported symbols.
