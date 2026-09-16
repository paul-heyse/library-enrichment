# The wire model and protocol eras

`mcp_types` carries dated copies of the wire model because FastMCP negotiates a protocol era per connection. Same-named types in `_v2026_07_28` and `_v2025_11_25` are different classes. Never merge them on leaf name.

## Entry points

| Type | Kind | Import as | Prose | Records |
|---|---|---|---|---|
| `mcp_types._types.Tool` | class | `mcp_types.Tool` | [prose](../api/mcp_types._types.md) | [records](../model/mcp_types._types.json) |
| `mcp_types._types.Result` | class | `mcp_types.Result` | [prose](../api/mcp_types._types.md) | [records](../model/mcp_types._types.json) |
| `mcp_types._types.Request` | class | `mcp_types.Request` | [prose](../api/mcp_types._types.md) | [records](../model/mcp_types._types.json) |

## Upstream guides

- [`corpus/spec/2026-07-28/architecture/index.mdx`](../corpus/spec/2026-07-28/architecture/index.mdx)
- [`corpus/spec/2025-11-25/architecture/index.mdx`](../corpus/spec/2025-11-25/architecture/index.mdx)

## Decision rules

- Writing a type in an annotation? Use the `mcp_types.X` re-export, never the `_types` path.
- Proxying between eras is supported; assuming one era is not.

## Anti-patterns

- Importing from `mcp_types._types`. The canonical path is not the writable one.

## Agent checklist

- `catalogs/protocol-eras.md` lists the eras.
- The specification text is vendored under `corpus/spec/`.
