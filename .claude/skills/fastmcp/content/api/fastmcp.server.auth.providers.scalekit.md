# `fastmcp.server.auth.providers.scalekit`

Distribution: `fastmcp`

## logger

`fastmcp.server.auth.providers.scalekit.logger`

```python
logger = get_logger(__name__)
```

**Inferred type** (`ty`, not declared in the source): `Logger`

## ScalekitProvider

`fastmcp.server.auth.providers.scalekit.ScalekitProvider`

```python
class ScalekitProvider(RemoteAuthProvider)
```

**Bases** `RemoteAuthProvider`

**Declared members (4)**

- `environment_url = str(environment_url).rstrip('/')`  _instance-attribute_
- `def get_routes(self, mcp_path: str | None = None) -> list[Route]`
  Get OAuth routes including Scalekit authorization server metadata forwarding.
- `required_scopes = parsed_scopes`  _instance-attribute_
- `resource_id = resource_id`  _instance-attribute_

**Inherited (13)**

- from `fastmcp.server.auth.auth.AuthProvider`: `challenge_scopes`, `get_middleware`, `get_well_known_routes`, `resource_base_url`, `set_mcp_path`
- from `fastmcp.server.auth.auth.RemoteAuthProvider`: `authorization_servers`, `base_url`, `get_challenge_scopes`, `resource_documentation`, `resource_name`, `scopes_supported`, `token_verifier`, `verify_token`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

Scalekit resource server provider for OAuth 2.1 authentication.

This provider implements Scalekit integration using resource server pattern.
FastMCP acts as a protected resource server that validates access tokens issued
by Scalekit's authorization server.

IMPORTANT SETUP REQUIREMENTS:

1. Create an MCP Server in Scalekit Dashboard:
   - Go to your [Scalekit Dashboard](https://app.scalekit.com/)
   - Navigate to MCP Servers section
   - Register a new MCP Server with appropriate scopes
   - Ensure the Resource Identifier matches exactly what you configure as MCP URL
   - Note the Resource ID

2. Environment Configuration:
   - Set SCALEKIT_ENVIRONMENT_URL (e.g., https://your-env.scalekit.com)
   - Set SCALEKIT_RESOURCE_ID from your created resource
   - Set BASE_URL to your FastMCP server's public URL

For detailed setup instructions, see:
https://docs.scalekit.com/mcp/overview/

Example:
    ```python
    from fastmcp.server.auth.providers.scalekit import ScalekitProvider

    # Create Scalekit resource server provider
    scalekit_auth = ScalekitProvider(
        environment_url="https://your-env.scalekit.com",
        resource_id="sk_resource_...",
        base_url="https://your-fastmcp-server.com",
    )

    # Use with FastMCP
    mcp = FastMCP("My App", auth=scalekit_auth)
    ```


