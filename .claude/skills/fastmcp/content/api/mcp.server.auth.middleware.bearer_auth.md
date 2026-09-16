# `mcp.server.auth.middleware.bearer_auth`

Distribution: `mcp`

## logger

`mcp.server.auth.middleware.bearer_auth.logger`

```python
logger = logging.getLogger(__name__)
```

**Inferred type** (`ty`, not declared in the source): `Logger`

## AuthenticatedUser

Import as `mcp.server.sse.AuthenticatedUser`  ·  defined at `mcp.server.auth.middleware.bearer_auth.AuthenticatedUser`

```python
class AuthenticatedUser(SimpleUser)
```

_4 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `SimpleUser`

**Declared members (2)**

- `access_token = auth_info`  _instance-attribute_
- `scopes = auth_info.scopes`  _instance-attribute_

User with authentication info.


## AuthorizationContext

Import as `mcp.server.sse.AuthorizationContext`  ·  defined at `mcp.server.auth.middleware.bearer_auth.AuthorizationContext`

```python
class AuthorizationContext(TypedDict)
```

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `TypedDict`

**Declared members (3)**

- `client_id: str`  _instance-attribute_
- `issuer: str | None`  _instance-attribute_
- `subject: str | None`  _instance-attribute_

## BearerAuthBackend

Import as `mcp.server.lowlevel.server.BearerAuthBackend`  ·  defined at `mcp.server.auth.middleware.bearer_auth.BearerAuthBackend`

```python
class BearerAuthBackend(AuthenticationBackend)
```

_3 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `AuthenticationBackend`

**Declared members (3)**

- `async def authenticate(self, conn: HTTPConnection)`  _async_
- `resource_server_url = resource_server_url`  _instance-attribute_
- `token_verifier = token_verifier`  _instance-attribute_

Authentication backend that validates Bearer tokens using a TokenVerifier.

When `resource_server_url` is given, only a token whose `AccessToken.resource`
(its RFC 8707 resource indicator / audience) is that URL is accepted.


## RequireAuthMiddleware

Import as `mcp.server.lowlevel.server.RequireAuthMiddleware`  ·  defined at `mcp.server.auth.middleware.bearer_auth.RequireAuthMiddleware`

```python
class RequireAuthMiddleware
```

_4 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Declared members (3)**

- `app = app`  _instance-attribute_
- `required_scopes = required_scopes`  _instance-attribute_
- `resource_metadata_url = resource_metadata_url`  _instance-attribute_

Middleware that requires a valid Bearer token in the Authorization header.

This will validate the token with the auth provider and store the resulting
auth info in the request state.


## authorization_context

Import as `mcp.server.sse.authorization_context`  ·  defined at `mcp.server.auth.middleware.bearer_auth.authorization_context`

```python
def authorization_context(user: AuthenticatedUser) -> AuthorizationContext
```

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Identify the principal `user` represents, for transports to compare
against the principal that created a session. Components the token
verifier does not supply are `None`, so the comparison degrades to the
remaining components.

See `examples/servers/simple-auth/mcp_simple_auth/token_verifier.py` for
a verifier that populates `subject` and `claims` from an introspection
response.


