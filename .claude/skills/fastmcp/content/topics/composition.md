# Composing servers: providers, transforms, mounts and proxies

Three distinct axes, and conflating them is the commonest 4.0 mistake. A **Provider** decides where components come from. A **Transform** rewrites the catalog, observably. **Middleware** intercepts requests in flight. `mount()` and `create_proxy()` are thin wrappers over Providers, and a `FastMCP` server is itself a Provider -- which is why mounting one into another works at all.

## Entry points

| Type | Kind | Import as | Prose | Records |
|---|---|---|---|---|
| `fastmcp.server.providers.base.Provider` | class | `fastmcp.server.providers.Provider` | [prose](../api/fastmcp.server.providers.base.md) | [records](../model/fastmcp.server.providers.base.json) |
| `fastmcp.server.transforms.Transform` | class | `fastmcp.server.server.Transform` | [prose](../api/fastmcp.server.transforms.md) | [records](../model/fastmcp.server.transforms.json) |
| `fastmcp.server.providers.aggregate.AggregateProvider` | class | `fastmcp.server.providers.AggregateProvider` | [prose](../api/fastmcp.server.providers.aggregate.md) | [records](../model/fastmcp.server.providers.aggregate.json) |
| `fastmcp.server.server.FastMCP` | class | `fastmcp.FastMCP` | [prose](../api/fastmcp.server.server.md) | [records](../model/fastmcp.server.server.json) |

## Upstream guides

- [`corpus/docs/servers/composition.mdx`](../corpus/docs/servers/composition.mdx)
- [`corpus/docs/servers/extensions.mdx`](../corpus/docs/servers/extensions.mdx)

## Decision rules

- Reaching for `import_server` or `as_proxy`? Both were removed in 4.0. You want `mount()` or `create_proxy()`.
- Need components from elsewhere? Provider. Need to change how they look? Transform. Need to act per request? Middleware.
- A Transform is observable by the system; a Provider that filters is not. If the catalog must honestly report what happened, use a Transform.

## Anti-patterns

- Writing a Provider hook as `def`. Every one is `async`.
- Assuming `mount()` copies components. It composes Providers; the source stays live.

## Agent checklist

- Check `catalogs/removed-api.md` before writing any composition call from memory.
- `extension-points/Provider.md` has 26 implementors -- read one.
