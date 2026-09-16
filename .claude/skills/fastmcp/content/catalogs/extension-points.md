# Extension points

The base classes you subclass, with every in-index descendant. Counts are the
**transitive** closure, which is the only way `FastMCP` shows up as a `Provider` --
it reaches one through `AggregateProvider`, and that is the central fact of the 4.0
composition model.

| Base | Role | Descendants | Import as |
|---|---|---:|---|
| `Provider` | Where components come from | 26 | `fastmcp.server.providers.Provider` |
| `Transform` | Rewriting the component catalog, observably | 11 | `fastmcp.server.server.Transform` |
| `Middleware` | Intercepting requests | 17 | `fastmcp.server.middleware.Middleware` |
| `AuthProvider` | Authenticating callers | 38 | `fastmcp.server.auth.AuthProvider` |
| `ClientTransport` | How a client reaches a server | 11 | `fastmcp.client.ClientTransport` |

Three of these are distinct axes that are easy to conflate:

- **Provider** decides *where components come from*. `mount()` and `create_proxy()`
  are both thin wrappers over it.
- **Transform** rewrites the component catalog and is observable by the system.
- **Middleware** intercepts requests in flight.

Per-base descendant lists are in `content/index/descendants.tsv`; what you must
override is in `content/index/overrides.tsv`.

## Provider

`fastmcp.server.providers.Provider` -- 26 descendants

- `fastmcp.FastMCPApp`
- `fastmcp.apps.approval.Approval`
- `fastmcp.apps.choice.Choice`
- `fastmcp.apps.file_upload.FileUpload`
- `fastmcp.apps.form.FormInput`
- `fastmcp.apps.generative.GenerativeUI`
- `fastmcp.server.providers.AggregateProvider`
- `fastmcp.server.providers.FastMCPProvider`
- `fastmcp.server.providers.FileSystemProvider`
- `fastmcp.server.providers.LocalProvider`
- `fastmcp.server.providers.OpenAPIProvider`
- `fastmcp.server.server.FastMCPProxy`
- `fastmcp.server.providers.ProxyProvider`
- `fastmcp.server.providers.ClaudeSkillsProvider`
- `fastmcp.server.providers.SkillsDirectoryProvider`
- `fastmcp.server.providers.SkillProvider`
- `fastmcp.server.providers.skills.CodexSkillsProvider`
- `fastmcp.server.providers.skills.CopilotSkillsProvider`
- `fastmcp.server.providers.skills.CursorSkillsProvider`
- `fastmcp.server.providers.skills.GeminiSkillsProvider`
- `fastmcp.server.providers.skills.GooseSkillsProvider`
- `fastmcp.server.providers.skills.OpenCodeSkillsProvider`
- `fastmcp.server.providers.skills.VSCodeSkillsProvider`
- `fastmcp.FastMCP`
- `fastmcp.server.sessions.SessionProvider`
- _plus 1 defined behind private modules_

## Transform

`fastmcp.server.server.Transform` -- 11 descendants

- `fastmcp.experimental.transforms.code_mode.CodeMode`
- `fastmcp.experimental.transforms.code_mode.CatalogTransform`
- `fastmcp.server.transforms.Namespace`
- `fastmcp.server.transforms.PromptsAsTools`
- `fastmcp.server.transforms.ResourcesAsTools`
- `fastmcp.server.transforms.search.bm25.BaseSearchTransform`
- `fastmcp.server.transforms.search.BM25SearchTransform`
- `fastmcp.server.transforms.search.RegexSearchTransform`
- `fastmcp.server.transforms.ToolTransform`
- `fastmcp.server.transforms.VersionFilter`
- `fastmcp.server.transforms.Visibility`

## Middleware

`fastmcp.server.middleware.Middleware` -- 17 descendants

- `fastmcp.server.middleware.AuthMiddleware`
- `fastmcp.server.middleware.caching.ResponseCachingMiddleware`
- `fastmcp.server.middleware.dereference.DereferenceRefsMiddleware`
- `fastmcp.server.middleware.error_handling.ErrorHandlingMiddleware`
- `fastmcp.server.middleware.error_handling.RetryMiddleware`
- `fastmcp.server.middleware.logging.BaseLoggingMiddleware`
- `fastmcp.server.middleware.logging.LoggingMiddleware`
- `fastmcp.server.middleware.logging.StructuredLoggingMiddleware`
- `fastmcp.server.middleware.PingMiddleware`
- `fastmcp.server.middleware.rate_limiting.RateLimitingMiddleware`
- `fastmcp.server.middleware.rate_limiting.SlidingWindowRateLimitingMiddleware`
- `fastmcp.server.middleware.response_limiting.ResponseLimitingMiddleware`
- `fastmcp.server.middleware.timing.DetailedTimingMiddleware`
- `fastmcp.server.middleware.timing.TimingMiddleware`
- `fastmcp.server.middleware.tool_injection.ToolInjectionMiddleware`
- `fastmcp.server.providers.proxy.ProxyInitializeMiddleware`
- `fastmcp.server.providers.proxy.ProxyMetadataMiddleware`

## AuthProvider

`fastmcp.server.auth.AuthProvider` -- 38 descendants

- `fastmcp.server.auth.MultiAuth`
- `fastmcp.server.auth.OAuthProvider`
- `fastmcp.server.auth.RemoteAuthProvider`
- `fastmcp.server.auth.TokenVerifier`
- `fastmcp.server.auth.OAuthProxy`
- `fastmcp.server.auth.OIDCProxy`
- `fastmcp.server.auth.providers.auth0.Auth0JWTVerifier`
- `fastmcp.server.auth.providers.auth0.Auth0MCPProvider`
- `fastmcp.server.auth.providers.auth0.Auth0Provider`
- `fastmcp.server.auth.providers.aws.AWSCognitoProvider`
- `fastmcp.server.auth.providers.aws.AWSCognitoTokenVerifier`
- `fastmcp.server.auth.providers.azure.AzureJWTVerifier`
- `fastmcp.server.auth.providers.azure.AzureProvider`
- `fastmcp.server.auth.providers.clerk.ClerkProvider`
- `fastmcp.server.auth.providers.clerk.ClerkTokenVerifier`
- `fastmcp.server.auth.DebugTokenVerifier`
- `fastmcp.server.auth.providers.descope.DescopeProvider`
- `fastmcp.server.auth.providers.discord.DiscordProvider`
- `fastmcp.server.auth.providers.discord.DiscordTokenVerifier`
- `fastmcp.server.auth.providers.github.GitHubProvider`
- `fastmcp.server.auth.providers.github.GitHubTokenVerifier`
- `fastmcp.server.auth.providers.google.GoogleProvider`
- `fastmcp.server.auth.providers.google.GoogleTokenVerifier`
- `fastmcp.server.auth.providers.huggingface.HuggingFaceProvider`
- `fastmcp.server.auth.providers.huggingface.HuggingFaceTokenVerifier`
- `fastmcp.server.auth.providers.in_memory.InMemoryOAuthProvider`
- `fastmcp.server.auth.providers.propelauth.IntrospectionTokenVerifier`
- `fastmcp.server.auth.JWTVerifier`
- `fastmcp.server.auth.StaticTokenVerifier`
- `fastmcp.server.auth.providers.keycloak.KeycloakAuthProvider`
- _plus 1 defined behind private modules_

## ClientTransport

`fastmcp.client.ClientTransport` -- 11 descendants

- `fastmcp.client.client.MCPConfigTransport`
- `fastmcp.client.StreamableHttpTransport`
- `fastmcp.client.FastMCPTransport`
- `fastmcp.client.SSETransport`
- `fastmcp.client.transports.FastMCPStdioTransport`
- `fastmcp.client.NodeStdioTransport`
- `fastmcp.client.NpxStdioTransport`
- `fastmcp.client.PythonStdioTransport`
- `fastmcp.client.StdioTransport`
- `fastmcp.client.UvStdioTransport`
- `fastmcp.client.UvxStdioTransport`

## Referenced how often

From `index/usage.tsv` — how much this library uses each of these itself. A low
count is not a warning: it measures internal use, not what callers need.

| Item | Callers | References |
|---|---:|---:|
| `fastmcp.server.server.FastMCP` | 89 | 152 |
| `fastmcp.server.providers.local_provider.local_provider.LocalProvider` | 41 | 34 |
| `fastmcp.server.providers.base.Provider` | 37 | 42 |
| `fastmcp.server.auth.oauth_proxy.proxy.OAuthProxy` | 27 | 42 |
| `fastmcp.server.auth.providers.jwt.JWTVerifier` | 15 | 37 |
| `fastmcp.server.transforms.visibility.Visibility` | 13 | 11 |
| `fastmcp.server.providers.proxy.ProxyProvider` | 13 | 5 |
| `fastmcp.apps.file_upload.FileUpload` | 12 | 0 |
| `fastmcp.server.providers.filesystem.FileSystemProvider` | 11 | 2 |
| `fastmcp.server.auth.auth.AuthProvider` | 10 | 16 |
| `fastmcp.server.providers.aggregate.AggregateProvider` | 10 | 8 |
| `fastmcp.server.transforms.namespace.Namespace` | 9 | 4 |
