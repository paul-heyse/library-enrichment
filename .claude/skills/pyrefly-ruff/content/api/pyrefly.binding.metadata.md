# `pyrefly::binding::metadata`

Crate `pyrefly` · 2 public items · structured records in [`model/pyrefly.binding.metadata.json`](../model/pyrefly.binding.metadata.json)

## BindingsMetadata

`struct` · `pyrefly::binding::metadata::BindingsMetadata`

```rust
struct BindingsMetadata
```

**Derives**: Clone, Debug

**Methods** (7)

```rust
fn get_class(&self, idx: ClassDefIndex) -> &ClassMetadata
fn get_class_checked(&self, idx: ClassDefIndex) -> Option<&ClassMetadata>
fn get_class_mut(&mut self, idx: ClassDefIndex) -> &mut ClassMetadata
fn new() -> Self
fn push_class(&mut self) -> ClassDefIndex
fn push_shape_dsl(&mut self, name: Name, dsl_fn: Arc<ShapeDslFunction>)
fn shape_dsl_functions(&self) -> &[(Name, Arc<ShapeDslFunction>)]
```

Metadata collected during the binding phase for all classes in a module.

Stored in an `Arc` so it can be shared between `Bindings`/`Answers` and
`Solutions` without copying. During access, callers hold a `Guard` or
borrow through the `Arc` rather than cloning it, avoiding contended
atomic reference count operations.

---

## ClassMetadata

`struct` · `pyrefly::binding::metadata::ClassMetadata`

```rust
struct ClassMetadata
```

**Fields**: `fields`

**Derives**: Clone, Debug, Default

Metadata for a single class definition, populated during binding.

---
