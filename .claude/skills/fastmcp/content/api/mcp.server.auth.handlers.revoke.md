# `mcp.server.auth.handlers.revoke`

Distribution: `mcp`

## RevocationErrorResponse

`mcp.server.auth.handlers.revoke.RevocationErrorResponse`

```python
class RevocationErrorResponse(BaseModel)
```

**Bases** `BaseModel`

**Declared members (2)**

- `error: Literal['invalid_request', 'unauthorized_client']`  _instance-attribute_
- `error_description: str | None = None`  _class-attribute, instance-attribute_

## RevocationHandler

Import as `mcp.server.auth.routes.RevocationHandler`  ·  defined at `mcp.server.auth.handlers.revoke.RevocationHandler`

```python
class RevocationHandler
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Declared members (3)**

- `client_authenticator: ClientAuthenticator`  _instance-attribute_
- `async def handle(self, request: Request) -> Response`  _async_
  Handler for the OAuth 2.0 Token Revocation endpoint.
- `provider: OAuthAuthorizationServerProvider[Any, Any, Any]`  _instance-attribute_

## RevocationRequest

`mcp.server.auth.handlers.revoke.RevocationRequest`

```python
class RevocationRequest(BaseModel)
```

**Bases** `BaseModel`

**Declared members (4)**

- `client_id: str`  _instance-attribute_
- `client_secret: str | None`  _instance-attribute_
- `token: str`  _instance-attribute_
- `token_type_hint: Literal['access_token', 'refresh_token'] | None = None`  _class-attribute, instance-attribute_

See https://datatracker.ietf.org/doc/html/rfc7009#section-2.1


