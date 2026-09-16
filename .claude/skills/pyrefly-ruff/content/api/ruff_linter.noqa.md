# `ruff_linter::noqa`

Crate `ruff_linter` · 3 public items · structured records in [`model/ruff_linter.noqa.json`](../model/ruff_linter.noqa.json)

## SuppressionKind

`enum` · `ruff_linter::noqa::SuppressionKind`

Also reachable as `ruff_linter::SuppressionKind`

```rust
enum SuppressionKind
```

**Variants**: `Noqa`, `Ignore`

**Derives**: Clone, Copy, Debug

The kind of suppression comment to be added to suppress a diagnostic.

---

## generate_suppression_edits

`function` · `ruff_linter::noqa::generate_suppression_edits`

Also reachable as `ruff_linter::generate_suppression_edits`

```rust
fn generate_suppression_edits(path: &std::path::Path, diagnostics: &[ruff_db::diagnostic::Diagnostic], locator: &Locator<'_>, comment_ranges: &ruff_python_trivia::CommentRanges, external: &[String], noqa_line_for: &NoqaMapping, line_ending: ruff_source_file::LineEnding, suppressions: &suppression::Suppressions, suppression_kind: SuppressionKind, preview: settings::types::PreviewMode) -> Vec<Option<ruff_diagnostics::Edit>>
```

Generates an array of edits that matches the length of `diagnostics`.
Each potential edit in the array is paired, in order, with the associated diagnostic.
Each edit will add a suppression comment to the appropriate line in the source to hide
the diagnostic. These edits may conflict with each other and should not be applied
simultaneously.

---

## NoqaMapping

`struct` · `ruff_linter::noqa::NoqaMapping`

```rust
struct NoqaMapping
```

Remaps offsets falling into one of the ranges to instead check for a noqa comment on the
line specified by the offset.

---
