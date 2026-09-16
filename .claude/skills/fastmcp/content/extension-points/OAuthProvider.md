# OAuthProvider

A full OAuth authorization server, rather than verification alone.

Import as `fastmcp.server.auth.OAuthProvider`
Defined at `fastmcp.server.auth.auth.OAuthProvider`.

```python
class OAuthProvider(AuthProvider, OAuthAuthorizationServerProvider[AuthorizationCode, RefreshToken, AccessToken])
```

## Required

Nothing. Every member has a default, so a subclass that overrides none of them is valid -- and does nothing useful.

## Provided

Defaulted, and this is where the capability hides. The default is almost always the conservative answer, so an implementation that overrides none of these works correctly and supplies nothing.

```python
client_registration_options = client_registration_options
def get_routes(self, mcp_path: str | None = None) -> list[Route]
def get_well_known_routes(self, mcp_path: str | None = None) -> list[Route]
issuer_url = self.base_url
revocation_options = revocation_options
scopes_supported: list[str]
service_documentation_url = service_documentation_url
async def verify_token(self, token: str) -> AccessToken | None
```

## Implementors (13)

Transitive. Read one before writing your own.

- `fastmcp.server.auth.oauth_proxy.proxy.OAuthProxy`
- `fastmcp.server.auth.oidc_proxy.OIDCProxy`
- `fastmcp.server.auth.providers.auth0.Auth0Provider`
- `fastmcp.server.auth.providers.aws.AWSCognitoProvider`
- `fastmcp.server.auth.providers.azure.AzureProvider`
- `fastmcp.server.auth.providers.clerk.ClerkProvider`
- `fastmcp.server.auth.providers.discord.DiscordProvider`
- `fastmcp.server.auth.providers.github.GitHubProvider`
- `fastmcp.server.auth.providers.google.GoogleProvider`
- `fastmcp.server.auth.providers.huggingface.HuggingFaceProvider`
- `fastmcp.server.auth.providers.in_memory.InMemoryOAuthProvider`
- `fastmcp.server.auth.providers.oci.OCIProvider`
- `fastmcp.server.auth.providers.workos.WorkOSProvider`

## Demonstrated by 2 upstream file(s)

Matched as syntax -- `class X(Base)` -- not by searching for the name.

- [`corpus/tests/client/auth/test_oauth_client.py`](../corpus/tests/client/auth/test_oauth_client.py)
- [`corpus/tests/server/auth/test_enhanced_error_responses.py`](../corpus/tests/server/auth/test_enhanced_error_responses.py)

## Documentation

Prose: [`api/fastmcp.server.auth.auth.md`](../api/fastmcp.server.auth.auth.md) · records: [`model/fastmcp.server.auth.auth.json`](../model/fastmcp.server.auth.auth.json)
