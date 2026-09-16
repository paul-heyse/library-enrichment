# `fastmcp.server.auth.providers.clerk`

Distribution: `fastmcp`

## logger

`fastmcp.server.auth.providers.clerk.logger`

```python
logger = get_logger(__name__)
```

**Inferred type** (`ty`, not declared in the source): `Logger`

## ClerkProvider

`fastmcp.server.auth.providers.clerk.ClerkProvider`

```python
class ClerkProvider(OAuthProxy)
```

**Bases** `OAuthProxy`

**Inherited (28)**

- from `fastmcp.server.auth.auth.AuthProvider`: `base_url`, `challenge_scopes`, `get_challenge_scopes`, `get_middleware`, `required_scopes`, `resource_base_url`
- from `fastmcp.server.auth.auth.OAuthProvider`: `client_registration_options`, `get_well_known_routes`, `issuer_url`, `revocation_options`, `scopes_supported`, `service_documentation_url`, `verify_token`
- from `fastmcp.server.auth.oauth_proxy.proxy.OAuthProxy`: `authorize`, `exchange_authorization_code`, `exchange_identity_assertion`, `exchange_refresh_token`, `get_client`, `get_routes`, `jwt_issuer`, `load_access_token`, `load_authorization_code`, `load_refresh_token`, `register_client`, `revoke_token`, `set_mcp_path`, `token_endpoint_url`, `update_default_scopes`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

Complete Clerk OAuth provider for FastMCP.

This provider makes it trivial to add Clerk OAuth protection to any
FastMCP server. Provide your Clerk instance domain, OAuth app credentials,
and a base URL, and you're ready to go.

Clerk uses standard OIDC endpoints derived from the instance domain.
All endpoint URLs are constructed automatically from the domain parameter.

Features:
- Transparent OAuth proxy to Clerk
- Automatic token validation via Clerk's userinfo & introspection APIs
- User information extraction from Clerk's OIDC claims
- PKCE support (S256)
- Minimal configuration required

Example:
    ```python
    from fastmcp import FastMCP
    from fastmcp.server.auth.providers.clerk import ClerkProvider

    auth = ClerkProvider(
        domain="saving-primate-16.clerk.accounts.dev",
        client_id="your-clerk-client-id",
        client_secret="your-clerk-client-secret",
        base_url="https://my-server.com",
    )

    mcp = FastMCP("My App", auth=auth)
    ```


## ClerkTokenVerifier

`fastmcp.server.auth.providers.clerk.ClerkTokenVerifier`

```python
class ClerkTokenVerifier(TokenVerifier)
```

**Bases** `TokenVerifier`

**Declared members (3)**

- `domain = domain.rstrip('/')`  _instance-attribute_
- `timeout_seconds = timeout_seconds`  _instance-attribute_
- `async def verify_token(self, token: str) -> AccessToken | None`  _async_
  Verify a Clerk OAuth token via introspection and userinfo.

**Inherited (10)**

- from `fastmcp.server.auth.auth.AuthProvider`: `base_url`, `challenge_scopes`, `get_challenge_scopes`, `get_middleware`, `get_routes`, `get_well_known_routes`, `required_scopes`, `resource_base_url`, `scopes_supported`, `set_mcp_path`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

Token verifier for Clerk OAuth tokens.

Clerk issues standard OIDC tokens. Verification uses the introspection
endpoint (RFC 7662) as the primary security gate — it confirms the token
is active and provides metadata (scopes, expiry, audience). The userinfo
endpoint is called second for profile enrichment (name, email, picture)
and its failure is non-fatal.

When a ``client_id`` is configured, the audience from introspection is
validated against it. When ``required_scopes`` are configured,
introspection must return the token's scopes — the verifier will not
assume scopes when introspection is unavailable.


