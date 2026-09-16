# Cross-references and call graphs

`pyrefly check --report-glean <dir>` writes one JSON document per module, named by
content digest, each a list of `{predicate, facts}` entries. It is the only route either
toolchain offers to **definitions, references and caller/callee edges** as data.

Measured during acquisition against a three-module fixture, so the counts below are
shape evidence, not scale evidence:

| Predicate | Facts | Answers |
|---|---:|---|
| `digest.FileDigest.1` | 3 | the content digest of the file the facts came from |
| `gencode.GenCode.1` | 0 | whether the file is generated |
| `python.CalleeToCaller.4` | 9 | who calls this — the call-graph edge, reversed |
| `python.ContainingTopLevelDeclaration.4` | 22 | which top-level declaration encloses a declaration |
| `python.DeclarationDocstring.4` | 7 | the docstring attached to a declaration |
| `python.DeclarationLocation.4` | 22 | where each name is declared, with its span |
| `python.DefinitionLocation.4` | 20 | where each name is defined, with its span |
| `python.FileCall.4` | 8 | every call site in a file, with its argument positions |
| `python.ImportStarLocation.4` | 0 | where a star-import brings names in |
| `python.Module.4` | 5 | the module entity itself |
| `python.Name.4` | 3 | interned name strings |
| `python.NameToSName.4` | 36 | a flat name to its structured (dotted) name |
| `python.XRefsViaNameByFile.4` | 3 | every reference in a file, grouped by the file |
| `python.XRefsViaNameByTarget.4` | 26 | every reference to a target, grouped by what it refers to |
| `python.xrefs.XRefsByFile.1` | 3 | the same cross-references in the newer by-file schema |
| `src.FileLanguage.1` | 3 | the language a file was parsed as |
| `src.FileLines.1` | 3 | line-offset table, needed to turn a byte span into a line |

`python.CalleeToCaller` is the direct answer to *who calls this*, and
`python.XRefsViaNameByTarget` to *where is this referenced*. Both are absent from the LSP
surface at any useful scale, because LSP answers one position at a time.

## Recipe

```bash
pyrefly check --report-glean ./glean-out
jq -r '.[]' ./glean-out/*.json \
  | jq -s 'map(select(.predicate=="python.CalleeToCaller.4")) | .[].facts'
```

## `--report-pysa` — richer, behind a flag

Its help text promises "a Pysa-compatible JSON file for each module" and its DEFAULT
format does not deliver one: 7099 files in formats
{'.bin': 185, '.pyi': 6914} — Cap'n Proto binary plus a copy of the bundled
typeshed. Reading that needs the `.capnp` schema from the source tree.

`--report-pysa-format json` is a different report. Measured at the same pin:
7099 files in formats {'.json': 185, '.pyi': 6914} —
real JSON under `definitions/`, `call_graphs/` and `type_of_expressions/`.

Where it beats Glean, and it is not close:

| Fact | Glean | pysa JSON |
|---|---|---|
| Position | UTF-8 byte span, needs `src.FileLines` | `line:col-line:col` |
| Call target | the callee's dotted name | `init_targets`/`new_targets` with `receiver_class`, `implicit_receiver`, `is_static_method` |
| Parameters | absent | kind (`Pos`/`PosOnly`/`KwOnly`/`VarArg`/`Kwargs`), requiredness, annotation resolved to a defining module and class |
| Expression types | absent | every expression, per function |
| MRO | absent | linearised, each entry naming its module |
| References | `python.xrefs.XRefsByFile.1`, target carries its defining file | absent |

So: **pysa JSON for calls, definitions, parameters and types; Glean for references.**
The default format is what made this look unusable, and measuring only the default is
how that conclusion survived a full acquisition pass.
