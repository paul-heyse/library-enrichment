# Authenticating callers

`AuthProvider` has 38 descendants and 11 abstract methods, and upstream never subclasses it directly. The useful entry points are `TokenVerifier` for bearer-token verification and `OAuthProxy` for fronting an existing OAuth server.

## Entry points

| Type | Kind | Import as | Prose | Records |
|---|---|---|---|---|
| `fastmcp.server.auth.auth.AuthProvider` | class | `fastmcp.server.auth.AuthProvider` | [prose](../api/fastmcp.server.auth.auth.md) | [records](../model/fastmcp.server.auth.auth.json) |
| `fastmcp.server.auth.auth.TokenVerifier` | class | `fastmcp.server.auth.TokenVerifier` | [prose](../api/fastmcp.server.auth.auth.md) | [records](../model/fastmcp.server.auth.auth.json) |
| `fastmcp.server.auth.auth.OAuthProvider` | class | `fastmcp.server.auth.OAuthProvider` | [prose](../api/fastmcp.server.auth.auth.md) | [records](../model/fastmcp.server.auth.auth.json) |

## Upstream guides

- [`corpus/docs/servers/authorization.mdx`](../corpus/docs/servers/authorization.mdx)
- [`corpus/docs/clients/auth`](../corpus/docs/clients/auth)

## Decision rules

- Verifying a token someone else issued? `TokenVerifier`.
- Fronting an existing OAuth server? `OAuthProxy`.
- Issuing tokens yourself? `OAuthProvider`, and read the 13 existing implementations first.

## Anti-patterns

- Subclassing `AuthProvider` directly because the catalog names it first.
- Assuming a provider named for a vendor implies that vendor's extra is installed.

## Agent checklist

- `catalogs/auth-providers.md` lists all 37.
- `index/extras.tsv` says which optional install a vendor provider needs.
