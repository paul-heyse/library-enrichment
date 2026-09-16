# `mcp.server.auth.handlers.token`

Distribution: `mcp`

## TokenRequest

`mcp.server.auth.handlers.token.TokenRequest`

```python
TokenRequest = Annotated[AuthorizationCodeRequest | RefreshTokenRequest | JwtBearerRequest, Field(discriminator='grant_type')]
```

**Inferred type** (`ty`, not declared in the source): ````xml <special-form 'typing.Annotated[AuthorizationCodeRequest | RefreshTokenRequest | JwtBearerRequest, <metadata>]'> ````

## TokenSuccessResponse

`mcp.server.auth.handlers.token.TokenSuccessResponse`

```python
TokenSuccessResponse = OAuthToken
```

**Inferred type** (`ty`, not declared in the source): ````xml <class 'OAuthToken'> ````

## token_request_adapter

`mcp.server.auth.handlers.token.token_request_adapter`

```python
token_request_adapter = TypeAdapter[TokenRequest](TokenRequest)
```

**Inferred type** (`ty`, not declared in the source): `TypeAdapter[AuthorizationCodeRequest | RefreshTokenRequest | JwtBearerRequest]`

## AuthorizationCodeRequest

`mcp.server.auth.handlers.token.AuthorizationCodeRequest`

```python
class AuthorizationCodeRequest(BaseModel)
```

**Bases** `BaseModel`

**Declared members (7)**

- `client_id: str`  _instance-attribute_
- `client_secret: str | None = None`  _class-attribute, instance-attribute_
- `code: str = Field(..., description='The authorization code')`  _class-attribute, instance-attribute_
- `code_verifier: str = Field(..., description='PKCE code verifier')`  _class-attribute, instance-attribute_
- `grant_type: Literal['authorization_code']`  _instance-attribute_
- `redirect_uri: AnyUrl | None = Field(None, description='Must be the same as redirect URI provided in /authorize')`  _class-attribute, instance-attribute_
- `resource: str | None = Field(None, description='Resource indicator for the token')`  _class-attribute, instance-attribute_

## JwtBearerRequest

`mcp.server.auth.handlers.token.JwtBearerRequest`

```python
class JwtBearerRequest(BaseModel)
```

**Bases** `BaseModel`

**Declared members (6)**

- `assertion: str = Field(..., description='The ID-JAG (a signed JWT) being presented as the grant')`  _class-attribute, instance-attribute_
- `client_id: str`  _instance-attribute_
- `client_secret: str | None = None`  _class-attribute, instance-attribute_
- `grant_type: Literal['urn:ietf:params:oauth:grant-type:jwt-bearer']`  _instance-attribute_
- `resource: str | None = Field(None, description='Resource indicator for the token')`  _class-attribute, instance-attribute_
- `scope: str | None = Field(None, description='Optional scope parameter')`  _class-attribute, instance-attribute_

## RefreshTokenRequest

`mcp.server.auth.handlers.token.RefreshTokenRequest`

```python
class RefreshTokenRequest(BaseModel)
```

**Bases** `BaseModel`

**Declared members (6)**

- `client_id: str`  _instance-attribute_
- `client_secret: str | None = None`  _class-attribute, instance-attribute_
- `grant_type: Literal['refresh_token']`  _instance-attribute_
- `refresh_token: str = Field(..., description='The refresh token')`  _class-attribute, instance-attribute_
- `resource: str | None = Field(None, description='Resource indicator for the token')`  _class-attribute, instance-attribute_
- `scope: str | None = Field(None, description='Optional scope parameter')`  _class-attribute, instance-attribute_

## TokenErrorResponse

Import as `fastmcp.server.auth.auth.TokenErrorResponse`  ·  defined at `mcp.server.auth.handlers.token.TokenErrorResponse`

```python
class TokenErrorResponse(BaseModel)
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `BaseModel`

**Declared members (3)**

- `error: TokenErrorCode`  _instance-attribute_
- `error_description: str | None = None`  _class-attribute, instance-attribute_
- `error_uri: AnyHttpUrl | None = None`  _class-attribute, instance-attribute_

See https://datatracker.ietf.org/doc/html/rfc6749#section-5.2


## TokenHandler

Import as `mcp.server.auth.routes.TokenHandler`  ·  defined at `mcp.server.auth.handlers.token.TokenHandler`

```python
class TokenHandler
```

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Declared members (5)**

- `client_authenticator: ClientAuthenticator`  _instance-attribute_
- `async def handle(self, request: Request)`  _async_
- `identity_assertion_enabled: bool = False`  _class-attribute, instance-attribute_
- `provider: OAuthAuthorizationServerProvider[Any, Any, Any]`  _instance-attribute_
- `def response(self, obj: TokenSuccessResponse | TokenErrorResponse)`

