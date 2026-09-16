# `fastmcp.server.auth.providers.keycloak`

Distribution: `fastmcp`

## logger

`fastmcp.server.auth.providers.keycloak.logger`

```python
logger = get_logger(__name__)
```

**Inferred type** (`ty`, not declared in the source): `Logger`

## KeycloakAuthProvider

`fastmcp.server.auth.providers.keycloak.KeycloakAuthProvider`

```python
class KeycloakAuthProvider(RemoteAuthProvider)
```

**Bases** `RemoteAuthProvider`

**Declared members (1)**

- `realm_url = str(realm_url).rstrip('/')`  _instance-attribute_

**Inherited (15)**

- from `fastmcp.server.auth.auth.AuthProvider`: `challenge_scopes`, `get_middleware`, `get_well_known_routes`, `required_scopes`, `resource_base_url`, `set_mcp_path`
- from `fastmcp.server.auth.auth.RemoteAuthProvider`: `authorization_servers`, `base_url`, `get_challenge_scopes`, `get_routes`, `resource_documentation`, `resource_name`, `scopes_supported`, `token_verifier`, `verify_token`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

Keycloak authentication provider using Dynamic Client Registration (DCR).

Requires Keycloak 26.6.0 or later, which includes the fix for DCR compatibility
with MCP clients (https://github.com/keycloak/keycloak/pull/45309).

Example:
    ```python
    from fastmcp import FastMCP
    from fastmcp.server.auth.providers.keycloak import KeycloakAuthProvider

    auth = KeycloakAuthProvider(
        realm_url="https://keycloak.example.com/realms/myrealm",
        base_url="https://my-mcp-server.example.com",
    )

    mcp = FastMCP("My App", auth=auth)
    ```


