# `mcp.client.auth.oauth2`

Distribution: `mcp`

## _KNOWN_TOKEN_ENDPOINT_AUTH_METHODS

`mcp.client.auth.oauth2._KNOWN_TOKEN_ENDPOINT_AUTH_METHODS`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_KNOWN_TOKEN_ENDPOINT_AUTH_METHODS: tuple[str | None, ...] = (None, *get_args(TokenEndpointAuthMethod))
```

## _ORIGIN_URL

`mcp.client.auth.oauth2._ORIGIN_URL`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_ORIGIN_URL = TypeAdapter(AnyHttpUrl, config=ConfigDict(url_preserve_empty_path=True))
```

## _REGISTRATION_USABLE_TOKEN_ENDPOINT_AUTH_METHODS

`mcp.client.auth.oauth2._REGISTRATION_USABLE_TOKEN_ENDPOINT_AUTH_METHODS`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_REGISTRATION_USABLE_TOKEN_ENDPOINT_AUTH_METHODS: tuple[str | None, ...] = tuple(method for method in _KNOWN_TOKEN_ENDPOINT_AUTH_METHODS if method != 'private_key_jwt')
```

## _SECRET_TOKEN_ENDPOINT_AUTH_METHODS

`mcp.client.auth.oauth2._SECRET_TOKEN_ENDPOINT_AUTH_METHODS`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_SECRET_TOKEN_ENDPOINT_AUTH_METHODS = ('client_secret_post', 'client_secret_basic')
```

## logger

`mcp.client.auth.oauth2.logger`

```python
logger = logging.getLogger(__name__)
```

**Inferred type** (`ty`, not declared in the source): `Logger`

## OAuthClientProvider

Import as `mcp.client.auth.OAuthClientProvider`  ·  defined at `mcp.client.auth.oauth2.OAuthClientProvider`

```python
class OAuthClientProvider(RedirectAwareAuth)
```

**Also exported as** `mcp.client.auth.OAuthClientProvider`

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `RedirectAwareAuth`

**Declared members (2)**

- `context = OAuthContext(server_url=server_url, client_metadata=client_metadata, storage=storage, redirect_handler=redirect_handler, callback_handler=callback_handler, client_metadata_url=client_metadata_url)`  _instance-attribute_
- `requires_response_body = True`  _class-attribute, instance-attribute_

**Inherited (1)**

- from `mcp.shared._httpx_utils.RedirectAwareAuth`: `async_auth_flow`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

OAuth2 authentication for httpx2.

Handles OAuth flow with automatic client registration and token storage.


## OAuthContext

Import as `mcp.client.auth.extensions.client_credentials.OAuthContext`  ·  defined at `mcp.client.auth.oauth2.OAuthContext`

```python
class OAuthContext
```

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Declared members (22)**

- `auth_server_url: str | None = None`  _class-attribute, instance-attribute_
- `callback_handler: Callable[[], Awaitable[AuthorizationCodeResult]] | None`  _instance-attribute_
- `def can_refresh_token(self) -> bool`
  Check if token can be refreshed.
- `def clear_tokens(self) -> None`
  Clear current tokens.
- `client_info: OAuthClientInformationFull | None = None`  _class-attribute, instance-attribute_
- `client_metadata: OAuthClientMetadata`  _instance-attribute_
- `client_metadata_url: str | None = None`  _class-attribute, instance-attribute_
- `current_tokens: OAuthToken | None = None`  _class-attribute, instance-attribute_
- `def get_authorization_base_url(self, server_url: str) -> str`
  Extract base URL by removing path component.
- `def get_resource_url(self) -> str`
  Get resource URL for RFC 8707.
- `def is_token_valid(self) -> bool`
  Check if current token is valid.
- `lock: anyio.Lock = field(default_factory=anyio.Lock)`  _class-attribute, instance-attribute_
- `oauth_metadata: OAuthMetadata | None = None`  _class-attribute, instance-attribute_
- `def prepare_token_auth(self, data: dict[str, str], headers: dict[str, str] | None = None) -> tuple[dict[str, str], dict[str, str]]`
  Prepare authentication for token requests.
- `protected_resource_metadata: ProtectedResourceMetadata | None = None`  _class-attribute, instance-attribute_
- `protocol_version: str | None = None`  _class-attribute, instance-attribute_
- `redirect_handler: Callable[[str], Awaitable[None]] | None`  _instance-attribute_
- `server_url: str`  _instance-attribute_
- `def should_include_resource_param(self, protocol_version: str | None = None) -> bool`
  Determine if the resource parameter should be included in OAuth requests.
- `storage: TokenStorage`  _instance-attribute_
- `token_expiry_time: float | None = None`  _class-attribute, instance-attribute_
- `def update_token_expiry(self, token: OAuthToken) -> None`
  Update token expiry time using shared util function.

OAuth flow context.


## PKCEParameters

Import as `mcp.client.auth.PKCEParameters`  ·  defined at `mcp.client.auth.oauth2.PKCEParameters`

```python
class PKCEParameters(BaseModel)
```

**Also exported as** `mcp.client.auth.PKCEParameters`

**Bases** `BaseModel`

**Declared members (3)**

- `code_challenge: str = Field(..., min_length=43, max_length=128)`  _class-attribute, instance-attribute_
- `code_verifier: str = Field(..., min_length=43, max_length=128)`  _class-attribute, instance-attribute_
- `def generate(cls) -> PKCEParameters`  _classmethod_
  Generate new PKCE parameters.

PKCE (Proof Key for Code Exchange) parameters.


## TokenStorage

Import as `mcp.client.auth.TokenStorage`  ·  defined at `mcp.client.auth.oauth2.TokenStorage`

```python
class TokenStorage(Protocol)
```

**Also exported as** `mcp.client.auth.TokenStorage`

_3 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `Protocol`

**Declared members (4)**

- `async def get_client_info(self) -> OAuthClientInformationFull | None`  _async_
  Get stored client information.
- `async def get_tokens(self) -> OAuthToken | None`  _async_
  Get stored tokens.
- `async def set_client_info(self, client_info: OAuthClientInformationFull) -> None`  _async_
  Store client information.
- `async def set_tokens(self, tokens: OAuthToken) -> None`  _async_
  Store tokens.

Protocol for token storage implementations.


## _origin_issuer

`mcp.client.auth.oauth2._origin_issuer`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _origin_issuer(server_url: str) -> str
```

The resource server's origin as an issuer identifier: `scheme://authority`, rendered the way
`OAuthMetadata.issuer` renders URLs (host case, default ports) so the two compare as strings.


## check_registration_usable

`mcp.client.auth.oauth2.check_registration_usable`

```python
def check_registration_usable(client_info: OAuthClientInformationFull) -> None
```

Confirm a registration this flow completed is one it can act on.

RFC 7591 §3.2.1 lets the authorization server replace requested metadata and leaves it to
the client to "check the values in the response to determine if the registration is
sufficient for use". Two substitutions make the minted credentials unusable, and both are
judged here - before the record is persisted or any interactive authorization begins -
rather than surfacing later as an opaque failure at the token endpoint: a token-endpoint
auth method the authorization-code flow cannot apply (one it does not implement, or
`private_key_jwt`, whose assertion this flow has no key to sign), and a secret-based
method the flow could apply but for which the server issued no `client_secret`.

Raises:
    OAuthRegistrationError: The server registered the client with a
        `token_endpoint_auth_method` this flow cannot apply, or with a secret-based
        method but no `client_secret`.


