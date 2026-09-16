# `fastmcp.server.auth.providers.oci`

Distribution: `fastmcp`

## logger

`fastmcp.server.auth.providers.oci.logger`

```python
logger = get_logger(__name__)
```

**Inferred type** (`ty`, not declared in the source): `Logger`

## OCIProvider

`fastmcp.server.auth.providers.oci.OCIProvider`

```python
class OCIProvider(OIDCProxy)
```

**Bases** `OIDCProxy`

**Inherited (31)**

- from `fastmcp.server.auth.auth.AuthProvider`: `base_url`, `challenge_scopes`, `get_challenge_scopes`, `get_middleware`, `resource_base_url`
- from `fastmcp.server.auth.auth.OAuthProvider`: `client_registration_options`, `get_well_known_routes`, `issuer_url`, `revocation_options`, `scopes_supported`, `service_documentation_url`, `verify_token`
- from `fastmcp.server.auth.oauth_proxy.proxy.OAuthProxy`: `authorize`, `exchange_authorization_code`, `exchange_identity_assertion`, `exchange_refresh_token`, `get_client`, `get_routes`, `jwt_issuer`, `load_access_token`, `load_authorization_code`, `load_refresh_token`, `register_client`, `revoke_token`, `set_mcp_path`, `token_endpoint_url`, `update_default_scopes`
- from `fastmcp.server.auth.oidc_proxy.OIDCProxy`: `get_oidc_configuration`, `get_token_verifier`, `oidc_config`, `required_scopes`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

An OCI IAM Domain provider implementation for FastMCP.

This provider is a complete OCI integration that's ready to use with
just the configuration URL, client ID, client secret, and base URL.

Example:
    ```python
    from fastmcp import FastMCP
    from fastmcp.server.auth.providers.oci import OCIProvider

    import os

    # Load configuration from environment
    auth = OCIProvider(
        config_url=os.environ.get("OCI_CONFIG_URL"),  # OCI IAM Domain OIDC discovery URL
        client_id=os.environ.get("OCI_CLIENT_ID"),  # Client ID configured for the OCI IAM Domain Integrated Application
        client_secret=os.environ.get("OCI_CLIENT_SECRET"),  # Client secret configured for the OCI IAM Domain Integrated Application
        base_url="http://localhost:8000",
        required_scopes=["openid", "profile", "email"],
        redirect_path="/auth/callback",
    )

    mcp = FastMCP("My Protected Server", auth=auth)
    ```


