# TokenVerifier

Verifying a bearer token. The smallest useful auth surface, and the one the corpus actually extends.

Import as `fastmcp.server.auth.TokenVerifier`
Defined at `fastmcp.server.auth.auth.TokenVerifier`.

```python
class TokenVerifier(AuthProvider)
```

## Required

Nothing. Every member has a default, so a subclass that overrides none of them is valid -- and does nothing useful.

## Provided

Defaulted, and this is where the capability hides. The default is almost always the conservative answer, so an implementation that overrides none of these works correctly and supplies nothing.

```python
async def verify_token(self, token: str) -> AccessToken | None
```

## Implementors (14)

Transitive. Read one before writing your own.

- `fastmcp.server.auth.providers.auth0.Auth0JWTVerifier`
- `fastmcp.server.auth.providers.aws.AWSCognitoTokenVerifier`
- `fastmcp.server.auth.providers.azure.AzureJWTVerifier`
- `fastmcp.server.auth.providers.clerk.ClerkTokenVerifier`
- `fastmcp.server.auth.providers.debug.DebugTokenVerifier`
- `fastmcp.server.auth.providers.descope._DescopeJWTVerifier`
- `fastmcp.server.auth.providers.discord.DiscordTokenVerifier`
- `fastmcp.server.auth.providers.github.GitHubTokenVerifier`
- `fastmcp.server.auth.providers.google.GoogleTokenVerifier`
- `fastmcp.server.auth.providers.huggingface.HuggingFaceTokenVerifier`
- `fastmcp.server.auth.providers.introspection.IntrospectionTokenVerifier`
- `fastmcp.server.auth.providers.jwt.JWTVerifier`
- `fastmcp.server.auth.providers.jwt.StaticTokenVerifier`
- `fastmcp.server.auth.providers.workos.WorkOSTokenVerifier`

## Demonstrated by 8 upstream file(s)

Matched as syntax -- `class X(Base)` -- not by searching for the name.

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
