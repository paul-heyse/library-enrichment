# `pyrefly::binding::pydantic`

Crate `pyrefly` · 18 public items · structured records in [`model/pyrefly.binding.pydantic.json`](../model/pyrefly.binding.pydantic.json)

## ALIAS_GENERATOR

`constant` · `pyrefly::binding::pydantic::ALIAS_GENERATOR`

```rust
const ALIAS_GENERATOR: ruff_python_ast::name::Name = _
```

---

## EXTRA

`constant` · `pyrefly::binding::pydantic::EXTRA`

```rust
const EXTRA: ruff_python_ast::name::Name = _
```

---

## FIELD_VALIDATOR

`constant` · `pyrefly::binding::pydantic::FIELD_VALIDATOR`

```rust
const FIELD_VALIDATOR: ruff_python_ast::name::Name = _
```

---

## FROZEN

`constant` · `pyrefly::binding::pydantic::FROZEN`

```rust
const FROZEN: ruff_python_ast::name::Name = _
```

---

## FROZEN_DEFAULT

`constant` · `pyrefly::binding::pydantic::FROZEN_DEFAULT`

```rust
const FROZEN_DEFAULT: bool = false
```

---

## GE

`constant` · `pyrefly::binding::pydantic::GE`

```rust
const GE: ruff_python_ast::name::Name = _
```

---

## GT

`constant` · `pyrefly::binding::pydantic::GT`

```rust
const GT: ruff_python_ast::name::Name = _
```

---

## LE

`constant` · `pyrefly::binding::pydantic::LE`

```rust
const LE: ruff_python_ast::name::Name = _
```

---

## LT

`constant` · `pyrefly::binding::pydantic::LT`

```rust
const LT: ruff_python_ast::name::Name = _
```

---

## POPULATE_BY_NAME

`constant` · `pyrefly::binding::pydantic::POPULATE_BY_NAME`

```rust
const POPULATE_BY_NAME: ruff_python_ast::name::Name = _
```

---

## ROOT

`constant` · `pyrefly::binding::pydantic::ROOT`

```rust
const ROOT: ruff_python_ast::name::Name = _
```

---

## STRICT

`constant` · `pyrefly::binding::pydantic::STRICT`

```rust
const STRICT: ruff_python_ast::name::Name = _
```

---

## STRICT_DEFAULT

`constant` · `pyrefly::binding::pydantic::STRICT_DEFAULT`

```rust
const STRICT_DEFAULT: bool = false
```

---

## VALIDATE_BY_ALIAS

`constant` · `pyrefly::binding::pydantic::VALIDATE_BY_ALIAS`

```rust
const VALIDATE_BY_ALIAS: ruff_python_ast::name::Name = _
```

---

## VALIDATE_BY_NAME

`constant` · `pyrefly::binding::pydantic::VALIDATE_BY_NAME`

```rust
const VALIDATE_BY_NAME: ruff_python_ast::name::Name = _
```

---

## VALIDATION_ALIAS

`constant` · `pyrefly::binding::pydantic::VALIDATION_ALIAS`

```rust
const VALIDATION_ALIAS: ruff_python_ast::name::Name = _
```

---

## PydanticAliasGenerator

`enum` · `pyrefly::binding::pydantic::PydanticAliasGenerator`

```rust
enum PydanticAliasGenerator
```

**Variants**: `ToCamel`, `ToPascal`, `ToSnake`

**Implements**: `pyrefly_types::equality::TypeEq`, `pyrefly_util::visit::VisitMut`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**Methods** (2)

```rust
fn from_special_export(export: SpecialExport) -> Option<Self>
fn generate(&self, field_name: &str) -> String
```

**via `pyrefly_types::equality::TypeEq`**

```rust
fn type_eq(&self, other: &Self, ctx: &mut equality::TypeEqCtx) -> bool
```

**via `pyrefly_util::visit::VisitMut`**

```rust
fn recurse_mut(&mut self, f: &mut dyn FnMut(&mut To))
```

---

## PydanticConfigDict

`struct` · `pyrefly::binding::pydantic::PydanticConfigDict`

```rust
struct PydanticConfigDict
```

**Fields**: `frozen`, `extra`, `strict`, `validate_by_name`, `validate_by_alias`, `populate_by_name`, `alias_generator`

**Derives**: Clone, Debug, Default

If a class body contains a `model_config` attribute assigned to a `pydantic.ConfigDict`, the
configuration options from the `ConfigDict`. In the answers phase, this will be merged with
configuration options from the class keywords to produce a full Pydantic model configuration.

---
