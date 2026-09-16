# `fastmcp.server.auth.providers.supabase`

Distribution: `fastmcp`

## logger

`fastmcp.server.auth.providers.supabase.logger`

```python
logger = get_logger(__name__)
```

**Inferred type** (`ty`, not declared in the source): `Logger`

## SupabaseProvider

`fastmcp.server.auth.providers.supabase.SupabaseProvider`

```python
class SupabaseProvider(RemoteAuthProvider)
```

**Bases** `RemoteAuthProvider`

**Declared members (4)**

- `auth_route = auth_route.strip('/')`  _instance-attribute_
- `base_url = AnyHttpUrl(str(base_url).rstrip('/'))`  _instance-attribute_
- `def get_routes(self, mcp_path: str | None = None) -> list[Route]`
  Get OAuth routes including Supabase authorization server metadata forwarding.
- `project_url = str(project_url).rstrip('/')`  _instance-attribute_

**Inherited (13)**

- from `fastmcp.server.auth.auth.AuthProvider`: `challenge_scopes`, `get_middleware`, `get_well_known_routes`, `required_scopes`, `resource_base_url`, `set_mcp_path`
- from `fastmcp.server.auth.auth.RemoteAuthProvider`: `authorization_servers`, `get_challenge_scopes`, `resource_documentation`, `resource_name`, `scopes_supported`, `token_verifier`, `verify_token`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

Supabase metadata provider for DCR (Dynamic Client Registration).

This provider implements Supabase Auth integration using metadata forwarding.
This approach allows Supabase to handle the OAuth flow directly while FastMCP acts
as a resource server, verifying JWTs issued by Supabase Auth.

IMPORTANT SETUP REQUIREMENTS:

1. Supabase Project Setup:
   - Create a Supabase project at https://supabase.com
   - Note your project URL (e.g., "https://abc123.supabase.co")
   - Configure your JWT algorithm in Supabase Auth settings (RS256 or ES256)
   - Asymmetric keys (RS256/ES256) are recommended for production

2. JWT Verification:
   - FastMCP verifies JWTs using the JWKS endpoint at {project_url}{auth_route}/.well-known/jwks.json
   - JWTs are issued by {project_url}{auth_route}
   - Default auth_route is "/auth/v1" (can be customized for self-hosted setups)
   - Tokens are cached for up to 10 minutes by Supabase's edge servers
   - Algorithm must match your Supabase Auth configuration

3. Authorization:
   - Supabase uses Row Level Security (RLS) policies for database authorization
   - OAuth-level scopes are an upcoming feature in Supabase Auth
   - Both approaches will be supported once scope handling is available

For detailed setup instructions, see:
https://supabase.com/docs/guides/auth/jwts

Example:
    ```python
    from fastmcp.server.auth.providers.supabase import SupabaseProvider

    # Create Supabase metadata provider (JWT verifier created automatically)
    supabase_auth = SupabaseProvider(
        project_url="https://abc123.supabase.co",
        base_url="https://your-fastmcp-server.com",
        algorithm="ES256",  # Match your Supabase Auth configuration
    )

    # Use with FastMCP
    mcp = FastMCP("My App", auth=supabase_auth)
    ```


