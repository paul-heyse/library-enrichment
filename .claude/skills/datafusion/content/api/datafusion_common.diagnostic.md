# `datafusion_common::diagnostic`

Crate `datafusion-common` · 4 public items · structured records in [`model/datafusion_common.diagnostic.json`](../model/datafusion_common.diagnostic.json)

## DiagnosticKind

`enum` · `datafusion_common::diagnostic::DiagnosticKind`

```rust
enum DiagnosticKind
```

**Variants**: `Error`, `Warning`

**Derives**: Clone, Copy, Debug, Eq, PartialEq, StructuralPartialEq

A [`Diagnostic`] can either be a hard error that prevents the query from
being planned and executed, or a warning that indicates potential issues,
performance problems, or causes for unexpected results, but is non-fatal.
This enum expresses these two possibilities.

---

## Diagnostic

`struct` · `datafusion_common::diagnostic::Diagnostic`

Also reachable as `datafusion::common::Diagnostic`, `datafusion_common::Diagnostic`

```rust
struct Diagnostic
```

**Fields**: `kind`, `message`, `span`, `notes`, `helps`

**Derives**: Clone, Debug

**Methods** (6)

```rust
fn add_help(&mut self, message: impl Into<String>, span: Option<Span>)
fn add_note(&mut self, message: impl Into<String>, span: Option<Span>)
fn new_error(message: impl Into<String>, span: Option<Span>) -> Self
fn new_warning(message: impl Into<String>, span: Option<Span>) -> Self
fn with_help(self, message: impl Into<String>, span: Option<Span>) -> Self
fn with_note(self, message: impl Into<String>, span: Option<Span>) -> Self
```

Additional contextual information intended for end users, to help them
understand what went wrong by providing human-readable messages, and
locations in the source query that relate to the error in some way.

You can think of a single [`Diagnostic`] as a single "block" of output from
rustc. i.e. either an error or a warning, optionally with some notes and
help messages.

Example:

```rust
# use datafusion_common::{Location, Span, Diagnostic};
let span = Some(Span {
    start: Location { line: 2, column: 1 },
    end: Location {
        line: 4,
        column: 15,
    },
});
let diagnostic = Diagnostic::new_error("Something went wrong", span)
    .with_help("Have you tried turning it on and off again?", None);
```

---

## DiagnosticHelp

`struct` · `datafusion_common::diagnostic::DiagnosticHelp`

```rust
struct DiagnosticHelp
```

**Fields**: `message`, `span`

**Derives**: Clone, Debug

A "help" enriches a [`Diagnostic`] with extra information, possibly
referring to different locations in the original SQL query, that helps the
user understand how they might fix the error or warning.

Example:
SELECT id, name FROM users GROUP BY id
Help: Add 'name' here                 ^^^^

---

## DiagnosticNote

`struct` · `datafusion_common::diagnostic::DiagnosticNote`

```rust
struct DiagnosticNote
```

**Fields**: `message`, `span`

**Derives**: Clone, Debug

A note enriches a [`Diagnostic`] with extra information, possibly referring
to different locations in the original SQL query, that helps contextualize
the error and helps the end user understand why it occurred.

Example:
SELECT id, name FROM users GROUP BY id
Note:      ^^^^ 'name' is not in the GROUP BY clause

---
