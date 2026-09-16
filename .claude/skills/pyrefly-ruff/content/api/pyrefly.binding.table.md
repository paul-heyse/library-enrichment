# `pyrefly::binding::table`

Crate `pyrefly` · 1 public items · structured records in [`model/pyrefly.binding.table.json`](../model/pyrefly.binding.table.json)

## TableKeyed

`trait` · `pyrefly::binding::table::TableKeyed`

```rust
trait TableKeyed<K>
```

**Implementors** (4)

- `pyrefly::alt::answers::AnswerTable`
- `pyrefly::alt::answers::SolutionsTable`
- `pyrefly::binding::bindings::BindingTable`
- `pyrefly::report::binding_memory::PhantomTable`

**Methods** (2)

```rust
fn get(&self) -> &Self::Value
fn get_mut(&mut self) -> &mut Self::Value
```

---
