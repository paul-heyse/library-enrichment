# The import and module graph

`ruff analyze graph` maps each **file** to the files it imports -- not modules, not symbols, not calls. It is fast and needs no configuration, and it is narrower than its name in four ways that are all silent. `../catalogs/import-graph.md` states each with the measurement behind it; the one that bites hardest is that a path which does not exist returns `{}` with exit status 0.

## Entry points

| Type | Kind | Methods | Prose | Records |
|---|---|---:|---|---|
| `ruff_graph::ModuleImports` | struct | 7 | [prose](../api/ruff_graph.md#moduleimports) | [records](../model/ruff_graph.json) |
| `ruff_graph::db::ModuleDb` | struct | 7 | [prose](../api/ruff_graph.db.md#moduledb) | [records](../model/ruff_graph.db.json) |
| `ruff_graph::settings::Direction` | enum | 6 | [prose](../api/ruff_graph.settings.md#direction) | [records](../model/ruff_graph.settings.json) |

## Upstream guides

- [`corpus/pyrefly/import-resolution.mdx`](../corpus/pyrefly/import-resolution.mdx)
- [`corpus/ruff/configuration.md`](../corpus/ruff/configuration.md)

## Decision rules

- Need third-party edges? Pass `--python <venv>`; without it only first-party files resolve.
- Need edges the scanner cannot see? `analyze.include-dependencies` in configuration.
- Need dotted module names? You will have to map paths yourself; the tool has no such notion.

## Anti-patterns

- Checking the exit status instead of the key count.
- Assuming `--detect-string-imports` is on by default.

## Agent checklist

- Point it at a package root; a wrong path is indistinguishable from an empty result.
- `--direction dependents` inverts the map without a second parse.
