# Connecting a client

`Client` is an async context manager. The connection opens on entry and closes on exit; a method called on a client you never entered raises rather than connecting lazily. 963 upstream sites open one the same way.

## Entry points

| Type | Kind | Import as | Prose | Records |
|---|---|---|---|---|
| `fastmcp.client.client.Client` | class | `fastmcp.Client` | [prose](../api/fastmcp.client.client.md) | [records](../model/fastmcp.client.client.json) |
| `fastmcp.client.transports.base.ClientTransport` | class | `fastmcp.client.ClientTransport` | [prose](../api/fastmcp.client.transports.base.md) | [records](../model/fastmcp.client.transports.base.json) |

## Upstream guides

- [`corpus/docs/clients/client.mdx`](../corpus/docs/clients/client.mdx)
- [`corpus/docs/clients/transports.mdx`](../corpus/docs/clients/transports.mdx)

## Decision rules

- In-process testing? Pass the server object; `FastMCPTransport` is inferred.
- Remote HTTP? Pass the URL. The transport is inferred from its shape.
- Need control over headers or auth? Construct the transport explicitly.

## Anti-patterns

- Calling `client.call_tool` outside `async with`.
- Assuming transport inference from a string is unambiguous. It is documented, not obvious.

## Agent checklist

- `catalogs/transports.md` covers 4 server transports and 11 client classes.
- `extension-points/ClientTransport.md` if you need a new one.
