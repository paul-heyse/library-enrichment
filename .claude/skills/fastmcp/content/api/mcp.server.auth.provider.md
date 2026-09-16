# `mcp.server.auth.provider`

Distribution: `mcp`

## AccessTokenT

`mcp.server.auth.provider.AccessTokenT`

```python
AccessTokenT = TypeVar('AccessTokenT', bound=AccessToken)
```

**Inferred type** (`ty`, not declared in the source): `TypeVar`

## AuthorizationCodeT

`mcp.server.auth.provider.AuthorizationCodeT`

```python
AuthorizationCodeT = TypeVar('AuthorizationCodeT', bound=AuthorizationCode)
```

**Inferred type** (`ty`, not declared in the source): `TypeVar`

## AuthorizationErrorCode

Import as `mcp.server.auth.handlers.authorize.AuthorizationErrorCode`  ·  defined at `mcp.server.auth.provider.AuthorizationErrorCode`

```python
AuthorizationErrorCode = Literal['invalid_request', 'unauthorized_client', 'access_denied', 'unsupported_response_type', 'invalid_scope', 'server_error', 'temporarily_unavailable', 'invalid_target']
```

**Inferred type** (`ty`, not declared in the source): ````xml <special-form 'Literal["invalid_request", "unauthorized_client", "access_denied", "unsupported_response_type", "invalid_scope", ... omitted 3 literals]'> ````

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

## RefreshTokenT

`mcp.server.auth.provider.RefreshTokenT`

```python
RefreshTokenT = TypeVar('RefreshTokenT', bound=RefreshToken)
```

**Inferred type** (`ty`, not declared in the source): `TypeVar`

## RegistrationErrorCode

Import as `mcp.server.auth.handlers.register.RegistrationErrorCode`  ·  defined at `mcp.server.auth.provider.RegistrationErrorCode`

```python
RegistrationErrorCode = Literal['invalid_redirect_uri', 'invalid_client_metadata', 'invalid_software_statement', 'unapproved_software_statement']
```

**Inferred type** (`ty`, not declared in the source): ````xml <special-form 'Literal["invalid_redirect_uri", "invalid_client_metadata", "invalid_software_statement", "unapproved_software_statement"]'> ````

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

## TokenErrorCode

Import as `mcp.server.auth.handlers.token.TokenErrorCode`  ·  defined at `mcp.server.auth.provider.TokenErrorCode`

```python
TokenErrorCode = Literal['invalid_request', 'invalid_client', 'invalid_grant', 'unauthorized_client', 'unsupported_grant_type', 'invalid_scope', 'invalid_target']
```

**Inferred type** (`ty`, not declared in the source): ````xml <special-form 'Literal["invalid_request", "invalid_client", "invalid_grant", "unauthorized_client", "unsupported_grant_type", "invalid_scope", "invalid_target"]'> ````

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

## AccessToken

Import as `mcp.server.auth.handlers.revoke.AccessToken`  ·  defined at `mcp.server.auth.provider.AccessToken`

```python
class AccessToken(BaseModel)
```

**Also exported as** `fastmcp.server.dependencies._SDKAccessToken`

_6 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `BaseModel`

**Declared members (7)**

- `claims: dict[str, Any] | None = None`  _class-attribute, instance-attribute_
- `client_id: str`  _instance-attribute_
- `expires_at: int | None = None`  _class-attribute, instance-attribute_
- `resource: str | None = None`  _class-attribute, instance-attribute_
- `scopes: list[str]`  _instance-attribute_
- `subject: str | None = None`  _class-attribute, instance-attribute_
- `token: str`  _instance-attribute_

## AuthorizationCode

Import as `fastmcp.server.auth.auth.AuthorizationCode`  ·  defined at `mcp.server.auth.provider.AuthorizationCode`

```python
class AuthorizationCode(BaseModel)
```

_3 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `BaseModel`

**Declared members (9)**

- `client_id: str`  _instance-attribute_
- `code: str`  _instance-attribute_
- `code_challenge: str`  _instance-attribute_
- `expires_at: float`  _instance-attribute_
- `redirect_uri: AnyUrl`  _instance-attribute_
- `redirect_uri_provided_explicitly: bool`  _instance-attribute_
- `resource: str | None = None`  _class-attribute, instance-attribute_
- `scopes: list[str]`  _instance-attribute_
- `subject: str | None = None`  _class-attribute, instance-attribute_

## AuthorizationParams

Import as `mcp.server.auth.handlers.authorize.AuthorizationParams`  ·  defined at `mcp.server.auth.provider.AuthorizationParams`

```python
class AuthorizationParams(BaseModel)
```

_4 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `BaseModel`

**Declared members (6)**

- `code_challenge: str`  _instance-attribute_
- `redirect_uri: AnyUrl`  _instance-attribute_
- `redirect_uri_provided_explicitly: bool`  _instance-attribute_
- `resource: str | None = None`  _class-attribute, instance-attribute_
- `scopes: list[str] | None`  _instance-attribute_
- `state: str | None`  _instance-attribute_

## AuthorizeError

Import as `mcp.server.auth.handlers.authorize.AuthorizeError`  ·  defined at `mcp.server.auth.provider.AuthorizeError`

```python
class AuthorizeError(Exception)
```

_3 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `Exception`

**Declared members (2)**

- `error: AuthorizationErrorCode`  _instance-attribute_
- `error_description: str | None = None`  _class-attribute, instance-attribute_

## IdentityAssertionParams

Import as `mcp.server.auth.handlers.token.IdentityAssertionParams`  ·  defined at `mcp.server.auth.provider.IdentityAssertionParams`

```python
class IdentityAssertionParams(BaseModel)
```

_3 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `BaseModel`

**Declared members (3)**

- `assertion: str`  _instance-attribute_
- `resource: str | None = None`  _class-attribute, instance-attribute_
- `scopes: list[str] | None = None`  _class-attribute, instance-attribute_

Validated parameters of a SEP-990 identity-assertion (RFC 7523 jwt-bearer) request.

Passed to ``OAuthAuthorizationServerProvider.exchange_identity_assertion``. ``assertion`` is the
ID-JAG (a signed JWT) the enterprise identity provider issued; the provider validates it per
RFC 7523 §3 and the SEP-990 §5.1 processing rules before issuing an access token.


## OAuthAuthorizationServerProvider

Import as `mcp.server.auth.routes.OAuthAuthorizationServerProvider`  ·  defined at `mcp.server.auth.provider.OAuthAuthorizationServerProvider`

```python
class OAuthAuthorizationServerProvider(Protocol, Generic[AuthorizationCodeT, RefreshTokenT, AccessTokenT])
```

_10 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `Protocol`, `Generic[AuthorizationCodeT, RefreshTokenT, AccessTokenT]`

**Declared members (10)**

- `async def authorize(self, client: OAuthClientInformationFull, params: AuthorizationParams) -> str`  _async_
  Handle the /authorize endpoint and return a URL that the client will be redirected to.
- `async def exchange_authorization_code(self, client: OAuthClientInformationFull, authorization_code: AuthorizationCodeT) -> OAuthToken`  _async_
  Exchanges an authorization code for an access token and refresh token.
- `async def exchange_identity_assertion(self, client: OAuthClientInformationFull, params: IdentityAssertionParams) -> OAuthToken`  _async_
  Exchanges an Identity Assertion Authorization Grant (ID-JAG) for an access token.
- `async def exchange_refresh_token(self, client: OAuthClientInformationFull, refresh_token: RefreshTokenT, scopes: list[str]) -> OAuthToken`  _async_
  Exchanges a refresh token for an access token and refresh token.
- `async def get_client(self, client_id: str) -> OAuthClientInformationFull | None`  _async_
  Retrieves client information by client ID.
- `async def load_access_token(self, token: str) -> AccessTokenT | None`  _async_
  Loads an access token by its token string.
- `async def load_authorization_code(self, client: OAuthClientInformationFull, authorization_code: str) -> AuthorizationCodeT | None`  _async_
  Loads an AuthorizationCode by its code.
- `async def load_refresh_token(self, client: OAuthClientInformationFull, refresh_token: str) -> RefreshTokenT | None`  _async_
  Loads a RefreshToken by its token string.
- `async def register_client(self, client_info: OAuthClientInformationFull) -> None`  _async_
  Saves client information as part of registering it.
- `async def revoke_token(self, token: AccessTokenT | RefreshTokenT) -> None`  _async_
  Revokes an access or refresh token.

## ProviderTokenVerifier

Import as `mcp.server.mcpserver.server.ProviderTokenVerifier`  ·  defined at `mcp.server.auth.provider.ProviderTokenVerifier`

```python
class ProviderTokenVerifier(TokenVerifier)
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `TokenVerifier`

**Declared members (2)**

- `provider = provider`  _instance-attribute_
- `async def verify_token(self, token: str) -> AccessToken | None`  _async_
  Verify token using the provider's load_access_token method.

Token verifier that uses an OAuthAuthorizationServerProvider.

This is provided for backwards compatibility with existing auth_server_provider
configurations. For new implementations using AS/RS separation, consider using
the TokenVerifier protocol with a dedicated implementation like IntrospectionTokenVerifier.


## RefreshToken

Import as `mcp.server.auth.handlers.revoke.RefreshToken`  ·  defined at `mcp.server.auth.provider.RefreshToken`

```python
class RefreshToken(BaseModel)
```

_4 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `BaseModel`

**Declared members (6)**

- `client_id: str`  _instance-attribute_
- `expires_at: int | None = None`  _class-attribute, instance-attribute_
- `resource: str | None = None`  _class-attribute, instance-attribute_
- `scopes: list[str]`  _instance-attribute_
- `subject: str | None = None`  _class-attribute, instance-attribute_
- `token: str`  _instance-attribute_

## RegistrationError

Import as `mcp.server.auth.handlers.register.RegistrationError`  ·  defined at `mcp.server.auth.provider.RegistrationError`

```python
class RegistrationError(Exception)
```

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `Exception`

**Declared members (2)**

- `error: RegistrationErrorCode`  _instance-attribute_
- `error_description: str | None = None`  _class-attribute, instance-attribute_

## TokenError

Import as `mcp.server.auth.handlers.token.TokenError`  ·  defined at `mcp.server.auth.provider.TokenError`

```python
class TokenError(Exception)
```

_4 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `Exception`

**Declared members (2)**

- `error: TokenErrorCode`  _instance-attribute_
- `error_description: str | None = None`  _class-attribute, instance-attribute_

## TokenVerifier

Import as `mcp.server.lowlevel.server.TokenVerifier`  ·  defined at `mcp.server.auth.provider.TokenVerifier`

```python
class TokenVerifier(Protocol)
```

_4 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `Protocol`

**Declared members (1)**

- `async def verify_token(self, token: str) -> AccessToken | None`  _async_
  Verify a bearer token and return access info if valid.

Protocol for verifying bearer tokens.


## construct_redirect_uri

Import as `mcp.server.auth.handlers.authorize.construct_redirect_uri`  ·  defined at `mcp.server.auth.provider.construct_redirect_uri`

```python
def construct_redirect_uri(redirect_uri_base: str, params: str | None = {}) -> str
```

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

## principal_components

Import as `mcp.server.request_state.principal_components`  ·  defined at `mcp.server.auth.provider.principal_components`

```python
def principal_components(token: AccessToken) -> tuple[str, str | None, str | None]
```

_3 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

The (client_id, issuer, subject) triple identifying the principal a token represents.

The single source for "who is this token's principal": session ownership and
request-state binding both build on it. Components the token verifier does
not supply are `None`, so comparisons degrade to the remaining components.


