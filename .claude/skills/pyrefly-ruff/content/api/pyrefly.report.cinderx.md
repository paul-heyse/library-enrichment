# `pyrefly::report::cinderx`

Crate `pyrefly` · 4 public items · structured records in [`model/pyrefly.report.cinderx.json`](../model/pyrefly.report.cinderx.json)

## CinderxClassInfo

`struct` · `pyrefly::report::cinderx::CinderxClassInfo`

```rust
struct CinderxClassInfo
```

**Derives**: Clone, Debug

---

## CinderxClassRef

`struct` · `pyrefly::report::cinderx::CinderxClassRef`

```rust
struct CinderxClassRef
```

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

---

## CinderxReporter

`struct` · `pyrefly::report::cinderx::CinderxReporter`

```rust
struct CinderxReporter
```

**Methods** (3)

```rust
fn new(output_dir: &Path, handles: Option<&[Handle]>, readable: bool) -> anyhow::Result<Box<Self>>
fn report_module(&self, handle: &Handle, transaction: &Transaction<'_>) -> anyhow::Result<()>
fn write_project_files(&self, transaction: &Transaction<'_>) -> anyhow::Result<()>
```

Inline writer for CinderX report output during type checking.

---

## CinderxSolutions

`struct` · `pyrefly::report::cinderx::CinderxSolutions`

```rust
struct CinderxSolutions
```

**Derives**: Debug

**Methods** (3)

```rust
fn build<Ans: LookupAnswer>(bindings: &Bindings, answers: &AnswersSolver<'_, '_, Ans>) -> Arc<Self>
fn build_from_answers(bindings: &Bindings, answers: &Answers) -> Arc<Self>
fn get_class_info(&self, class_ref: &CinderxClassRef) -> Option<&CinderxClassInfo>
```

---
