# `pyrefly::report::pysa::global_variable`

Crate `pyrefly` · 2 public items · structured records in [`model/pyrefly.report.pysa.global_variable.json`](../model/pyrefly.report.pysa.global_variable.json)

## GlobalVariableBase

`struct` · `pyrefly::report::pysa::global_variable::GlobalVariableBase`

```rust
struct GlobalVariableBase
```

**Fields**: `type_`, `name`

**Derives**: Clone, Debug

Represents information about a global variable, collected in a pre-analysis step.
See `GlobalVariable` for the type exported to Pysa. This only store memory-efficient information.

---

## ModuleGlobalVariables

`struct` · `pyrefly::report::pysa::global_variable::ModuleGlobalVariables`

```rust
struct ModuleGlobalVariables
```

**Derives**: Clone, Debug

**Methods** (2)

```rust
fn contains(&self, name: &Name) -> bool
fn get(&self, short_identifier: ShortIdentifier) -> Option<&GlobalVariableBase>
```

---
