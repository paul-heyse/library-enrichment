# Testing a server

Pass the server object straight to `Client` and no process or socket is involved. That is how essentially all 420 upstream test files work, which makes them the readable specification of every capability here.

## Entry points

| Type | Kind | Import as | Prose | Records |
|---|---|---|---|---|
| `fastmcp.client.client.Client` | class | `fastmcp.Client` | [prose](../api/fastmcp.client.client.md) | [records](../model/fastmcp.client.client.json) |
| `fastmcp.server.server.FastMCP` | class | `fastmcp.FastMCP` | [prose](../api/fastmcp.server.server.md) | [records](../model/fastmcp.server.server.json) |

## Upstream guides

- [`corpus/docs/servers/testing.mdx`](../corpus/docs/servers/testing.mdx)

## Decision rules

- Testing behaviour? In-process. Testing transport? Then you need the transport.

## Anti-patterns

- Spawning a subprocess server to test tool logic.

## Agent checklist

- `rg -l 'async with Client' content/corpus/tests | head` finds a worked example for nearly anything.
