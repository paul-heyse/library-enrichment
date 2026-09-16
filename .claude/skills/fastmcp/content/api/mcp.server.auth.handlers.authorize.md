# `mcp.server.auth.handlers.authorize`

Distribution: `mcp`

## _ANY_URL_ADAPTER

`mcp.server.auth.handlers.authorize._ANY_URL_ADAPTER`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_ANY_URL_ADAPTER = TypeAdapter(AnyUrl)
```

## logger

`mcp.server.auth.handlers.authorize.logger`

```python
logger = logging.getLogger(__name__)
```

**Inferred type** (`ty`, not declared in the source): `Logger`

## AuthorizationErrorResponse

`mcp.server.auth.handlers.authorize.AuthorizationErrorResponse`

```python
class AuthorizationErrorResponse(BaseModel)
```

**Bases** `BaseModel`

**Declared members (4)**

- `error: AuthorizationErrorCode`  _instance-attribute_
- `error_description: str | None`  _instance-attribute_
- `error_uri: AnyUrl | None = None`  _class-attribute, instance-attribute_
- `state: str | None = None`  _class-attribute, instance-attribute_

## AuthorizationHandler

Import as `mcp.server.auth.routes.AuthorizationHandler`  ·  defined at `mcp.server.auth.handlers.authorize.AuthorizationHandler`

```python
class AuthorizationHandler
```

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Declared members (2)**

- `async def handle(self, request: Request) -> Response`  _async_
- `provider: OAuthAuthorizationServerProvider[Any, Any, Any]`  _instance-attribute_

## AuthorizationRequest

`mcp.server.auth.handlers.authorize.AuthorizationRequest`

```python
class AuthorizationRequest(BaseModel)
```

**Bases** `BaseModel`

**Declared members (8)**

- `client_id: str = Field(..., description='The client ID')`  _class-attribute, instance-attribute_
- `code_challenge: str = Field(..., description='PKCE code challenge')`  _class-attribute, instance-attribute_
- `code_challenge_method: Literal['S256'] = Field('S256', description='PKCE code challenge method, must be S256')`  _class-attribute, instance-attribute_
- `redirect_uri: AnyUrl | None = Field(None, description='URL to redirect to after authorization')`  _class-attribute, instance-attribute_
- `resource: str | None = Field(None, description='RFC 8707 resource indicator - the MCP server this token will be used with')`  _class-attribute, instance-attribute_
- `response_type: Literal['code'] = Field(..., description="Must be 'code' for authorization code flow")`  _class-attribute, instance-attribute_
- `scope: str | None = Field(None, description='Optional scope; if specified, should be a space-separated list of scope strings')`  _class-attribute, instance-attribute_
- `state: str | None = Field(None, description='Optional state parameter')`  _class-attribute, instance-attribute_

## best_effort_extract_string

`mcp.server.auth.handlers.authorize.best_effort_extract_string`

```python
def best_effort_extract_string(key: str, params: None | FormData | QueryParams) -> str | None
```

