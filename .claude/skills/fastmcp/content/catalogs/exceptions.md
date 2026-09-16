# Exceptions

What a caller branches on. `fastmcp.server.server.FastMCPError` has 8 descendants.

| Exception | Import as | Summary |
|---|---|---|
| `AuthorizationError` | `fastmcp.server.server.AuthorizationError` | Error when authorization check fails. |
| `InsufficientScopeError` | `fastmcp.server.middleware.authorization.InsufficientScopeError` | Authorization failed because the token is missing required OAuth scopes. |
| `PromptError` | `fastmcp.server.server.PromptError` | Error in prompt operations. |
| `ResourceError` | `fastmcp.server.server.ResourceError` | Error in resource operations. |
| `ToolError` | `fastmcp.server.server.ToolError` | Error in tool operations. |
| `ValidationError` | `fastmcp.server.server.ValidationError` | Error in validating parameters or return values. |
| `InvalidSession` | `fastmcp.server.sessions.InvalidSession` | A session id did not resolve to a session created under the current principal. |
| `SessionAuthError` | `fastmcp.server.sessions.SessionAuthError` | An injected `session: UserSession` was requested with no authenticated principal |

## One conditional definition

`fastmcp.exceptions.MCPError` is bound by a `try`/`except ImportError` pair: the
preferred branch imports it from `mcp`, and the fallback defines a local stand-in.
This index was built with `mcp` installed, so the row you see is the real one --
but in an install without it, the same name is a different class. See
`content/index/conditional.tsv`.
