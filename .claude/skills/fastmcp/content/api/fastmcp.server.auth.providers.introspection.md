# `fastmcp.server.auth.providers.introspection`

Distribution: `fastmcp`

## ClientAuthMethod

`fastmcp.server.auth.providers.introspection.ClientAuthMethod`

```python
ClientAuthMethod = Literal['client_secret_basic', 'client_secret_post']
```

**Inferred type** (`ty`, not declared in the source): ````xml <special-form 'Literal["client_secret_basic", "client_secret_post"]'> ````

## logger

`fastmcp.server.auth.providers.introspection.logger`

```python
logger = get_logger(__name__)
```

**Inferred type** (`ty`, not declared in the source): `Logger`

## IntrospectionTokenVerifier

Import as `fastmcp.server.auth.providers.propelauth.IntrospectionTokenVerifier`  ·  defined at `fastmcp.server.auth.providers.introspection.IntrospectionTokenVerifier`

```python
class IntrospectionTokenVerifier(TokenVerifier)
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `TokenVerifier`

**Declared members (7)**

- `client_auth_method: ClientAuthMethod = client_auth_method`  _instance-attribute_
- `client_id = client_id`  _instance-attribute_
- `client_secret = client_secret.get_secret_value() if isinstance(client_secret, SecretStr) else client_secret`  _instance-attribute_
- `introspection_url = introspection_url`  _instance-attribute_
- `logger = get_logger(__name__)`  _instance-attribute_
- `timeout_seconds = timeout_seconds`  _instance-attribute_
- `async def verify_token(self, token: str) -> AccessToken | None`  _async_
  Verify a bearer token using OAuth 2.0 Token Introspection (RFC 7662).

**Inherited (10)**

- from `fastmcp.server.auth.auth.AuthProvider`: `base_url`, `challenge_scopes`, `get_challenge_scopes`, `get_middleware`, `get_routes`, `get_well_known_routes`, `required_scopes`, `resource_base_url`, `scopes_supported`, `set_mcp_path`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

OAuth 2.0 Token Introspection verifier (RFC 7662).

This verifier validates opaque tokens by calling an OAuth 2.0 token introspection
endpoint. Unlike JWT verification which is stateless, token introspection requires
a network call to the authorization server for each token validation.

The verifier authenticates to the introspection endpoint using either:
- HTTP Basic Auth (client_secret_basic, default): credentials in Authorization header
- POST body authentication (client_secret_post): credentials in request body

Both methods are specified in RFC 6749 (OAuth 2.0) and RFC 7662 (Token Introspection).

Use this when:
- Your authorization server issues opaque (non-JWT) tokens
- You need to validate tokens from Auth0, Okta, Keycloak, or other OAuth servers
- Your tokens require real-time revocation checking
- Your authorization server supports RFC 7662 introspection

Caching is disabled by default to preserve real-time revocation semantics.
Set ``cache_ttl_seconds`` to enable caching and reduce load on the
introspection endpoint (e.g., ``cache_ttl_seconds=300`` for 5 minutes).

Example:
    ```python
    verifier = IntrospectionTokenVerifier(
        introspection_url="https://auth.example.com/oauth/introspect",
        client_id="my-service",
        client_secret="secret-key",
        required_scopes=["api:read"]
    )
    ```


