# `ruff_linter::settings::flags`

Crate `ruff_linter` · 3 public items · structured records in [`model/ruff_linter.settings.flags.json`](../model/ruff_linter.settings.flags.json)

## Cache

`enum` · `ruff_linter::settings::flags::Cache`

```rust
enum Cache
```

**Variants**: `Enabled`, `Disabled`

**Implements**: `core::convert::From`

**Derives**: Clone, Copy, Debug, Hash

**Methods** (1)

```rust
const fn is_enabled(self) -> bool
```

**via `core::convert::From`**

```rust
fn from(value: bool) -> Self
```

---

## FixMode

`enum` · `ruff_linter::settings::flags::FixMode`

```rust
enum FixMode
```

**Variants**: `Generate`, `Apply`, `Diff`

**Derives**: Clone, Copy, Debug, Hash

**Methods** (3)

```rust
const fn is_apply(&self) -> bool
const fn is_diff(&self) -> bool
const fn is_generate(&self) -> bool
```

---

## Noqa

`enum` · `ruff_linter::settings::flags::Noqa`

```rust
enum Noqa
```

**Variants**: `Enabled`, `Disabled`

**Implements**: `core::convert::From`

**Derives**: Clone, Copy, Debug, Hash

**Methods** (1)

```rust
const fn is_enabled(self) -> bool
```

**via `core::convert::From`**

```rust
fn from(value: bool) -> Self
```

---
