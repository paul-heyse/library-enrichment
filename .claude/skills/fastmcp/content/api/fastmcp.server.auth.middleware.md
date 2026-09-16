# `fastmcp.server.auth.middleware`

Distribution: `fastmcp`

## logger

`fastmcp.server.auth.middleware.logger`

```python
logger = get_logger(__name__)
```

**Inferred type** (`ty`, not declared in the source): `Logger`

## RequireAuthMiddleware

Import as `fastmcp.server.http.RequireAuthMiddleware`  ·  defined at `fastmcp.server.auth.middleware.RequireAuthMiddleware`

```python
class RequireAuthMiddleware(SDKRequireAuthMiddleware)
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `SDKRequireAuthMiddleware`

**Declared members (1)**

- `challenge_scopes = required_scopes if challenge_scopes is None else challenge_scopes`  _instance-attribute_

Enhanced authentication middleware with detailed error messages.

Extends the SDK's RequireAuthMiddleware to provide more actionable
error messages when authentication fails. This helps developers
understand what went wrong and how to fix it.

Also implements RFC 6750 §3.1 compliance by distinguishing between
missing authentication (initial discovery) and invalid authentication
(token validation failure).


