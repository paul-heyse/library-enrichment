# `fastmcp.server.auth.providers.in_memory`

Distribution: `fastmcp`

## DEFAULT_ACCESS_TOKEN_EXPIRY_SECONDS

`fastmcp.server.auth.providers.in_memory.DEFAULT_ACCESS_TOKEN_EXPIRY_SECONDS`

```python
DEFAULT_ACCESS_TOKEN_EXPIRY_SECONDS = 60 * 60
```

**Inferred type** (`ty`, not declared in the source): `Literal[3600]`

## DEFAULT_AUTH_CODE_EXPIRY_SECONDS

`fastmcp.server.auth.providers.in_memory.DEFAULT_AUTH_CODE_EXPIRY_SECONDS`

```python
DEFAULT_AUTH_CODE_EXPIRY_SECONDS = 5 * 60
```

**Inferred type** (`ty`, not declared in the source): `Literal[300]`

## DEFAULT_REFRESH_TOKEN_EXPIRY_SECONDS

`fastmcp.server.auth.providers.in_memory.DEFAULT_REFRESH_TOKEN_EXPIRY_SECONDS`

```python
DEFAULT_REFRESH_TOKEN_EXPIRY_SECONDS = None
```

**Inferred type** (`ty`, not declared in the source): `None`

## InMemoryOAuthProvider

`fastmcp.server.auth.providers.in_memory.InMemoryOAuthProvider`

```python
class InMemoryOAuthProvider(OAuthProvider)
```

**Bases** `OAuthProvider`

**Declared members (14)**

- `access_tokens: dict[str, AccessToken] = {}`  _instance-attribute_
- `auth_codes: dict[str, AuthorizationCode] = {}`  _instance-attribute_
- `async def authorize(self, client: OAuthClientInformationFull, params: AuthorizationParams) -> str`  _async_
  Simulates user authorization and generates an authorization code. Returns a redirect URI with the code and state.
- `clients: dict[str, OAuthClientInformationFull] = {}`  _instance-attribute_
- `async def exchange_authorization_code(self, client: OAuthClientInformationFull, authorization_code: AuthorizationCode) -> OAuthToken`  _async_
- `async def exchange_refresh_token(self, client: OAuthClientInformationFull, refresh_token: RefreshToken, scopes: list[str]) -> OAuthToken`  _async_
- `async def get_client(self, client_id: str) -> OAuthClientInformationFull | None`  _async_
- `async def load_access_token(self, token: str) -> AccessToken | None`  _async_
- `async def load_authorization_code(self, client: OAuthClientInformationFull, authorization_code: str) -> AuthorizationCode | None`  _async_
- `async def load_refresh_token(self, client: OAuthClientInformationFull, refresh_token: str) -> RefreshToken | None`  _async_
- `refresh_tokens: dict[str, RefreshToken] = {}`  _instance-attribute_
- `async def register_client(self, client_info: OAuthClientInformationFull) -> None`  _async_
- `async def revoke_token(self, token: AccessToken | RefreshToken) -> None`  _async_
  Revokes an access or refresh token and its counterpart.
- `async def verify_token(self, token: str) -> AccessToken | None`  _async_
  Verify a bearer token and return access info if valid.

**Inherited (14)**

- from `fastmcp.server.auth.auth.AuthProvider`: `base_url`, `challenge_scopes`, `get_challenge_scopes`, `get_middleware`, `required_scopes`, `resource_base_url`, `set_mcp_path`
- from `fastmcp.server.auth.auth.OAuthProvider`: `client_registration_options`, `get_routes`, `get_well_known_routes`, `issuer_url`, `revocation_options`, `scopes_supported`, `service_documentation_url`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

An in-memory OAuth provider for testing purposes.
It simulates the OAuth 2.1 flow locally without external calls.


