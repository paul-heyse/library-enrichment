# `pyrefly::alt::types::pydantic`

Crate `pyrefly` · 3 public items · structured records in [`model/pyrefly.alt.types.pydantic.json`](../model/pyrefly.alt.types.pydantic.json)

## PydanticModelKind

`enum` · `pyrefly::alt::types::pydantic::PydanticModelKind`

```rust
enum PydanticModelKind
```

**Variants**: `BaseModel`, `RootModel`, `BaseSettings`, `DataClass`

**Implements**: `pyrefly_types::equality::TypeEq`, `pyrefly_util::visit::VisitMut`

**Derives**: Clone, Debug, Default, Eq, PartialEq, StructuralPartialEq

**via `pyrefly_types::equality::TypeEq`**

```rust
fn type_eq(&self, other: &Self, ctx: &mut equality::TypeEqCtx) -> bool
```

**via `pyrefly_util::visit::VisitMut`**

```rust
fn recurse_mut(&mut self, f: &mut dyn FnMut(&mut To))
```

---

## PydanticConfig

`struct` · `pyrefly::alt::types::pydantic::PydanticConfig`

```rust
struct PydanticConfig
```

**Fields**: `frozen`, `validation_flags`, `validation_alias_generator`, `extra`, `strict`, `pydantic_model_kind`

**Implements**: `pyrefly_types::equality::TypeEq`, `pyrefly_util::visit::VisitMut`

**Derives**: Clone, Debug, Default, Eq, PartialEq, StructuralPartialEq

**via `pyrefly_types::equality::TypeEq`**

```rust
fn type_eq(&self, other: &Self, ctx: &mut equality::TypeEqCtx) -> bool
```

**via `pyrefly_util::visit::VisitMut`**

```rust
fn recurse_mut(&mut self, f: &mut dyn FnMut(&mut To))
```

Configuration for a Pydantic model.
For pydantic dataclasses, `frozen`, `extra`, and `strict` are `None` because
they come from decorator arguments via dataclass_transform, not from this config.

---

## PydanticValidationFlags

`struct` · `pyrefly::alt::types::pydantic::PydanticValidationFlags`

```rust
struct PydanticValidationFlags
```

**Fields**: `validate_by_name`, `validate_by_alias`

**Implements**: `pyrefly_types::equality::TypeEq`

**Derives**: Clone, Debug, Default, Eq, PartialEq, StructuralPartialEq

**Methods** (2)

```rust
fn validate_by_alias(&self) -> bool
fn validate_by_name(&self) -> bool
```

**via `pyrefly_types::equality::TypeEq`**

```rust
fn type_eq(&self, other: &Self, ctx: &mut equality::TypeEqCtx) -> bool
```

Options that control whether fields are populated by their names or aliases.
`None` means the option was not configured and its Pydantic default applies.

---
