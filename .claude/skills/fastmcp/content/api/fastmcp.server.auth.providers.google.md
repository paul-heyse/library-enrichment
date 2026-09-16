# `fastmcp.server.auth.providers.google`

Distribution: `fastmcp`

## GOOGLE_SCOPE_ALIASES

`fastmcp.server.auth.providers.google.GOOGLE_SCOPE_ALIASES`

```python
GOOGLE_SCOPE_ALIASES: dict[str, str] = {'email': 'https://www.googleapis.com/auth/userinfo.email', 'profile': 'https://www.googleapis.com/auth/userinfo.profile'}
```

## logger

`fastmcp.server.auth.providers.google.logger`

```python
logger = get_logger(__name__)
```

**Inferred type** (`ty`, not declared in the source): `Logger`

## GoogleProvider

`fastmcp.server.auth.providers.google.GoogleProvider`

```python
class GoogleProvider(OAuthProxy)
```

**Bases** `OAuthProxy`

**Inherited (28)**

- from `fastmcp.server.auth.auth.AuthProvider`: `base_url`, `challenge_scopes`, `get_challenge_scopes`, `get_middleware`, `required_scopes`, `resource_base_url`
- from `fastmcp.server.auth.auth.OAuthProvider`: `client_registration_options`, `get_well_known_routes`, `issuer_url`, `revocation_options`, `scopes_supported`, `service_documentation_url`, `verify_token`
- from `fastmcp.server.auth.oauth_proxy.proxy.OAuthProxy`: `authorize`, `exchange_authorization_code`, `exchange_identity_assertion`, `exchange_refresh_token`, `get_client`, `get_routes`, `jwt_issuer`, `load_access_token`, `load_authorization_code`, `load_refresh_token`, `register_client`, `revoke_token`, `set_mcp_path`, `token_endpoint_url`, `update_default_scopes`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

Complete Google OAuth provider for FastMCP.

This provider makes it trivial to add Google OAuth protection to any
FastMCP server. Just provide your Google OAuth app credentials and
a base URL, and you're ready to go.

Features:
- Transparent OAuth proxy to Google
- Automatic token validation via Google's tokeninfo API
- User information extraction from Google APIs
- Minimal configuration required

Example:
    ```python
    from fastmcp import FastMCP
    from fastmcp.server.auth.providers.google import GoogleProvider

    auth = GoogleProvider(
        client_id="123456789.apps.googleusercontent.com",
        client_secret="GOCSPX-abc123...",
        base_url="https://my-server.com"
    )

    mcp = FastMCP("My App", auth=auth)
    ```


## GoogleTokenVerifier

`fastmcp.server.auth.providers.google.GoogleTokenVerifier`

```python
class GoogleTokenVerifier(TokenVerifier)
```

**Bases** `TokenVerifier`

**Declared members (3)**

- `audience = audience`  _instance-attribute_
- `timeout_seconds = timeout_seconds`  _instance-attribute_
- `async def verify_token(self, token: str) -> AccessToken | None`  _async_
  Verify a Google OAuth token using the tokeninfo endpoint.

**Inherited (10)**

- from `fastmcp.server.auth.auth.AuthProvider`: `base_url`, `challenge_scopes`, `get_challenge_scopes`, `get_middleware`, `get_routes`, `get_well_known_routes`, `required_scopes`, `resource_base_url`, `scopes_supported`, `set_mcp_path`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

Token verifier for Google OAuth tokens.

Google OAuth tokens are opaque (not JWTs), so we verify them by calling
Google's tokeninfo endpoint with the access token as a query parameter.
This returns the OAuth app ID (``aud``), granted scopes, and expiry time.
User profile data (name, picture, etc.) is fetched separately from the
v2 userinfo endpoint when the token is valid.


## _normalize_google_scope

`fastmcp.server.auth.providers.google._normalize_google_scope`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _normalize_google_scope(scope: str) -> str
```

Normalize a Google scope shorthand to its canonical full URI.

Google accepts shorthand scopes like "email" and "profile" in authorization
requests, but returns the full URI form in token responses. This normalizes
to the full URI so comparisons work regardless of which form was used.


