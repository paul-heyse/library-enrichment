# What fails, and what you catch

`fastmcp.exceptions.MCPError` is bound under `try`/`except ImportError`: with `mcp` installed it is one class, without it another. Anything that branches on exception identity has to know which environment it is in.

## Entry points

| Type | Kind | Import as | Prose | Records |
|---|---|---|---|---|
| `fastmcp.exceptions.FastMCPError` | class | `fastmcp.server.server.FastMCPError` | [prose](../api/fastmcp.exceptions.md) | [records](../model/fastmcp.exceptions.json) |
| `fastmcp.exceptions.ToolError` | class | `fastmcp.server.server.ToolError` | [prose](../api/fastmcp.exceptions.md) | [records](../model/fastmcp.exceptions.json) |

## Upstream guides

- [`corpus/docs/servers/tools.mdx`](../corpus/docs/servers/tools.mdx)

## Decision rules

- Raising from a tool body? `ToolError` reaches the caller as a tool error rather than a transport failure.

## Anti-patterns

- Catching `Exception` in a hook and swallowing the cause.

## Agent checklist

- `index/conditional.tsv` records the 17 names bound conditionally.
