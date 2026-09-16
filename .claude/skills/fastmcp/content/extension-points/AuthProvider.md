# AuthProvider

Authenticating callers. Upstream never subclasses this directly -- read `TokenVerifier` or `OAuthProxy` first.

Import as `fastmcp.server.auth.AuthProvider`
Defined at `fastmcp.server.auth.auth.AuthProvider`.

```python
class AuthProvider(TokenVerifierProtocol)
```

## Required

You must write these. Nothing works until you do.

```python
base_url = base_url
challenge_scopes: list[str]
def get_challenge_scopes(self, required_scopes: list[str] | None = None) -> list[str]
def get_middleware(self) -> list
def get_routes(self, mcp_path: str | None = None) -> list[Route]
def get_well_known_routes(self, mcp_path: str | None = None) -> list[Route]
required_scopes = required_scopes or []
resource_base_url = resource_base_url
scopes_supported: list[str]
def set_mcp_path(self, mcp_path: str | None) -> None
async def verify_token(self, token: str) -> AccessToken | None
```

## Implementors (38)

Transitive. Read one before writing your own.

- `fastmcp.server.auth.auth.MultiAuth`
- `fastmcp.server.auth.auth.OAuthProvider`
- `fastmcp.server.auth.auth.RemoteAuthProvider`
- `fastmcp.server.auth.auth.TokenVerifier`
- `fastmcp.server.auth.oauth_proxy.proxy.OAuthProxy`
- `fastmcp.server.auth.oidc_proxy.OIDCProxy`
- `fastmcp.server.auth.providers.auth0.Auth0JWTVerifier`
- `fastmcp.server.auth.providers.auth0.Auth0MCPProvider`
- `fastmcp.server.auth.providers.auth0.Auth0Provider`
- `fastmcp.server.auth.providers.aws.AWSCognitoProvider`
- `fastmcp.server.auth.providers.aws.AWSCognitoTokenVerifier`
- `fastmcp.server.auth.providers.azure.AzureJWTVerifier`
- `fastmcp.server.auth.providers.azure.AzureProvider`
- `fastmcp.server.auth.providers.clerk.ClerkProvider`
- `fastmcp.server.auth.providers.clerk.ClerkTokenVerifier`
- `fastmcp.server.auth.providers.debug.DebugTokenVerifier`
- `fastmcp.server.auth.providers.descope.DescopeProvider`
- `fastmcp.server.auth.providers.descope._DescopeJWTVerifier`
- `fastmcp.server.auth.providers.discord.DiscordProvider`
- `fastmcp.server.auth.providers.discord.DiscordTokenVerifier`
- `fastmcp.server.auth.providers.github.GitHubProvider`
- `fastmcp.server.auth.providers.github.GitHubTokenVerifier`
- `fastmcp.server.auth.providers.google.GoogleProvider`
- `fastmcp.server.auth.providers.google.GoogleTokenVerifier`
- `fastmcp.server.auth.providers.huggingface.HuggingFaceProvider`
- `fastmcp.server.auth.providers.huggingface.HuggingFaceTokenVerifier`
- `fastmcp.server.auth.providers.in_memory.InMemoryOAuthProvider`
- `fastmcp.server.auth.providers.introspection.IntrospectionTokenVerifier`
- `fastmcp.server.auth.providers.jwt.JWTVerifier`
- `fastmcp.server.auth.providers.jwt.StaticTokenVerifier`
- `fastmcp.server.auth.providers.keycloak.KeycloakAuthProvider`
- `fastmcp.server.auth.providers.oci.OCIProvider`
- `fastmcp.server.auth.providers.propelauth.PropelAuthProvider`
- `fastmcp.server.auth.providers.scalekit.ScalekitProvider`
- `fastmcp.server.auth.providers.supabase.SupabaseProvider`
- `fastmcp.server.auth.providers.workos.AuthKitProvider`
- `fastmcp.server.auth.providers.workos.WorkOSProvider`
- `fastmcp.server.auth.providers.workos.WorkOSTokenVerifier`

## Demonstrated by 9 upstream file(s)

Matched as syntax -- `class X(Base)` -- not by searching for the name.

- [`corpus/tests/client/auth/test_oauth_client.py`](../corpus/tests/client/auth/test_oauth_client.py)
- [`corpus/tests/server/auth/oauth_proxy/conftest.py`](../corpus/tests/server/auth/oauth_proxy/conftest.py)
- [`corpus/tests/server/auth/test_auth_provider.py`](../corpus/tests/server/auth/test_auth_provider.py)
- [`corpus/tests/server/auth/test_enhanced_error_responses.py`](../corpus/tests/server/auth/test_enhanced_error_responses.py)
- [`corpus/tests/server/auth/test_issuer_url_identity.py`](../corpus/tests/server/auth/test_issuer_url_identity.py)
- [`corpus/tests/server/auth/test_multi_auth.py`](../corpus/tests/server/auth/test_multi_auth.py)
- [`corpus/tests/server/auth/test_oauth_consent_flow.py`](../corpus/tests/server/auth/test_oauth_consent_flow.py)
- [`corpus/tests/server/auth/test_oauth_consent_page.py`](../corpus/tests/server/auth/test_oauth_consent_page.py)
- [`corpus/tests/server/auth/test_oauth_proxy_redirect_validation.py`](../corpus/tests/server/auth/test_oauth_proxy_redirect_validation.py)

## Documentation

Prose: [`api/fastmcp.server.auth.auth.md`](../api/fastmcp.server.auth.auth.md) · records: [`model/fastmcp.server.auth.auth.json`](../model/fastmcp.server.auth.auth.json)
