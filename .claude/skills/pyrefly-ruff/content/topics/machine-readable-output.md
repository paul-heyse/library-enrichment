# Getting machine-readable output

Never parse either tool's human output. Both emit structured forms for everything that matters, and both say their human formats are unstable. `ruff check` alone offers twelve formats including SARIF and rdjson. The two traps are measured and recorded in `../catalogs/structured-outputs.md`: `ruff analyze graph` is the one subcommand with **no** `--output-format` flag, and `ruff check` exits non-zero when it finds diagnostics, which is a finding rather than a failure -- use `--exit-zero` when the JSON is the product.

## Entry points

| Type | Kind | Methods | Prose | Records |
|---|---|---:|---|---|
| `ruff::args::CheckCommand` | struct | 11 | [prose](../api/ruff.args.md#checkcommand) | [records](../model/ruff.args.json) |
| `ruff::args::AnalyzeGraphCommand` | struct | 11 | [prose](../api/ruff.args.md#analyzegraphcommand) | [records](../model/ruff.args.json) |
| `ruff::args::FormatCommand` | struct | 12 | [prose](../api/ruff.args.md#formatcommand) | [records](../model/ruff.args.json) |
| `ruff_db::diagnostic::Diagnostic` | struct | 56 | [prose](../api/ruff_db.diagnostic.md#diagnostic) | [records](../model/ruff_db.diagnostic.json) |

## Upstream guides

- [`corpus/ruff/linter.md`](../corpus/ruff/linter.md)
- [`corpus/pyrefly/report.mdx`](../corpus/pyrefly/report.mdx)

## Decision rules

- Want diagnostics for a human? `full`. For a machine? `json`. For CI? `github` or `sarif`.
- Want the import graph? `analyze graph`, and check the key count rather than the exit status.
- Want references or call edges? Neither of those -- see the cross-references topic.

## Anti-patterns

- Regex over `ruff check` text output, which changes between patch releases.
- Treating a non-zero `ruff check` status as a crash.

## Agent checklist

- Pin the tool version; the catalogs here describe 0.16.7 and 1.3.1 exactly.
- Redirect stderr separately -- `analyze graph` always prints an experimental banner.
