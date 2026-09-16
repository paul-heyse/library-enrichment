# `pyrefly::error::collector`

Crate `pyrefly` · 3 public items · structured records in [`model/pyrefly.error.collector.json`](../model/pyrefly.error.collector.json)

## CollectedErrors

`struct` · `pyrefly::error::collector::CollectedErrors`

```rust
struct CollectedErrors
```

**Fields**: `ordinary`, `directives`, `suppressed`, `disabled`, `baseline`

**Derives**: Debug, Default

---

## ErrorBuilder

`struct` · `pyrefly::error::collector::ErrorBuilder`

```rust
struct ErrorBuilder<'a>
```

**Methods** (9)

```rust
fn emit(self)
fn with_annotation(self, range: TextRange, label: String) -> Self
fn with_context(self, ctx: Option<impl FnOnce() -> ErrorContext>) -> Self
fn with_detail(self, msg: String) -> Self
fn with_detail_from(self, msg: impl FnOnce() -> Option<String>) -> Self
fn with_details(self, details: Vec<String>) -> Self
fn with_errors_as_details(self, collector: ErrorCollector) -> Self
fn with_quick_fix(self, fix: ErrorQuickFix) -> Self
fn without_deprecated_tag(self) -> Self
```

A builder for constructing and emitting errors incrementally.
Chain decoration methods and call `.emit()` to push the error into the collector.

---

## ErrorCollector

`struct` · `pyrefly::error::collector::ErrorCollector`

```rust
struct ErrorCollector
```

**Derives**: Debug

**Methods** (13)

```rust
fn collect(&self, error_config: &ErrorConfig<'_>) -> CollectedErrors
fn collect_into(&self, error_config: &ErrorConfig<'_>, fstring_ranges: &[(LineNumber, LineNumber)], ignore_all: &[Suppression], misplaced: &[LineNumber], result: &mut CollectedErrors)
fn error_builder(&self, range: TextRange, kind: ErrorKind, header: String) -> ErrorBuilder<'_>
fn extend(&self, other: ErrorCollector)
fn has_hard(&self) -> bool
fn internal_error(&self, range: TextRange, header: String)
fn is_active(&self) -> bool
fn is_empty(&self) -> bool
fn len(&self) -> usize
fn len_hard(&self) -> usize
fn module(&self) -> &pyrefly_python::module::Module
fn new(module_info: pyrefly_python::module::Module, style: ErrorStyle) -> Self
fn style(&self) -> ErrorStyle
```

Collects the user errors (e.g. type errors) associated with a module.

---
