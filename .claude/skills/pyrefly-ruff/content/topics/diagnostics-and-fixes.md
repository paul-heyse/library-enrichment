# Diagnostics, edits and fixes

A `Fix` is a set of `Edit`s plus an `Applicability`, and applicability is a safety claim, not a confidence score: `Safe` fixes are applied by default, `Unsafe` ones only with `--unsafe-fixes`, and `DisplayOnly` never. Of the 970 rules, 233 always offer a fix and 221 sometimes do; the rest are findings only. `../catalogs/rules.md` has the breakdown.

## Entry points

| Type | Kind | Methods | Prose | Records |
|---|---|---:|---|---|
| `ruff_db::diagnostic::Diagnostic` | struct | 56 | [prose](../api/ruff_db.diagnostic.md#diagnostic) | [records](../model/ruff_db.diagnostic.json) |
| `ruff_diagnostics::fix::Fix` | struct | 22 | [prose](../api/ruff_diagnostics.fix.md#fix) | [records](../model/ruff_diagnostics.fix.json) |
| `ruff_diagnostics::edit::Edit` | struct | 19 | [prose](../api/ruff_diagnostics.edit.md#edit) | [records](../model/ruff_diagnostics.edit.json) |
| `ruff_diagnostics::fix::Applicability` | enum | 11 | [prose](../api/ruff_diagnostics.fix.md#applicability) | [records](../model/ruff_diagnostics.fix.json) |

## Upstream guides

- [`corpus/ruff/linter.md`](../corpus/ruff/linter.md)

## Decision rules

- Automating a codemod? Filter to `Safe` unless you are reviewing every diff.
- Need the edits without applying them? `--diff`, or read the JSON `fix` field.

## Anti-patterns

- Applying unsafe fixes in CI without review.
- Assuming every rule has a fix -- 480 of 970 do not.

## Agent checklist

- Edits are byte offsets; apply them back-to-front or use the provided applier.
