# `mcp.server.auth.middleware.auth_context`

Distribution: `mcp`

## auth_context_var

`mcp.server.auth.middleware.auth_context.auth_context_var`

```python
auth_context_var = contextvars.ContextVar[AuthenticatedUser | None]('auth_context', default=None)
```

**Inferred type** (`ty`, not declared in the source): `ContextVar[AuthenticatedUser | None]`

## AuthContextMiddleware

Import as `mcp.server.lowlevel.server.AuthContextMiddleware`  ·  defined at `mcp.server.auth.middleware.auth_context.AuthContextMiddleware`

```python
class AuthContextMiddleware
```

_3 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Declared members (1)**

- `app = app`  _instance-attribute_

Middleware that extracts the authenticated user from the request
and sets it in a contextvar for easy access throughout the request lifecycle.

This middleware should be added after the AuthenticationMiddleware in the
middleware stack to ensure that the user is properly authenticated before
being stored in the context.


## get_access_token

Import as `mcp.server.request_state.get_access_token`  ·  defined at `mcp.server.auth.middleware.auth_context.get_access_token`

```python
def get_access_token() -> AccessToken | None
```

**Also exported as** `fastmcp.server.dependencies._sdk_get_access_token`

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Get the access token from the current context.

Returns:
    The access token if an authenticated user is available, None otherwise.


