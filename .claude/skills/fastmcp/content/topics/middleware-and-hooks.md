# Intercepting requests with middleware

Every hook is `async def` and every hook receives `call_next`. Forgetting to await `call_next` returns a coroutine where a value was expected, so the failure surfaces as an empty response somewhere else entirely rather than as an error naming the cause.

## Entry points

| Type | Kind | Import as | Prose | Records |
|---|---|---|---|---|
| `fastmcp.server.middleware.middleware.Middleware` | class | `fastmcp.server.middleware.Middleware` | [prose](../api/fastmcp.server.middleware.middleware.md) | [records](../model/fastmcp.server.middleware.middleware.json) |
| `fastmcp.server.middleware.middleware.MiddlewareContext` | class | `fastmcp.server.middleware.MiddlewareContext` | [prose](../api/fastmcp.server.middleware.middleware.md) | [records](../model/fastmcp.server.middleware.middleware.json) |

## Upstream guides

- [`corpus/docs/servers/middleware.mdx`](../corpus/docs/servers/middleware.mdx)

## Decision rules

- Override only the hooks you need; the rest pass through.
- `on_message` sees everything; the specific hooks see one kind. Prefer the specific one.

## Anti-patterns

- Declaring a hook with `def`. `project-sync-hook-override` finds this in your own code.
- Returning without awaiting `call_next`.

## Agent checklist

- `extension-points/Middleware.md` lists all twelve with full signatures.
- 17 built-in middleware already exist -- check before writing one.
