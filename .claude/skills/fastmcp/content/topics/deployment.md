# Running and deploying a server

`run()` chooses a transport; the choice determines what deployment looks like. stdio is a subprocess contract, HTTP is a service.

## Entry points

| Type | Kind | Import as | Prose | Records |
|---|---|---|---|---|
| `fastmcp.server.server.FastMCP` | class | `fastmcp.FastMCP` | [prose](../api/fastmcp.server.server.md) | [records](../model/fastmcp.server.server.json) |
| `fastmcp.settings.Settings` | class | `fastmcp.Settings` | [prose](../api/fastmcp.settings.md) | [records](../model/fastmcp.settings.json) |

## Upstream guides

- [`corpus/docs/deployment/running-server.mdx`](../corpus/docs/deployment/running-server.mdx)
- [`corpus/docs/deployment/http.mdx`](../corpus/docs/deployment/http.mdx)
- [`corpus/docs/deployment/server-configuration.mdx`](../corpus/docs/deployment/server-configuration.mdx)

## Decision rules

- Launched by a client as a child process? stdio.
- Reachable over a network? HTTP, and then auth is not optional.

## Anti-patterns

- Exposing an HTTP server with no `AuthProvider`.

## Agent checklist

- `catalogs/transports.md` for the four server transports.
