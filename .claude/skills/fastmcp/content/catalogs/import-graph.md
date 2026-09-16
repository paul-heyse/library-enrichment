# The import graph

2,563 edges between 367 files, resolved by the
type checker. This is **file-level**: keys and values are paths relative to
site-packages, and there is no module-name representation, so joining it to anything
that speaks dotted names is your job.

It is not a call graph. An import is not a call, and `cross-references.md` is where
call edges live.

## Most depended upon

| File | Imported by |
|---|---:|
| `mcp_types/_types.py` | 120 |
| `mcp_types/__init__.py` | 115 |
| `fastmcp/utilities/logging.py` | 100 |
| `fastmcp/utilities/types.py` | 49 |
| `fastmcp/utilities/components.py` | 47 |
| `fastmcp/server/server.py` | 46 |
| `mcp/server/auth/provider.py` | 45 |
| `fastmcp/tools/base.py` | 43 |
| `mcp_types/jsonrpc.py` | 43 |
| `mcp/shared/exceptions.py` | 41 |
| `fastmcp/server/auth/auth.py` | 39 |
| `fastmcp/server/providers/base.py` | 39 |
| `fastmcp/resources/base.py` | 34 |
| `mcp/shared/message.py` | 32 |
| `fastmcp/utilities/versions.py` | 31 |

## Recipes

```bash
# what does this file import?
rg -P '^fastmcp/server/server\.py\t' content/index/imports.tsv | cut -f2
# what imports it?
rg -P '\tfastmcp/server/server\.py$' content/index/imports.tsv | cut -f1
```
