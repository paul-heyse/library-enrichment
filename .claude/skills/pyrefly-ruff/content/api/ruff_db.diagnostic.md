# `ruff_db::diagnostic`

Crate `ruff_db` · 17 public items · structured records in [`model/ruff_db.diagnostic.json`](../model/ruff_db.diagnostic.json)

## ConciseMessage

`enum` · `ruff_db::diagnostic::ConciseMessage`

```rust
enum ConciseMessage<'a>
```

**Variants**: `MainDiagnostic`, `Both`, `Custom`

**Implements**: `core::fmt::Display`

**Methods** (1)

```rust
fn to_str(&self) -> Cow<'a, str>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

A representation of the kinds of messages inside a diagnostic.

---

## DiagnosticFormat

`enum` · `ruff_db::diagnostic::DiagnosticFormat`

```rust
enum DiagnosticFormat
```

**Variants**: `Full`, `Concise`, `Azure`, `Pylint`, `Github`

**Implements**: `core::convert::TryFrom`

**Derives**: Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

The diagnostic output format.

---

## DiagnosticId

`enum` · `ruff_db::diagnostic::DiagnosticId`

```rust
enum DiagnosticId
```

**Variants**: `Panic`, `Io`, `InvalidSyntax`, `Lint`, `RevealedType`, `UnknownRule`, `InvalidGlob`, `InvalidScriptMetadata`, `UvMetadata`, `EmptyInclude`, `UnnecessaryOverridesSection`, `UselessOverridesSection`, `DeprecatedSetting`, `UnsupportedPythonVersion`, `Unformatted`, `InvalidCliOption`, `PreviewFeature`, `InternalError`

**Implements**: `core::fmt::Display`, `get_size2::GetSize`

**Derives**: Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (6)

```rust
const fn as_lint(&self) -> Option<LintName>
fn as_str(&self) -> &'static str
fn is_lint(&self) -> bool
fn is_lint_named(&self, name: &str) -> bool
const fn lint(name: &'static str) -> Self
fn strip_category(code: &str) -> Option<&str>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result
```

**via `get_size2::GetSize`**

```rust
fn get_heap_size(&self) -> usize
fn get_heap_size_with_tracker<TRACKER: ::get_size2::GetSizeTracker>(&self, tracker: TRACKER) -> (usize, TRACKER)
```

Uniquely identifies the kind of a diagnostic.

---

## DiagnosticTag

`enum` · `ruff_db::diagnostic::DiagnosticTag`

```rust
enum DiagnosticTag
```

**Variants**: `Unnecessary`, `Deprecated`

**Implements**: `get_size2::GetSize`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**via `get_size2::GetSize`**

```rust
fn get_heap_size(&self) -> usize
fn get_heap_size_with_tracker<TRACKER: ::get_size2::GetSizeTracker>(&self, tracker: TRACKER) -> (usize, TRACKER)
```

Tags that can be associated with an annotation.

These tags are used to provide additional information about the annotation.
and are passed through to the language server protocol.

---

## HyperlinkMode

`enum` · `ruff_db::diagnostic::HyperlinkMode`

```rust
enum HyperlinkMode
```

**Variants**: `Auto`, `Always`, `Never`

**Derives**: Clone, Copy, Debug, Default, Eq, PartialEq, StructuralPartialEq

Controls whether colored diagnostic output includes hyperlinks.

---

## Severity

`enum` · `ruff_db::diagnostic::Severity`

```rust
enum Severity
```

**Variants**: `Info`, `Warning`, `Error`, `Fatal`

**Implements**: `get_size2::GetSize`

**Derives**: Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (1)

```rust
const fn is_fatal(self) -> bool
```

**via `get_size2::GetSize`**

```rust
fn get_heap_size(&self) -> usize
fn get_heap_size_with_tracker<TRACKER: ::get_size2::GetSizeTracker>(&self, tracker: TRACKER) -> (usize, TRACKER)
```

---

## SubDiagnosticSeverity

`enum` · `ruff_db::diagnostic::SubDiagnosticSeverity`

```rust
enum SubDiagnosticSeverity
```

**Variants**: `Help`, `Info`, `Warning`, `Error`, `Fatal`

**Implements**: `core::fmt::Display`, `get_size2::GetSize`

**Derives**: Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `get_size2::GetSize`**

```rust
fn get_heap_size(&self) -> usize
fn get_heap_size_with_tracker<TRACKER: ::get_size2::GetSizeTracker>(&self, tracker: TRACKER) -> (usize, TRACKER)
```

Like [`Severity`] but exclusively for sub-diagnostics.

This type only exists to add an additional `Help` severity that isn't present in `Severity` or
used for main diagnostics. If we want to add `Severity::Help` in the future, this type could be
deleted and the two combined again.

---

## UnifiedFile

`enum` · `ruff_db::diagnostic::UnifiedFile`

```rust
enum UnifiedFile
```

**Variants**: `Ty`, `Ruff`

**Implements**: `get_size2::GetSize`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**via `get_size2::GetSize`**

```rust
fn get_heap_size(&self) -> usize
fn get_heap_size_with_tracker<TRACKER: ::get_size2::GetSizeTracker>(&self, tracker: TRACKER) -> (usize, TRACKER)
```

A unified file representation for both ruff and ty.

Such a representation is needed for rendering [`Diagnostic`]s that can optionally contain
[`Annotation`]s with [`Span`]s that need to refer to the text of a file. However, ty and ruff
use very different file types: a `Copy`-able salsa-interned [`File`], and a heavier-weight
[`SourceFile`], respectively.

This enum presents a unified interface to these two types for the sake of creating [`Span`]s and
emitting diagnostics from both ty and ruff.

---

## Annotation

`struct` · `ruff_db::diagnostic::Annotation`

```rust
struct Annotation
```

**Implements**: `get_size2::GetSize`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (10)

```rust
fn get_message(&self) -> Option<&str>
fn get_span(&self) -> &Span
fn hide_snippet(&mut self, yes: bool)
fn is_primary(&self) -> bool
fn message<'a>(self, message: impl IntoDiagnosticMessage + 'a) -> Annotation
fn primary(span: Span) -> Annotation
fn push_tag(&mut self, tag: DiagnosticTag)
fn secondary(span: Span) -> Annotation
fn set_message<'a>(&mut self, message: impl IntoDiagnosticMessage + 'a)
fn set_span(&mut self, span: Span)
```

**via `get_size2::GetSize`**

```rust
fn get_heap_size(&self) -> usize
fn get_heap_size_with_tracker<TRACKER: ::get_size2::GetSizeTracker>(&self, tracker: TRACKER) -> (usize, TRACKER)
```

A pointer to a subsequence in the end user's input.

Also known as an annotation, the pointer can optionally contain a short
message, typically describing in general terms what is being pointed to.

An annotation is either primary or secondary, depending on whether it was
constructed via [`Annotation::primary`] or [`Annotation::secondary`].
Semantically, a primary annotation is meant to point to the "locus" of a
diagnostic. Visually, the difference between a primary and a secondary
annotation is usually just a different form of highlighting on the
corresponding span.

# Advice

The span on an annotation should be as _specific_ as possible. For example,
if there is a problem with a function call because one of its arguments has
an invalid type, then the span should point to the specific argument and
not to the entire function call.

Messages attached to annotations should also be as brief and specific as
possible. Long messages could negative impact the quality of rendering.

---

## Diagnostic

`struct` · `ruff_db::diagnostic::Diagnostic`

```rust
struct Diagnostic
```

**Implements**: `core::convert::From`, `get_size2::GetSize`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (50)

```rust
fn add_bug_sub_diagnostics(&mut self, url_encoded_title: &str)
fn annotate(&mut self, ann: Annotation)
fn annotations(&self) -> &[Annotation]
fn annotations_mut(&mut self) -> impl Iterator<Item = &mut Annotation>
fn clear_concise_message(&mut self)
fn concise_message(&self) -> ConciseMessage<'_>
fn display<'a>(&'a self, resolver: &'a dyn FileResolver, config: &'a DisplayDiagnosticConfig) -> DisplayDiagnostic<'a>
fn documentation_url(&self) -> Option<&str>
fn expect_primary_span(&self) -> Span
fn expect_ruff_filename(&self) -> String
fn first_help_text(&self) -> Option<&str>
fn fix(&self) -> Option<&Fix>
fn has_applicable_fix(&self, fix_applicability: Applicability) -> bool
fn headline_message(&self) -> &str
fn help<'a>(&mut self, message: impl IntoDiagnosticMessage + 'a)
fn id(&self) -> DiagnosticId
fn info<'a>(&mut self, message: impl IntoDiagnosticMessage + 'a)
fn invalid_syntax(span: impl Into<Span>, message: impl IntoDiagnosticMessage, range: impl Ranged) -> Diagnostic
fn is_invalid_syntax(&self) -> bool
fn name(&self) -> &'static str
fn new<'a>(id: DiagnosticId, severity: Severity, message: impl IntoDiagnosticMessage + 'a) -> Diagnostic
fn parent(&self) -> Option<TextSize>
fn prepend_info<'a>(&mut self, message: impl IntoDiagnosticMessage + 'a)
fn primary_annotation(&self) -> Option<&Annotation>
fn primary_annotation_mut(&mut self) -> Option<&mut Annotation>
fn primary_span(&self) -> Option<Span>
fn primary_tags(&self) -> Option<&[DiagnosticTag]>
fn range(&self) -> Option<TextRange>
fn remove_fix(&mut self)
fn rendering_sort_key<'a>(&'a self, db: &'a dyn Db) -> impl Ord + 'a
fn ruff_end_location(&self) -> Option<LineColumn>
fn ruff_source_file(&self) -> Option<&SourceFile>
fn ruff_start_location(&self) -> Option<LineColumn>
fn ruff_start_ordering(&self, other: &Self) -> std::cmp::Ordering
fn secondary_annotations(&self) -> impl Iterator<Item = &Annotation>
fn secondary_code(&self) -> Option<&SecondaryCode>
fn secondary_code_or_id(&self) -> &str
fn set_concise_message(&mut self, message: impl IntoDiagnosticMessage)
fn set_documentation_url(&mut self, url: Option<String>)
fn set_fix(&mut self, fix: Fix)
fn set_header_offset(&mut self, offset: usize)
fn set_headline_message(&mut self, message: impl IntoDiagnosticMessage)
fn set_noqa_offset(&mut self, noqa_offset: TextSize)
fn set_optional_fix(&mut self, fix: Option<Fix>)
fn set_parent(&mut self, parent: TextSize)
fn set_secondary_code(&mut self, code: SecondaryCode)
fn severity(&self) -> Severity
fn sub(&mut self, sub: SubDiagnostic)
fn sub_diagnostics(&self) -> &[SubDiagnostic]
fn sub_diagnostics_mut(&mut self) -> impl Iterator<Item = &mut SubDiagnostic>
```

**via `get_size2::GetSize`**

```rust
fn get_heap_size(&self) -> usize
fn get_heap_size_with_tracker<TRACKER: ::get_size2::GetSizeTracker>(&self, tracker: TRACKER) -> (usize, TRACKER)
```

A collection of information that can be rendered into a diagnostic.

A diagnostic is a collection of information gathered by a tool intended
for presentation to an end user, and which describes a group of related
characteristics in the inputs given to the tool. Typically, but not always,
a characteristic is a deficiency. An example of a characteristic that is
_not_ a deficiency is the `reveal_type` diagnostic for our type checker.

---

## DiagnosticMessage

`struct` · `ruff_db::diagnostic::DiagnosticMessage`

```rust
struct DiagnosticMessage
```

**Implements**: `core::convert::From`, `get_size2::GetSize`, `ruff_db::diagnostic::IntoDiagnosticMessage`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn as_str(&self) -> &str
```

**via `core::convert::From`**

```rust
fn from(s: String) -> DiagnosticMessage
fn from(s: Box<str>) -> DiagnosticMessage
fn from(s: &str) -> DiagnosticMessage
```

**via `get_size2::GetSize`**

```rust
fn get_heap_size(&self) -> usize
fn get_heap_size_with_tracker<TRACKER: ::get_size2::GetSizeTracker>(&self, tracker: TRACKER) -> (usize, TRACKER)
```

**via `ruff_db::diagnostic::IntoDiagnosticMessage`**

```rust
fn into_diagnostic_message(self) -> DiagnosticMessage
```

A diagnostic message string.

This is, for all intents and purposes, equivalent to a `Box<str>`.
But it does not implement `std::fmt::Display`. Indeed, that it its
entire reason for existence. It provides a way to pass a string
directly into diagnostic methods that accept messages without copying
that string. This works via the `IntoDiagnosticMessage` trait.

In most cases, callers shouldn't need to use this. Instead, there is
a blanket trait implementation for `IntoDiagnosticMessage` for
anything that implements `std::fmt::Display`.

---

## DisplayDiagnosticConfig

`struct` · `ruff_db::diagnostic::DisplayDiagnosticConfig`

```rust
struct DisplayDiagnosticConfig
```

**Derives**: Clone, Debug

**Methods** (16)

```rust
fn anonymized_line_numbers(self, yes: bool) -> DisplayDiagnosticConfig
fn color(self, yes: bool) -> DisplayDiagnosticConfig
fn context(self, lines: usize) -> DisplayDiagnosticConfig
fn fix_applicability(&self) -> Applicability
fn format(self, format: DiagnosticFormat) -> DisplayDiagnosticConfig
fn hide_severity(self, yes: bool) -> DisplayDiagnosticConfig
fn hyperlinks(self, mode: HyperlinkMode) -> DisplayDiagnosticConfig
fn is_prefer_rule_codes_enabled(&self) -> bool
fn new(program: &'static str) -> DisplayDiagnosticConfig
fn prefer_rule_codes(self, yes: bool) -> DisplayDiagnosticConfig
fn preview(self, yes: bool) -> DisplayDiagnosticConfig
fn preview_enabled(&self) -> bool
fn show_fix_status(&self) -> bool
fn with_cancellation_token(self, token: Option<CancellationToken>) -> DisplayDiagnosticConfig
fn with_fix_applicability(self, applicability: Applicability) -> DisplayDiagnosticConfig
fn with_show_fix_status(self, yes: bool) -> DisplayDiagnosticConfig
```

Configuration for rendering diagnostics.

---

## LintName

`struct` · `ruff_db::diagnostic::LintName`

```rust
struct LintName
```

**Implements**: `core::fmt::Display`, `core::ops::deref::Deref`, `get_size2::GetSize`

**Derives**: Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (2)

```rust
const fn as_str(&self) -> &'static str
const fn of(name: &'static str) -> Self
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result
```

**via `core::ops::deref::Deref`**

```rust
fn deref(&self) -> &Self::Target
```

**via `get_size2::GetSize`**

```rust
fn get_heap_size(&self) -> usize
fn get_heap_size_with_tracker<TRACKER: ::get_size2::GetSizeTracker>(&self, tracker: TRACKER) -> (usize, TRACKER)
```

A string identifier for a lint rule.

This string is used in command line and configuration interfaces. The name should always
be in kebab case, e.g. `no-foo` (all lower case).

Rules use kebab case, e.g. `no-foo`.

---

## SecondaryCode

`struct` · `ruff_db::diagnostic::SecondaryCode`

```rust
struct SecondaryCode
```

**Implements**: `core::convert::From`, `core::fmt::Display`, `core::ops::deref::Deref`, `get_size2::GetSize`

**Derives**: Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (2)

```rust
fn as_str(&self) -> &str
fn new(code: String) -> Self
```

**via `core::convert::From`**

```rust
fn from(value: &SecondaryCode) -> Self
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `core::ops::deref::Deref`**

```rust
fn deref(&self) -> &Self::Target
```

**via `get_size2::GetSize`**

```rust
fn get_heap_size(&self) -> usize
fn get_heap_size_with_tracker<TRACKER: ::get_size2::GetSizeTracker>(&self, tracker: TRACKER) -> (usize, TRACKER)
```

A secondary identifier for a lint diagnostic.

For Ruff rules this means the noqa code.

---

## Span

`struct` · `ruff_db::diagnostic::Span`

```rust
struct Span
```

**Implements**: `core::convert::From`, `get_size2::GetSize`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (6)

```rust
fn as_ruff_file(&self) -> Option<&SourceFile>
fn expect_ty_file(&self) -> File
fn file(&self) -> &UnifiedFile
fn range(&self) -> Option<TextRange>
fn with_optional_range(self, range: Option<TextRange>) -> Span
fn with_range(self, range: TextRange) -> Span
```

**via `core::convert::From`**

```rust
fn from(file: SourceFile) -> Self
fn from(file_range: files::FileRange) -> Span
fn from(file: File) -> Span
```

**via `get_size2::GetSize`**

```rust
fn get_heap_size(&self) -> usize
fn get_heap_size_with_tracker<TRACKER: ::get_size2::GetSizeTracker>(&self, tracker: TRACKER) -> (usize, TRACKER)
```

A span represents the source of a diagnostic.

It consists of a `File` and an optional range into that file. When the
range isn't present, it semantically implies that the diagnostic refers to
the entire file. For example, when the file should be executable but isn't.

---

## SubDiagnostic

`struct` · `ruff_db::diagnostic::SubDiagnostic`

```rust
struct SubDiagnostic
```

**Implements**: `get_size2::GetSize`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (10)

```rust
fn annotate(&mut self, ann: Annotation)
fn annotations(&self) -> &[Annotation]
fn annotations_mut(&mut self) -> impl Iterator<Item = &mut Annotation>
fn concise_message(&self) -> ConciseMessage<'_>
fn headline_message(&self) -> &str
fn new<'a>(severity: SubDiagnosticSeverity, message: impl IntoDiagnosticMessage + 'a) -> SubDiagnostic
fn primary_annotation(&self) -> Option<&Annotation>
fn primary_span_ref(&self) -> Option<&Span>
fn secondary_annotations(&self) -> impl Iterator<Item = &Annotation>
fn severity(&self) -> SubDiagnosticSeverity
```

**via `get_size2::GetSize`**

```rust
fn get_heap_size(&self) -> usize
fn get_heap_size_with_tracker<TRACKER: ::get_size2::GetSizeTracker>(&self, tracker: TRACKER) -> (usize, TRACKER)
```

A collection of information subservient to a diagnostic.

A sub-diagnostic is always rendered after the parent diagnostic it is
attached to. A parent diagnostic may have many sub-diagnostics, and it is
guaranteed that they will not interleave with one another in rendering.

Currently, the order in which sub-diagnostics are rendered relative to one
another (for a single parent diagnostic) is the order in which they were
attached to the diagnostic.

---

## IntoDiagnosticMessage

`trait` · `ruff_db::diagnostic::IntoDiagnosticMessage`

```rust
trait IntoDiagnosticMessage
```

**Implementors** (1)

- `ruff_db::diagnostic::DiagnosticMessage`

**Methods** (1)

```rust
fn into_diagnostic_message(self) -> DiagnosticMessage
```

A trait for values that can be converted into a diagnostic message.

Users of the diagnostic API can largely think of this trait as effectively
equivalent to `std::fmt::Display`. Indeed, everything that implements
`Display` also implements this trait. That means wherever this trait is
accepted, you can use things like `format_args!`.

The purpose of this trait is to provide a means to give arguments _other_
than `std::fmt::Display` trait implementations. Or rather, to permit
the diagnostic API to treat them differently. For example, this lets
callers wrap a string in a `DiagnosticMessage` and provide it directly
to any of the diagnostic APIs that accept a message. This will move the
string and avoid any unnecessary copies. (If we instead required only
`std::fmt::Display`, then this would potentially result in a copy via the
`ToString` trait implementation.)

---
