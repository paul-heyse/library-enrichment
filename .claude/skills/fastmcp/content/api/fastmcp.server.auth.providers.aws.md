# `fastmcp.server.auth.providers.aws`

Distribution: `fastmcp`

## logger

`fastmcp.server.auth.providers.aws.logger`

```python
logger = get_logger(__name__)
```

**Inferred type** (`ty`, not declared in the source): `Logger`

## AWSCognitoProvider

`fastmcp.server.auth.providers.aws.AWSCognitoProvider`

```python
class AWSCognitoProvider(OIDCProxy)
```

**Bases** `OIDCProxy`

**Declared members (4)**

- `aws_region = aws_region`  _instance-attribute_
- `client_id = client_id`  _instance-attribute_
- `def get_token_verifier(self, algorithm: str | None = None, audience: str | None = None, required_scopes: list[str] | None = None, timeout_seconds: int | None = None) -> AWSCognitoTokenVerifier`
  Creates a Cognito-specific token verifier with claim filtering.
- `user_pool_id = user_pool_id`  _instance-attribute_

**Inherited (30)**

- from `fastmcp.server.auth.auth.AuthProvider`: `base_url`, `challenge_scopes`, `get_challenge_scopes`, `get_middleware`, `resource_base_url`
- from `fastmcp.server.auth.auth.OAuthProvider`: `client_registration_options`, `get_well_known_routes`, `issuer_url`, `revocation_options`, `scopes_supported`, `service_documentation_url`, `verify_token`
- from `fastmcp.server.auth.oauth_proxy.proxy.OAuthProxy`: `authorize`, `exchange_authorization_code`, `exchange_identity_assertion`, `exchange_refresh_token`, `get_client`, `get_routes`, `jwt_issuer`, `load_access_token`, `load_authorization_code`, `load_refresh_token`, `register_client`, `revoke_token`, `set_mcp_path`, `token_endpoint_url`, `update_default_scopes`
- from `fastmcp.server.auth.oidc_proxy.OIDCProxy`: `get_oidc_configuration`, `oidc_config`, `required_scopes`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

Complete AWS Cognito OAuth provider for FastMCP.

This provider makes it trivial to add AWS Cognito OAuth protection to any
FastMCP server using OIDC Discovery. Just provide your Cognito User Pool details,
client credentials, and a base URL, and you're ready to go.

Features:
- Automatic OIDC Discovery from AWS Cognito User Pool
- Automatic JWT token validation via Cognito's public keys
- Cognito-specific claim filtering (sub, username, cognito:groups)
- Support for Cognito User Pools

Example:
    ```python
    from fastmcp import FastMCP
    from fastmcp.server.auth.providers.aws_cognito import AWSCognitoProvider

    auth = AWSCognitoProvider(
        user_pool_id="eu-central-1_XXXXXXXXX",
        aws_region="eu-central-1",
        client_id="your-cognito-client-id",
        client_secret="your-cognito-client-secret",
        base_url="https://my-server.com",
        redirect_path="/custom/callback",
    )

    mcp = FastMCP("My App", auth=auth)
    ```


## AWSCognitoTokenVerifier

`fastmcp.server.auth.providers.aws.AWSCognitoTokenVerifier`

```python
class AWSCognitoTokenVerifier(JWTVerifier)
```

**Bases** `JWTVerifier`

**Declared members (1)**

- `async def verify_token(self, token: str) -> AccessToken | None`  _async_
  Verify token and filter claims to Cognito-specific subset.

**Inherited (18)**

- from `fastmcp.server.auth.auth.AuthProvider`: `base_url`, `challenge_scopes`, `get_challenge_scopes`, `get_middleware`, `get_routes`, `get_well_known_routes`, `required_scopes`, `resource_base_url`, `scopes_supported`, `set_mcp_path`
- from `fastmcp.server.auth.providers.jwt.JWTVerifier`: `algorithm`, `audience`, `issuer`, `jwks_uri`, `load_access_token`, `logger`, `public_key`, `ssrf_safe`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

Token verifier for Cognito access tokens.

Cognito access tokens use a ``client_id`` claim instead of the
standard ``aud`` claim.  This subclass passes ``audience=None``
to the parent (skipping the ``aud`` check) and validates the
``client_id`` claim directly.


