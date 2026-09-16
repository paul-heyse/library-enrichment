# `pyrefly::error::error`

Crate `pyrefly` · 5 public items · structured records in [`model/pyrefly.error.error.json`](../model/pyrefly.error.error.json)

## BaselineStatus

`enum` · `pyrefly::error::error::BaselineStatus`

```rust
enum BaselineStatus
```

**Variants**: `NotConfigured`, `NotCompared`, `Unmatched`, `Matched`

**Derives**: Clone, Copy, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (2)

```rust
fn display_suffix(self) -> &'static str
fn legacy_baselined_flag(self) -> Option<bool>
```

Whether an error was compared with the configured baseline.

---

## ErrorQuickFix

`enum` · `pyrefly::error::error::ErrorQuickFix`

```rust
enum ErrorQuickFix
```

**Variants**: `ReplaceWithEnumMember`, `AssertNotNone`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

---

## Error

`struct` · `pyrefly::error::error::Error`

```rust
struct Error
```

**Implements**: `ruff_text_size::traits::Ranged`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (22)

```rust
fn baseline_status(&self) -> BaselineStatus
fn display_range(&self) -> &DisplayRange
fn error_kind(&self) -> ErrorKind
fn get_notebook_cell(&self) -> Option<usize>
fn lined_buffer(&self) -> &LinedBuffer
fn module(&self) -> &Module
fn msg(&self) -> String
fn msg_details(&self) -> Option<&str>
fn msg_header(&self) -> &str
fn new(module: Module, range: TextRange, header: String, details: Vec<String>, error_kind: ErrorKind) -> Self
fn path(&self) -> &ModulePath
fn path_string_with_fragment(&self, project_root: &Path) -> String
fn quick_fixes(&self) -> &[ErrorQuickFix]
fn secondary_annotations(&self) -> &[SecondaryAnnotation]
fn severity(&self) -> Severity
fn suppression_effect(&self, enabled_ignores: &SmallSet<Tool>, type_ignore_unknown_tag_behavior: TypeIgnoreUnknownTagBehavior) -> SuppressionEffect
fn to_diagnostic(&self) -> Diagnostic
fn with_annotation(self, range: TextRange, label: String) -> Self
fn with_baseline_status(self, baseline_status: BaselineStatus) -> Self
fn with_quick_fix(self, quick_fix: ErrorQuickFix) -> Self
fn with_severity(&self, severity: Severity) -> Self
fn without_deprecated_tag(self) -> Self
```

**via `ruff_text_size::traits::Ranged`**

```rust
fn range(&self) -> TextRange
```

---

## SecondaryAnnotation

`struct` · `pyrefly::error::error::SecondaryAnnotation`

```rust
struct SecondaryAnnotation
```

**Fields**: `range`, `label`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

A secondary annotation that labels a span in the same file as the primary error.
Used to show additional context, e.g. the types of both operands in a binary operation.

---

## SerializableError

`struct` · `pyrefly::error::error::SerializableError`

Also reachable as `pyrefly::library::library::library::library::SerializableError`

```rust
struct SerializableError
```

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Debug, Eq, PartialEq, StructuralPartialEq

**Methods** (3)

```rust
fn from_error(relative_to: &Path, error: &Error) -> Self
fn into_legacy_error(self) -> LegacyError
fn legacy_error(&self) -> &LegacyError
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

An error representation that preserves the data needed for supported CLI output formats.

---
