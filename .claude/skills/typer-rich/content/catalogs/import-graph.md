# The import graph

748 edges between 112 files, resolved by the
type checker. This is **file-level**: keys and values are paths relative to
site-packages, and there is no module-name representation, so joining it to anything
that speaks dotted names is your job.

It is not a call graph. An import is not a call, and `cross-references.md` is where
call edges live.

## Most depended upon

| File | Imported by |
|---|---:|
| `rich/style.py` | 56 |
| `rich/console.py` | 54 |
| `rich/jupyter.py` | 45 |
| `rich/text.py` | 43 |
| `rich/segment.py` | 39 |
| `rich/highlighter.py` | 38 |
| `rich/theme.py` | 36 |
| `rich/__init__.py` | 21 |
| `rich/table.py` | 19 |
| `rich/measure.py` | 18 |
| `typer/_click/core.py` | 18 |
| `typer/_click/utils.py` | 15 |
| `rich/color.py` | 14 |
| `rich/panel.py` | 13 |
| `typer/exceptions.py` | 12 |

## Recipes

```bash
# what does this file import?
rg -P '^fastmcp/server/server\.py\t' content/index/imports.tsv | cut -f2
# what imports it?
rg -P '\tfastmcp/server/server\.py$' content/index/imports.tsv | cut -f1
```
