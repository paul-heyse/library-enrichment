# `ruff_linter::violation`

Crate `ruff_linter` · 4 public items · structured records in [`model/ruff_linter.violation.json`](../model/ruff_linter.violation.json)

## FixAvailability

`enum` · `ruff_linter::violation::FixAvailability`

Also reachable as `ruff_linter::FixAvailability`

```rust
enum FixAvailability
```

**Variants**: `Sometimes`, `Always`, `None`

**Implements**: `core::fmt::Display`, `serde_core::ser::Serialize`

**Derives**: Clone, Copy, Debug

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

---

## AlwaysFixableViolation

`trait` · `ruff_linter::violation::AlwaysFixableViolation`

Also reachable as `ruff_linter::AlwaysFixableViolation`

```rust
trait AlwaysFixableViolation: ViolationMetadata
```

**Methods** (3)

```rust
fn fix_title(&self) -> String
fn message(&self) -> String
fn message_formats() -> &'static [&'static str]
```

This trait exists just to make implementing the [`Violation`] trait more
convenient for violations that can always be fixed.

---

## Violation

`trait` · `ruff_linter::violation::Violation`

Also reachable as `ruff_linter::Violation`

```rust
trait Violation: ViolationMetadata + Sized
```

**Implementors** (1)

- `ruff_linter::rules::pycodestyle::rules::errors::IOError`

**Methods** (4)

```rust
fn fix_title(&self) -> Option<String>
fn into_diagnostic(self, range: TextRange, file: &SourceFile) -> Diagnostic
fn message(&self) -> String
fn message_formats() -> &'static [&'static str]
```

---

## ViolationMetadata

`trait` · `ruff_linter::violation::ViolationMetadata`

Also reachable as `ruff_linter::ViolationMetadata`

```rust
trait ViolationMetadata
```

**Implementors** (1)

- `ruff_linter::rules::pycodestyle::rules::errors::IOError`

**Methods** (6)

```rust
fn category() -> Category
fn explain() -> Option<&'static str>
fn file() -> &'static str
fn line() -> u32
fn rule() -> Rule
fn status() -> RuleStatus
```

---
