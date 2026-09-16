# `ruff_linter::logging`

Crate `ruff_linter` · 5 public items · structured records in [`model/ruff_linter.logging.json`](../model/ruff_linter.logging.json)

## LogLevel

`enum` · `ruff_linter::logging::LogLevel`

```rust
enum LogLevel
```

**Variants**: `Silent`, `Quiet`, `Default`, `Verbose`

**Implements**: `core::convert::From`

**Derives**: Clone, Copy, Debug, Default, Eq, Ord, PartialEq, PartialOrd, StructuralPartialEq

---

## set_up_logging

`function` · `ruff_linter::logging::set_up_logging`

```rust
fn set_up_logging(level: LogLevel) -> anyhow::Result<()>
```

---

## IDENTIFIERS

`static` · `ruff_linter::logging::IDENTIFIERS`

```rust
static IDENTIFIERS: std::sync::LazyLock<std::sync::Mutex<Vec<&'static str>>>
```

---

## MESSAGES

`static` · `ruff_linter::logging::MESSAGES`

```rust
static MESSAGES: std::sync::LazyLock<std::sync::Mutex<rustc_hash::FxHashSet<String>>>
```

---

## DisplayParseError

`struct` · `ruff_linter::logging::DisplayParseError`

```rust
struct DisplayParseError
```

**Implements**: `core::error::Error`, `core::fmt::Display`

**Derives**: Debug

**Methods** (3)

```rust
fn error(&self) -> &ParseError
fn from_source_kind(error: ParseError, path: Option<PathBuf>, source_kind: &SourceKind) -> Self
fn path(&self) -> Option<&Path>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result
```

A wrapper around [`ParseError`] to translate byte offsets to user-facing
source code locations (typically, line and column numbers).

---
