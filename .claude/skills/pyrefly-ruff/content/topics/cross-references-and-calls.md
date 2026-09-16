# Cross-references and call graphs

This is the capability neither project advertises and the reason this repository exists. `pyrefly check --report-glean <dir>` writes one JSON document per module containing `python.CalleeToCaller` (the call edge, reversed), `python.XRefsViaNameByTarget` (every reference to a target) and `python.DeclarationLocation` / `python.DefinitionLocation`. An LSP answers one position at a time; this answers the whole project at once. The full predicate inventory, measured from a real payload, is in `../catalogs/cross-references.md`. `--report-pysa` is **not** an alternative: despite its help text promising JSON, it writes Cap'n Proto binary plus a copy of the bundled typeshed.

## Entry points

| Type | Kind | Methods | Prose | Records |
|---|---|---:|---|---|
| `pyrefly_glean_schema::report::glean::schema::digest::FileDigest` | struct | 8 | [prose](../api/pyrefly_glean_schema.report.glean.schema.digest.md#filedigest) | [records](../model/pyrefly_glean_schema.report.glean.schema.digest.json) |
| `pyrefly_python::module_name::ModuleName` | struct | 59 | [prose](../api/pyrefly_python.module_name.md#modulename) | [records](../model/pyrefly_python.module_name.json) |
| `pyrefly_python::module::Module` | struct | 27 | [prose](../api/pyrefly_python.module.md#module) | [records](../model/pyrefly_python.module.json) |

## Upstream guides

- [`corpus/pyrefly/report.mdx`](../corpus/pyrefly/report.mdx)

## Decision rules

- One position, interactively? LSP `references` or `callHierarchy`.
- Whole project, as data? The Glean report.
- Imports only, and cheaply? `ruff analyze graph`.

## Anti-patterns

- Reading `--report-pysa` output as JSON.
- Inferring a call graph from import edges -- an import is not a call.

## Agent checklist

- The filenames are content digests, so enumerate the directory rather than guessing names.
- Facts are interned by id; resolve `python.Name` before reporting a name to a human.
