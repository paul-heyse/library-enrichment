# `fastmcp.server.auth.providers.github`

Distribution: `fastmcp`

## logger

`fastmcp.server.auth.providers.github.logger`

```python
logger = get_logger(__name__)
```

**Inferred type** (`ty`, not declared in the source): `Logger`

## GitHubProvider

`fastmcp.server.auth.providers.github.GitHubProvider`

```python
class GitHubProvider(OAuthProxy)
```

**Bases** `OAuthProxy`

**Inherited (28)**

- from `fastmcp.server.auth.auth.AuthProvider`: `base_url`, `challenge_scopes`, `get_challenge_scopes`, `get_middleware`, `required_scopes`, `resource_base_url`
- from `fastmcp.server.auth.auth.OAuthProvider`: `client_registration_options`, `get_well_known_routes`, `issuer_url`, `revocation_options`, `scopes_supported`, `service_documentation_url`, `verify_token`
- from `fastmcp.server.auth.oauth_proxy.proxy.OAuthProxy`: `authorize`, `exchange_authorization_code`, `exchange_identity_assertion`, `exchange_refresh_token`, `get_client`, `get_routes`, `jwt_issuer`, `load_access_token`, `load_authorization_code`, `load_refresh_token`, `register_client`, `revoke_token`, `set_mcp_path`, `token_endpoint_url`, `update_default_scopes`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

Complete GitHub OAuth provider for FastMCP.

This provider makes it trivial to add GitHub OAuth protection to any
FastMCP server. Just provide your GitHub OAuth app credentials and
a base URL, and you're ready to go.

Features:
- Transparent OAuth proxy to GitHub
- Automatic token validation via GitHub API
- User information extraction
- Minimal configuration required

Example:
    ```python
    from fastmcp import FastMCP
    from fastmcp.server.auth.providers.github import GitHubProvider

    auth = GitHubProvider(
        client_id="Ov23li...",
        client_secret="abc123...",
        base_url="https://my-server.com"
    )

    mcp = FastMCP("My App", auth=auth)
    ```


## GitHubTokenVerifier

`fastmcp.server.auth.providers.github.GitHubTokenVerifier`

```python
class GitHubTokenVerifier(TokenVerifier)
```

**Bases** `TokenVerifier`

**Declared members (2)**

- `timeout_seconds = timeout_seconds`  _instance-attribute_
- `async def verify_token(self, token: str) -> AccessToken | None`  _async_
  Verify GitHub OAuth token by calling GitHub API.

**Inherited (10)**

- from `fastmcp.server.auth.auth.AuthProvider`: `base_url`, `challenge_scopes`, `get_challenge_scopes`, `get_middleware`, `get_routes`, `get_well_known_routes`, `required_scopes`, `resource_base_url`, `scopes_supported`, `set_mcp_path`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

Token verifier for GitHub OAuth tokens.

GitHub OAuth tokens are opaque (not JWTs), so we verify them
by calling GitHub's API to check if they're valid and get user info.

Warning:
    GitHub tokens carry no audience claim, so this verifier cannot tell
    which OAuth app (if any) a token was issued for — any valid GitHub
    credential, including a personal access token, will verify. Used
    inside `GitHubProvider` this is safe, because the proxy only ever
    checks tokens it obtained through its own OAuth flow. As a standalone
    verifier it authenticates "some GitHub user", not "a user of your
    app" — only use it that way if that is genuinely your access model.

Caching is disabled by default.  Set ``cache_ttl_seconds`` to a positive
integer to cache successful verification results and avoid repeated
GitHub API calls for the same token.


