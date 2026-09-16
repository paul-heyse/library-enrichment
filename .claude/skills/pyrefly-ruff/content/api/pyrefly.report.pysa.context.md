# `pyrefly::report::pysa::context`

Crate `pyrefly` · 3 public items · structured records in [`model/pyrefly.report.pysa.context.json`](../model/pyrefly.report.pysa.context.json)

## ModuleAnswersContext

`struct` · `pyrefly::report::pysa::context::ModuleAnswersContext`

```rust
struct ModuleAnswersContext
```

**Fields**: `handle`, `module_id`, `module_info`, `stdlib`, `ast`, `bindings`, `answers`

**Implements**: `dupe::Dupe`

**Derives**: Clone

**Methods** (2)

```rust
fn create(handle: Handle, transaction: &Transaction<'_>, module_ids: &ModuleIds) -> ModuleAnswersContext
fn undecorated_function(&self, idx: Idx<KeyDecoratedFunction>) -> &UndecoratedFunction
```

Pyrefly information about a single module.

This is available when building `PysaSolutions`, which includes all the
information we will need about a module.

---

## ModuleContext

`struct` · `pyrefly::report::pysa::context::ModuleContext`

```rust
struct ModuleContext<'a>
```

**Fields**: `answers_context`, `resolver`

**Methods** (1)

```rust
fn module_ids(&self) -> &ModuleIds
```

Pyrefly information about a module.

This includes the `resolver` which allows access to cross-module information.

---

## PysaResolver

`struct` · `pyrefly::report::pysa::context::PysaResolver`

```rust
struct PysaResolver<'a>
```

**Methods** (8)

```rust
fn current_module_solutions(&self) -> &PysaSolutions
fn find_definition(&self, position: TextSize, preference: FindPreference) -> Vec<FindDefinitionItemWithDocstring>
fn find_definition_for_attribute(&self, base_range: TextRange, name: &Name, preference: FindPreference) -> Vec<FindDefinitionItemWithDocstring>
fn find_definition_for_name_use(&self, name: &Identifier, preference: FindPreference) -> Option<FindDefinitionItemWithDocstring>
fn get_cached_solutions(&self, module_id: ModuleId) -> Arc<PysaSolutions>
fn module_ids(&self) -> &ModuleIds
fn new(transaction: &'a Transaction<'a>, module_ids: &'a ModuleIds, current_handle: Handle) -> Self
fn resolve_pysa_solutions(&self, module: &Module) -> Arc<PysaSolutions>
```

Wraps `&Transaction` to provide only the cross-module operations that pysa needs.
Owns a local cache of resolved PysaSolutions to avoid redundant lookups.

---
