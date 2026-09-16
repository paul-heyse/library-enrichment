# `ruff_python_semantic::context`

Crate `ruff_python_semantic` · 1 public items · structured records in [`model/ruff_python_semantic.context.json`](../model/ruff_python_semantic.context.json)

## ExecutionContext

`enum` · `ruff_python_semantic::context::ExecutionContext`

Also reachable as `ruff_python_semantic::ExecutionContext`

```rust
enum ExecutionContext
```

**Variants**: `Runtime`, `Typing`

**Derives**: Clone, Copy, Debug

**Methods** (2)

```rust
const fn is_runtime(&self) -> bool
const fn is_typing(&self) -> bool
```

---
