# The formatter

The formatter is a two-stage design: `ruff_formatter` is a language-agnostic document IR and printer, and `ruff_python_formatter` lowers Python into it. That split is why `--range` formatting works at all, and why the formatter can be embedded without the linter.

## Entry points

| Type | Kind | Methods | Prose | Records |
|---|---|---:|---|---|
| `ruff_formatter::formatter::Formatter` | struct | 16 | [prose](../api/ruff_formatter.formatter.md#formatter) | [records](../model/ruff_formatter.formatter.json) |
| `ruff_formatter::Printed` | struct | 14 | [prose](../api/ruff_formatter.md#printed) | [records](../model/ruff_formatter.json) |
| `ruff_formatter::buffer::VecBuffer` | struct | 11 | [prose](../api/ruff_formatter.buffer.md#vecbuffer) | [records](../model/ruff_formatter.buffer.json) |

## Upstream guides

- [`corpus/ruff/formatter.md`](../corpus/ruff/formatter.md)

## Decision rules

- Formatting a fragment? `--range=start-end`, 1-based, end exclusive.
- Formatting code inside docstrings? `docstring-code-format`.

## Anti-patterns

- Running the formatter and linter in the wrong order in a hook; format last.

## Agent checklist

- `--check` and `--diff` both avoid writing; only `--diff` shows what would change.
