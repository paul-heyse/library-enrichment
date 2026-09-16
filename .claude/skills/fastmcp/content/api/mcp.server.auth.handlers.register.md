# `mcp.server.auth.handlers.register`

Distribution: `mcp`

## RegistrationRequest

`mcp.server.auth.handlers.register.RegistrationRequest`

```python
RegistrationRequest = OAuthClientMetadata
```

**Inferred type** (`ty`, not declared in the source): ````xml <class 'OAuthClientMetadata'> ````

## RegistrationErrorResponse

`mcp.server.auth.handlers.register.RegistrationErrorResponse`

```python
class RegistrationErrorResponse(BaseModel)
```

**Bases** `BaseModel`

**Declared members (2)**

- `error: RegistrationErrorCode`  _instance-attribute_
- `error_description: str | None`  _instance-attribute_

## RegistrationHandler

Import as `mcp.server.auth.routes.RegistrationHandler`  ·  defined at `mcp.server.auth.handlers.register.RegistrationHandler`

```python
class RegistrationHandler
```

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Declared members (3)**

- `async def handle(self, request: Request) -> Response`  _async_
- `options: ClientRegistrationOptions`  _instance-attribute_
- `provider: OAuthAuthorizationServerProvider[Any, Any, Any]`  _instance-attribute_

