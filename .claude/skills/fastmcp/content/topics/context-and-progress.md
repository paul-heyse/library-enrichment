# What a tool body can do: context, progress, logging

Take `Context` as an annotated parameter and FastMCP injects it. Most of its surface is `async`, so the same sync/async trap that catches hooks catches tool bodies.

## Entry points

| Type | Kind | Import as | Prose | Records |
|---|---|---|---|---|
| `fastmcp.server.context.Context` | class | `fastmcp.Context` | [prose](../api/fastmcp.server.context.md) | [records](../model/fastmcp.server.context.json) |

## Upstream guides

- [`corpus/docs/servers/context.mdx`](../corpus/docs/servers/context.mdx)
- [`corpus/docs/servers/progress.mdx`](../corpus/docs/servers/progress.mdx)
- [`corpus/docs/servers/logging.mdx`](../corpus/docs/servers/logging.mdx)

## Decision rules

- Long-running work? Report progress; the client may be waiting on it.
- Need a value from the caller mid-call? That is elicitation, not logging.

## Anti-patterns

- Calling a `Context` method without awaiting it.
- Constructing a `Context` yourself rather than taking it as a parameter.

## Agent checklist

- `catalogs/context.md` lists all 34 members and marks the async ones.
