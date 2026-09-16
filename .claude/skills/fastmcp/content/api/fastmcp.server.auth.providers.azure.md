# `fastmcp.server.auth.providers.azure`

Distribution: `fastmcp`

## OIDC_SCOPES

`fastmcp.server.auth.providers.azure.OIDC_SCOPES`

```python
OIDC_SCOPES = frozenset({'openid', 'profile', 'email', 'offline_access'})
```

**Inferred type** (`ty`, not declared in the source): `frozenset[str]`

## logger

`fastmcp.server.auth.providers.azure.logger`

```python
logger = get_logger(__name__)
```

**Inferred type** (`ty`, not declared in the source): `Logger`

## AzureJWTVerifier

`fastmcp.server.auth.providers.azure.AzureJWTVerifier`

```python
class AzureJWTVerifier(JWTVerifier)
```

**Bases** `JWTVerifier`

**Declared members (2)**

- `def get_challenge_scopes(self, required_scopes: list[str] | None = None) -> list[str]`
  Prefix any effective validation scopes for Azure authorization.
- `scopes_supported: list[str]`  _property_
  Return scopes with Azure URI prefix for OAuth metadata.

**Inherited (17)**

- from `fastmcp.server.auth.auth.AuthProvider`: `base_url`, `challenge_scopes`, `get_middleware`, `get_routes`, `get_well_known_routes`, `required_scopes`, `resource_base_url`, `set_mcp_path`
- from `fastmcp.server.auth.providers.jwt.JWTVerifier`: `algorithm`, `audience`, `issuer`, `jwks_uri`, `load_access_token`, `logger`, `public_key`, `ssrf_safe`, `verify_token`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

JWT verifier pre-configured for Azure AD / Microsoft Entra ID.

Auto-configures JWKS URI, issuer, audience, and scope handling from your
Azure app registration details. Designed for Managed Identity and other
token-verification-only scenarios where AzureProvider's full OAuth proxy
isn't needed.

Handles Azure's scope format automatically:
- Validates tokens using short-form scopes (what Azure puts in ``scp`` claims)
- Advertises full-URI scopes in OAuth metadata (what clients need to request)

Example::

    from fastmcp.server.auth import RemoteAuthProvider
    from fastmcp.server.auth.providers.azure import AzureJWTVerifier
    from pydantic import AnyHttpUrl

    verifier = AzureJWTVerifier(
        client_id="your-client-id",
        tenant_id="your-tenant-id",
        required_scopes=["access_as_user"],
    )

    auth = RemoteAuthProvider(
        token_verifier=verifier,
        authorization_servers=[
            AnyHttpUrl("https://login.microsoftonline.com/your-tenant-id/v2.0")
        ],
        base_url="https://my-server.com",
    )


## AzureProvider

`fastmcp.server.auth.providers.azure.AzureProvider`

```python
class AzureProvider(OAuthProxy)
```

**Bases** `OAuthProxy`

**Declared members (6)**

- `additional_authorize_scopes: list[str] = parsed_additional_scopes`  _instance-attribute_
- `async def authorize(self, client: OAuthClientInformationFull, params: AuthorizationParams) -> str`  _async_
  Start OAuth transaction and redirect to Azure AD.
- `async def close_obo_credentials(self) -> None`  _async_
  Close all cached OBO credentials.
- `def from_b2c(cls, tenant_name: str, policy_name: str, client_id: str, client_secret: str | None = None, required_scopes: list[str], base_url: str, custom_domain: str | None = None, identifier_uri: str | None = None, token_issuer: str | None = None, kwargs: Any = {}) -> AzureProvider`  _classmethod_
  Create an AzureProvider pre-configured for Azure AD B2C.
- `async def get_obo_credential(self, user_assertion: str) -> OnBehalfOfCredential`  _async_
  Get a cached or new OnBehalfOfCredential for OBO token exchange.
- `identifier_uri = identifier_uri or f'api://{client_id}'`  _instance-attribute_

**Inherited (27)**

- from `fastmcp.server.auth.auth.AuthProvider`: `base_url`, `challenge_scopes`, `get_challenge_scopes`, `get_middleware`, `required_scopes`, `resource_base_url`
- from `fastmcp.server.auth.auth.OAuthProvider`: `client_registration_options`, `get_well_known_routes`, `issuer_url`, `revocation_options`, `scopes_supported`, `service_documentation_url`, `verify_token`
- from `fastmcp.server.auth.oauth_proxy.proxy.OAuthProxy`: `exchange_authorization_code`, `exchange_identity_assertion`, `exchange_refresh_token`, `get_client`, `get_routes`, `jwt_issuer`, `load_access_token`, `load_authorization_code`, `load_refresh_token`, `register_client`, `revoke_token`, `set_mcp_path`, `token_endpoint_url`, `update_default_scopes`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

Azure (Microsoft Entra) OAuth provider for FastMCP.

This provider implements Azure/Microsoft Entra ID authentication using the
OAuth Proxy pattern. It supports both organizational accounts and personal
Microsoft accounts depending on the tenant configuration.

Scope Handling:
- required_scopes: Provide unprefixed scope names (e.g., ["read", "write"])
  → Automatically prefixed with identifier_uri during initialization
  → Validated on all tokens and advertised to MCP clients
- additional_authorize_scopes: Provide full format (e.g., ["User.Read"])
  → NOT prefixed, NOT validated, NOT advertised to clients
  → Used to request Microsoft Graph or other upstream API permissions

Features:
- OAuth proxy to Azure/Microsoft identity platform
- JWT validation using tenant issuer and JWKS
- Supports tenant configurations: specific tenant ID, "organizations", or "consumers"
- Custom API scopes and Microsoft Graph scopes in a single provider

Setup:
1. Create an App registration in Azure Portal
2. Configure Web platform redirect URI: http://localhost:8000/auth/callback (or your custom path)
3. Add an Application ID URI under "Expose an API" (defaults to api://{client_id})
4. Add custom scopes (e.g., "read", "write") under "Expose an API"
5. Set access token version to 2 in the App manifest: "requestedAccessTokenVersion": 2
6. Create a client secret
7. Get Application (client) ID, Directory (tenant) ID, and client secret

Example:
    ```python
    from fastmcp import FastMCP
    from fastmcp.server.auth.providers.azure import AzureProvider

    # Standard Azure (Public Cloud)
    auth = AzureProvider(
        client_id="your-client-id",
        client_secret="your-client-secret",
        tenant_id="your-tenant-id",
        required_scopes=["read", "write"],  # Unprefixed scope names
        additional_authorize_scopes=["User.Read", "Mail.Read"],  # Optional Graph scopes
        base_url="http://localhost:8000",
        # identifier_uri defaults to api://{client_id}
    )

    # Azure Government
    auth_gov = AzureProvider(
        client_id="your-client-id",
        client_secret="your-client-secret",
        tenant_id="your-tenant-id",
        required_scopes=["read", "write"],
        base_authority="login.microsoftonline.us",  # Override for Azure Gov
        base_url="http://localhost:8000",
    )

    mcp = FastMCP("My App", auth=auth)
    ```


## _EntraOBOToken

`fastmcp.server.auth.providers.azure._EntraOBOToken`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class _EntraOBOToken(Dependency[str])
```

**Bases** `Dependency[str]`

**Declared members (1)**

- `scopes = scopes`  _instance-attribute_

Dependency that performs OBO token exchange for Microsoft Entra.

Uses azure.identity's OnBehalfOfCredential for async-native OBO,
with automatic token caching and refresh. Credentials are cached on
the AzureProvider so repeated tool calls reuse existing credentials
and benefit from the Azure SDK's internal token cache.


## EntraOBOToken

`fastmcp.server.auth.providers.azure.EntraOBOToken`

```python
def EntraOBOToken(scopes: list[str]) -> str
```

Exchange the user's Entra token for a downstream API token via OBO.

This dependency performs a Microsoft Entra On-Behalf-Of (OBO) token exchange,
allowing your MCP server to call downstream APIs (like Microsoft Graph) on
behalf of the authenticated user.

Args:
    scopes: The scopes to request for the downstream API. For Microsoft Graph,
        use scopes like ["https://graph.microsoft.com/Mail.Read"] or
        ["https://graph.microsoft.com/.default"].

Returns:
    A dependency that resolves to the downstream API access token string

Raises:
    ImportError: If fastmcp[azure] is not installed
    RuntimeError: If no access token is available, provider is not Azure,
        or OBO exchange fails

Example:
    ```python
    from fastmcp.server.auth.providers.azure import EntraOBOToken
    import httpx2

    @mcp.tool()
    async def get_my_emails(
        graph_token: str = EntraOBOToken(["https://graph.microsoft.com/Mail.Read"])
    ):
        async with httpx2.AsyncClient() as client:
            resp = await client.get(
                "https://graph.microsoft.com/v1.0/me/messages",
                headers={"Authorization": f"Bearer {graph_token}"}
            )
            return resp.json()
    ```

Note:
    For OBO to work, ensure the scopes are included in the AzureProvider's
    `additional_authorize_scopes` parameter, and that admin consent has been
    granted for those scopes in your Entra app registration.


## _find_azure_provider

`fastmcp.server.auth.providers.azure._find_azure_provider`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _find_azure_provider(auth: AuthProvider | None) -> AzureProvider | None
```

Extract an AzureProvider from an auth provider, unwrapping MultiAuth if needed.


## _require_azure_identity

`fastmcp.server.auth.providers.azure._require_azure_identity`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _require_azure_identity(feature: str) -> None
```

Raise ImportError with install instructions if azure-identity is not available.


