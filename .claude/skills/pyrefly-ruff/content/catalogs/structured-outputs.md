# Structured output

Never parse either tool's human output. Both emit machine-readable forms for everything
that matters, and the human formats are explicitly unstable.

| Tool | Command | Format | Notes |
|---|---|---|---|
| `ruff` | `check` | 12 formats | `--output-format`: concise, full, json, json-lines, junit, grouped, github, gitlab, pylint, rdjson, azure, sarif |
| `ruff` | `rule --all` | JSON | the complete rule inventory; see `rules.md` |
| `ruff` | `config` | JSON | every configuration key, its type and default |
| `ruff` | `linter` | JSON | the 59 upstream linter families and their code prefixes |
| `ruff` | `version` | JSON | `{version, commit_info}` — the version assertion |
| `ruff` | `analyze graph` | JSON only | file-level import map; see `import-graph.md` |
| `pyrefly` | `check` | JSON | `--output-format json` diagnostics |
| `pyrefly` | `check --report-glean` | JSON | definitions, references, call edges |
| `pyrefly` | `check --report-pysa` | Cap'n Proto | binary at the DEFAULT format |
| `pyrefly` | `check --report-pysa --report-pysa-format json` | JSON | definitions, call graphs, expression types; see `cross-references.md` |
| `pyrefly` | `coverage report` | JSON | per-symbol typed/any/untyped coverage |
| `pyrefly` | `stubgen` | .pyi | reconstructed stubs; unresolved marked `Incomplete` |
| `pyrefly` | `check --dependency-graph` | JSON | file-level module imports |
| `pyrefly` | `check --report-timings` | JSON | per-module phase timings |
| `pyrefly` | `check --report-trace` | text | type traces |
| `pyrefly` | `check --baseline` | JSON | a diffable error baseline |
| `pyrefly` | `dump-config` | text | the resolved configuration and covered files |
| `pyrefly` | `lsp` | LSP | see `protocols.md` |
| `pyrefly` | `tsp` | TSP | LSP superset; see `protocols.md` |

Two traps, both measured:

- `ruff analyze graph` has **no** `--output-format` flag, unlike every other ruff
  subcommand. Asking for one is an error, not a no-op.
- `ruff check` exits non-zero when it finds diagnostics. That is a finding, not a
  failure — use `--exit-zero` when the JSON is the product.
