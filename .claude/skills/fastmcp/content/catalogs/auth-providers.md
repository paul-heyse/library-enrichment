# Authentication providers

38 classes descend from `fastmcp.server.auth.AuthProvider`. Most wrap one identity
service; the rest are the generic verifiers and the proxy layers they build on.

| Provider | Import as | Summary |
|---|---|---|
| `MultiAuth` | `fastmcp.server.auth.MultiAuth` | Composes an optional auth server with additional token verifiers. |
| `OAuthProvider` | `fastmcp.server.auth.OAuthProvider` | OAuth Authorization Server provider. |
| `RemoteAuthProvider` | `fastmcp.server.auth.RemoteAuthProvider` | Authentication provider for resource servers that verify tokens from known authorization s |
| `TokenVerifier` | `fastmcp.server.auth.TokenVerifier` | Base class for token verifiers (Resource Servers). |
| `OAuthProxy` | `fastmcp.server.auth.OAuthProxy` | OAuth provider that presents a DCR-compliant interface while proxying to non-DCR IDPs. |
| `OIDCProxy` | `fastmcp.server.auth.OIDCProxy` | OAuth provider that wraps OAuthProxy to provide configuration via an OIDC configuration UR |
| `Auth0JWTVerifier` | `fastmcp.server.auth.providers.auth0.Auth0JWTVerifier` | JWT verifier for Auth0 MCP access tokens. |
| `Auth0MCPProvider` | `fastmcp.server.auth.providers.auth0.Auth0MCPProvider` | Auth0 resource server provider for Auth for MCP (DCR/CIMD). |
| `Auth0Provider` | `fastmcp.server.auth.providers.auth0.Auth0Provider` | An Auth0 provider implementation for FastMCP. |
| `AWSCognitoProvider` | `fastmcp.server.auth.providers.aws.AWSCognitoProvider` | Complete AWS Cognito OAuth provider for FastMCP. |
| `AWSCognitoTokenVerifier` | `fastmcp.server.auth.providers.aws.AWSCognitoTokenVerifier` | Token verifier for Cognito access tokens. |
| `AzureJWTVerifier` | `fastmcp.server.auth.providers.azure.AzureJWTVerifier` | JWT verifier pre-configured for Azure AD / Microsoft Entra ID. |
| `AzureProvider` | `fastmcp.server.auth.providers.azure.AzureProvider` | Azure (Microsoft Entra) OAuth provider for FastMCP. |
| `ClerkProvider` | `fastmcp.server.auth.providers.clerk.ClerkProvider` | Complete Clerk OAuth provider for FastMCP. |
| `ClerkTokenVerifier` | `fastmcp.server.auth.providers.clerk.ClerkTokenVerifier` | Token verifier for Clerk OAuth tokens. |
| `DebugTokenVerifier` | `fastmcp.server.auth.DebugTokenVerifier` | Token verifier with custom validation logic. |
| `DescopeProvider` | `fastmcp.server.auth.providers.descope.DescopeProvider` | Descope metadata provider for Dynamic Client Registration (DCR). |
| `DiscordProvider` | `fastmcp.server.auth.providers.discord.DiscordProvider` | Complete Discord OAuth provider for FastMCP. |
| `DiscordTokenVerifier` | `fastmcp.server.auth.providers.discord.DiscordTokenVerifier` | Token verifier for Discord OAuth tokens. |
| `GitHubProvider` | `fastmcp.server.auth.providers.github.GitHubProvider` | Complete GitHub OAuth provider for FastMCP. |
| `GitHubTokenVerifier` | `fastmcp.server.auth.providers.github.GitHubTokenVerifier` | Token verifier for GitHub OAuth tokens. |
| `GoogleProvider` | `fastmcp.server.auth.providers.google.GoogleProvider` | Complete Google OAuth provider for FastMCP. |
| `GoogleTokenVerifier` | `fastmcp.server.auth.providers.google.GoogleTokenVerifier` | Token verifier for Google OAuth tokens. |
| `HuggingFaceProvider` | `fastmcp.server.auth.providers.huggingface.HuggingFaceProvider` | Complete Hugging Face OAuth provider for FastMCP. |
| `HuggingFaceTokenVerifier` | `fastmcp.server.auth.providers.huggingface.HuggingFaceTokenVerifier` | Token verifier for Hugging Face OAuth access tokens. |
| `InMemoryOAuthProvider` | `fastmcp.server.auth.providers.in_memory.InMemoryOAuthProvider` | An in-memory OAuth provider for testing purposes. It simulates the OAuth 2.1 flow locally  |
| `IntrospectionTokenVerifier` | `fastmcp.server.auth.providers.propelauth.IntrospectionTokenVerifier` | OAuth 2.0 Token Introspection verifier (RFC 7662). |
| `JWTVerifier` | `fastmcp.server.auth.JWTVerifier` | JWT token verifier supporting asymmetric (RSA/ECDSA/EdDSA) and symmetric (HMAC) algorithms |
| `StaticTokenVerifier` | `fastmcp.server.auth.StaticTokenVerifier` | Simple static token verifier for testing and development. |
| `KeycloakAuthProvider` | `fastmcp.server.auth.providers.keycloak.KeycloakAuthProvider` | Keycloak authentication provider using Dynamic Client Registration (DCR). |
| `OCIProvider` | `fastmcp.server.auth.providers.oci.OCIProvider` | An OCI IAM Domain provider implementation for FastMCP. |
| `PropelAuthProvider` | `fastmcp.server.auth.providers.propelauth.PropelAuthProvider` | PropelAuth resource server provider using OAuth 2.1 token introspection. |
| `ScalekitProvider` | `fastmcp.server.auth.providers.scalekit.ScalekitProvider` | Scalekit resource server provider for OAuth 2.1 authentication. |
| `SupabaseProvider` | `fastmcp.server.auth.providers.supabase.SupabaseProvider` | Supabase metadata provider for DCR (Dynamic Client Registration). |
| `AuthKitProvider` | `fastmcp.server.auth.providers.workos.AuthKitProvider` | AuthKit metadata provider for DCR (Dynamic Client Registration). |
| `WorkOSProvider` | `fastmcp.server.auth.providers.workos.WorkOSProvider` | Complete WorkOS OAuth provider for FastMCP. |
| `WorkOSTokenVerifier` | `fastmcp.server.auth.providers.workos.WorkOSTokenVerifier` | Token verifier for WorkOS OAuth tokens. |

Client-side authentication is separate: `BearerAuth`, `OAuth`,
`ClientCredentialsOAuthProvider` and `PrivateKeyJWTOAuthProvider` live under
`fastmcp.client.auth`.

## Referenced how often

From `index/usage.tsv` — how much this library uses each of these itself. A low
count is not a warning: it measures internal use, not what callers need.

| Item | Callers | References |
|---|---:|---:|
| `fastmcp.server.auth.oauth_proxy.proxy.OAuthProxy` | 27 | 42 |
| `fastmcp.server.auth.providers.jwt.JWTVerifier` | 15 | 37 |
| `fastmcp.server.auth.auth.AuthProvider` | 10 | 16 |
| `fastmcp.client.auth.oauth.OAuth` | 7 | 13 |
| `fastmcp.server.auth.providers.azure.AzureProvider` | 7 | 4 |
| `fastmcp.server.auth.providers.in_memory.InMemoryOAuthProvider` | 6 | 0 |
| `fastmcp.client.auth.client_credentials.PrivateKeyJWTOAuthProvider` | 5 | 7 |
| `fastmcp.client.auth.client_credentials.ClientCredentialsOAuthProvider` | 5 | 7 |
| `fastmcp.server.auth.auth.TokenVerifier` | 4 | 42 |
| `fastmcp.server.auth.auth.OAuthProvider` | 4 | 6 |
| `fastmcp.server.auth.providers.introspection.IntrospectionTokenVerifier` | 3 | 3 |
| `mcp.client.auth.extensions.client_credentials.PrivateKeyJWTOAuthProvider` | 3 | 2 |
